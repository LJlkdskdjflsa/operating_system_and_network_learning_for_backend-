//! Lab 5: Streaming HTTP (Rust)
//!
//! ## Goal
//! Build streaming HTTP responses (chunked + SSE)
//!
//! ## Requirements
//! 1. GET /stream - return text in chunks
//! 2. GET /sse    - return Server-Sent Events
//! 3. Each endpoint sends 10 messages with a short delay
//!
//! ## Expected Behavior
//! ```bash
//! curl -N http://localhost:8080/stream
//! curl -N http://localhost:8080/sse
//! ```
//!
//! ## Hints
//! - `StreamBody` enables chunked responses
//! - `Sse` handles `text/event-stream` formatting
//! - Avoid Content-Length for streaming
//!
//! ## Acceptance Criteria
//! - [ ] /stream streams data gradually
//! - [ ] /sse streams events gradually
//! - [ ] No buffering on client when using `curl -N`

use axum::{
    body::StreamBody,
    http::{header, HeaderMap, HeaderValue},
    response::{sse::{Event, KeepAlive, Sse}, IntoResponse, Response},
    routing::get,
    Router,
};
use bytes::Bytes;
use std::{convert::Infallible, time::Duration};
use tokio_stream::{wrappers::IntervalStream, Stream, StreamExt};

/// GET /stream - chunked text response
async fn stream_handler() -> Response {
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_millis(200)))
        .enumerate()
        .take(10)
        .map(|(i, _)| Ok::<Bytes, Infallible>(Bytes::from(format!("chunk {}\n", i))));

    let body = StreamBody::new(stream);
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    (headers, body).into_response()
}

/// GET /sse - Server-Sent Events
async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_millis(400)))
        .enumerate()
        .take(10)
        .map(|(i, _)| Ok(Event::default().data(format!("token {}", i))));

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(5))
            .text("keep-alive"),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/stream", get(stream_handler))
        .route("/sse", get(sse_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("bind failed");

    axum::serve(listener, app).await.expect("server failed");
}
