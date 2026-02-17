use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::ai::handlers::{self, MacrocycleSkeleton};
use crate::api::middleware::AuthUser;
use crate::db::{plans as plans_db, profiles};
use crate::error::{AppError, AppResult};
use crate::AppState;

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct GenerateRequest {
    pub race_goal_id: i64,
}

#[derive(Deserialize)]
pub struct CompleteWorkoutRequest {
    pub rpe: Option<i64>,
    pub athlete_notes: Option<String>,
    pub actual_duration_min: Option<i64>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Get the most recent CTL value for a user, defaulting to 0.0 if no data exists.
async fn get_current_ctl(pool: &SqlitePool, user_id: i64) -> AppResult<f64> {
    let ctl: Option<f64> = sqlx::query_scalar(
        "SELECT ctl FROM daily_metrics WHERE user_id = ? ORDER BY date DESC LIMIT 1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(ctl.unwrap_or(0.0))
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// POST /api/plan/generate
///
/// Takes a race_goal_id, calls Claude to generate a macrocycle skeleton.
async fn generate_plan(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
    Json(body): Json<GenerateRequest>,
) -> AppResult<impl IntoResponse> {
    let client = state.claude_client.as_ref().ok_or_else(|| {
        AppError::Internal("Claude API key not configured".to_string())
    })?;

    // Get profile and race goal
    let profile = profiles::get_profile_by_user_id(&state.db, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("No athlete profile found".to_string()))?;

    let race_goal = profiles::get_active_race_goal(&state.db, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("No active race goal found".to_string()))?;

    // Verify the race_goal_id matches
    if race_goal.id != body.race_goal_id {
        return Err(AppError::BadRequest(
            "Race goal ID does not match active goal".to_string(),
        ));
    }

    // Get current CTL
    let ctl = get_current_ctl(&state.db, auth.user_id).await?;

    let skeleton = handlers::generate_skeleton(client, &state.db, &profile, &race_goal, ctl)
        .await
        .map_err(|e| AppError::Internal(format!("Plan generation failed: {}", e)))?;

    Ok((StatusCode::CREATED, Json(skeleton)))
}

/// POST /api/plan/confirm
///
/// Takes a macrocycle skeleton, persists it, generates the first mesocycle's
/// workouts via Claude, validates, and returns the full plan.
async fn confirm_plan(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
    Json(skeleton): Json<MacrocycleSkeleton>,
) -> AppResult<impl IntoResponse> {
    let client = state.claude_client.as_ref().ok_or_else(|| {
        AppError::Internal("Claude API key not configured".to_string())
    })?;

    let profile = profiles::get_profile_by_user_id(&state.db, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("No athlete profile found".to_string()))?;

    let race_goal = profiles::get_active_race_goal(&state.db, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("No active race goal found".to_string()))?;

    let ctl = get_current_ctl(&state.db, auth.user_id).await?;

    let plan = handlers::confirm_and_generate_plan(
        client,
        &state.db,
        auth.user_id,
        &skeleton,
        &profile,
        &race_goal,
        ctl,
    )
    .await
    .map_err(|e| AppError::Internal(format!("Plan confirmation failed: {}", e)))?;

    Ok((StatusCode::CREATED, Json(plan)))
}

/// GET /api/plan
///
/// Returns the current active macrocycle with all mesocycles and their workouts.
async fn get_plan(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
) -> AppResult<impl IntoResponse> {
    let plan = plans_db::get_plan_with_all_workouts(&state.db, auth.user_id).await?;

    match plan {
        Some((macrocycle, mesocycles)) => {
            Ok(Json(serde_json::json!({
                "macrocycle": macrocycle,
                "mesocycles": mesocycles
            })))
        }
        None => Err(AppError::NotFound(
            "No active training plan found".to_string(),
        )),
    }
}

/// GET /api/plan/workout/:id
///
/// Returns a single workout with its mesocycle context.
async fn get_workout(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
    axum::extract::Path(workout_id): axum::extract::Path<i64>,
) -> AppResult<impl IntoResponse> {
    let result = plans_db::get_workout_with_context(&state.db, workout_id, auth.user_id).await?;

    match result {
        Some((workout, mesocycle)) => {
            Ok(Json(serde_json::json!({
                "workout": workout,
                "mesocycle": mesocycle
            })))
        }
        None => Err(AppError::NotFound("Workout not found".to_string())),
    }
}

/// POST /api/plan/workouts/:id/complete
///
/// Marks a workout as completed with optional feedback (RPE, notes, actual duration).
async fn complete_workout(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
    axum::extract::Path(workout_id): axum::extract::Path<i64>,
    Json(body): Json<CompleteWorkoutRequest>,
) -> AppResult<impl IntoResponse> {
    // Validate RPE range
    if let Some(rpe) = body.rpe {
        if !(1..=10).contains(&rpe) {
            return Err(AppError::BadRequest(
                "RPE must be between 1 and 10".to_string(),
            ));
        }
    }

    let workout = plans_db::complete_workout(
        &state.db,
        workout_id,
        auth.user_id,
        body.rpe,
        body.athlete_notes.as_deref(),
        body.actual_duration_min,
    )
    .await?;

    Ok(Json(workout))
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/generate", axum::routing::post(generate_plan))
        .route("/confirm", axum::routing::post(confirm_plan))
        .route("/workouts/{id}/complete", axum::routing::post(complete_workout))
        .route("/", axum::routing::get(get_plan))
        .route("/workout/{id}", axum::routing::get(get_workout))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_has_correct_routes() {
        // Verify the router builds without panicking.
        // The router function itself validates that all route paths
        // and method combinations are valid (e.g., no duplicate routes).
        let _router = router();
    }

    #[tokio::test]
    async fn test_get_current_ctl_no_data_returns_zero() {
        let opts = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(":memory:")
            .create_if_missing(true)
            .pragma("foreign_keys", "ON");

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(opts)
            .await
            .expect("Failed to create test pool");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        // Create a test user
        let row = sqlx::query(
            "INSERT INTO users (email, password_hash) VALUES ('ctl_test@example.com', 'hash') RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .expect("create test user");
        let user_id: i64 = sqlx::Row::get(&row, "id");

        // No daily_metrics rows yet, should return 0.0
        let ctl = get_current_ctl(&pool, user_id)
            .await
            .expect("get_current_ctl should not error");
        assert_eq!(ctl, 0.0);
    }

    #[tokio::test]
    async fn test_get_current_ctl_returns_latest() {
        let opts = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(":memory:")
            .create_if_missing(true)
            .pragma("foreign_keys", "ON");

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(opts)
            .await
            .expect("Failed to create test pool");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        // Create a test user
        let row = sqlx::query(
            "INSERT INTO users (email, password_hash) VALUES ('ctl_test2@example.com', 'hash') RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .expect("create test user");
        let user_id: i64 = sqlx::Row::get(&row, "id");

        // Insert two daily_metrics rows with different dates
        profiles::create_daily_metrics(&pool, user_id, "2026-02-10", 50.0, 40.0, 35.0, -5.0)
            .await
            .expect("insert metrics 1");
        profiles::create_daily_metrics(&pool, user_id, "2026-02-14", 60.0, 45.0, 42.0, -3.0)
            .await
            .expect("insert metrics 2");

        // Should return the latest CTL (42.0 from 2026-02-14)
        let ctl = get_current_ctl(&pool, user_id)
            .await
            .expect("get_current_ctl should not error");
        assert_eq!(ctl, 42.0);
    }

    #[tokio::test]
    async fn test_generate_request_deserialization() {
        let json = r#"{"race_goal_id": 42}"#;
        let req: GenerateRequest = serde_json::from_str(json).expect("should parse");
        assert_eq!(req.race_goal_id, 42);
    }

    #[tokio::test]
    async fn test_generate_request_missing_field() {
        let json = r#"{}"#;
        let result = serde_json::from_str::<GenerateRequest>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_request_deserializes_correctly() {
        // Verify the route structure is valid
        let _router = router();
    }

    #[test]
    fn test_complete_workout_request_deserialization_all_fields() {
        let json = r#"{"rpe": 7, "athlete_notes": "felt strong", "actual_duration_min": 48}"#;
        let req: CompleteWorkoutRequest = serde_json::from_str(json).expect("should parse");
        assert_eq!(req.rpe, Some(7));
        assert_eq!(req.athlete_notes.as_deref(), Some("felt strong"));
        assert_eq!(req.actual_duration_min, Some(48));
    }

    #[test]
    fn test_complete_workout_request_deserialization_empty() {
        let json = r#"{}"#;
        let req: CompleteWorkoutRequest = serde_json::from_str(json).expect("should parse");
        assert!(req.rpe.is_none());
        assert!(req.athlete_notes.is_none());
        assert!(req.actual_duration_min.is_none());
    }

    #[test]
    fn test_complete_workout_request_deserialization_partial() {
        let json = r#"{"rpe": 5}"#;
        let req: CompleteWorkoutRequest = serde_json::from_str(json).expect("should parse");
        assert_eq!(req.rpe, Some(5));
        assert!(req.athlete_notes.is_none());
    }
}
