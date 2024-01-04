use std::sync::Arc;

use todos_controller::{module::Modules, setup::create_app};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let module = Modules::new().await;
    let _ = create_app(Arc::new(module)).await;

    Ok(())
}
