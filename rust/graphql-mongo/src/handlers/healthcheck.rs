use anyhow::Context;
use mongodb::Database;
use poem::web::Data;
use poem::{error::Result, handler};
use poem::{IntoResponse, Response};
use reqwest::StatusCode;
use wither::bson::doc;

pub async fn get_redis_connection(
    redis: Data<&redis::Client>,
) -> anyhow::Result<redis::aio::Connection> {
    redis
        .get_async_connection()
        .await
        .context("Failed to get redis connection")
}

#[handler]
pub async fn healthz(
    db: Data<&Database>,
    redis: Data<&redis::Client>,
) -> Result<impl IntoResponse> {
    let mut connection = get_redis_connection(redis).await?;
    let redis = redis::cmd("PING")
        .query_async::<_, ()>(&mut connection)
        .await;

    // // Ping the server to see if you can connect to the cluster
    let mongo = db.run_command(doc! {"ping": 1}, None).await;

    let auth_url_data = match (redis, mongo) {
        (Ok(_), Ok(_)) => {
            log::info!("Connected successfully.");
            Response::builder().status(StatusCode::OK).body("ok")
        }
        _ => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Dependencies not ready"),
    };
    Ok(auth_url_data)
}

#[handler]
pub async fn liveness() -> impl IntoResponse {
    Response::builder().status(StatusCode::OK).body("Ok")
}
