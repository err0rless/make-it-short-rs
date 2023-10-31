use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use mysql::{prelude::*, *};

mod base62;
mod id_generator;
mod packet;

use packet::*;

#[derive(Clone, Debug)]
struct AppState {
    id_generator: Arc<id_generator::Snowflake>,
    sql: mysql::Pool,
}

async fn redirect_to_original(
    State(state): State<AppState>,
    Path(shorturl): Path<String>,
) -> Response {
    let id = base62::decode(&shorturl);
    let mut sql = state.sql.get_conn().unwrap();
    if let Ok(Some(s)) =
        sql.query_first::<String, _>(format!("SELECT fullurl FROM url WHERE id={}", id))
    {
        Redirect::permanent(s.as_str()).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn shorten_url(
    State(state): State<AppState>,
    Json(payload): Json<ShortenURLReq>,
) -> Response {
    let mut sql = state.sql.get_conn().unwrap();

    // succeeded to generate a unique ID for the URL
    if let Some(id) = state.id_generator.generate() {
        let short_url = base62::encode(id);
        _ = sql.exec_drop(
            "INSERT INTO url (id, fullurl, shorturl) VALUES (:id, :fullurl, :shorturl)",
            params! {
                "id" => id.to_string().as_str(),
                "fullurl" => payload.url.as_str(),
                "shorturl" => short_url.clone().as_str()
            },
        );
        (StatusCode::CREATED, Json(ShortenURLRes { short_url })).into_response()
    } else {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[tokio::main]
async fn main() {
    let port_num = if let Some(port_num) = std::env::args().nth(1) {
        port_num.parse::<i32>().expect("invalid port number")
    } else {
        80
    };
    let binding_addr = format!("0.0.0.0:{}", port_num);

    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let machine_id = 1;
    let id_generator = id_generator::Snowflake::new(machine_id);

    // mysql connection pool
    let pool = Pool::new("mysql://root@localhost:3306/makeitshort").unwrap();

    let mut conn = pool.get_conn().unwrap();
    _ = conn.query_drop(
        r"CREATE TABLE url (
            id          BIGINT PRIMARY KEY NOT NULL,
            fullurl     TEXT NOT NULL,
            shorturl    TEXT NOT NULL
        )",
    );

    let state = AppState {
        id_generator: Arc::new(id_generator),
        sql: pool.clone(),
    };

    let app = Router::new()
        .route("/:shorturl", get(redirect_to_original))
        .route("/shorten", post(shorten_url))
        .with_state(state);

    axum::Server::bind(&binding_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Failed to serve!");
}
