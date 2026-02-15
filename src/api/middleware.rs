use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::AppState;
use crate::db::{sessions, users};
use crate::error::AppError;

/// Authenticated user extracted from the session cookie.
/// Include this as a handler parameter to require authentication.
pub struct AuthUser {
    pub user_id: i64,
    pub email: String,
    /// The session ID, needed for logout to know which session to delete.
    pub session_id: String,
}

/// Parse a named cookie value from a `Cookie` header string.
///
/// The header format is: `name1=value1; name2=value2; ...`
fn parse_cookie(header: &str, name: &str) -> Option<String> {
    header.split(';').find_map(|pair| {
        let pair = pair.trim();
        let (key, value) = pair.split_once('=')?;
        if key.trim() == name {
            Some(value.trim().to_string())
        } else {
            None
        }
    })
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract the Cookie header
        let cookie_header = parts
            .headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        // Parse out the session_id cookie
        let session_id =
            parse_cookie(cookie_header, "session_id").ok_or(AppError::Unauthorized)?;

        // Validate the session (checks expiration)
        let session = sessions::get_valid_session(&state.db, &session_id)
            .await?
            .ok_or(AppError::Unauthorized)?;

        // Load the user
        let user = users::get_user_by_id(&state.db, session.user_id)
            .await?
            .ok_or(AppError::Unauthorized)?;

        Ok(AuthUser {
            user_id: user.id,
            email: user.email,
            session_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cookie_single() {
        let header = "session_id=abc123";
        assert_eq!(parse_cookie(header, "session_id"), Some("abc123".into()));
    }

    #[test]
    fn parse_cookie_multiple() {
        let header = "theme=dark; session_id=abc123; lang=en";
        assert_eq!(parse_cookie(header, "session_id"), Some("abc123".into()));
        assert_eq!(parse_cookie(header, "theme"), Some("dark".into()));
        assert_eq!(parse_cookie(header, "lang"), Some("en".into()));
    }

    #[test]
    fn parse_cookie_missing() {
        let header = "theme=dark; lang=en";
        assert_eq!(parse_cookie(header, "session_id"), None);
    }

    #[test]
    fn parse_cookie_empty_header() {
        assert_eq!(parse_cookie("", "session_id"), None);
    }

    #[test]
    fn parse_cookie_with_spaces() {
        let header = " session_id = abc123 ; theme = dark ";
        assert_eq!(parse_cookie(header, "session_id"), Some("abc123".into()));
    }
}
