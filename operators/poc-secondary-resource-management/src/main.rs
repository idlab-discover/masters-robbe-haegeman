use controller::Context;
use kube::Error;
use std::sync::Arc;

use kube::Client;

pub use controller::{self, crd};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let client = Client::try_default().await?;

    let context = Arc::new(Context {
        client: client.clone(),
    });

    controller::run(context).await;

    Ok(())
}
