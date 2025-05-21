use crate::telemetry;
use anyhow::{Context, Error};
use axum::{Json, extract::Path};
use kube::{
    Api, Client, Discovery,
    api::{DynamicObject, GroupVersionKind, ListParams},
    discovery::{Scope, verbs},
};
use serde::Serialize;
use tracing::{Span, field, info, instrument};

use crate::error::Result;

#[derive(Debug, Serialize)]
pub(crate) struct PrimaryWithSecondariesResponse {
    prim_res: DynamicObject,
    sec_res: Vec<DynamicObject>,
}

#[instrument(skip(group, version, kind, namespace, name), fields(trace_id))]
pub(crate) async fn get_primary_with_secondaries(
    Path((mut group, version, kind, namespace, name)): Path<(
        String,
        String,
        String,
        String,
        String,
    )>,
) -> Result<Json<PrimaryWithSecondariesResponse>> {
    let trace_id = telemetry::get_trace_id();
    if trace_id != opentelemetry::trace::TraceId::INVALID {
        Span::current().record("trace_id", field::display(&trace_id));
    }

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

    Ok(Json(PrimaryWithSecondariesResponse { prim_res, sec_res }))
}
