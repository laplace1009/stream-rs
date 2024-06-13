use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {"Hello, World!"}));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 3000 port");
    axum::serve(listener, app).await.unwrap();
}