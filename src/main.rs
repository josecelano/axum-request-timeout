pub mod app;

#[tokio::main]
async fn main() {
    let handle = app::run();
    handle.await.expect("server to run");
}
