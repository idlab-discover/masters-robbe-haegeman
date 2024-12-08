use std::sync::Arc;

use crate::crd::{self};
use crate::error::Error;
use futures::StreamExt;
use k8s_openapi::api::batch;
use kube::runtime::{watcher, Controller};
use kube::Client;
use kube::{runtime::controller::Action, Api};
use std::time::Duration;

#[derive(Clone)]
pub struct Context {
    /// Kubernetes client
    pub client: Client,
}

pub async fn run(ctx: Arc<Context>) {
    let client = ctx.client.clone();
    let api: Api<crd::Database> = Api::all(client.clone());

    Controller::new(api.clone(), watcher::Config::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, ctx)
        .filter_map(|x| async move { std::result::Result::ok(x) })
        .for_each(|_| futures::future::ready(()))
        .await;
}

pub async fn reconcile(g: Arc<crd::Database>, ctx: Arc<Context>) -> Result<Action, Error> {
    let client = &ctx.client;

    let ns = g
        .as_ref()
        .metadata
        .namespace
        .clone()
        .unwrap_or("default".to_string());

    Ok(Action::await_change())
}

pub fn error_policy(_obj: Arc<crd::Database>, _error: &Error, _ctx: Arc<Context>) -> Action {
    Action::requeue(Duration::from_secs(5))
}
