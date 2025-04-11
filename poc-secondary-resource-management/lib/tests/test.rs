mod crd;

#[cfg(test)]
mod tests {
    use either::Either;
    use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
    use kube::{
        Api, Client, CustomResourceExt, ResourceExt,
        api::{DeleteParams, PostParams},
    };
    use kube_core::ObjectMeta;
    use lib::PrimaryResource;
    use std::sync::Once;
    use tracing::{debug, info, level_filters::LevelFilter};
    use tracing_subscriber::filter::EnvFilter;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{fmt, layer::SubscriberExt};

    static INIT: Once = Once::new();

    use super::crd::*;

    fn create_tracing_subscriber() {
        INIT.call_once(|| {
            tracing_subscriber::registry()
                .with(fmt::layer())
                .with(
                    EnvFilter::builder()
                        .with_default_directive(LevelFilter::DEBUG.into())
                        .from_env_lossy(),
                )
                .init();
        });
    }

    async fn apply_database_crd() -> CustomResourceDefinition {
        let client = Client::try_default().await.unwrap();

        let crds: Api<CustomResourceDefinition> = Api::all(client.clone());
        let crd = Database::crd();

        match crds
            .get_opt(&crd.metadata.name.clone().unwrap())
            .await
            .unwrap()
        {
            Some(crds) => crds,
            None => {
                debug!("Creating CRD");
                crds.create(&PostParams::default(), &crd).await.unwrap()
            }
        }
    }

    async fn create_test_primary(client: Client) -> Database {
        apply_database_crd().await;
        let db = Database {
            metadata: ObjectMeta {
                name: Some(String::from("test")),
                namespace: Some(String::from("api-extension")),
                ..Default::default()
            },
            ..Default::default()
        };

        // Create an Api client for the `Database` CRD
        let db_api: Api<Database> = Api::namespaced(client.clone(), "api-extension");
        match db_api.get_opt(&db.name_any()).await.unwrap() {
            Some(db) => db,
            None => db_api.create(&PostParams::default(), &db).await.unwrap(),
        }
    }

    async fn remove_test_primary(
        client: Client,
        db_name: &str,
    ) -> Either<Database, kube_core::Status> {
        let db_api: Api<Database> = Api::namespaced(client.clone(), "api-extension");
        db_api
            .delete(db_name, &DeleteParams::default())
            .await
            .unwrap()
    }

    #[test]
    fn test_initialize_status() {
        create_tracing_subscriber();

        let mut db = Database::default();
        assert!(db.status.is_none());

        db.initialize_status();
        assert!(db.status.is_some());
    }

    // https://kube.rs/controllers/testing/#integration-tests
    #[tokio::test]
    #[ignore = "uses k8s current-context"]
    async fn test_get_latest_with_secondaries() {
        create_tracing_subscriber();
        let client = Client::try_default().await.unwrap();
        let db = create_test_primary(client.clone()).await;

        let prim_res = db
            .get_latest_with_secondaries(client.clone())
            .await
            .unwrap();

        info!("{prim_res:?}");

        remove_test_primary(client, &db.name_any()).await;
    }
}
