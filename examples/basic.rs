//! Basic Lambda Router Example
//!
//! This example demonstrates how to create a simple REST API using lambda-router.
//!
//! To run this example:
//! ```bash
//! cargo build --example basic --release --target x86_64-unknown-linux-musl
//! ```

use lambda_router::{Context, Request, Response, Router};
use lambda_runtime::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

/// Handler for GET /api/users
async fn list_users(_req: Request, _ctx: Context) -> Result<Response, Error> {
    let users = vec![
        User {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
        },
    ];

    Ok(Response::ok(json!({
        "users": users,
        "count": users.len()
    })))
}

/// Handler for GET /api/users/:userId
async fn get_user(req: Request, _ctx: Context) -> Result<Response, Error> {
    let user_id = req.path_param("userId").unwrap_or_default();

    // Simulate fetching user from database
    if user_id == "1" {
        Ok(Response::ok(json!({
            "user": User {
                id: user_id,
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
            }
        })))
    } else {
        Ok(Response::not_found("User not found"))
    }
}

/// Handler for POST /api/users
async fn create_user(req: Request, _ctx: Context) -> Result<Response, Error> {
    let body: CreateUserRequest = req.json_body()?;

    let new_user = User {
        id: uuid::Uuid::new_v4().to_string(),
        name: body.name,
        email: body.email,
    };

    Ok(Response::created(json!({
        "message": "User created successfully",
        "user": new_user
    })))
}

/// Handler for PUT /api/users/:userId
async fn update_user(req: Request, _ctx: Context) -> Result<Response, Error> {
    let user_id = req.path_param("userId").unwrap_or_default();
    let body: CreateUserRequest = req.json_body()?;

    let updated_user = User {
        id: user_id,
        name: body.name,
        email: body.email,
    };

    Ok(Response::ok(json!({
        "message": "User updated successfully",
        "user": updated_user
    })))
}

/// Handler for DELETE /api/users/:userId
async fn delete_user(req: Request, _ctx: Context) -> Result<Response, Error> {
    let user_id = req.path_param("userId").unwrap_or_default();

    Ok(Response::ok(json!({
        "message": format!("User {} deleted successfully", user_id)
    })))
}

/// Handler for GET /api/health
async fn health_check(_req: Request, _ctx: Context) -> Result<Response, Error> {
    Ok(Response::ok(json!({
        "status": "healthy",
        "version": "1.0.0"
    })))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    // Create the router
    let mut router = Router::new();

    // Health check endpoint
    router.get("/api/health", health_check);

    // User CRUD endpoints
    router.get("/api/users", list_users);
    router.get("/api/users/:userId", get_user);
    router.post("/api/users", create_user);
    router.put("/api/users/:userId", update_user);
    router.delete("/api/users/:userId", delete_user);

    // Custom 404 handler
    router.not_found(|_req, _ctx| async {
        Ok(Response::not_found("The requested resource was not found"))
    });

    // Run the Lambda service
    lambda_runtime::run(router.into_service()).await
}
