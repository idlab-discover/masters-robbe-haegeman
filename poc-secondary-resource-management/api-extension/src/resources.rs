use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use k8s_openapi::api::core;
use kube::{Api, Client, ResourceExt};

use crate::{API_VERSION, GROUP};

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
) -> impl IntoResponse {
    let client = Client::try_default().await.expect("Client Creation Error");

    let mut prim_res = MockResource {
        name,
        sec_res: vec![],
    };

    match Api::<core::v1::Pod>::namespaced(client, &namespace)
        .get(&prim_res.name)
        .await
    {
        Ok(pod) => {
            prim_res.sec_res.push(pod.name_any());
            Json(serde_json::json!({
                "apiVersion": MockResource::api_version(),
                "kind": MockResource::kind(),
                "sec_res": prim_res.sec_res,
            }))
            .into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            format!(
                "The resource {} in ns: {} does not exist",
                prim_res.name, namespace
            ),
        )
            .into_response(),
    }
}
