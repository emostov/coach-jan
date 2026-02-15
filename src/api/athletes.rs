use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::api::middleware::AuthUser;
use crate::db::profiles::{
    self, AthleteProfile, CreateProfile, CreateRaceGoal, UpdateProfile,
};
use crate::domain::bootstrap::bootstrap_ctl;
use crate::domain::types::{ExperienceLevel, HrZones, PaceZones};
use crate::domain::zones::{calculate_hr_zones, calculate_pace_zones};
use crate::error::{AppError, AppResult};
use crate::AppState;

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    pub name: String,
    pub age: i64,
    pub weight_kg: f64,
    pub resting_hr: i64,
    pub max_hr: i64,
    pub lthr: i64,
    pub ftpace_m_per_s: Option<f64>,
    pub current_weekly_volume_km: f64,
    pub experience_level: String,
    pub sports_background: Option<String>,
    // Race goal
    pub race_name: Option<String>,
    pub race_distance_m: f64,
    pub race_date: String,
    pub target_time_seconds: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub age: Option<i64>,
    pub weight_kg: Option<f64>,
    pub resting_hr: Option<i64>,
    pub max_hr: Option<i64>,
    pub lthr: Option<i64>,
    pub ftpace_m_per_s: Option<f64>,
    pub current_weekly_volume_km: Option<f64>,
    pub experience_level: Option<String>,
    pub sports_background: Option<String>,
}

#[derive(Serialize)]
pub struct ProfileData {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub age: i64,
    pub weight_kg: f64,
    pub resting_hr: i64,
    pub max_hr: i64,
    pub lthr: i64,
    pub ftpace_m_per_s: Option<f64>,
    pub current_weekly_volume_km: f64,
    pub experience_level: String,
    pub sports_background: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct ProfileResponse {
    pub profile: ProfileData,
    pub hr_zones: HrZones,
    pub pace_zones: Option<PaceZones>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn profile_to_data(p: &AthleteProfile) -> ProfileData {
    ProfileData {
        id: p.id,
        user_id: p.user_id,
        name: p.name.clone(),
        age: p.age,
        weight_kg: p.weight_kg,
        resting_hr: p.resting_hr,
        max_hr: p.max_hr,
        lthr: p.lthr,
        ftpace_m_per_s: p.ftpace_m_per_s,
        current_weekly_volume_km: p.current_weekly_volume_km,
        experience_level: p.experience_level.clone(),
        sports_background: p.sports_background.clone(),
        created_at: p.created_at.clone(),
        updated_at: p.updated_at.clone(),
    }
}

fn build_profile_response(profile: &AthleteProfile) -> ProfileResponse {
    let hr_zones = calculate_hr_zones(profile.lthr as u16);
    let pace_zones = profile
        .ftpace_m_per_s
        .map(calculate_pace_zones);

    ProfileResponse {
        profile: profile_to_data(profile),
        hr_zones,
        pace_zones,
    }
}

/// Validate the create profile request fields.
fn validate_create_request(req: &CreateProfileRequest) -> Result<(), AppError> {
    if req.age <= 0 || req.age >= 120 {
        return Err(AppError::BadRequest(
            "Age must be between 1 and 119".to_string(),
        ));
    }
    if req.weight_kg <= 0.0 {
        return Err(AppError::BadRequest(
            "Weight must be greater than 0".to_string(),
        ));
    }
    if req.resting_hr <= 20 || req.resting_hr >= 120 {
        return Err(AppError::BadRequest(
            "Resting heart rate must be between 21 and 119".to_string(),
        ));
    }
    if req.max_hr <= req.resting_hr || req.max_hr >= 250 {
        return Err(AppError::BadRequest(
            "Max heart rate must be greater than resting HR and less than 250".to_string(),
        ));
    }
    if req.lthr <= req.resting_hr || req.lthr >= req.max_hr {
        return Err(AppError::BadRequest(
            "LTHR must be between resting HR and max HR".to_string(),
        ));
    }
    if req.current_weekly_volume_km < 0.0 {
        return Err(AppError::BadRequest(
            "Weekly volume must be non-negative".to_string(),
        ));
    }
    if ExperienceLevel::from_str(&req.experience_level).is_none() {
        return Err(AppError::BadRequest(
            "Experience level must be one of: beginner, intermediate, advanced".to_string(),
        ));
    }
    if req.race_distance_m <= 0.0 {
        return Err(AppError::BadRequest(
            "Race distance must be greater than 0".to_string(),
        ));
    }
    if req.race_date.is_empty() {
        return Err(AppError::BadRequest(
            "Race date must not be empty".to_string(),
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// POST /api/athlete/profile
async fn create_athlete_profile(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
    Json(body): Json<CreateProfileRequest>,
) -> AppResult<impl IntoResponse> {
    // Validate input
    validate_create_request(&body)?;

    let experience_level = ExperienceLevel::from_str(&body.experience_level)
        .ok_or_else(|| {
            AppError::BadRequest(
                "Invalid experience level".to_string(),
            )
        })?;

    // 1. Create profile
    let create = CreateProfile {
        user_id: auth.user_id,
        name: body.name,
        age: body.age,
        weight_kg: body.weight_kg,
        resting_hr: body.resting_hr,
        max_hr: body.max_hr,
        lthr: body.lthr,
        ftpace_m_per_s: body.ftpace_m_per_s,
        current_weekly_volume_km: body.current_weekly_volume_km,
        experience_level: body.experience_level,
        sports_background: body.sports_background,
    };

    let profile = profiles::create_profile(&state.db, &create).await?;

    // 2. Create race goal
    let race_goal = CreateRaceGoal {
        user_id: auth.user_id,
        race_name: body.race_name,
        distance_m: body.race_distance_m,
        race_date: body.race_date,
        target_time_seconds: body.target_time_seconds,
    };
    profiles::create_race_goal(&state.db, &race_goal).await?;

    // 3. Record FTPace history if provided
    if let Some(ftpace) = body.ftpace_m_per_s {
        profiles::create_ftpace_entry(&state.db, auth.user_id, ftpace, "estimate")
            .await?;
    }

    // 4. Record LTHR history
    profiles::create_lthr_entry(&state.db, auth.user_id, body.lthr, "estimate")
        .await?;

    // 5. Bootstrap CTL/ATL and create initial daily_metrics
    let (ctl, atl) = bootstrap_ctl(body.current_weekly_volume_km, &experience_level);
    let tsb = ctl - atl; // 0 at bootstrap since CTL == ATL
    let today = Utc::now().format("%Y-%m-%d").to_string();
    profiles::create_daily_metrics(&state.db, auth.user_id, &today, 0.0, atl, ctl, tsb)
        .await?;

    // 6. Build response with zones
    let response = build_profile_response(&profile);

    Ok((StatusCode::CREATED, Json(response)))
}

/// GET /api/athlete/profile
async fn get_athlete_profile(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
) -> AppResult<impl IntoResponse> {
    let profile = profiles::get_profile_by_user_id(&state.db, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("No athlete profile found".to_string()))?;

    let response = build_profile_response(&profile);

    Ok(Json(response))
}

/// PUT /api/athlete/profile
async fn update_athlete_profile(
    state: axum::extract::State<AppState>,
    auth: AuthUser,
    Json(body): Json<UpdateProfileRequest>,
) -> AppResult<impl IntoResponse> {
    // Load current profile to compare values
    let current = profiles::get_profile_by_user_id(&state.db, auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("No athlete profile found".to_string()))?;

    // Validate experience_level if provided
    if let Some(ref level_str) = body.experience_level {
        if ExperienceLevel::from_str(level_str).is_none() {
            return Err(AppError::BadRequest(
                "Experience level must be one of: beginner, intermediate, advanced".to_string(),
            ));
        }
    }

    // 1. Update profile
    let update = UpdateProfile {
        name: body.name,
        age: body.age,
        weight_kg: body.weight_kg,
        resting_hr: body.resting_hr,
        max_hr: body.max_hr,
        lthr: body.lthr,
        ftpace_m_per_s: body.ftpace_m_per_s,
        current_weekly_volume_km: body.current_weekly_volume_km,
        experience_level: body.experience_level,
        sports_background: body.sports_background,
    };

    let updated = profiles::update_profile(&state.db, auth.user_id, &update).await?;

    // 2. If LTHR changed, record LTHR history entry
    if let Some(new_lthr) = update.lthr {
        if new_lthr != current.lthr {
            profiles::create_lthr_entry(&state.db, auth.user_id, new_lthr, "manual")
                .await?;
        }
    }

    // 3. If FTPace changed, record FTPace history entry
    if let Some(new_ftpace) = update.ftpace_m_per_s {
        let ftpace_changed = match current.ftpace_m_per_s {
            Some(old) => (new_ftpace - old).abs() > f64::EPSILON,
            None => true, // Was None, now has a value
        };
        if ftpace_changed {
            profiles::create_ftpace_entry(
                &state.db,
                auth.user_id,
                new_ftpace,
                "manual",
            )
            .await?;
        }
    }

    // 4. Build response with updated zones
    let response = build_profile_response(&updated);

    Ok(Json(response))
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/profile",
            get(get_athlete_profile)
                .post(create_athlete_profile)
                .put(update_athlete_profile),
        )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_create_request_valid() {
        let req = CreateProfileRequest {
            name: "Test Runner".into(),
            age: 30,
            weight_kg: 70.0,
            resting_hr: 50,
            max_hr: 185,
            lthr: 170,
            ftpace_m_per_s: Some(4.5),
            current_weekly_volume_km: 40.0,
            experience_level: "intermediate".into(),
            sports_background: None,
            race_name: Some("Test Race".into()),
            race_distance_m: 42195.0,
            race_date: "2026-09-27".into(),
            target_time_seconds: Some(12600),
        };
        assert!(validate_create_request(&req).is_ok());
    }

    #[test]
    fn test_validate_create_request_bad_age() {
        let req = CreateProfileRequest {
            name: "Test".into(),
            age: 0,
            weight_kg: 70.0,
            resting_hr: 50,
            max_hr: 185,
            lthr: 170,
            ftpace_m_per_s: None,
            current_weekly_volume_km: 40.0,
            experience_level: "beginner".into(),
            sports_background: None,
            race_name: None,
            race_distance_m: 5000.0,
            race_date: "2026-06-01".into(),
            target_time_seconds: None,
        };
        assert!(validate_create_request(&req).is_err());
    }

    #[test]
    fn test_validate_create_request_bad_experience() {
        let req = CreateProfileRequest {
            name: "Test".into(),
            age: 30,
            weight_kg: 70.0,
            resting_hr: 50,
            max_hr: 185,
            lthr: 170,
            ftpace_m_per_s: None,
            current_weekly_volume_km: 40.0,
            experience_level: "elite".into(),
            sports_background: None,
            race_name: None,
            race_distance_m: 5000.0,
            race_date: "2026-06-01".into(),
            target_time_seconds: None,
        };
        assert!(validate_create_request(&req).is_err());
    }

    #[test]
    fn test_validate_create_request_bad_hr_relationships() {
        // LTHR must be > resting_hr
        let req = CreateProfileRequest {
            name: "Test".into(),
            age: 30,
            weight_kg: 70.0,
            resting_hr: 50,
            max_hr: 185,
            lthr: 40, // less than resting_hr
            ftpace_m_per_s: None,
            current_weekly_volume_km: 40.0,
            experience_level: "beginner".into(),
            sports_background: None,
            race_name: None,
            race_distance_m: 5000.0,
            race_date: "2026-06-01".into(),
            target_time_seconds: None,
        };
        assert!(validate_create_request(&req).is_err());
    }

    #[test]
    fn test_validate_create_request_empty_race_date() {
        let req = CreateProfileRequest {
            name: "Test".into(),
            age: 30,
            weight_kg: 70.0,
            resting_hr: 50,
            max_hr: 185,
            lthr: 170,
            ftpace_m_per_s: None,
            current_weekly_volume_km: 40.0,
            experience_level: "beginner".into(),
            sports_background: None,
            race_name: None,
            race_distance_m: 5000.0,
            race_date: "".into(),
            target_time_seconds: None,
        };
        assert!(validate_create_request(&req).is_err());
    }

    #[test]
    fn test_build_profile_response_with_ftpace() {
        let profile = AthleteProfile {
            id: 1,
            user_id: 1,
            name: "Test".into(),
            age: 30,
            weight_kg: 70.0,
            resting_hr: 50,
            max_hr: 185,
            lthr: 170,
            ftpace_m_per_s: Some(4.0),
            current_weekly_volume_km: 40.0,
            experience_level: "intermediate".into(),
            sports_background: None,
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-01-01T00:00:00Z".into(),
        };

        let resp = build_profile_response(&profile);
        assert_eq!(resp.hr_zones.len(), 7);
        assert!(resp.pace_zones.is_some());
        assert_eq!(resp.pace_zones.unwrap().len(), 6);
    }

    #[test]
    fn test_build_profile_response_without_ftpace() {
        let profile = AthleteProfile {
            id: 1,
            user_id: 1,
            name: "Test".into(),
            age: 30,
            weight_kg: 70.0,
            resting_hr: 50,
            max_hr: 185,
            lthr: 170,
            ftpace_m_per_s: None,
            current_weekly_volume_km: 40.0,
            experience_level: "beginner".into(),
            sports_background: None,
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-01-01T00:00:00Z".into(),
        };

        let resp = build_profile_response(&profile);
        assert_eq!(resp.hr_zones.len(), 7);
        assert!(resp.pace_zones.is_none());
    }
}
