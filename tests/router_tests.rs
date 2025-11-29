//! Integration tests for the Router

use aws_lambda_router::{Context, Request, Response, Router};

use serde_json::json;

/// Helper to create a mock Lambda event
fn mock_event(method: &str, path: &str, body: Option<&str>) -> serde_json::Value {
    json!({
        "requestContext": {
            "http": {
                "method": method
            },
            "requestId": "test-request-id"
        },
        "rawPath": path,
        "headers": {
            "content-type": "application/json",
            "authorization": "Bearer test-token"
        },
        "queryStringParameters": {
            "page": "1",
            "limit": "10"
        },
        "body": body
    })
}

#[test]
fn test_router_creation() {
    let _router = Router::new();
    assert!(true, "Router created successfully");
}

#[test]
fn test_router_default() {
    let _router = Router::default();
    assert!(true, "Router default created successfully");
}

#[tokio::test]
async fn test_request_from_lambda_event() {
    let event = mock_event("GET", "/api/users", None);
    let req = Request::from_lambda_event(event);
    
    assert_eq!(req.method, "GET");
    assert_eq!(req.path, "/api/users");
    assert_eq!(req.query("page"), Some(&"1".to_string()));
    assert_eq!(req.query("limit"), Some(&"10".to_string()));
}

#[tokio::test]
async fn test_request_headers() {
    let event = mock_event("POST", "/api/users", Some(r#"{"name":"John"}"#));
    let req = Request::from_lambda_event(event);
    
    assert_eq!(req.header("content-type"), Some(&"application/json".to_string()));
    assert_eq!(req.header("authorization"), Some(&"Bearer test-token".to_string()));
}

#[tokio::test]
async fn test_request_json_body() {
    let event = mock_event("POST", "/api/users", Some(r#"{"name":"John","age":30}"#));
    let req = Request::from_lambda_event(event);
    
    #[derive(serde::Deserialize)]
    struct User {
        name: String,
        age: u32,
    }
    
    let user: User = req.json().unwrap();
    assert_eq!(user.name, "John");
    assert_eq!(user.age, 30);
}

#[test]
fn test_response_ok() {
    let response = Response::ok(json!({"message": "success"}));
    assert_eq!(response.status_code, 200);
    assert!(response.body.contains("success"));
}

#[test]
fn test_response_created() {
    let response = Response::created(json!({"id": "123"}));
    assert_eq!(response.status_code, 201);
    assert!(response.body.contains("123"));
}

#[test]
fn test_response_bad_request() {
    let response = Response::bad_request("Invalid input");
    assert_eq!(response.status_code, 400);
    assert!(response.body.contains("Invalid input"));
}

#[test]
fn test_response_unauthorized() {
    let response = Response::unauthorized("Invalid token");
    assert_eq!(response.status_code, 401);
    assert!(response.body.contains("Invalid token"));
}

#[test]
fn test_response_forbidden() {
    let response = Response::forbidden("Access denied");
    assert_eq!(response.status_code, 403);
    assert!(response.body.contains("Access denied"));
}

#[test]
fn test_response_not_found() {
    let response = Response::not_found("Resource not found");
    assert_eq!(response.status_code, 404);
    assert!(response.body.contains("Resource not found"));
}

#[test]
fn test_response_internal_error() {
    let response = Response::internal_error("Something went wrong");
    assert_eq!(response.status_code, 500);
    assert!(response.body.contains("Something went wrong"));
}

#[test]
fn test_response_no_content() {
    let response = Response::no_content();
    assert_eq!(response.status_code, 204);
}

#[test]
fn test_response_custom_header() {
    let response = Response::ok(json!({}))
        .header("X-Custom-Header", "custom-value");
    
    assert_eq!(
        response.headers.get("X-Custom-Header"),
        Some(&"custom-value".to_string())
    );
}

#[test]
fn test_response_cors_headers() {
    let response = Response::ok(json!({}));
    
    assert!(response.headers.contains_key("Access-Control-Allow-Origin"));
}

#[test]
fn test_response_to_json() {
    let response = Response::ok(json!({"data": "test"}));
    let json_value = response.to_json();
    
    assert_eq!(json_value["statusCode"], 200);
    assert!(json_value["headers"].is_object());
    assert!(json_value["body"].is_string());
}

#[test]
fn test_response_text() {
    let response = Response::new(200).text("Hello, World!");
    
    assert_eq!(response.body, "Hello, World!");
    assert_eq!(
        response.headers.get("Content-Type"),
        Some(&"text/plain".to_string())
    );
}

#[test]
fn test_context_creation() {
    let ctx = Context::new("test-id".to_string());
    assert_eq!(ctx.request_id, "test-id");
    assert!(ctx.user_id.is_none());
}

#[test]
fn test_context_with_user() {
    let ctx = Context::new("test-id".to_string())
        .with_user("user-123".to_string(), Some("user@example.com".to_string()));
    
    assert_eq!(ctx.user_id, Some("user-123".to_string()));
    assert_eq!(ctx.email, Some("user@example.com".to_string()));
}

#[test]
fn test_context_with_custom() {
    let ctx = Context::new("test-id".to_string())
        .with_custom("key".to_string(), json!("value"));
    
    assert_eq!(ctx.custom.get("key"), Some(&json!("value")));
}
