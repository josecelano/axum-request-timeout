use axum::{
    error_handling::HandleErrorLayer,
    http::{header, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    BoxError, Router,
};
use futures::Future;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::time::sleep;
use tower::ServiceBuilder;
use tower_http::timeout::TimeoutLayer;

/// # Panics
///
/// It panics if an invalid socket address is provided.
pub fn run() -> impl Future<Output = hyper::Result<()>> {
    let config_socket_addr: SocketAddr = "127.0.0.1:3001".parse().expect("a valid socket address");

    let tcp_listener = std::net::TcpListener::bind(config_socket_addr)
        .expect("tcp listener to bind to a socket address");

    let bound_addr = tcp_listener
        .local_addr()
        .expect("tcp listener to be bound to a socket address.");

    println!("API server listening on http://{bound_addr}");

    let app = Router::new().route("/", get(entrypoint_handler)).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .layer(TimeoutLayer::new(Duration::from_secs(5)))
            .timeout(Duration::from_secs(5)),
    );

    let server = axum::Server::from_tcp(tcp_listener)
        .expect("a new server from the previously created tcp listener.")
        .http1_header_read_timeout(Duration::from_secs(5))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>());

    server.with_graceful_shutdown(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen to shutdown signal.");
        println!("Stopping API server on http://{bound_addr} ...");
    })
}

#[allow(clippy::unused_async)]
pub async fn entrypoint_handler() -> Response {
    sleep(Duration::from_secs(10)).await;
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        "Hello, World!",
    )
        .into_response()
}

#[allow(clippy::unused_async)]
async fn handle_timeout_error(
    // `Method` and `Uri` are extractors so they can be used here
    method: Method,
    uri: Uri,
    // the last argument must be the error itself
    err: BoxError,
) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{method} {uri}` failed with {err}"),
    )
}
