use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Row};

use crate::error::{AppError, AppResult};

// ---------------------------------------------------------------------------
// AthleteProfile
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct AthleteProfile {
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

#[derive(Debug, Deserialize)]
pub struct CreateProfile {
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
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfile {
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

/// Create a new athlete profile. Returns `AppError::Conflict` if the user already has a profile.
pub async fn create_profile(
    pool: &SqlitePool,
    profile: &CreateProfile,
) -> AppResult<AthleteProfile> {
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"INSERT INTO athlete_profiles
            (user_id, name, age, weight_kg, resting_hr, max_hr, lthr, ftpace_m_per_s,
             current_weekly_volume_km, experience_level, sports_background, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
           RETURNING id, user_id, name, age, weight_kg, resting_hr, max_hr, lthr,
                     ftpace_m_per_s, current_weekly_volume_km, experience_level,
                     sports_background, created_at, updated_at"#,
    )
    .bind(profile.user_id)
    .bind(&profile.name)
    .bind(profile.age)
    .bind(profile.weight_kg)
    .bind(profile.resting_hr)
    .bind(profile.max_hr)
    .bind(profile.lthr)
    .bind(profile.ftpace_m_per_s)
    .bind(profile.current_weekly_volume_km)
    .bind(&profile.experience_level)
    .bind(&profile.sports_background)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => Ok(AthleteProfile {
            id: row.get("id"),
            user_id: row.get("user_id"),
            name: row.get("name"),
            age: row.get("age"),
            weight_kg: row.get("weight_kg"),
            resting_hr: row.get("resting_hr"),
            max_hr: row.get("max_hr"),
            lthr: row.get("lthr"),
            ftpace_m_per_s: row.get("ftpace_m_per_s"),
            current_weekly_volume_km: row.get("current_weekly_volume_km"),
            experience_level: row.get("experience_level"),
            sports_background: row.get("sports_background"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }),
        Err(sqlx::Error::Database(db_err)) if db_err.message().contains("UNIQUE constraint") => {
            Err(AppError::Conflict(
                "Athlete profile already exists for this user".to_string(),
            ))
        }
        Err(e) => Err(AppError::Database(e)),
    }
}

/// Get an athlete profile by user ID. Returns `None` if no profile exists.
pub async fn get_profile_by_user_id(
    pool: &SqlitePool,
    user_id: i64,
) -> AppResult<Option<AthleteProfile>> {
    let profile = sqlx::query_as::<_, AthleteProfile>(
        r#"SELECT id, user_id, name, age, weight_kg, resting_hr, max_hr, lthr,
                  ftpace_m_per_s, current_weekly_volume_km, experience_level,
                  sports_background, created_at, updated_at
           FROM athlete_profiles WHERE user_id = ?"#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(profile)
}

/// Update an athlete profile. Only provided (Some) fields are updated.
/// Returns `AppError::NotFound` if no profile exists for this user.
pub async fn update_profile(
    pool: &SqlitePool,
    user_id: i64,
    update: &UpdateProfile,
) -> AppResult<AthleteProfile> {
    let now = Utc::now().to_rfc3339();

    // Build SET clauses dynamically for provided fields
    let mut sets: Vec<String> = Vec::new();

    if update.name.is_some() {
        sets.push("name = ?".to_string());
    }
    if update.age.is_some() {
        sets.push("age = ?".to_string());
    }
    if update.weight_kg.is_some() {
        sets.push("weight_kg = ?".to_string());
    }
    if update.resting_hr.is_some() {
        sets.push("resting_hr = ?".to_string());
    }
    if update.max_hr.is_some() {
        sets.push("max_hr = ?".to_string());
    }
    if update.lthr.is_some() {
        sets.push("lthr = ?".to_string());
    }
    if update.ftpace_m_per_s.is_some() {
        sets.push("ftpace_m_per_s = ?".to_string());
    }
    if update.current_weekly_volume_km.is_some() {
        sets.push("current_weekly_volume_km = ?".to_string());
    }
    if update.experience_level.is_some() {
        sets.push("experience_level = ?".to_string());
    }
    if update.sports_background.is_some() {
        sets.push("sports_background = ?".to_string());
    }

    if sets.is_empty() {
        // Nothing to update; just return the current profile
        return get_profile_by_user_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("No athlete profile found".to_string()));
    }

    // Always update updated_at
    sets.push("updated_at = ?".to_string());

    let sql = format!(
        r#"UPDATE athlete_profiles SET {} WHERE user_id = ?
           RETURNING id, user_id, name, age, weight_kg, resting_hr, max_hr, lthr,
                     ftpace_m_per_s, current_weekly_volume_km, experience_level,
                     sports_background, created_at, updated_at"#,
        sets.join(", ")
    );

    // Build query with dynamic binds in the same order as SET clauses
    let mut query = sqlx::query(&sql);

    if let Some(ref v) = update.name {
        query = query.bind(v);
    }
    if let Some(v) = update.age {
        query = query.bind(v);
    }
    if let Some(v) = update.weight_kg {
        query = query.bind(v);
    }
    if let Some(v) = update.resting_hr {
        query = query.bind(v);
    }
    if let Some(v) = update.max_hr {
        query = query.bind(v);
    }
    if let Some(v) = update.lthr {
        query = query.bind(v);
    }
    if let Some(v) = update.ftpace_m_per_s {
        query = query.bind(v);
    }
    if let Some(v) = update.current_weekly_volume_km {
        query = query.bind(v);
    }
    if let Some(ref v) = update.experience_level {
        query = query.bind(v);
    }
    if let Some(ref v) = update.sports_background {
        query = query.bind(v);
    }

    // Bind updated_at and the WHERE user_id
    query = query.bind(&now);
    query = query.bind(user_id);

    let row = query
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("No athlete profile found".to_string()))?;

    Ok(AthleteProfile {
        id: row.get("id"),
        user_id: row.get("user_id"),
        name: row.get("name"),
        age: row.get("age"),
        weight_kg: row.get("weight_kg"),
        resting_hr: row.get("resting_hr"),
        max_hr: row.get("max_hr"),
        lthr: row.get("lthr"),
        ftpace_m_per_s: row.get("ftpace_m_per_s"),
        current_weekly_volume_km: row.get("current_weekly_volume_km"),
        experience_level: row.get("experience_level"),
        sports_background: row.get("sports_background"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

// ---------------------------------------------------------------------------
// RaceGoal
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct RaceGoal {
    pub id: i64,
    pub user_id: i64,
    pub race_name: Option<String>,
    pub distance_m: f64,
    pub race_date: String,
    pub target_time_seconds: Option<i64>,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateRaceGoal {
    pub user_id: i64,
    pub race_name: Option<String>,
    pub distance_m: f64,
    pub race_date: String,
    pub target_time_seconds: Option<i64>,
}

/// Create a new race goal. The goal is active by default.
pub async fn create_race_goal(
    pool: &SqlitePool,
    goal: &CreateRaceGoal,
) -> AppResult<RaceGoal> {
    let now = Utc::now().to_rfc3339();

    let row = sqlx::query(
        r#"INSERT INTO race_goals (user_id, race_name, distance_m, race_date, target_time_seconds, created_at)
           VALUES (?, ?, ?, ?, ?, ?)
           RETURNING id, user_id, race_name, distance_m, race_date, target_time_seconds, is_active, created_at"#,
    )
    .bind(goal.user_id)
    .bind(&goal.race_name)
    .bind(goal.distance_m)
    .bind(&goal.race_date)
    .bind(goal.target_time_seconds)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(RaceGoal {
        id: row.get("id"),
        user_id: row.get("user_id"),
        race_name: row.get("race_name"),
        distance_m: row.get("distance_m"),
        race_date: row.get("race_date"),
        target_time_seconds: row.get("target_time_seconds"),
        is_active: row.get::<i32, _>("is_active") != 0,
        created_at: row.get("created_at"),
    })
}

/// Get the most recent active race goal for a user. Returns `None` if no active goal exists.
pub async fn get_active_race_goal(
    pool: &SqlitePool,
    user_id: i64,
) -> AppResult<Option<RaceGoal>> {
    let row = sqlx::query(
        r#"SELECT id, user_id, race_name, distance_m, race_date, target_time_seconds, is_active, created_at
           FROM race_goals WHERE user_id = ? AND is_active = 1
           ORDER BY created_at DESC LIMIT 1"#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| RaceGoal {
        id: r.get("id"),
        user_id: r.get("user_id"),
        race_name: r.get("race_name"),
        distance_m: r.get("distance_m"),
        race_date: r.get("race_date"),
        target_time_seconds: r.get("target_time_seconds"),
        is_active: r.get::<i32, _>("is_active") != 0,
        created_at: r.get("created_at"),
    }))
}

// ---------------------------------------------------------------------------
// Physiological history helpers
// ---------------------------------------------------------------------------

/// Record a new FTPace measurement in the history table.
pub async fn create_ftpace_entry(
    pool: &SqlitePool,
    user_id: i64,
    pace: f64,
    source: &str,
) -> AppResult<()> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO ftpace_history (user_id, pace_m_per_s, source, recorded_at) VALUES (?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(pace)
    .bind(source)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(())
}

/// Record a new LTHR measurement in the history table.
pub async fn create_lthr_entry(
    pool: &SqlitePool,
    user_id: i64,
    lthr: i64,
    source: &str,
) -> AppResult<()> {
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO lthr_history (user_id, lthr, source, recorded_at) VALUES (?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(lthr)
    .bind(source)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Daily metrics
// ---------------------------------------------------------------------------

/// Insert or replace a daily metrics entry (TSS, ATL, CTL, TSB) for a given date.
/// Uses INSERT OR REPLACE to handle the UNIQUE(user_id, date) constraint.
pub async fn create_daily_metrics(
    pool: &SqlitePool,
    user_id: i64,
    date: &str,
    tss: f64,
    atl: f64,
    ctl: f64,
    tsb: f64,
) -> AppResult<()> {
    sqlx::query(
        r#"INSERT OR REPLACE INTO daily_metrics (user_id, date, total_tss, atl, ctl, tsb)
           VALUES (?, ?, ?, ?, ?, ?)"#,
    )
    .bind(user_id)
    .bind(date)
    .bind(tss)
    .bind(atl)
    .bind(ctl)
    .bind(tsb)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

    async fn setup_pool() -> SqlitePool {
        let opts = SqliteConnectOptions::new()
            .filename(":memory:")
            .create_if_missing(true)
            .pragma("foreign_keys", "ON");

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(opts)
            .await
            .expect("Failed to create test pool");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    async fn create_test_user(pool: &SqlitePool) -> i64 {
        let row = sqlx::query(
            "INSERT INTO users (email, password_hash) VALUES ('profile@example.com', 'hash') RETURNING id",
        )
        .fetch_one(pool)
        .await
        .expect("create test user");
        row.get("id")
    }

    fn test_create_profile_input(user_id: i64) -> CreateProfile {
        CreateProfile {
            user_id,
            name: "Test Runner".to_string(),
            age: 30,
            weight_kg: 70.0,
            resting_hr: 50,
            max_hr: 185,
            lthr: 170,
            ftpace_m_per_s: Some(4.5),
            current_weekly_volume_km: 40.0,
            experience_level: "intermediate".to_string(),
            sports_background: Some("soccer, cycling".to_string()),
        }
    }

    #[tokio::test]
    async fn test_create_and_get_profile() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        let input = test_create_profile_input(user_id);
        let profile = create_profile(&pool, &input)
            .await
            .expect("create_profile should succeed");

        assert_eq!(profile.user_id, user_id);
        assert_eq!(profile.name, "Test Runner");
        assert_eq!(profile.age, 30);
        assert_eq!(profile.experience_level, "intermediate");

        let found = get_profile_by_user_id(&pool, user_id)
            .await
            .expect("get_profile should succeed");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test Runner");
    }

    #[tokio::test]
    async fn test_duplicate_profile_returns_conflict() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        let input = test_create_profile_input(user_id);
        create_profile(&pool, &input)
            .await
            .expect("first create should succeed");

        let result = create_profile(&pool, &input).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Conflict(msg) => {
                assert!(msg.contains("already exists"));
            }
            other => panic!("Expected Conflict, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_update_profile() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        let input = test_create_profile_input(user_id);
        create_profile(&pool, &input)
            .await
            .expect("create should succeed");

        let update = UpdateProfile {
            name: Some("Updated Runner".to_string()),
            age: Some(31),
            weight_kg: None,
            resting_hr: None,
            max_hr: None,
            lthr: None,
            ftpace_m_per_s: None,
            current_weekly_volume_km: None,
            experience_level: None,
            sports_background: None,
        };

        let updated = update_profile(&pool, user_id, &update)
            .await
            .expect("update should succeed");

        assert_eq!(updated.name, "Updated Runner");
        assert_eq!(updated.age, 31);
        // Unchanged field should remain
        assert_eq!(updated.weight_kg, 70.0);
    }

    #[tokio::test]
    async fn test_update_nonexistent_profile_returns_not_found() {
        let pool = setup_pool().await;

        let update = UpdateProfile {
            name: Some("Ghost".to_string()),
            age: None,
            weight_kg: None,
            resting_hr: None,
            max_hr: None,
            lthr: None,
            ftpace_m_per_s: None,
            current_weekly_volume_km: None,
            experience_level: None,
            sports_background: None,
        };

        let result = update_profile(&pool, 9999, &update).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(_) => {}
            other => panic!("Expected NotFound, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_create_and_get_race_goal() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        let goal_input = CreateRaceGoal {
            user_id,
            race_name: Some("Berlin Marathon".to_string()),
            distance_m: 42195.0,
            race_date: "2026-09-27".to_string(),
            target_time_seconds: Some(12600), // 3:30:00
        };

        let goal = create_race_goal(&pool, &goal_input)
            .await
            .expect("create_race_goal should succeed");

        assert_eq!(goal.user_id, user_id);
        assert_eq!(goal.distance_m, 42195.0);
        assert!(goal.is_active);

        let active = get_active_race_goal(&pool, user_id)
            .await
            .expect("get_active_race_goal should succeed");
        assert!(active.is_some());
        assert_eq!(active.unwrap().race_name.as_deref(), Some("Berlin Marathon"));
    }

    #[tokio::test]
    async fn test_no_active_race_goal_returns_none() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        let result = get_active_race_goal(&pool, user_id)
            .await
            .expect("should not error");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_create_ftpace_entry() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        create_ftpace_entry(&pool, user_id, 4.5, "race")
            .await
            .expect("create_ftpace_entry should succeed");

        // Verify it was inserted
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ftpace_history WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .expect("count query should succeed");
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_create_lthr_entry() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        create_lthr_entry(&pool, user_id, 170, "time_trial")
            .await
            .expect("create_lthr_entry should succeed");

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM lthr_history WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .expect("count query should succeed");
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_create_daily_metrics() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        create_daily_metrics(&pool, user_id, "2026-02-14", 80.0, 60.0, 45.0, -15.0)
            .await
            .expect("create_daily_metrics should succeed");

        // Insert again for same date should replace (INSERT OR REPLACE)
        create_daily_metrics(&pool, user_id, "2026-02-14", 90.0, 65.0, 46.0, -19.0)
            .await
            .expect("upsert should succeed");

        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM daily_metrics WHERE user_id = ? AND date = '2026-02-14'",
        )
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .expect("count query should succeed");
        assert_eq!(count, 1, "should have exactly one row after upsert");
    }
}
