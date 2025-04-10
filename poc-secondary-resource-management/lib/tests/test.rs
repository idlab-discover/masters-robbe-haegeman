#![allow(unused_imports)]

mod crd;
use error::*;
use lib::*;

#[cfg(test)]
mod tests {
    use lib::PrimaryResource;
    use std::sync::Once;
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::filter::EnvFilter;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::{FmtSubscriber, fmt, layer::SubscriberExt};

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
}
