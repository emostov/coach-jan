use axum::http::header::SET_COOKIE;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::api::middleware::AuthUser;
use crate::auth::password;
use crate::db::{profiles, sessions, users};
use crate::error::{AppError, AppResult};
use crate::AppState;

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct MeResponse {
    pub user: UserResponse,
    pub has_profile: bool,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

// ---------------------------------------------------------------------------
// Cookie helpers
// ---------------------------------------------------------------------------

/// Build the Set-Cookie header value to establish a session.
fn session_cookie(session_id: &str) -> String {
    format!(
        "session_id={session_id}; HttpOnly; SameSite=Strict; Path=/; Max-Age=2592000"
    )
}

/// Build the Set-Cookie header value to clear the session cookie.
fn clear_session_cookie() -> String {
    "session_id=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0".to_string()
}

// ---------------------------------------------------------------------------
// Validation helpers
// ---------------------------------------------------------------------------

/// Perform a basic email format check (contains exactly one '@' with non-empty parts).
fn validate_email(email: &str) -> Result<(), AppError> {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() || !parts[1].contains('.') {
        return Err(AppError::BadRequest(
            "Invalid email format".to_string(),
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// POST /api/auth/register
async fn register(
    state: axum::extract::State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> AppResult<impl IntoResponse> {
    // Validate input
    validate_email(&body.email)?;
    if body.password.len() < 8 {
        return Err(AppError::BadRequest(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Hash password
    let hash = password::hash_password(&body.password)?;

    // Create user (returns Conflict if email already exists)
    let user = users::create_user(&state.db, &body.email, &hash).await?;

    // Create session
    let session = sessions::create_session(&state.db, user.id).await?;

    // Return response with session cookie
    let response = (
        StatusCode::CREATED,
        [(SET_COOKIE, session_cookie(&session.id))],
        Json(AuthResponse {
            user: UserResponse {
                id: user.id,
                email: user.email,
            },
        }),
    );

    Ok(response)
}

/// POST /api/auth/login
async fn login(
    state: axum::extract::State<AppState>,
    Json(body): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    // Look up user by email
    let user = users::get_user_by_email(&state.db, &body.email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // Verify password
    let valid = password::verify_password(&body.password, &user.password_hash)?;
    if !valid {
        return Err(AppError::Unauthorized);
    }

    // Create session
    let session = sessions::create_session(&state.db, user.id).await?;

    // Return response with session cookie
    let response = (
        StatusCode::OK,
        [(SET_COOKIE, session_cookie(&session.id))],
        Json(AuthResponse {
            user: UserResponse {
                id: user.id,
                email: user.email,
            },
        }),
    );

    Ok(response)
}

/// POST /api/auth/logout
async fn logout(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
) -> AppResult<impl IntoResponse> {
    // Delete the session
    sessions::delete_session(&state.db, &auth.session_id).await?;

    // Clear cookie and return message
    let response = (
        StatusCode::OK,
        [(SET_COOKIE, clear_session_cookie())],
        Json(MessageResponse {
            message: "Logged out".to_string(),
        }),
    );

    Ok(response)
}

/// GET /api/auth/me
async fn me(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
) -> AppResult<impl IntoResponse> {
    // Check if the user has an athlete profile
    let profile = profiles::get_profile_by_user_id(&state.db, auth.user_id).await?;

    Ok(Json(MeResponse {
        user: UserResponse {
            id: auth.user_id,
            email: auth.email,
        },
        has_profile: profile.is_some(),
    }))
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(me))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_cookie_format() {
        let cookie = session_cookie("test-session-id");
        assert!(cookie.contains("session_id=test-session-id"));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Strict"));
        assert!(cookie.contains("Path=/"));
        assert!(cookie.contains("Max-Age=2592000"));
        // Should NOT contain Secure in development
        assert!(!cookie.contains("Secure"));
    }

    #[test]
    fn test_clear_session_cookie_format() {
        let cookie = clear_session_cookie();
        assert!(cookie.contains("session_id=;"));
        assert!(cookie.contains("Max-Age=0"));
        assert!(cookie.contains("HttpOnly"));
    }

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("a@b.co").is_ok());
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(validate_email("").is_err());
        assert!(validate_email("noat").is_err());
        assert!(validate_email("@noleft.com").is_err());
        assert!(validate_email("noright@").is_err());
        assert!(validate_email("two@@at.com").is_err());
        assert!(validate_email("nodot@example").is_err());
    }
}
