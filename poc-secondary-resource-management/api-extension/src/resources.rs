use anyhow::Context;
use axum::{Json, extract::Path};
use kube::{
    Api, Client, Discovery, ResourceExt,
    api::DynamicObject,
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
    Path((namespace, name)): Path<(String, String)>,
) -> Result<Json<Value>> {
    let mut prim_res = MockResource {
        name,
        sec_res: vec![],
    };

    // Source: https://github.com/kube-rs/kube/blob/d171d2620e8ad82235230fe589bbea7c9306963d/examples/dynamic_api.rs
    let client = Client::try_default()
        .await
        .context("Client Creation Error")?;
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

            let list = api
                .list(&Default::default())
                .await
                .with_context(|| format!("No secondary resource of kind \"{}\" found", ar.kind))?;

            prim_res.sec_res.extend(list);
        }
    }

    Ok(Json(serde_json::json!({
        "apiVersion": MockResource::api_version(),
        "kind": MockResource::kind(),
        "name": prim_res.name,
        "sec_res": prim_res.sec_res.iter().map(|obj| obj.name_any()).collect::<Vec<String>>(),
    })))
}
