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

// ============================================================
// TODO: Implement streaming handlers
// ============================================================

/// GET /stream - chunked text response
async fn stream_handler() -> Response {
    // TODO: Build an IntervalStream, emit 10 chunks, wrap with StreamBody
    // Each chunk should be like: "chunk {n}\n"
    // Set Content-Type to text/plain; charset=utf-8

    todo!("Implement stream_handler")
}

/// GET /sse - Server-Sent Events
async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // TODO: Build an IntervalStream, emit 10 events
    // Each event should be like: "token {n}"
    // Optionally set keep-alive interval

    todo!("Implement sse_handler")
}

#[tokio::main]
async fn main() {
    // TODO: Build router with /stream and /sse
    // TODO: Bind to 127.0.0.1:8080 and serve

    todo!("Implement main")
}
