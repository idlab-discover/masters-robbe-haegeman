use std::sync::Arc;
use std::time::Duration;

use crate::crd;
use crate::reconcile_sec_res;
use futures::StreamExt;
use k8s_openapi::api::apps;
use kube::api::DeleteParams;
use kube::core::object::HasSpec;
use kube::runtime::{Controller, watcher};
use kube::{Api, runtime::controller::Action};
use kube::{Client, ResourceExt};
use lib::PrimaryResource;
use lib::error::{Error, Result};

const LABEL_KUBERNETES_REPLSET: &str = "app.kubernetes.io/replset";
const LABEL_KUBERNETES_COMPONENT: &str = "app.kubernetes.io/component";

#[derive(Clone)]
pub struct Context {
    /// Kubernetes client
    pub client: Client,
}

pub async fn run(ctx: Arc<Context>) {
    let client = ctx.client.clone();
    let api: Api<crd::Database> = Api::all(client.clone());

    crd::Database::setup_watches(
        Controller::new(api, watcher::Config::default()),
        client.clone(),
        None,
    )
    .shutdown_on_signal()
    .run(reconcile, error_policy, ctx)
    .filter_map(|x| async move { std::result::Result::ok(x) })
    .for_each(|_| futures::future::ready(()))
    .await;
}

pub async fn reconcile(g: Arc<crd::Database>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;

    let ns = g
        .as_ref()
        .metadata
        .namespace
        .clone()
        .unwrap_or(String::from("default"));

    let rr = Ok(Action::requeue(Duration::from_secs(5)));

    // Fetch the Database instance
    let db_api: Api<crd::Database> = Api::namespaced(client.clone(), &ns);
    let mut db = db_api.get(&g.name_any()).await.map_err(Error::KubeError)?;

    // We don't have to have explicit error handling, since the default error_policy can be set to the same value as is used in the Percona operator
    // The functions below are the ones used in the Reconcile function of the ReconcilePerconaServerMongoDB object

    reconcile_sec_res::set_cr_version(&db, ctx.clone()).await?;

    if reconcile_sec_res::check_n_set_defaults(&db, ctx.clone())
        .await
        .is_err()
        && db.metadata.deletion_timestamp.is_some()
    {
        // Original function retries here
        return Ok(Action::await_change());
    };

    // We will currently not handle this case (PoC)
    if db.metadata.deletion_timestamp.is_some()
        && reconcile_sec_res::check_finalizers(&db, ctx.clone())
            .await
            .is_err()
    {
        return rr;
    }

    reconcile_sec_res::reconcile_pause(&db, ctx.clone()).await?;
    reconcile_sec_res::check_configuration(&db, ctx.clone()).await?;
    let is_downscale = reconcile_sec_res::safe_downscale(&db, ctx.clone()).await?;
    reconcile_sec_res::reconcile_user_secret(&mut db, ctx.clone()).await?;

    let mut repls = Vec::<reconcile_sec_res::ReplsetSpec>::new();
    reconcile_sec_res::reconcile_db_daemon_config_maps(&db, ctx.clone(), &mut repls).await?;
    reconcile_sec_res::reconcile_db_config_maps(&db, ctx.clone()).await?;
    reconcile_sec_res::reconcile_users(&db, ctx.clone(), &mut repls).await?;
    let removed = reconcile_sec_res::get_sts_for_removal(&db, ctx.clone()).await?;

    // For this Poc, I will currently refrain from deleting items, so this code will never trigger
    for sts in removed {
        let rs_name = sts
            .labels()
            .get(LABEL_KUBERNETES_REPLSET)
            .ok_or(Error::MissingLabelError(sts.name_any()))?;
        reconcile_sec_res::check_if_possible_to_remove(&db, ctx.clone(), rs_name).await?;

        if sts
            .labels()
            .get(LABEL_KUBERNETES_COMPONENT)
            .ok_or(Error::MissingLabelError(sts.name_any()))?
            == "database_deamon"
        {
            reconcile_sec_res::remove_rs_from_shard(&db, ctx.clone(), rs_name).await?;
        };

        // This is quite a bit worse of a dev exp than the Go: `r.client.Delete(ctx, &sts)`
        let sts_api: Api<apps::v1::StatefulSet> = Api::namespaced(client.clone(), &ns);
        let delete_params = DeleteParams::default();
        sts_api.delete(&sts.name_any(), &delete_params).await?;
    }

    reconcile_sec_res::reconcile_ssl(&db, ctx.clone()).await?;
    // This function actually gets called twice with different parameters, but this is just a dummy
    reconcile_sec_res::ensure_security_key(&db, ctx.clone()).await?;

    if db.spec().backup_enabled {
        reconcile_sec_res::reconcile_backup_tasks(&db, ctx.clone()).await?;
    }

    reconcile_sec_res::reconcile_backup_tasks(&db, ctx.clone()).await?;
    reconcile_sec_res::reconcile_repl_sets(&db, ctx.clone(), &mut repls).await?;
    reconcile_sec_res::reconcile_db_query_router(&db, ctx.clone()).await?;
    reconcile_sec_res::upgrade_fcv_if_needed(&db, ctx.clone()).await?;

    // This value will always be false, but is just to show the interactions in the original code
    if is_downscale {
        reconcile_sec_res::delete_orphan_pvcs(&db, ctx.clone()).await?;
    }

    reconcile_sec_res::reconcile_custom_users(&db, ctx.clone()).await?;
    reconcile_sec_res::export_services(&db, ctx.clone()).await?;
    reconcile_sec_res::schedule_ensure_function(&db, ctx.clone()).await?;
    reconcile_sec_res::update_pitr(&db, ctx.clone()).await?;
    reconcile_sec_res::resync_backup_solution_if_needed(&db, ctx.clone()).await?;

    if let Err(err) = reconcile_sec_res::update_status(&db, ctx.clone()).await {
        log::error!("Failed to update cluster status. Err: {err}");
    };

    // rr
    Ok(Action::await_change())
}

pub fn error_policy(_obj: Arc<crd::Database>, error: &Error, _ctx: Arc<Context>) -> Action {
    log::error!("Error occurred: {error:?}");
    Action::await_change()
}
