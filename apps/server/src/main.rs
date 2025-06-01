mod app;
mod features;
mod shared;

use app::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Application::new().await.run().await
}
