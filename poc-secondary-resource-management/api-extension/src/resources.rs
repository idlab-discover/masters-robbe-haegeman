use anyhow::Context;
use axum::{Json, extract::Path};
use k8s_openapi::api::apps;
use kube::{Api, Client, ResourceExt};
use serde_json::Value;

use crate::{API_VERSION, GROUP, error::Result};

pub(crate) struct MockResource {
    name: String,
    sec_res: Vec<String>,
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
    let client = Client::try_default().await.expect("Client Creation Error");

    let mut prim_res = MockResource {
        name,
        sec_res: vec![],
    };

    let deploy = Api::<apps::v1::Deployment>::namespaced(client, &namespace)
        .get(&prim_res.name)
        .await
        .with_context(|| {
            format!(
                "The resource \"{}\" in ns \"{}\" does not exist",
                prim_res.name, namespace
            )
        })?;

    prim_res.sec_res.push(deploy.name_any());
    Ok(Json(serde_json::json!({
        "apiVersion": MockResource::api_version(),
        "kind": MockResource::kind(),
        "sec_res": prim_res.sec_res,
    })))
}
