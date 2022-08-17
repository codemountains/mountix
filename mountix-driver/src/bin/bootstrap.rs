use mountix_driver::module::Modules;
use mountix_driver::startup::{init_app, startup};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_app();

    let modules = Modules::new().await;
    let _ = startup(Arc::new(modules)).await;

    Ok(())
}
