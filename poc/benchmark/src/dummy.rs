use std::collections::BTreeMap;

use k8s_openapi::{
    ByteString, Metadata,
    api::{
        apps::v1::{Deployment, DeploymentSpec, DeploymentStrategy},
        core::v1::{
            ConfigMap, Container, Pod, PodSpec, PodTemplateSpec, Secret, Service, ServicePort,
            ServiceSpec,
        },
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::{
    Client,
    api::{ObjectMeta, PostParams},
};
use kube_primary::PrimaryResourceExt;

use crate::crd::Database;

/// Creates dummy resources within the cluster
/// If resource_count is not devisable by kind_count, it just rounds to the nearest int
pub async fn create_dummy_resources(
    client: Client,
    db: &mut Database,
    resource_count: usize,
    kind_count: usize,
    namespace: String,
) {
    let resource_count_per_kind = resource_count / kind_count;
    if resource_count_per_kind == 0 {
        return;
    }

    if kind_count > 0 {
        create_dummy_secrets(
            client.clone(),
            db,
            resource_count_per_kind,
            namespace.clone(),
        )
        .await
        .unwrap();
    }

    if kind_count > 1 {
        create_dummy_pods(
            client.clone(),
            db,
            resource_count_per_kind,
            namespace.clone(),
        )
        .await
        .unwrap();
    }

    if kind_count > 2 {
        create_dummy_services(
            client.clone(),
            db,
            resource_count_per_kind,
            namespace.clone(),
        )
        .await
        .unwrap();
    }

    if kind_count > 3 {
        create_dummy_configmaps(
            client.clone(),
            db,
            resource_count_per_kind,
            namespace.clone(),
        )
        .await
        .unwrap();
    }

    if kind_count > 4 {
        create_dummy_deployments(
            client.clone(),
            db,
            resource_count_per_kind,
            namespace.clone(),
        )
        .await
        .unwrap();
    }
}

async fn create_dummy_secrets(
    client: Client,
    db: &mut Database,
    amount: usize,
    namespace: String,
) -> Result<(), kube_primary::error::Error> {
    let data = Secret {
        metadata: ObjectMeta {
            name: Some(String::from("test-secret")),
            namespace: Some(namespace),
            ..Default::default()
        },
        data: {
            let mut data_map = BTreeMap::new();
            data_map.insert(
                String::from("test_key"),
                ByteString("test_value".as_bytes().to_vec()),
            );
            Some(data_map)
        },
        ..Default::default()
    };
    for i in 0..amount {
        let mut data = data.clone();
        data.metadata_mut().name = Some(format!("test-secret-{i}"));
        db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
            .await?;
    }

    Ok(())
}

async fn create_dummy_pods(
    client: Client,
    db: &mut Database,
    amount: usize,
    namespace: String,
) -> Result<(), kube_primary::error::Error> {
    let data = Pod {
        metadata: ObjectMeta {
            name: Some(String::from("test-pod")),
            namespace: Some(namespace),
            ..Default::default()
        },
        spec: Some(PodSpec {
            containers: vec![Container {
                name: String::from("test-container"),
                image: Some(String::from("nginx:latest")),
                ..Default::default()
            }],
            ..Default::default()
        }),
        ..Default::default()
    };
    for i in 0..amount {
        let mut data = data.clone();
        data.metadata_mut().name = Some(format!("test-pod-{i}"));
        db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
            .await?;
    }

    Ok(())
}

async fn create_dummy_services(
    client: Client,
    db: &mut Database,
    amount: usize,
    namespace: String,
) -> Result<(), kube_primary::error::Error> {
    let data = Service {
        metadata: ObjectMeta {
            name: Some(String::from("test-service")),
            namespace: Some(namespace),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            selector: Some(BTreeMap::from([("app".to_string(), "demo".to_string())])),
            ports: Some(vec![ServicePort {
                port: 80,
                target_port: Some(
                    k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(80),
                ),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    };

    for i in 0..amount {
        let mut data = data.clone();
        data.metadata_mut().name = Some(format!("test-service-{i}"));
        db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
            .await?;
    }

    Ok(())
}

async fn create_dummy_configmaps(
    client: Client,
    db: &mut Database,
    amount: usize,
    namespace: String,
) -> Result<(), kube_primary::error::Error> {
    let data = ConfigMap {
        metadata: ObjectMeta {
            name: Some(String::from("test-configmap")),
            namespace: Some(namespace),
            ..Default::default()
        },
        data: Some(BTreeMap::from([("key".to_string(), "value".to_string())])),
        ..Default::default()
    };

    for i in 0..amount {
        let mut data = data.clone();
        data.metadata_mut().name = Some(format!("test-configmap-{i}"));
        db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
            .await?;
    }

    Ok(())
}

async fn create_dummy_deployments(
    client: Client,
    db: &mut Database,
    amount: usize,
    namespace: String,
) -> Result<(), kube_primary::error::Error> {
    let labels = BTreeMap::from([("app".to_string(), "demo".to_string())]);

    let data = Deployment {
        metadata: ObjectMeta {
            name: Some(String::from("test-deployment")),
            namespace: Some(namespace),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(1),
            selector: LabelSelector {
                match_labels: Some(labels.clone()),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(labels.clone()),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: "test-container".to_string(),
                        image: Some("nginx:latest".to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            },
            strategy: Some(DeploymentStrategy::default()),
            ..Default::default()
        }),
        ..Default::default()
    };

    for i in 0..amount {
        let mut data = data.clone();
        data.metadata_mut().name = Some(format!("test-deployment-{i}"));
        db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
            .await?;
    }

    Ok(())
}
