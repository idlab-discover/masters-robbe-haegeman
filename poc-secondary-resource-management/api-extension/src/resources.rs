use axum::{Json, extract::Path, response::IntoResponse};
use k8s_openapi::api::core;
use kube::{Api, Client};

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

    let prim_res = MockResource {
        name,
        sec_res: vec![],
    };

    Json(serde_json::json!({
        "apiVersion": MockResource::api_version(),
        "kind": MockResource::kind(),
        "sec_res": prim_res.sec_res,
    }))
}
