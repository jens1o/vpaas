extern crate axum;
extern crate common;
extern crate redis;
extern crate tokio;
extern crate tracing;
extern crate tracing_subscriber;

use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use common::{dimensions::Dimensions, job::TranscoderJob};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, str, sync::Mutex};
use tower_http::cors::{self, CorsLayer};
use tracing::{info, Level};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    match tokio::fs::create_dir("uploads").await {
        Ok(()) => {
            info!("successfully created uploads/ dir");
        }
        Err(err) => {
            info!("creating uploads/ directory failed: {}", err);
        }
    }

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/videos", post(new_video_job))
        .layer(CorsLayer::new().allow_origin(cors::Any))
        .layer(DefaultBodyLimit::max(1024 * 100_000))
        .layer(Extension(client));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // let job = TranscoderJob::new(
    //     "./bla.mp4".into(),
    //     "./transcoded.mp4".into(),
    //     Dimensions::new(320, 240),
    //     Some("copy".into()),
    // );

    // let job_json = serde_json::to_string(&job).unwrap();

    // con.rpush::<_, _, u128>("vpaas:queue", job_json)
    //     .await
    //     .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn new_video_job(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Extension(rclient): Extension<redis::Client>,
    mut payload: Multipart,
) -> (StatusCode, Json<String>) {
    let mut filename = Mutex::new(None);
    let mut new_dimensions = Mutex::new(None);

    let mut con = rclient.get_async_connection().await.unwrap();

    while let Some(field) = payload.next_field().await.unwrap() {
        let name = field.name().unwrap();

        match name {
            "file" => {
                let data = field.bytes().await.unwrap();
                let tmpfile = format!(
                    "{}/uploads/{}.mov",
                    env::current_dir().unwrap().to_str().unwrap(),
                    Uuid::new_v4()
                );
                tokio::fs::write(&tmpfile, data).await.unwrap();
                *filename.get_mut().unwrap() = Some(tmpfile);
            }
            "new_dimension" => {
                *new_dimensions.get_mut().unwrap() = Some(
                    serde_json::from_str::<Dimensions>(
                        str::from_utf8(&field.bytes().await.unwrap()).unwrap(),
                    )
                    .unwrap(),
                );
            }
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(String::from("unknown field given in multipart")),
                );
            }
        };
    }

    let filename: String = match filename.into_inner().unwrap() {
        Some(filename) => filename,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(String::from("missing filename")),
            );
        }
    };

    let new_dimensions: Dimensions = match new_dimensions.into_inner().unwrap() {
        Some(new_dimensions) => new_dimensions,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(String::from("missing new dimensions")),
            );
        }
    };

    let output_uri = format!("{}.mp4", filename);

    let job = TranscoderJob::new(filename, output_uri, new_dimensions.into(), None);
    // dbg!(&filename, new_dimensions);

    let outstanding_jobs = con
        .rpush::<_, _, u128>(common::QUEUE_NAME, serde_json::to_string(&job).unwrap())
        .await;

    if let Ok(outstanding_jobs) = outstanding_jobs {
        info!("Enqueued new job. Outstanding job count: {outstanding_jobs}");
    }

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(String::new()))
}

#[derive(Deserialize)]
struct NewVideoForm {
    file: String,
    #[serde(rename = "newResolution")]
    new_resolution: Dimensions,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
