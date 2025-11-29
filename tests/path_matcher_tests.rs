//! Tests for PathMatcher

use aws_lambda_router::PathMatcher;

#[test]
fn test_simple_static_path() {
    let matcher = PathMatcher::new("/api/users");
    
    assert!(matcher.matches("/api/users").is_some());
    assert!(matcher.matches("/api/posts").is_none());
    assert!(matcher.matches("/api/users/123").is_none());
    assert!(matcher.matches("/api").is_none());
}

#[test]
fn test_root_path() {
    let matcher = PathMatcher::new("/");
    
    assert!(matcher.matches("/").is_some());
    assert!(matcher.matches("/api").is_none());
}

#[test]
fn test_single_path_parameter() {
    let matcher = PathMatcher::new("/api/users/:userId");
    
    // Should match
    let params = matcher.matches("/api/users/123");
    assert!(params.is_some());
    let params = params.unwrap();
    assert_eq!(params.get("userId"), Some(&"123".to_string()));
    
    // Should match with different ID
    let params = matcher.matches("/api/users/abc-def-456").unwrap();
    assert_eq!(params.get("userId"), Some(&"abc-def-456".to_string()));
    
    // Should not match
    assert!(matcher.matches("/api/users").is_none());
    assert!(matcher.matches("/api/users/123/posts").is_none());
}

#[test]
fn test_multiple_path_parameters() {
    let matcher = PathMatcher::new("/api/users/:userId/posts/:postId");
    
    let params = matcher.matches("/api/users/user123/posts/post456").unwrap();
    assert_eq!(params.get("userId"), Some(&"user123".to_string()));
    assert_eq!(params.get("postId"), Some(&"post456".to_string()));
}

#[test]
fn test_parameter_at_start() {
    let matcher = PathMatcher::new("/:version/api/users");
    
    let params = matcher.matches("/v1/api/users").unwrap();
    assert_eq!(params.get("version"), Some(&"v1".to_string()));
}

#[test]
fn test_parameter_with_numbers() {
    let matcher = PathMatcher::new("/api/items/:item123Id");
    
    let params = matcher.matches("/api/items/my-item").unwrap();
    assert_eq!(params.get("item123Id"), Some(&"my-item".to_string()));
}

#[test]
fn test_underscore_in_parameter() {
    let matcher = PathMatcher::new("/api/:user_id/profile");
    
    let params = matcher.matches("/api/12345/profile").unwrap();
    assert_eq!(params.get("user_id"), Some(&"12345".to_string()));
}

#[test]
fn test_complex_nested_path() {
    let matcher = PathMatcher::new("/api/chatbots/:chatbotId/conversations/:conversationId/messages/:messageId");
    
    let params = matcher.matches("/api/chatbots/cb-1/conversations/conv-2/messages/msg-3").unwrap();
    assert_eq!(params.get("chatbotId"), Some(&"cb-1".to_string()));
    assert_eq!(params.get("conversationId"), Some(&"conv-2".to_string()));
    assert_eq!(params.get("messageId"), Some(&"msg-3".to_string()));
}

#[test]
fn test_uuid_style_parameters() {
    let matcher = PathMatcher::new("/api/users/:userId");
    
    let uuid = "550e8400-e29b-41d4-a716-446655440000";
    let params = matcher.matches(&format!("/api/users/{}", uuid)).unwrap();
    assert_eq!(params.get("userId"), Some(&uuid.to_string()));
}

#[test]
fn test_pattern_accessor() {
    let matcher = PathMatcher::new("/api/users/:userId");
    assert_eq!(matcher.pattern(), "/api/users/:userId");
}

#[test]
fn test_no_match_partial_path() {
    let matcher = PathMatcher::new("/api/users/:userId");
    
    assert!(matcher.matches("/api/user/123").is_none());
    assert!(matcher.matches("/api/users/").is_none());
}

#[test]
fn test_exact_match_required() {
    let matcher = PathMatcher::new("/api/users");
    
    // Should not match paths with trailing content
    assert!(matcher.matches("/api/users/").is_none());
    assert!(matcher.matches("/api/users/extra").is_none());
    
    // Should only match exact path
    assert!(matcher.matches("/api/users").is_some());
}
