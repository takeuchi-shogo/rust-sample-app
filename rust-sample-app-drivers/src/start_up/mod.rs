
use crate::routes::health::hc;

use axum::{
	routing::get,
	Router,
};

use std::net::SocketAddr;

pub async fn start_up() {
	let hc_routes = Router::new()
		.route("/", get(hc));
	
	let app = Router::new()
		.nest("/hc", hc_routes);

	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

	tracing::info!("Server listening on {}", addr);

	let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();

	// axum::Server::bind(&addr)
	// 	.serve(app.into_make_service())
	// 	.await
	// 	.unwrap_or_else(|_| panic!("Server cannot launch!"))
}

pub fn init_app() {
	// 環境変数の読み込み
}
