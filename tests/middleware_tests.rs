//! Tests for middleware functionality

use aws_lambda_router::{Middleware, Request, Response};
use async_trait::async_trait;
use lambda_runtime::Error;
use serde_json::json;

/// Helper to create a mock request
fn mock_request(method: &str, path: &str) -> Request {
    let event = json!({
        "requestContext": {
            "http": {
                "method": method
            },
            "requestId": "test-request-id"
        },
        "rawPath": path,
        "headers": {},
        "queryStringParameters": null,
        "body": null
    });
    Request::from_lambda_event(event)
}

/// Custom test middleware
struct TestMiddleware {
    header_name: String,
    header_value: String,
}

impl TestMiddleware {
    fn new(name: &str, value: &str) -> Self {
        Self {
            header_name: name.to_string(),
            header_value: value.to_string(),
        }
    }
}

#[async_trait]
impl Middleware for TestMiddleware {
    async fn handle(
        &self,
        req: Request,
        next: aws_lambda_router::Next,
    ) -> Result<Response, Error> {
        let response = next(req).await?;
        Ok(response.header(&self.header_name, &self.header_value))
    }
}

#[test]
fn test_custom_middleware_creation() {
    let middleware = TestMiddleware::new("X-Test", "test-value");
    assert_eq!(middleware.header_name, "X-Test");
    assert_eq!(middleware.header_value, "test-value");
}

#[test]
fn test_request_is_preflight() {
    let req = mock_request("OPTIONS", "/api/users");
    assert!(req.is_preflight());
    
    let req = mock_request("GET", "/api/users");
    assert!(!req.is_preflight());
}

#[test]
fn test_cors_preflight_response() {
    let response = Response::cors_preflight();
    assert_eq!(response.status_code, 200);
    assert!(response.headers.contains_key("Access-Control-Allow-Origin"));
    assert!(response.headers.contains_key("Access-Control-Allow-Methods"));
    assert!(response.headers.contains_key("Access-Control-Allow-Headers"));
}
