use std::collections::BTreeMap;

use k8s_openapi::{
    ByteString,
    api::{
        apps::v1::{Deployment, DeploymentSpec, DeploymentStrategy},
        core::v1::{
            ConfigMap, Container, Pod, PodSpec, PodTemplateSpec, Secret, Service, ServicePort,
        },
    },
    apimachinery::pkg::{apis::meta::v1::LabelSelector, util::intstr::IntOrString},
};
use kube::{
    Client,
    api::{ObjectMeta, PostParams},
};
use kube_primary::PrimaryResourceExt;

use crate::crd::Database;

#[derive(Debug, Clone)]
enum ResourceType {
    Secret,
    Pod,
    Service,
    ConfigMap,
    Deployment,
}

/// Creates dummy resources within the cluster
/// If resource_count is not devisable by kind_count, it just rounds to the nearest int
pub async fn create_dummy_resources_by_count(
    client: Client,
    db: &mut Database,
    resource_count: usize,
    kind_count: usize,
    namespace: String,
) {
    if kind_count == 0 {
        if resource_count != 0 {
            panic!("Resource count has to be 0 if kind count is 0");
        }
        return;
    }
    let resource_count_per_kind = resource_count / kind_count;
    if resource_count_per_kind == 0 {
        return;
    }

    // List all resource kinds in order
    let resource_types = [
        ResourceType::Secret,
        ResourceType::Pod,
        ResourceType::Service,
        ResourceType::ConfigMap,
        ResourceType::Deployment,
    ];

    for resource_type in resource_types.iter().take(kind_count) {
        if let Err(e) = create_dummy_resources(
            client.clone(),
            db,
            resource_count_per_kind,
            namespace.clone(),
            resource_type.clone(),
        )
        .await
        {
            eprintln!("Failed to create {:?}: {}", resource_type, e);
        }
    }
}

async fn create_dummy_resources(
    client: Client,
    db: &mut Database,
    amount: usize,
    namespace: String,
    resource_type: ResourceType,
) -> Result<(), kube_primary::error::Error> {
    for i in 0..amount {
        let name = format!(
            "test-{}-{}",
            match resource_type {
                ResourceType::Secret => "secret",
                ResourceType::Pod => "pod",
                ResourceType::Service => "service",
                ResourceType::ConfigMap => "configmap",
                ResourceType::Deployment => "deployment",
            },
            i
        );

        match resource_type {
            ResourceType::Secret => {
                let mut data = Secret {
                    metadata: ObjectMeta {
                        name: Some(name),
                        namespace: Some(namespace.clone()),
                        ..Default::default()
                    },
                    data: {
                        let mut map = BTreeMap::new();
                        map.insert(
                            "test_key".to_string(),
                            ByteString("test_value".as_bytes().to_vec()),
                        );
                        Some(map)
                    },
                    ..Default::default()
                };
                db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
                    .await?;
            }

            ResourceType::Pod => {
                let mut data = Pod {
                    metadata: ObjectMeta {
                        name: Some(name),
                        namespace: Some(namespace.clone()),
                        ..Default::default()
                    },
                    spec: Some(PodSpec {
                        containers: vec![Container {
                            name: "test-container".to_string(),
                            image: Some("nginx:latest".to_string()),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                };
                db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
                    .await?;
            }

            ResourceType::Service => {
                let mut data = Service {
                    metadata: ObjectMeta {
                        name: Some(name),
                        namespace: Some(namespace.clone()),
                        ..Default::default()
                    },
                    spec: Some(k8s_openapi::api::core::v1::ServiceSpec {
                        selector: Some(BTreeMap::from([("app".to_string(), "demo".to_string())])),
                        ports: Some(vec![ServicePort {
                            port: 80,
                            target_port: Some(IntOrString::Int(80)),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }),
                    ..Default::default()
                };
                db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
                    .await?;
            }

            ResourceType::ConfigMap => {
                let mut data = ConfigMap {
                    metadata: ObjectMeta {
                        name: Some(name),
                        namespace: Some(namespace.clone()),
                        ..Default::default()
                    },
                    data: Some(BTreeMap::from([("key".to_string(), "value".to_string())])),
                    ..Default::default()
                };
                db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
                    .await?;
            }

            ResourceType::Deployment => {
                let labels = BTreeMap::from([("app".to_string(), "demo".to_string())]);
                let mut data = Deployment {
                    metadata: ObjectMeta {
                        name: Some(name),
                        namespace: Some(namespace.clone()),
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
                db.create_secondary(client.clone(), &mut PostParams::default(), &mut data)
                    .await?;
            }
        }
    }

    Ok(())
}
