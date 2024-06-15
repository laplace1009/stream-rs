use axum::{http::{HeaderMap, HeaderValue, StatusCode}, response::{IntoResponse, Response}, Router, routing::{get}};
use axum::extract::Query;
use tokio::{
    fs::File,
    io::AsyncReadExt,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Video {
    v: String,
    fmt: i32,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async {"Hello, Axum"}))
        .route("/watch", get(video_sample))
        .route("/dash/manifest.mpd", get(get_dash_manifest))
        .route("/dash/:segment", get(get_dash_segment));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 3000 port");
    axum::serve(listener, app).await.unwrap();
}

async fn video_sample() -> Result<Response, StatusCode> {
    let file_path = "static/video.html";
    let mut file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => return Err(StatusCode::NOT_FOUND)
    };
    let mut buffer = Vec::new();
    if let Err(_) = file.read_to_end(&mut buffer).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));
    Ok((headers, buffer).into_response())
}

async fn get_dash_manifest(Query(params): Query<Video>) -> Result<Response, StatusCode> {
    let video_name = &params.v;
    let fmt = &params.fmt;
    let file_path = format!("video/{video_name}_manifest_{fmt}p.mpd");
    let mut file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let mut buffer = Vec::new();
    if let Err(_) = file.read_to_end(&mut buffer).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/dash+xml"));
    Ok((headers, buffer).into_response())
}

async fn get_dash_segment(uri: axum::extract::Path<String>) -> Result<Response, StatusCode> {
    let file_path = format!("video/{}", uri.0);
    let mut file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let mut buffer = Vec::new();
    if let Err(_) = file.read_to_end(&mut buffer).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("video/mp4"));
    Ok((headers, buffer).into_response())
}