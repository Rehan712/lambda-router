# Lambda Router

[![Crates.io](https://img.shields.io/crates/v/aws-lambda-router.svg)](https://crates.io/crates/aws-lambda-router)
[![Documentation](https://docs.rs/aws-lambda-router/badge.svg)](https://docs.rs/aws-lambda-router)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight, Express-like REST API routing framework for AWS Lambda functions with support for middleware, authentication, and CORS.

## Features

- 🚀 **Express-like routing** - Familiar API with path parameters (`:userId`, `:id`)
- 🔌 **Middleware support** - Add authentication, logging, CORS, and custom middleware
- 🌐 **Automatic CORS** - Built-in CORS preflight handling
- 📦 **Type-safe** - Full Rust type safety for requests and responses
- 🔍 **Path parameters** - Easy extraction of URL parameters
- 📝 **Query parsing** - Automatic query string parsing
- 📄 **JSON handling** - Built-in JSON body parsing and response serialization
- ⚡ **Error handling** - Proper HTTP status codes and error responses

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
aws-lambda-router = "0.1"
lambda_runtime = "0.8"
tokio = { version = "1.0", features = ["rt", "macros"] }
serde_json = "1.0"
```

## Quick Start

```rust
use aws_lambda_router::{Router, Request, Response, Context};
use lambda_runtime::Error;
use serde_json::json;

async fn get_user(req: Request, _ctx: Context) -> Result<Response, Error> {
    let user_id = req.path_param("userId").unwrap_or_default();
    Ok(Response::ok(json!({
        "userId": user_id,
        "name": "John Doe"
    })))
}

async fn create_user(req: Request, _ctx: Context) -> Result<Response, Error> {
    let body = req.json_body::<serde_json::Value>()?;
    Ok(Response::created(json!({
        "message": "User created",
        "data": body
    })))
}

async fn list_users(_req: Request, _ctx: Context) -> Result<Response, Error> {
    Ok(Response::ok(json!({
        "users": []
    })))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut router = Router::new();
    
    // Define routes
    router.get("/api/users", list_users);
    router.get("/api/users/:userId", get_user);
    router.post("/api/users", create_user);
    
    // Run the Lambda
    lambda_runtime::run(router.into_service()).await
}
```

## Route Definition

### HTTP Methods

```rust
router.get("/path", handler);
router.post("/path", handler);
router.put("/path", handler);
router.delete("/path", handler);
router.patch("/path", handler);
router.options("/path", handler);
```

### Path Parameters

Use `:paramName` syntax to define path parameters:

```rust
router.get("/users/:userId", handler);
router.get("/users/:userId/posts/:postId", handler);

async fn handler(req: Request, _ctx: Context) -> Result<Response, Error> {
    let user_id = req.path_param("userId").unwrap();
    let post_id = req.path_param("postId").unwrap();
    // ...
}
```

## Request Handling

### Access Request Data

```rust
async fn handler(req: Request, ctx: Context) -> Result<Response, Error> {
    // Path parameters
    let id = req.path_param("id").unwrap_or_default();
    
    // Query parameters
    let page = req.query_param("page").unwrap_or("1".to_string());
    
    // Headers
    let auth = req.header("Authorization").unwrap_or_default();
    
    // JSON body
    let body: MyStruct = req.json_body()?;
    
    // Raw body
    let raw = req.body();
    
    Ok(Response::ok(json!({})))
}
```

## Response Building

```rust
// Success responses
Response::ok(json!({"message": "Success"}))           // 200
Response::created(json!({"id": "123"}))               // 201
Response::no_content()                                 // 204

// Error responses
Response::bad_request("Invalid input")                 // 400
Response::unauthorized("Invalid token")                // 401
Response::forbidden("Access denied")                   // 403
Response::not_found("Resource not found")              // 404
Response::internal_error("Something went wrong")       // 500

// Custom status
Response::new(418, json!({"message": "I'm a teapot"}))

// With headers
Response::ok(json!({}))
    .with_header("X-Custom-Header", "value")
```

## Middleware

### Built-in CORS Middleware

CORS is automatically handled. Configure it as needed:

```rust
use aws_lambda_router::CorsConfig;

let cors = CorsConfig::new()
    .allow_origin("https://example.com")
    .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
    .allow_headers(vec!["Content-Type", "Authorization"]);
```

### Custom Middleware

```rust
use aws_lambda_router::{Middleware, Request, Response, Context, Next};
use async_trait::async_trait;

struct LoggingMiddleware;

#[async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(&self, req: Request, ctx: Context, next: Next<'_>) -> Result<Response, Error> {
        println!("Request: {} {}", req.method(), req.path());
        let response = next.run(req, ctx).await?;
        println!("Response: {}", response.status_code());
        Ok(response)
    }
}

// Add middleware to router
router.use_middleware(LoggingMiddleware);
```

## Error Handling

The router provides structured error handling:

```rust
use aws_lambda_router::{RouterError, Result};

async fn handler(req: Request, _ctx: Context) -> Result<Response> {
    let body: MyStruct = req.json_body()
        .map_err(|_| RouterError::BadRequest("Invalid JSON".to_string()))?;
    
    // Your logic here...
    
    Ok(Response::ok(json!({})))
}
```

## Complete Example

See the [examples](./examples) directory for complete working examples.

## AWS Lambda Deployment

### Cargo.toml for Lambda

```toml
[package]
name = "my-lambda"
version = "0.1.0"
edition = "2021"

[dependencies]
aws-lambda-router = "0.1"
lambda_runtime = "0.8"
tokio = { version = "1.0", features = ["rt", "macros"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### Build for Lambda

```bash
# Install cross-compilation target
rustup target add x86_64-unknown-linux-musl

# Build for Lambda
cargo build --release --target x86_64-unknown-linux-musl

# Package for deployment
cp target/x86_64-unknown-linux-musl/release/my-lambda bootstrap
zip lambda.zip bootstrap
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
