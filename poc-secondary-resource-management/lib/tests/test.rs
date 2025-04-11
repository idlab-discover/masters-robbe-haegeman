mod crd;

#[cfg(test)]
mod tests {
    use kube::{
        Api, Client, ResourceExt,
        api::{DeleteParams, PostParams},
    };
    use kube_core::ObjectMeta;
    use lib::PrimaryResource;
    use std::sync::Once;
    use tracing::{info, level_filters::LevelFilter};
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

    #[test]
    fn test_initialize_status() {
        create_tracing_subscriber();

        let mut db = Database::default();
        assert!(db.status.is_none());

        db.initialize_status();
        assert!(db.status.is_some());
    }

    // https://kube.rs/controllers/testing/#integration-tests
    // Currently assumes the database resource is already created
    #[tokio::test]
    #[ignore = "uses k8s current-context"]
    async fn test_get_latest_with_secondaries() {
        create_tracing_subscriber();
        let db = Database {
            metadata: ObjectMeta {
                name: Some(String::from("test")),
                namespace: Some(String::from("api-extension")),
                ..Default::default()
            },
            ..Default::default()
        };

        let client = Client::try_default().await.unwrap();

        // Create an Api client for the `Database` CRD
        let db_api: Api<Database> = Api::namespaced(client.clone(), "api-extension");
        let existing_db = match db_api.get_opt(&db.name_any()).await.unwrap() {
            Some(db) => db,
            None => db_api.create(&PostParams::default(), &db).await.unwrap(),
        };

        let prim_res = existing_db
            .get_latest_with_secondaries(client)
            .await
            .unwrap();

        info!("{prim_res:?}");

        db_api
            .delete(&existing_db.name_any(), &DeleteParams::default())
            .await
            .unwrap();
    }
}
