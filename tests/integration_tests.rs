//! Integration tests for the CoachJan backend.
//!
//! These tests exercise the full HTTP request/response cycle through the Axum
//! router, using an in-memory SQLite database for isolation.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tower::ServiceExt;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use coachjan::ai::client::ClaudeClient;
use coachjan::config::Config;
use coachjan::{build_app, AppState};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// Build an Axum app backed by a fresh in-memory SQLite database.
///
/// Each call creates a completely isolated database, so tests do not
/// interfere with each other.
async fn test_app() -> Router {
    let connect_options = SqliteConnectOptions::new()
        .filename(":memory:")
        .create_if_missing(true)
        .pragma("foreign_keys", "ON");

    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(connect_options)
        .await
        .expect("Failed to create in-memory SQLite pool");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let config = Config {
        database_url: String::new(),
        host: "127.0.0.1".to_string(),
        port: 0,
        anthropic_api_key: None,
    };

    let state = AppState { db, config, claude_client: None };
    build_app(state)
}

/// Send a request through the app and return the response.
async fn send_request(
    app: Router,
    req: Request<Body>,
) -> axum::response::Response {
    app.oneshot(req).await.expect("Request failed")
}

/// Read the response body as a parsed JSON `Value`.
async fn body_json(response: axum::response::Response) -> Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("Failed to read response body")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("Response body is not valid JSON")
}

/// Extract the `session_id` value from the first `Set-Cookie` header.
fn extract_session_cookie(response: &axum::response::Response) -> Option<String> {
    response
        .headers()
        .get_all("set-cookie")
        .iter()
        .find_map(|val| {
            let s = val.to_str().ok()?;
            // Parse cookie: "session_id=<value>; HttpOnly; ..."
            s.split(';').next().and_then(|pair| {
                let (key, value) = pair.split_once('=')?;
                if key.trim() == "session_id" && !value.trim().is_empty() {
                    Some(value.trim().to_string())
                } else {
                    None
                }
            })
        })
}

/// Build a POST request with a JSON body.
fn post_json(uri: &str, body: &Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

/// Build a POST request with a JSON body and a session cookie.
fn post_json_authed(uri: &str, body: &Value, session_id: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .header("cookie", format!("session_id={session_id}"))
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

/// Build a PUT request with a JSON body and a session cookie.
fn put_json_authed(uri: &str, body: &Value, session_id: &str) -> Request<Body> {
    Request::builder()
        .method("PUT")
        .uri(uri)
        .header("content-type", "application/json")
        .header("cookie", format!("session_id={session_id}"))
        .body(Body::from(serde_json::to_vec(body).unwrap()))
        .unwrap()
}

/// Build a GET request (no auth).
fn get_request(uri: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
        .unwrap()
}

/// Build a GET request with a session cookie.
fn get_authed(uri: &str, session_id: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .header("cookie", format!("session_id={session_id}"))
        .body(Body::empty())
        .unwrap()
}

/// Register a test user and return the session ID from the response cookie.
///
/// Panics if registration fails.
async fn register_user(app: Router, email: &str, password: &str) -> (Router, String) {
    let body = json!({ "email": email, "password": password });
    // We need to clone the app because oneshot consumes it
    let response = send_request(app.clone(), post_json("/api/auth/register", &body)).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let session_id = extract_session_cookie(&response)
        .expect("Registration should return a session cookie");
    (app, session_id)
}

/// A valid profile creation payload for reuse across tests.
fn valid_profile_body() -> Value {
    json!({
        "name": "Test Runner",
        "age": 30,
        "weight_kg": 70.0,
        "resting_hr": 50,
        "max_hr": 185,
        "lthr": 170,
        "ftpace_m_per_s": 4.5,
        "current_weekly_volume_km": 40.0,
        "experience_level": "intermediate",
        "sports_background": "cycling",
        "race_name": "Spring Marathon",
        "race_distance_m": 42195.0,
        "race_date": "2026-09-27",
        "target_time_seconds": 12600
    })
}

// ===========================================================================
// Auth flow tests
// ===========================================================================

#[tokio::test]
async fn register_returns_session_cookie_and_user() {
    let app = test_app().await;

    let body = json!({
        "email": "runner@example.com",
        "password": "securepass123"
    });
    let response = send_request(app, post_json("/api/auth/register", &body)).await;

    assert_eq!(response.status(), StatusCode::CREATED);

    // Should have a Set-Cookie header with session_id
    let session_id = extract_session_cookie(&response);
    assert!(session_id.is_some(), "Expected session cookie in response");

    let json = body_json(response).await;
    assert_eq!(json["user"]["email"], "runner@example.com");
    assert!(json["user"]["id"].is_number());
}

#[tokio::test]
async fn register_then_me_returns_user() {
    let app = test_app().await;

    // Register
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    // GET /me with the session cookie
    let response = send_request(app, get_authed("/api/auth/me", &session_id)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response).await;
    assert_eq!(json["user"]["email"], "runner@example.com");
    assert_eq!(json["has_profile"], false);
}

#[tokio::test]
async fn logout_then_me_returns_401() {
    let app = test_app().await;

    // Register
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    // Logout
    let logout_req = Request::builder()
        .method("POST")
        .uri("/api/auth/logout")
        .header("cookie", format!("session_id={session_id}"))
        .body(Body::empty())
        .unwrap();
    let response = send_request(app.clone(), logout_req).await;
    assert_eq!(response.status(), StatusCode::OK);

    // GET /me should now be 401
    let response = send_request(app, get_authed("/api/auth/me", &session_id)).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_with_correct_password_returns_session() {
    let app = test_app().await;

    // Register first
    let (app, _) = register_user(app, "runner@example.com", "securepass123").await;

    // Login
    let body = json!({
        "email": "runner@example.com",
        "password": "securepass123"
    });
    let response = send_request(app.clone(), post_json("/api/auth/login", &body)).await;

    assert_eq!(response.status(), StatusCode::OK);

    let new_session_id = extract_session_cookie(&response)
        .expect("Login should return a session cookie");

    let json = body_json(response).await;
    assert_eq!(json["user"]["email"], "runner@example.com");

    // New session should work for /me
    let response = send_request(app, get_authed("/api/auth/me", &new_session_id)).await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn register_duplicate_email_returns_409() {
    let app = test_app().await;

    let body = json!({
        "email": "runner@example.com",
        "password": "securepass123"
    });

    // First registration should succeed
    let response = send_request(app.clone(), post_json("/api/auth/register", &body)).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    // Second registration with same email should fail with 409
    let response = send_request(app, post_json("/api/auth/register", &body)).await;
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn login_wrong_password_returns_401() {
    let app = test_app().await;

    // Register
    let (app, _) = register_user(app, "runner@example.com", "securepass123").await;

    // Login with wrong password
    let body = json!({
        "email": "runner@example.com",
        "password": "wrongpassword"
    });
    let response = send_request(app, post_json("/api/auth/login", &body)).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_nonexistent_user_returns_401() {
    let app = test_app().await;

    let body = json!({
        "email": "nobody@example.com",
        "password": "doesnotmatter"
    });
    let response = send_request(app, post_json("/api/auth/login", &body)).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_without_cookie_returns_401() {
    let app = test_app().await;

    let response = send_request(app, get_request("/api/auth/me")).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_with_invalid_session_returns_401() {
    let app = test_app().await;

    let response = send_request(
        app,
        get_authed("/api/auth/me", "not-a-real-session-id"),
    )
    .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn register_short_password_returns_400() {
    let app = test_app().await;

    let body = json!({
        "email": "runner@example.com",
        "password": "short"
    });
    let response = send_request(app, post_json("/api/auth/register", &body)).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let json = body_json(response).await;
    assert!(json["error"]
        .as_str()
        .unwrap()
        .contains("at least 8 characters"));
}

#[tokio::test]
async fn register_invalid_email_returns_400() {
    let app = test_app().await;

    let body = json!({
        "email": "not-an-email",
        "password": "securepass123"
    });
    let response = send_request(app, post_json("/api/auth/register", &body)).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ===========================================================================
// Profile CRUD tests
// ===========================================================================

#[tokio::test]
async fn create_profile_returns_profile_with_zones() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    let profile_body = valid_profile_body();
    let response = send_request(
        app,
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let json = body_json(response).await;

    // Profile fields
    assert_eq!(json["profile"]["name"], "Test Runner");
    assert_eq!(json["profile"]["age"], 30);
    assert_eq!(json["profile"]["weight_kg"], 70.0);
    assert_eq!(json["profile"]["resting_hr"], 50);
    assert_eq!(json["profile"]["max_hr"], 185);
    assert_eq!(json["profile"]["lthr"], 170);
    assert_eq!(json["profile"]["ftpace_m_per_s"], 4.5);
    assert_eq!(json["profile"]["current_weekly_volume_km"], 40.0);
    assert_eq!(json["profile"]["experience_level"], "intermediate");
    assert_eq!(json["profile"]["sports_background"], "cycling");

    // HR zones should be present (7-zone model) — serialized as {"zones": [...]}
    assert!(json["hr_zones"]["zones"].is_array());
    assert_eq!(json["hr_zones"]["zones"].as_array().unwrap().len(), 7);

    // Pace zones should be present when ftpace is provided (6-zone model)
    assert!(json["pace_zones"]["zones"].is_array());
    assert_eq!(json["pace_zones"]["zones"].as_array().unwrap().len(), 6);
}

#[tokio::test]
async fn create_profile_without_ftpace_omits_pace_zones() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    let mut profile_body = valid_profile_body();
    profile_body["ftpace_m_per_s"] = Value::Null;

    let response = send_request(
        app,
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let json = body_json(response).await;
    assert!(json["hr_zones"]["zones"].is_array());
    assert!(json["pace_zones"].is_null());
}

#[tokio::test]
async fn get_profile_returns_same_data() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    // Create
    let profile_body = valid_profile_body();
    let create_response = send_request(
        app.clone(),
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let created = body_json(create_response).await;

    // Get
    let get_response = send_request(
        app,
        get_authed("/api/athlete/profile", &session_id),
    )
    .await;
    assert_eq!(get_response.status(), StatusCode::OK);
    let fetched = body_json(get_response).await;

    // The profile data should match
    assert_eq!(created["profile"]["id"], fetched["profile"]["id"]);
    assert_eq!(created["profile"]["name"], fetched["profile"]["name"]);
    assert_eq!(created["profile"]["age"], fetched["profile"]["age"]);
    assert_eq!(created["profile"]["lthr"], fetched["profile"]["lthr"]);
    assert_eq!(
        created["profile"]["experience_level"],
        fetched["profile"]["experience_level"]
    );

    // Zones should also match
    assert_eq!(created["hr_zones"], fetched["hr_zones"]);
    assert_eq!(created["pace_zones"], fetched["pace_zones"]);
}

#[tokio::test]
async fn update_profile_returns_updated_data() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    // Create profile first
    let profile_body = valid_profile_body();
    let response = send_request(
        app.clone(),
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);

    // Update name and weight
    let update_body = json!({
        "name": "Updated Runner",
        "weight_kg": 68.5
    });
    let response = send_request(
        app.clone(),
        put_json_authed("/api/athlete/profile", &update_body, &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response).await;
    assert_eq!(json["profile"]["name"], "Updated Runner");
    assert_eq!(json["profile"]["weight_kg"], 68.5);
    // Other fields should remain unchanged
    assert_eq!(json["profile"]["age"], 30);
    assert_eq!(json["profile"]["lthr"], 170);
}

#[tokio::test]
async fn update_profile_lthr_changes_zones() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    // Create profile
    let profile_body = valid_profile_body();
    let response = send_request(
        app.clone(),
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let original_zones = body_json(response).await;

    // Update LTHR
    let update_body = json!({ "lthr": 165 });
    let response = send_request(
        app,
        put_json_authed("/api/athlete/profile", &update_body, &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let updated_zones = body_json(response).await;

    // LTHR should be updated
    assert_eq!(updated_zones["profile"]["lthr"], 165);

    // HR zones should differ from original since LTHR changed
    assert_ne!(original_zones["hr_zones"], updated_zones["hr_zones"]);
}

#[tokio::test]
async fn create_profile_without_auth_returns_401() {
    let app = test_app().await;

    let profile_body = valid_profile_body();
    let response = send_request(
        app,
        post_json("/api/athlete/profile", &profile_body),
    )
    .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_profile_without_auth_returns_401() {
    let app = test_app().await;

    let response = send_request(app, get_request("/api/athlete/profile")).await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_duplicate_profile_returns_409() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    let profile_body = valid_profile_body();

    // First creation should succeed
    let response = send_request(
        app.clone(),
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);

    // Second creation should fail with 409
    let response = send_request(
        app,
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn get_profile_when_none_exists_returns_404() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    let response = send_request(
        app,
        get_authed("/api/athlete/profile", &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn update_profile_when_none_exists_returns_404() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    let update_body = json!({ "name": "Updated" });
    let response = send_request(
        app,
        put_json_authed("/api/athlete/profile", &update_body, &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn me_shows_has_profile_true_after_creation() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    // Before profile creation
    let response = send_request(
        app.clone(),
        get_authed("/api/auth/me", &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let json = body_json(response).await;
    assert_eq!(json["has_profile"], false);

    // Create profile
    let profile_body = valid_profile_body();
    let response = send_request(
        app.clone(),
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);

    // After profile creation, /me should report has_profile: true
    let response = send_request(
        app,
        get_authed("/api/auth/me", &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let json = body_json(response).await;
    assert_eq!(json["has_profile"], true);
}

#[tokio::test]
async fn create_profile_with_invalid_experience_returns_400() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    let mut profile_body = valid_profile_body();
    profile_body["experience_level"] = json!("elite");

    let response = send_request(
        app,
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_profile_with_invalid_hr_returns_400() {
    let app = test_app().await;
    let (app, session_id) = register_user(app, "runner@example.com", "securepass123").await;

    // LTHR less than resting HR is invalid
    let mut profile_body = valid_profile_body();
    profile_body["lthr"] = json!(40);

    let response = send_request(
        app,
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ===========================================================================
// Cross-user isolation tests
// ===========================================================================

#[tokio::test]
async fn users_cannot_see_each_others_profiles() {
    let app = test_app().await;

    // Register two users
    let (app, session_a) = register_user(app, "alice@example.com", "password123").await;
    let (app, session_b) = register_user(app, "bob@example.com", "password456").await;

    // Alice creates a profile
    let mut profile = valid_profile_body();
    profile["name"] = json!("Alice");
    let response = send_request(
        app.clone(),
        post_json_authed("/api/athlete/profile", &profile, &session_a),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);

    // Alice can see her profile
    let response = send_request(
        app.clone(),
        get_authed("/api/athlete/profile", &session_a),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);
    let json = body_json(response).await;
    assert_eq!(json["profile"]["name"], "Alice");

    // Bob should get 404 (no profile for Bob yet)
    let response = send_request(
        app,
        get_authed("/api/athlete/profile", &session_b),
    )
    .await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ===========================================================================
// Root endpoint test
// ===========================================================================

#[tokio::test]
async fn root_returns_hello() {
    let app = test_app().await;

    let response = send_request(app, get_request("/")).await;

    assert_eq!(response.status(), StatusCode::OK);
}

// ===========================================================================
// Wiremock helpers
// ===========================================================================

/// Build an Axum app backed by a fresh in-memory SQLite database with a
/// ClaudeClient pointing at the given wiremock server URL.
async fn test_app_with_claude(mock_server_uri: &str) -> Router {
    let connect_options = SqliteConnectOptions::new()
        .filename(":memory:")
        .create_if_missing(true)
        .pragma("foreign_keys", "ON");

    let db = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(connect_options)
        .await
        .expect("Failed to create in-memory SQLite pool");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let client = ClaudeClient::new_with_base_url(
        "test-api-key".to_string(),
        mock_server_uri.to_string(),
    );

    let config = Config {
        database_url: String::new(),
        host: "127.0.0.1".to_string(),
        port: 0,
        anthropic_api_key: Some("test-api-key".to_string()),
    };

    let state = AppState {
        db,
        config,
        claude_client: Some(Arc::new(client)),
    };
    build_app(state)
}

/// Build a Claude API response wrapping a tool_use block.
fn claude_tool_use_response(tool_name: &str, input: Value) -> Value {
    json!({
        "id": "msg_test_123",
        "type": "message",
        "role": "assistant",
        "content": [{
            "type": "tool_use",
            "id": "toolu_test_123",
            "name": tool_name,
            "input": input
        }],
        "model": "claude-sonnet-4-5-20250929",
        "stop_reason": "tool_use",
        "usage": { "input_tokens": 100, "output_tokens": 200 }
    })
}

/// Register user and create an athlete profile (with race goal), returning
/// (app, session_id, race_goal_id).
async fn setup_user_with_profile(app: Router) -> (Router, String, i64) {
    let (app, session_id) =
        register_user(app, "planner@example.com", "securepass123").await;

    let profile_body = valid_profile_body();
    let response = send_request(
        app.clone(),
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let json = body_json(response).await;
    let race_goal_id = json["race_goal"]["id"]
        .as_i64()
        .expect("race_goal should have an id");

    (app, session_id, race_goal_id)
}

// ===========================================================================
// Plan generation integration tests (with wiremock)
// ===========================================================================

#[tokio::test]
async fn generate_plan_returns_macrocycle_skeleton() {
    let mock_server = MockServer::start().await;

    // Mock Claude to return a valid macrocycle skeleton
    let skeleton_input = json!({
        "target_ctl": 55.0,
        "coach_message": "We'll build your aerobic engine first, then sharpen for race day.",
        "mesocycles": [
            {
                "sequence_number": 1,
                "phase": "capacity",
                "focus": "aerobic_capacity",
                "load_weeks": 3,
                "recovery_weeks": 1,
                "target_volume_km": 40.0
            },
            {
                "sequence_number": 2,
                "phase": "utilization",
                "focus": "race_specific",
                "load_weeks": 2,
                "recovery_weeks": 1,
                "target_volume_km": 45.0
            }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(claude_tool_use_response(
                    "generate_macrocycle_skeleton",
                    skeleton_input,
                )),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let app = test_app_with_claude(&mock_server.uri()).await;
    let (app, session_id, race_goal_id) = setup_user_with_profile(app).await;

    // POST /api/plan/generate
    let generate_body = json!({ "race_goal_id": race_goal_id });
    let response = send_request(
        app,
        post_json_authed("/api/plan/generate", &generate_body, &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::CREATED);

    let json = body_json(response).await;
    assert_eq!(json["target_ctl"], 55.0);
    assert!(json["coach_message"]
        .as_str()
        .unwrap()
        .contains("aerobic engine"));
    assert_eq!(json["mesocycles"].as_array().unwrap().len(), 2);
    assert_eq!(json["mesocycles"][0]["phase"], "capacity");
    assert_eq!(json["mesocycles"][1]["phase"], "utilization");
}

#[tokio::test]
async fn full_plan_generation_and_confirm_flow() {
    let mock_server = MockServer::start().await;

    // We need to compute dates the same way the handler will so our mock
    // mesocycle plan returns matching dates.
    let today = chrono::Utc::now().date_naive();

    // Mesocycle 1: 3 load + 1 recovery = 4 weeks
    let meso1_start = today;
    let _meso1_end = today + chrono::Duration::weeks(4) - chrono::Duration::days(1);

    // Build one week of dates starting from meso1_start (a Monday-aligned week)
    // We'll generate 4 weeks of dates for the mesocycle plan mock.
    let mut weeks_json = Vec::new();
    for week_num in 0..4u32 {
        let week_start = meso1_start + chrono::Duration::weeks(week_num as i64);
        let is_recovery = week_num == 3;
        let week_type = if is_recovery { "recovery" } else { "load" };
        let target_vol = if is_recovery { 25.0 } else { 40.0 };
        let target_tss = if is_recovery { 150.0 } else { 250.0 };

        let mut days = Vec::new();
        // Day assignments for a valid week:
        // Mon: easy_run, Tue: strength_power, Wed: tempo_run, Thu: rest,
        // Fri: easy_run, Sat: long_run, Sun: recovery_run
        let day_specs = if is_recovery {
            vec![
                ("easy_run", Some("short")),
                ("rest", None),
                ("easy_run", Some("short")),
                ("rest", None),
                ("easy_run", Some("short")),
                ("recovery_run", Some("short")),
                ("rest", None),
            ]
        } else {
            vec![
                ("easy_run", Some("medium")),
                ("strength_power", None),
                ("tempo_run", Some("medium")),
                ("rest", None),
                ("easy_run", Some("short")),
                ("long_run", Some("medium")),
                ("recovery_run", Some("short")),
            ]
        };

        for (day_offset, (workout_type, duration_category)) in
            day_specs.iter().enumerate()
        {
            let date = week_start + chrono::Duration::days(day_offset as i64);
            let mut day_json = json!({
                "date": date.format("%Y-%m-%d").to_string(),
                "workout_type": workout_type,
            });
            if let Some(dc) = duration_category {
                day_json["duration_category"] = json!(dc);
            }
            days.push(day_json);
        }

        weeks_json.push(json!({
            "week_number": week_num + 1,
            "week_type": week_type,
            "target_volume_km": target_vol,
            "target_weekly_tss": target_tss,
            "days": days
        }));
    }

    let mesocycle_plan_input = json!({
        "mesocycle_overview": "Building aerobic capacity with progressive volume.",
        "weeks": weeks_json
    });

    // Collect all workout dates for coach notes
    let mut all_dates: Vec<String> = Vec::new();
    for week in weeks_json.iter() {
        for day in week["days"].as_array().unwrap() {
            let date = day["date"].as_str().unwrap().to_string();
            let wt = day["workout_type"].as_str().unwrap();
            if wt != "rest" {
                all_dates.push(date);
            }
        }
    }

    let workout_notes: Vec<Value> = all_dates
        .iter()
        .map(|date| {
            json!({
                "date": date,
                "coach_note": format!("Focus on good form today ({})", date)
            })
        })
        .collect();

    let coach_notes_input = json!({
        "mesocycle_overview": "Building aerobic capacity with progressive volume.",
        "workout_notes": workout_notes
    });

    // The skeleton for the generate step
    let skeleton_input = json!({
        "target_ctl": 55.0,
        "coach_message": "We'll build your aerobic engine first, then sharpen for race day.",
        "mesocycles": [
            {
                "sequence_number": 1,
                "phase": "capacity",
                "focus": "aerobic_capacity",
                "load_weeks": 3,
                "recovery_weeks": 1,
                "target_volume_km": 40.0
            },
            {
                "sequence_number": 2,
                "phase": "utilization",
                "focus": "race_specific",
                "load_weeks": 2,
                "recovery_weeks": 1,
                "target_volume_km": 45.0
            }
        ]
    });

    // Mount mocks in sequence:
    // Call 1 (generate): returns skeleton
    // Call 2 (confirm - mesocycle plan): returns mesocycle plan
    // Call 3 (confirm - coach notes): returns coach notes
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(claude_tool_use_response(
                "generate_macrocycle_skeleton",
                skeleton_input.clone(),
            )),
        )
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(claude_tool_use_response(
                "generate_mesocycle_plan",
                mesocycle_plan_input,
            )),
        )
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(claude_tool_use_response(
                "add_coach_notes",
                coach_notes_input,
            )),
        )
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    let app = test_app_with_claude(&mock_server.uri()).await;
    let (app, session_id, race_goal_id) = setup_user_with_profile(app).await;

    // Step 1: Generate the macrocycle skeleton
    let generate_body = json!({ "race_goal_id": race_goal_id });
    let response = send_request(
        app.clone(),
        post_json_authed("/api/plan/generate", &generate_body, &session_id),
    )
    .await;
    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "generate should succeed"
    );

    let skeleton_json = body_json(response).await;
    assert_eq!(skeleton_json["target_ctl"], 55.0);
    assert_eq!(
        skeleton_json["mesocycles"].as_array().unwrap().len(),
        2
    );

    // Step 2: Confirm the plan (which generates mesocycle workouts + coach notes)
    let response = send_request(
        app.clone(),
        post_json_authed("/api/plan/confirm", &skeleton_json, &session_id),
    )
    .await;
    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "confirm should succeed"
    );

    let plan_json = body_json(response).await;

    // Verify the response structure
    assert!(plan_json["macrocycle"]["id"].is_number());
    assert_eq!(plan_json["macrocycle"]["status"], "active");
    assert_eq!(plan_json["macrocycle"]["target_ctl"], 55.0);

    let mesocycles = plan_json["mesocycles"].as_array().unwrap();
    assert_eq!(mesocycles.len(), 2);
    assert_eq!(mesocycles[0]["phase"], "capacity");
    assert_eq!(mesocycles[1]["phase"], "utilization");

    let workouts = plan_json["workouts"].as_array().unwrap();
    // 4 weeks * 7 days = 28 workouts
    assert_eq!(workouts.len(), 28);

    // Verify some workout details
    let first_workout = &workouts[0];
    assert_eq!(first_workout["workout_type"], "easy_run");
    assert!(first_workout["duration_min"].is_number());
    assert!(first_workout["expected_tss"].is_number());

    // Verify coach notes were applied (non-rest workouts should have notes)
    let non_rest_with_notes: Vec<&Value> = workouts
        .iter()
        .filter(|w| {
            w["workout_type"].as_str().unwrap() != "rest"
                && w["coach_notes"].is_string()
        })
        .collect();
    assert!(
        !non_rest_with_notes.is_empty(),
        "Some non-rest workouts should have coach notes"
    );

    // Step 3: Verify GET /api/plan returns the persisted plan
    let response = send_request(
        app,
        get_authed("/api/plan", &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::OK);

    let get_plan_json = body_json(response).await;
    assert!(get_plan_json["macrocycle"]["id"].is_number());
    assert_eq!(get_plan_json["macrocycle"]["status"], "active");
    assert_eq!(
        get_plan_json["mesocycles"].as_array().unwrap().len(),
        2
    );
    assert_eq!(
        get_plan_json["workouts"].as_array().unwrap().len(),
        28
    );
}

#[tokio::test]
async fn generate_plan_without_claude_client_returns_500() {
    // Use the standard test_app which has claude_client: None
    let app = test_app().await;
    let (app, session_id) =
        register_user(app, "noclaudeuser@example.com", "securepass123").await;

    let profile_body = valid_profile_body();
    let response = send_request(
        app.clone(),
        post_json_authed("/api/athlete/profile", &profile_body, &session_id),
    )
    .await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let json = body_json(response).await;
    let race_goal_id = json["race_goal"]["id"].as_i64().unwrap();

    let generate_body = json!({ "race_goal_id": race_goal_id });
    let response = send_request(
        app,
        post_json_authed("/api/plan/generate", &generate_body, &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn generate_plan_without_profile_returns_404() {
    let mock_server = MockServer::start().await;
    let app = test_app_with_claude(&mock_server.uri()).await;

    let (app, session_id) =
        register_user(app, "noprofile@example.com", "securepass123").await;

    // Try to generate without creating a profile first
    let generate_body = json!({ "race_goal_id": 1 });
    let response = send_request(
        app,
        post_json_authed("/api/plan/generate", &generate_body, &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_plan_without_plan_returns_404() {
    let app = test_app().await;
    let (app, session_id) =
        register_user(app, "noplan@example.com", "securepass123").await;

    let response = send_request(
        app,
        get_authed("/api/plan", &session_id),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
