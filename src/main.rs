use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Router,
    routing::{get},
};
use tokio::{
    fs::File,
    io::AsyncReadExt,
};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {"Hello, Axum"}))
        .route("/video", get(video_sample));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 3000 port");
    axum::serve(listener, app).await.unwrap();
}

async fn video_sample() -> Result<Response, StatusCode> {
    let file_path = "video/sample_480p.mp4";
    let mut file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => return Err(StatusCode::NOT_FOUND)
    };
    let mut buffer = Vec::new();
    if let Err(_) = file.read_to_end(&mut buffer).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("video/mp4"));
    Ok((headers, buffer).into_response())
}