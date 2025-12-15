# HTTP Protocol

## Overview

HTTP (Hypertext Transfer Protocol) is the foundation of data communication on the web. Understanding HTTP is essential for building web services, APIs, and debugging network issues.

## HTTP Basics

### Request-Response Model

```
Client                                Server
   |                                     |
   |-------- HTTP Request ------------->|
   |         GET /index.html HTTP/1.1   |
   |         Host: example.com          |
   |                                     |
   |<------- HTTP Response -------------|
   |         HTTP/1.1 200 OK            |
   |         Content-Type: text/html    |
   |         <html>...</html>           |
```

HTTP is:
- **Stateless**: Each request is independent
- **Text-based**: Human-readable (at protocol level)
- **Request-response**: Client initiates, server responds

## HTTP Request Structure

```
GET /api/users?page=1 HTTP/1.1     <- Request line
Host: api.example.com              <- Headers start
User-Agent: curl/7.68.0
Accept: application/json
Authorization: Bearer token123
Content-Type: application/json
                                   <- Empty line (CRLF)
{"name": "John"}                   <- Body (optional)
```

### Request Line Components

```
METHOD  PATH             VERSION
  |       |                |
 GET  /api/users?id=1  HTTP/1.1
```

### HTTP Methods

| Method | Purpose | Has Body | Idempotent | Safe |
|--------|---------|----------|------------|------|
| GET | Retrieve resource | No | Yes | Yes |
| POST | Create resource | Yes | No | No |
| PUT | Replace resource | Yes | Yes | No |
| PATCH | Update resource | Yes | No | No |
| DELETE | Remove resource | No | Yes | No |
| HEAD | Get headers only | No | Yes | Yes |
| OPTIONS | Get allowed methods | No | Yes | Yes |

**Idempotent**: Multiple identical requests have same effect as one
**Safe**: Does not modify server state

## HTTP Response Structure

```
HTTP/1.1 200 OK                    <- Status line
Date: Mon, 15 Dec 2024 10:00:00 GMT
Server: nginx/1.18.0
Content-Type: application/json
Content-Length: 27
Connection: keep-alive
                                   <- Empty line
{"id": 1, "name": "John"}          <- Body
```

### Status Line Components

```
VERSION   CODE   REASON
   |        |      |
HTTP/1.1  200    OK
```

### Status Code Categories

| Range | Category | Examples |
|-------|----------|----------|
| 1xx | Informational | 100 Continue |
| 2xx | Success | 200 OK, 201 Created, 204 No Content |
| 3xx | Redirection | 301 Moved, 302 Found, 304 Not Modified |
| 4xx | Client Error | 400 Bad Request, 401 Unauthorized, 404 Not Found |
| 5xx | Server Error | 500 Internal Error, 502 Bad Gateway, 503 Unavailable |

### Common Status Codes

```rust
// Success
200 OK              // Request succeeded
201 Created         // Resource created (POST)
204 No Content      // Success, no body (DELETE)

// Redirection
301 Moved Permanently  // Resource moved, update bookmarks
302 Found              // Temporary redirect
304 Not Modified       // Use cached version

// Client Error
400 Bad Request     // Malformed request
401 Unauthorized    // Need authentication
403 Forbidden       // Not allowed even with auth
404 Not Found       // Resource doesn't exist
405 Method Not Allowed
422 Unprocessable Entity  // Validation failed

// Server Error
500 Internal Server Error  // Generic server error
502 Bad Gateway           // Upstream server error
503 Service Unavailable   // Server overloaded/maintenance
504 Gateway Timeout       // Upstream timeout
```

## Important Headers

### Request Headers

```
Host: api.example.com           # Required in HTTP/1.1
User-Agent: Mozilla/5.0...      # Client identification
Accept: application/json        # Preferred response format
Accept-Language: en-US,en       # Preferred language
Accept-Encoding: gzip, deflate  # Compression support
Authorization: Bearer xyz       # Auth credentials
Content-Type: application/json  # Body format
Content-Length: 123             # Body size in bytes
Cookie: session=abc123          # Cookies
```

### Response Headers

```
Content-Type: application/json  # Body format
Content-Length: 456             # Body size
Content-Encoding: gzip          # Compression used
Cache-Control: max-age=3600     # Caching rules
Set-Cookie: session=xyz         # Set cookies
Location: /new-url              # Redirect target
```

## Parsing HTTP in Rust

### Manual Parsing

```rust
fn parse_request(raw: &str) -> Option<Request> {
    let mut lines = raw.lines();

    // Parse request line
    let request_line = lines.next()?;
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let method = parts.get(0)?;
    let path = parts.get(1)?;
    let version = parts.get(2)?;

    // Parse headers
    let mut headers = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    // Rest is body
    let body: String = lines.collect();

    Some(Request { method, path, headers, body })
}
```

### Building Responses

```rust
fn build_response(status: u16, body: &str) -> String {
    let reason = match status {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    };

    format!(
        "HTTP/1.1 {} {}\r\n\
         Content-Type: text/plain\r\n\
         Content-Length: {}\r\n\
         Connection: close\r\n\
         \r\n\
         {}",
        status,
        reason,
        body.len(),
        body
    )
}
```

## REST API Design

### Resource-Based URLs

```
/users              # Collection
/users/123          # Specific resource
/users/123/posts    # Nested resource
/users/123/posts/1  # Specific nested resource
```

### CRUD Operations

```
GET    /users       # List all users
POST   /users       # Create new user
GET    /users/123   # Get user 123
PUT    /users/123   # Replace user 123
PATCH  /users/123   # Update user 123
DELETE /users/123   # Delete user 123
```

### JSON in REST APIs

```rust
// Request
POST /users HTTP/1.1
Content-Type: application/json

{
    "name": "John Doe",
    "email": "john@example.com"
}

// Response
HTTP/1.1 201 Created
Content-Type: application/json

{
    "id": 123,
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": "2024-12-15T10:00:00Z"
}
```

## HTTP with Axum

### Basic Setup

```rust
use axum::{
    Router,
    routing::{get, post},
    Json,
    extract::Path,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

async fn get_user(Path(id): Path<u64>) -> Json<User> {
    Json(User { id, name: "John".into() })
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    Json(User { id: 1, name: payload.name })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Extractors

```rust
// Path parameters
async fn get_user(Path(id): Path<u64>) -> String { ... }

// Query parameters
async fn list_users(Query(params): Query<ListParams>) -> Json<Vec<User>> { ... }

// JSON body
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> { ... }

// Headers
async fn auth(headers: HeaderMap) -> String {
    headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("none")
        .to_string()
}

// State
async fn handler(State(db): State<Database>) -> String { ... }
```

### Error Handling

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

enum AppError {
    NotFound,
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };
        (status, message).into_response()
    }
}

async fn fallible_handler() -> Result<String, AppError> {
    // ...
    Err(AppError::NotFound)
}
```

## HTTP/1.1 vs HTTP/2

### HTTP/1.1
- Text-based protocol
- One request per connection (without keep-alive)
- Head-of-line blocking
- No header compression

### HTTP/2
- Binary protocol
- Multiplexing (multiple requests over one connection)
- Header compression (HPACK)
- Server push
- Stream prioritization

```
HTTP/1.1:
Connection 1: [Request 1] -> [Response 1]
Connection 2: [Request 2] -> [Response 2]
Connection 3: [Request 3] -> [Response 3]

HTTP/2:
Connection 1: [Req 1, Req 2, Req 3] -> [Resp 2, Resp 1, Resp 3]
              (multiplexed streams)
```

## Debugging HTTP

### Using curl

```bash
# Simple GET
curl http://localhost:8080/

# Verbose output
curl -v http://localhost:8080/

# POST with JSON
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"name": "John"}' \
  http://localhost:8080/users

# With headers
curl -H "Authorization: Bearer token" http://localhost:8080/

# Follow redirects
curl -L http://localhost:8080/redirect

# Show only headers
curl -I http://localhost:8080/
```

### Using httpie (alternative)

```bash
# GET
http localhost:8080/

# POST JSON
http POST localhost:8080/users name=John

# With headers
http localhost:8080/ Authorization:"Bearer token"
```

## Common Patterns

### Content Negotiation

```rust
async fn handler(headers: HeaderMap) -> Response {
    let accept = headers.get("Accept")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("text/plain");

    if accept.contains("application/json") {
        Json(data).into_response()
    } else {
        format!("{:?}", data).into_response()
    }
}
```

### Pagination

```rust
#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

async fn list(Query(p): Query<Pagination>) -> Json<Vec<Item>> {
    let page = p.page.unwrap_or(1);
    let per_page = p.per_page.unwrap_or(20).min(100);

    // offset = (page - 1) * per_page
    // limit = per_page
}
```

### CORS

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

let app = Router::new()
    .route("/", get(handler))
    .layer(cors);
```

## Summary

- HTTP is **stateless**, **text-based**, **request-response** protocol
- **Methods** define actions (GET, POST, PUT, DELETE, etc.)
- **Status codes** indicate results (2xx success, 4xx client error, 5xx server error)
- **Headers** carry metadata (Content-Type, Authorization, etc.)
- **REST** maps HTTP methods to CRUD operations on resources
- **Axum** provides ergonomic HTTP server building in Rust

## Labs

1. **Lab 3: Raw HTTP Server** - Parse and respond to HTTP without frameworks
2. **Lab 4: Axum REST API** - Build a complete CRUD API with Axum
