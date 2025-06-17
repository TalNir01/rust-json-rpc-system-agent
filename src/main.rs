use axum::{Json, Router, routing::post};

mod api_models;
use crate::api_models::{ApiRequest, ApiResponse};

mod api_actions;
use crate::api_actions::ProcessApiRequest;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // Configure end-point
    let app = Router::new().route("/endpoint", post(endpoint));

    // run our app with hyper, listening globally on port 3000
    // TODO: Add TLS Support (limit only to http2?)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn endpoint(Json(client_request): Json<ApiRequest>) -> Json<ApiResponse> {
    match client_request {
        ApiRequest::ExecCmd(client_exec_command) => Json(client_exec_command.process().await),
    }
}
