// ここがアプリケーションの始まり
use rust_sample_app_drivers::start_up::start_up;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	start_up().await;

	Ok(())
}
