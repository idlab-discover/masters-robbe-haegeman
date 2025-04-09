use anyhow::{Context, Error};
use axum::{Json, extract::Path};
use kube::{
    Api, Client, Discovery, ResourceExt,
    api::{DynamicObject, GroupVersionKind, ListParams},
    discovery::{Scope, verbs},
};
use serde_json::Value;
use tracing::info;

use crate::{API_VERSION, GROUP, error::Result};

pub(crate) struct MockResource {
    name: String,
    sec_res: Vec<DynamicObject>,
}

impl MockResource {
    pub(crate) fn group() -> &'static str {
        GROUP
    }

    pub(crate) fn kind() -> &'static str {
        "Test"
    }

    pub(crate) fn plural() -> &'static str {
        "tests"
    }

    pub(crate) fn api_version() -> &'static str {
        API_VERSION
    }
}

pub(crate) async fn get_primary_resource(
    Path((mut group, version, kind, namespace, name)): Path<(
        String,
        String,
        String,
        String,
        String,
    )>,
) -> Result<Json<Value>> {
    let client = Client::try_default()
        .await
        .context("Client Creation Error")?;

    // Source: https://github.com/kube-rs/kube/blob/d171d2620e8ad82235230fe589bbea7c9306963d/examples/dynamic_watcher.rs
    if group == "core" {
        group.clear();
    }
    let gvk = GroupVersionKind {
        group,
        version,
        kind,
    };
    let (ar, _caps) = kube::discovery::pinned_kind(&client, &gvk)
        .await
        .with_context(|| format!("Failed to discover GVK {:?}", gvk))?;
    let api: Api<DynamicObject> = Api::namespaced_with(client.clone(), &namespace, &ar);

    let prim_res = api.get(&name).await.with_context(|| {
        format!(
            "Failed to get resource '{}' in namespace '{}' for GVK {:?}",
            name, namespace, gvk
        )
    })?;

    let label = prim_res
        .metadata
        .uid
        .as_ref()
        .ok_or_else(|| Error::msg("UID not found"))?;

    let mut sec_res = vec![];

    // Source: https://github.com/kube-rs/kube/blob/d171d2620e8ad82235230fe589bbea7c9306963d/examples/dynamic_api.rs
    let discovery = Discovery::new(client.clone())
        .run()
        .await
        .context("Discovery Creation Error")?;

    for group in discovery.groups() {
        for (ar, caps) in group.recommended_resources() {
            if !caps.supports_operation(verbs::LIST) || caps.scope != Scope::Namespaced {
                continue;
            }
            let api: Api<DynamicObject> = Api::namespaced_with(client.clone(), &namespace, &ar);

            info!(
                "({}) {}/{} : {}",
                namespace,
                group.name(),
                ar.version,
                ar.kind
            );

            let lp = ListParams::default().labels(&format!("primary_resource_label={}", label));

            let list = api
                .list(&lp)
                .await
                .with_context(|| format!("No secondary resource of kind \"{}\" found", ar.kind))?;

            sec_res.extend(list);
        }
    }

    Ok(Json(serde_json::json!({
        "prim_res": prim_res,
        "sec_res": sec_res.iter().map(|obj| obj.data.to_string()).collect::<Vec<String>>(),
    })))
}
