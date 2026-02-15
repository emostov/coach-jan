use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Database(e) => {
                tracing::error!("Database error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {msg}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = axum::Json(json!({ "error": message }));
        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use http_body_util::BodyExt;

    async fn status_of(error: AppError) -> StatusCode {
        error.into_response().status()
    }

    async fn body_of(error: AppError) -> String {
        let response = error.into_response();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        String::from_utf8(body.to_vec()).unwrap()
    }

    #[tokio::test]
    async fn test_not_found_status() {
        assert_eq!(
            status_of(AppError::NotFound("thing".into())).await,
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn test_bad_request_status() {
        assert_eq!(
            status_of(AppError::BadRequest("invalid".into())).await,
            StatusCode::BAD_REQUEST
        );
    }

    #[tokio::test]
    async fn test_unauthorized_status() {
        assert_eq!(
            status_of(AppError::Unauthorized).await,
            StatusCode::UNAUTHORIZED
        );
    }

    #[tokio::test]
    async fn test_conflict_status() {
        assert_eq!(
            status_of(AppError::Conflict("exists".into())).await,
            StatusCode::CONFLICT
        );
    }

    #[tokio::test]
    async fn test_internal_status() {
        assert_eq!(
            status_of(AppError::Internal("oops".into())).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn test_error_body_is_json() {
        let body = body_of(AppError::NotFound("missing item".into())).await;
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["error"], "missing item");
    }

    #[tokio::test]
    async fn test_internal_error_hides_message() {
        let body = body_of(AppError::Internal("secret details".into())).await;
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed["error"], "Internal server error");
    }
}
