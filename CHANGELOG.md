# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-11-29

### Added
- Initial release
- Express-like routing with path parameters
- Middleware support with async trait
- Built-in CORS middleware with automatic preflight handling
- Request helpers for path params, query params, headers, and JSON body
- Response builders for common HTTP status codes
- Custom error types with proper HTTP status code mapping
- Path matcher with regex-based parameter extraction
- Context object for passing data through middleware chain
- Custom 404 handler support
- Type-safe handler functions

### Features
- `Router::new()` - Create a new router instance
- `router.get()`, `router.post()`, `router.put()`, `router.delete()`, `router.patch()`, `router.options()` - Route definition
- `router.use_middleware()` - Add middleware to the chain
- `router.not_found()` - Custom 404 handler
- `router.into_service()` - Convert router to Lambda service
- `Request` - Request wrapper with helper methods
- `Response` - Response builder with status helpers
- `CorsConfig` - CORS configuration builder
- `Middleware` trait - Custom middleware support
