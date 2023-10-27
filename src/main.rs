use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Json, Router,
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

mod base62;
mod id_generator;

#[derive(Clone, Debug)]
struct AppState {
    id_generator: Arc<id_generator::Snowflake>,
    sql: Arc<Mutex<Connection>>,
}

#[derive(Deserialize)]
struct ShortenURL {
    url: String,
}

#[derive(Serialize)]
struct ShortenURLResponse {
    original_url: String,
    short_url: String,
}

async fn redirect_to_original(
    State(state): State<AppState>,
    Path(shorturl): Path<String>,
) -> Response {
    if let Ok(s) = state.sql.lock().unwrap().query_row(
        "SELECT fullurl FROM url WHERE shorturl=?1",
        [shorturl.as_str()],
        |row| row.get::<_, String>(0),
    ) {
        Redirect::permanent(s.as_str()).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn shorten_url(
    State(state): State<AppState>,
    Json(url_payload): Json<ShortenURL>,
) -> Response {
    let original_url = url_payload.url;
    if let Ok(sql) = state.sql.lock() {
        if sql
            .query_row(
                "SELECT * FROM url WHERE fullurl=?1",
                [original_url.as_str()],
                |row| row.get::<_, String>(0),
            )
            .is_ok()
        {
            // The URL is already registered on the system!
            return StatusCode::ACCEPTED.into_response();
        }

        // succeeded to generate a unique ID for the URL
        if let Some(id) = state.id_generator.generate() {
            let short_url = base62::encode(id);
            _ = sql.execute(
                "INSERT INTO url (fullurl, shorturl, id) VALUES (?1, ?2, ?3)",
                [
                    original_url.as_str(),
                    short_url.as_str(),
                    id.to_string().as_str(),
                ],
            );
            return (
                StatusCode::CREATED,
                Json(ShortenURLResponse {
                    original_url,
                    short_url,
                }),
            )
                .into_response();
        }
    }
    StatusCode::INTERNAL_SERVER_ERROR.into_response()
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let machine_id = 1;
    let id_generator = id_generator::Snowflake::new(machine_id);

    // Default SQL scheme
    let sqlconn = Arc::new(Mutex::new(
        rusqlite::Connection::open_in_memory().expect("failed to establish an SQL connection"),
    ));

    sqlconn
        .lock()
        .unwrap()
        .execute_batch(
            "BEGIN;
                CREATE TABLE url (
                    fullurl     TEXT PRIMARY KEY NOT NULL,
                    shorturl    TEXT NOT NULL,
                    id          BIGINT NOT NULL
                );
                COMMIT;
            ",
        )
        .unwrap();

    let state = AppState {
        id_generator: Arc::new(id_generator),
        sql: Arc::clone(&sqlconn),
    };

    let app = Router::new()
        .route("/:shorturl", get(redirect_to_original))
        .route("/shorten", post(shorten_url))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Failed to serve!");
}
