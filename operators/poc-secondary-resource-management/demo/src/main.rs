use controller::Context;
use kube::Error;
use std::sync::Arc;

use kube::Client;

pub use controller::{self, crd};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    log::info!("Logging enabled: setting up client");
    let client = Client::try_default().await?;

    let context = Arc::new(Context { client });

    log::info!("Client was successfully setup, now starting controller");
    controller::run(context).await;

    Ok(())
}
