use case::Case;
use case::timed_assert_ok;
use clap::Parser;
use cli::Args;
use crd::Database;
use crd::apply_database_crd;
use dummy::create_dummy_resources_by_count;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::core::v1::ConfigMap;
use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::api::core::v1::Service;
use kube::Error;
use kube::{
    Api, Client, ResourceExt,
    api::{DeleteParams, ListParams, ObjectMeta, PostParams},
};
use kube_primary::PrimaryResourceExt;
use tokio::time::{Duration, sleep};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

mod case;
mod cli;
mod crd;
mod dummy;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::ERROR.into())
                .from_env_lossy(),
        )
        .init();

    let args = Args::parse();

    let client = Client::try_default().await.unwrap();

    apply_database_crd(client.clone()).await;

    let db = Database {
        metadata: ObjectMeta {
            name: Some(String::from("test")),
            namespace: Some(args.namespace.clone()),
            ..Default::default()
        },
        ..Default::default()
    };

    // We assume that the previous Database resource is removed
    // Since all secondaries have OwnerReferences, these will be removed as well
    let db_api: Api<Database> = Api::namespaced(client.clone(), &args.namespace);
    let mut db = db_api.create(&PostParams::default(), &db).await.unwrap();

    create_dummy_resources_by_count(
        client.clone(),
        &mut db,
        args.resource_count,
        args.kind_count as usize,
        args.namespace.clone(),
    )
    .await;

    sleep(Duration::from_secs(args.delay_after_creation)).await;

    let mut case = Case::new(args.resource_count, args.kind_count as usize);

    for _ in 0..args.iterations {
        timed_assert_ok(
            &mut case.duration_get_latest,
            db.get_latest_with_secondaries(client.clone()),
        )
        .await;

        sleep(Duration::from_secs(args.delay_after_request)).await;

        let mut secrets = None;
        let mut pods = None;
        let mut services = None;
        let mut configmaps = None;
        let mut deployments = None;

        timed_assert_ok::<_, _, Error>(&mut case.duration_direct, async {
            let db_api: Api<Database> = Api::namespaced(client.clone(), &args.namespace);
            let db = db_api.get(&db.name_any()).await?;

            if args.kind_count >= 1 {
                let secret_api: Api<Secret> = Api::namespaced(client.clone(), &args.namespace);
                secrets = Some(secret_api.list(&ListParams::default()).await?);
            }
            if args.kind_count >= 2 {
                let pod_api: Api<Pod> = Api::namespaced(client.clone(), &args.namespace);
                pods = Some(pod_api.list(&ListParams::default()).await?);
            }
            if args.kind_count >= 3 {
                let service_api: Api<Service> = Api::namespaced(client.clone(), &args.namespace);
                services = Some(service_api.list(&ListParams::default()).await?);
            }
            if args.kind_count >= 4 {
                let configmap_api: Api<ConfigMap> =
                    Api::namespaced(client.clone(), &args.namespace);
                configmaps = Some(configmap_api.list(&ListParams::default()).await?);
            }
            if args.kind_count >= 5 {
                let deployment_api: Api<Deployment> =
                    Api::namespaced(client.clone(), &args.namespace);
                deployments = Some(deployment_api.list(&ListParams::default()).await?);
            }

            Ok((db, secrets, pods, services, configmaps, deployments))
        })
        .await;

        sleep(Duration::from_secs(args.delay_after_request)).await;
    }

    case.write_to_file(&args.file_path, !args.overwrite)
        .unwrap();

    if !args.keep_resources {
        db_api
            .delete(&db.name_any(), &DeleteParams::default())
            .await
            .unwrap();
    }
}
