mod controller;
pub mod crd;
mod reconcile_sec_res;

use controller::Context;
use kube::Error;
use std::sync::Arc;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use kube::Client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    info!("Logging enabled: setting up client");
    let client = Client::try_default().await?;

    let context = Arc::new(Context { client });

    info!("Client was successfully setup, now starting controller");
    controller::run(context).await;

    Ok(())
}
