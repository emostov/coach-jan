use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use sqlx::{FromRow, Row};

use crate::error::AppResult;

// ---------------------------------------------------------------------------
// Macrocycle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Macrocycle {
    pub id: i64,
    pub user_id: i64,
    pub race_goal_id: i64,
    pub start_date: String,
    pub end_date: String,
    pub target_ctl: Option<f64>,
    pub status: String,
    pub coach_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMacrocycle {
    pub user_id: i64,
    pub race_goal_id: i64,
    pub start_date: String,
    pub end_date: String,
    pub target_ctl: Option<f64>,
    pub coach_message: Option<String>,
}

/// Create a new macrocycle. Status defaults to 'active'.
pub async fn create_macrocycle(
    pool: &SqlitePool,
    input: &CreateMacrocycle,
) -> AppResult<Macrocycle> {
    let now = Utc::now().to_rfc3339();

    let row = sqlx::query(
        r#"INSERT INTO macrocycles (user_id, race_goal_id, start_date, end_date, target_ctl, coach_message, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?)
           RETURNING id, user_id, race_goal_id, start_date, end_date, target_ctl, status, coach_message, created_at"#,
    )
    .bind(input.user_id)
    .bind(input.race_goal_id)
    .bind(&input.start_date)
    .bind(&input.end_date)
    .bind(input.target_ctl)
    .bind(&input.coach_message)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(Macrocycle {
        id: row.get("id"),
        user_id: row.get("user_id"),
        race_goal_id: row.get("race_goal_id"),
        start_date: row.get("start_date"),
        end_date: row.get("end_date"),
        target_ctl: row.get("target_ctl"),
        status: row.get("status"),
        coach_message: row.get("coach_message"),
        created_at: row.get("created_at"),
    })
}

/// Get the current active macrocycle for a user.
/// Returns the most recently created macrocycle with status='active'.
pub async fn get_current_macrocycle(
    pool: &SqlitePool,
    user_id: i64,
) -> AppResult<Option<Macrocycle>> {
    let row = sqlx::query(
        r#"SELECT id, user_id, race_goal_id, start_date, end_date, target_ctl, status, coach_message, created_at
           FROM macrocycles WHERE user_id = ? AND status = 'active'
           ORDER BY created_at DESC LIMIT 1"#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| Macrocycle {
        id: r.get("id"),
        user_id: r.get("user_id"),
        race_goal_id: r.get("race_goal_id"),
        start_date: r.get("start_date"),
        end_date: r.get("end_date"),
        target_ctl: r.get("target_ctl"),
        status: r.get("status"),
        coach_message: r.get("coach_message"),
        created_at: r.get("created_at"),
    }))
}

// ---------------------------------------------------------------------------
// Mesocycle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Mesocycle {
    pub id: i64,
    pub macrocycle_id: i64,
    pub sequence_number: i64,
    pub phase: String,
    pub focus: String,
    pub load_weeks: i64,
    pub recovery_weeks: i64,
    pub target_volume_km: Option<f64>,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
    pub evaluation_summary: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMesocycle {
    pub macrocycle_id: i64,
    pub sequence_number: i64,
    pub phase: String,
    pub focus: String,
    pub load_weeks: i64,
    pub recovery_weeks: i64,
    pub target_volume_km: Option<f64>,
    pub start_date: String,
    pub end_date: String,
}

/// Create a new mesocycle within a macrocycle. Status defaults to 'pending'.
pub async fn create_mesocycle(
    pool: &SqlitePool,
    input: &CreateMesocycle,
) -> AppResult<Mesocycle> {
    let now = Utc::now().to_rfc3339();

    let row = sqlx::query(
        r#"INSERT INTO mesocycles
            (macrocycle_id, sequence_number, phase, focus, load_weeks, recovery_weeks,
             target_volume_km, start_date, end_date, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
           RETURNING id, macrocycle_id, sequence_number, phase, focus, load_weeks, recovery_weeks,
                     target_volume_km, start_date, end_date, status, evaluation_summary, created_at"#,
    )
    .bind(input.macrocycle_id)
    .bind(input.sequence_number)
    .bind(&input.phase)
    .bind(&input.focus)
    .bind(input.load_weeks)
    .bind(input.recovery_weeks)
    .bind(input.target_volume_km)
    .bind(&input.start_date)
    .bind(&input.end_date)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(Mesocycle {
        id: row.get("id"),
        macrocycle_id: row.get("macrocycle_id"),
        sequence_number: row.get("sequence_number"),
        phase: row.get("phase"),
        focus: row.get("focus"),
        load_weeks: row.get("load_weeks"),
        recovery_weeks: row.get("recovery_weeks"),
        target_volume_km: row.get("target_volume_km"),
        start_date: row.get("start_date"),
        end_date: row.get("end_date"),
        status: row.get("status"),
        evaluation_summary: row.get("evaluation_summary"),
        created_at: row.get("created_at"),
    })
}

/// Get all mesocycles for a macrocycle, ordered by sequence number.
pub async fn get_mesocycles(
    pool: &SqlitePool,
    macrocycle_id: i64,
) -> AppResult<Vec<Mesocycle>> {
    let rows = sqlx::query(
        r#"SELECT id, macrocycle_id, sequence_number, phase, focus, load_weeks, recovery_weeks,
                  target_volume_km, start_date, end_date, status, evaluation_summary, created_at
           FROM mesocycles WHERE macrocycle_id = ?
           ORDER BY sequence_number ASC"#,
    )
    .bind(macrocycle_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|r| Mesocycle {
            id: r.get("id"),
            macrocycle_id: r.get("macrocycle_id"),
            sequence_number: r.get("sequence_number"),
            phase: r.get("phase"),
            focus: r.get("focus"),
            load_weeks: r.get("load_weeks"),
            recovery_weeks: r.get("recovery_weeks"),
            target_volume_km: r.get("target_volume_km"),
            start_date: r.get("start_date"),
            end_date: r.get("end_date"),
            status: r.get("status"),
            evaluation_summary: r.get("evaluation_summary"),
            created_at: r.get("created_at"),
        })
        .collect())
}

// ---------------------------------------------------------------------------
// PlannedWorkout
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct PlannedWorkout {
    pub id: i64,
    pub mesocycle_id: i64,
    pub user_id: i64,
    pub scheduled_date: String,
    pub workout_type: String,
    pub duration_min: Option<i64>,
    pub duration_category: Option<String>,
    pub target_hr_zones: Option<String>,
    pub target_pace_zones: Option<String>,
    pub expected_tss: Option<f64>,
    pub description: Option<String>,
    pub coach_notes: Option<String>,
    pub target_distance_km: Option<f64>,
    pub is_completed: i64,
    pub completed_workout_id: Option<i64>,
    pub rpe: Option<i64>,
    pub athlete_notes: Option<String>,
    pub actual_duration_min: Option<i64>,
    pub completed_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlannedWorkout {
    pub mesocycle_id: i64,
    pub user_id: i64,
    pub scheduled_date: String,
    pub workout_type: String,
    pub duration_min: Option<i64>,
    pub duration_category: Option<String>,
    pub target_hr_zones: Option<String>,
    pub target_pace_zones: Option<String>,
    pub expected_tss: Option<f64>,
    pub description: Option<String>,
    pub coach_notes: Option<String>,
    pub target_distance_km: Option<f64>,
}

/// Create a new planned workout within a mesocycle. Defaults to not completed.
pub async fn create_planned_workout(
    pool: &SqlitePool,
    input: &CreatePlannedWorkout,
) -> AppResult<PlannedWorkout> {
    let now = Utc::now().to_rfc3339();

    let row = sqlx::query(
        r#"INSERT INTO planned_workouts
            (mesocycle_id, user_id, scheduled_date, workout_type, duration_min, duration_category,
             target_hr_zones, target_pace_zones, expected_tss, description, coach_notes,
             target_distance_km, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
           RETURNING id, mesocycle_id, user_id, scheduled_date, workout_type, duration_min,
                     duration_category, target_hr_zones, target_pace_zones, expected_tss,
                     description, coach_notes, target_distance_km, is_completed,
                     completed_workout_id, rpe, athlete_notes, actual_duration_min,
                     completed_at, created_at"#,
    )
    .bind(input.mesocycle_id)
    .bind(input.user_id)
    .bind(&input.scheduled_date)
    .bind(&input.workout_type)
    .bind(input.duration_min)
    .bind(&input.duration_category)
    .bind(&input.target_hr_zones)
    .bind(&input.target_pace_zones)
    .bind(input.expected_tss)
    .bind(&input.description)
    .bind(&input.coach_notes)
    .bind(input.target_distance_km)
    .bind(&now)
    .fetch_one(pool)
    .await?;

    Ok(PlannedWorkout {
        id: row.get("id"),
        mesocycle_id: row.get("mesocycle_id"),
        user_id: row.get("user_id"),
        scheduled_date: row.get("scheduled_date"),
        workout_type: row.get("workout_type"),
        duration_min: row.get("duration_min"),
        duration_category: row.get("duration_category"),
        target_hr_zones: row.get("target_hr_zones"),
        target_pace_zones: row.get("target_pace_zones"),
        expected_tss: row.get("expected_tss"),
        description: row.get("description"),
        coach_notes: row.get("coach_notes"),
        target_distance_km: row.get("target_distance_km"),
        is_completed: row.get("is_completed"),
        completed_workout_id: row.get("completed_workout_id"),
        rpe: row.get("rpe"),
        athlete_notes: row.get("athlete_notes"),
        actual_duration_min: row.get("actual_duration_min"),
        completed_at: row.get("completed_at"),
        created_at: row.get("created_at"),
    })
}

/// Get all planned workouts for a mesocycle, ordered by scheduled date.
pub async fn get_planned_workouts(
    pool: &SqlitePool,
    mesocycle_id: i64,
) -> AppResult<Vec<PlannedWorkout>> {
    let rows = sqlx::query(
        r#"SELECT id, mesocycle_id, user_id, scheduled_date, workout_type, duration_min,
                  duration_category, target_hr_zones, target_pace_zones, expected_tss,
                  description, coach_notes, target_distance_km, is_completed,
                  completed_workout_id, rpe, athlete_notes, actual_duration_min,
                  completed_at, created_at
           FROM planned_workouts WHERE mesocycle_id = ?
           ORDER BY scheduled_date ASC"#,
    )
    .bind(mesocycle_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|r| PlannedWorkout {
            id: r.get("id"),
            mesocycle_id: r.get("mesocycle_id"),
            user_id: r.get("user_id"),
            scheduled_date: r.get("scheduled_date"),
            workout_type: r.get("workout_type"),
            duration_min: r.get("duration_min"),
            duration_category: r.get("duration_category"),
            target_hr_zones: r.get("target_hr_zones"),
            target_pace_zones: r.get("target_pace_zones"),
            expected_tss: r.get("expected_tss"),
            description: r.get("description"),
            coach_notes: r.get("coach_notes"),
            target_distance_km: r.get("target_distance_km"),
            is_completed: r.get("is_completed"),
            completed_workout_id: r.get("completed_workout_id"),
            rpe: r.get("rpe"),
            athlete_notes: r.get("athlete_notes"),
            actual_duration_min: r.get("actual_duration_min"),
            completed_at: r.get("completed_at"),
            created_at: r.get("created_at"),
        })
        .collect())
}

/// Mark a workout as completed with optional feedback (RPE, notes, actual duration).
pub async fn complete_workout(
    pool: &SqlitePool,
    workout_id: i64,
    user_id: i64,
    rpe: Option<i64>,
    athlete_notes: Option<&str>,
    actual_duration_min: Option<i64>,
) -> AppResult<PlannedWorkout> {
    let now = Utc::now().to_rfc3339();

    let row = sqlx::query(
        r#"UPDATE planned_workouts
           SET is_completed = 1, completed_at = ?, rpe = ?, athlete_notes = ?, actual_duration_min = ?
           WHERE id = ? AND user_id = ?
           RETURNING id, mesocycle_id, user_id, scheduled_date, workout_type, duration_min,
                     duration_category, target_hr_zones, target_pace_zones, expected_tss,
                     description, coach_notes, target_distance_km, is_completed,
                     completed_workout_id, rpe, athlete_notes, actual_duration_min,
                     completed_at, created_at"#,
    )
    .bind(&now)
    .bind(rpe)
    .bind(athlete_notes)
    .bind(actual_duration_min)
    .bind(workout_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(PlannedWorkout {
            id: r.get("id"),
            mesocycle_id: r.get("mesocycle_id"),
            user_id: r.get("user_id"),
            scheduled_date: r.get("scheduled_date"),
            workout_type: r.get("workout_type"),
            duration_min: r.get("duration_min"),
            duration_category: r.get("duration_category"),
            target_hr_zones: r.get("target_hr_zones"),
            target_pace_zones: r.get("target_pace_zones"),
            expected_tss: r.get("expected_tss"),
            description: r.get("description"),
            coach_notes: r.get("coach_notes"),
            target_distance_km: r.get("target_distance_km"),
            is_completed: r.get("is_completed"),
            completed_workout_id: r.get("completed_workout_id"),
            rpe: r.get("rpe"),
            athlete_notes: r.get("athlete_notes"),
            actual_duration_min: r.get("actual_duration_min"),
            completed_at: r.get("completed_at"),
            created_at: r.get("created_at"),
        }),
        None => Err(crate::error::AppError::NotFound(
            "Workout not found or does not belong to user".to_string(),
        )),
    }
}

/// Get workouts from the previous mesocycle (by sequence_number) within the same macrocycle.
/// Returns empty Vec if this is the first mesocycle.
pub async fn get_previous_mesocycle_workouts(
    pool: &SqlitePool,
    user_id: i64,
    current_mesocycle_id: i64,
) -> AppResult<Vec<PlannedWorkout>> {
    let rows = sqlx::query(
        r#"SELECT pw.id, pw.mesocycle_id, pw.user_id, pw.scheduled_date, pw.workout_type,
                  pw.duration_min, pw.duration_category, pw.target_hr_zones, pw.target_pace_zones,
                  pw.expected_tss, pw.description, pw.coach_notes, pw.target_distance_km,
                  pw.is_completed, pw.completed_workout_id, pw.rpe, pw.athlete_notes,
                  pw.actual_duration_min, pw.completed_at, pw.created_at
           FROM planned_workouts pw
           JOIN mesocycles cur ON cur.id = ?
           JOIN mesocycles prev ON prev.macrocycle_id = cur.macrocycle_id
                               AND prev.sequence_number = cur.sequence_number - 1
           WHERE pw.mesocycle_id = prev.id AND pw.user_id = ?
           ORDER BY pw.scheduled_date ASC"#,
    )
    .bind(current_mesocycle_id)
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|r| PlannedWorkout {
            id: r.get("id"),
            mesocycle_id: r.get("mesocycle_id"),
            user_id: r.get("user_id"),
            scheduled_date: r.get("scheduled_date"),
            workout_type: r.get("workout_type"),
            duration_min: r.get("duration_min"),
            duration_category: r.get("duration_category"),
            target_hr_zones: r.get("target_hr_zones"),
            target_pace_zones: r.get("target_pace_zones"),
            expected_tss: r.get("expected_tss"),
            description: r.get("description"),
            coach_notes: r.get("coach_notes"),
            target_distance_km: r.get("target_distance_km"),
            is_completed: r.get("is_completed"),
            completed_workout_id: r.get("completed_workout_id"),
            rpe: r.get("rpe"),
            athlete_notes: r.get("athlete_notes"),
            actual_duration_min: r.get("actual_duration_min"),
            completed_at: r.get("completed_at"),
            created_at: r.get("created_at"),
        })
        .collect())
}

/// Get the current active plan for a user: the active macrocycle and all its mesocycles.
/// Returns `None` if the user has no active macrocycle.
pub async fn get_current_plan(
    pool: &SqlitePool,
    user_id: i64,
) -> AppResult<Option<(Macrocycle, Vec<Mesocycle>)>> {
    let macrocycle = get_current_macrocycle(pool, user_id).await?;

    match macrocycle {
        Some(mc) => {
            let mesocycles = get_mesocycles(pool, mc.id).await?;
            Ok(Some((mc, mesocycles)))
        }
        None => Ok(None),
    }
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
            "INSERT INTO users (email, password_hash) VALUES ('plans@example.com', 'hash') RETURNING id",
        )
        .fetch_one(pool)
        .await
        .expect("create test user");
        row.get("id")
    }

    async fn create_test_race_goal(pool: &SqlitePool, user_id: i64) -> i64 {
        let row = sqlx::query(
            r#"INSERT INTO race_goals (user_id, race_name, distance_m, race_date, target_time_seconds)
               VALUES (?, 'Test Marathon', 42195.0, '2026-09-27', 12600)
               RETURNING id"#,
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .expect("create test race goal");
        row.get("id")
    }

    async fn create_test_macrocycle(pool: &SqlitePool, user_id: i64, race_goal_id: i64) -> Macrocycle {
        let input = CreateMacrocycle {
            user_id,
            race_goal_id,
            start_date: "2026-03-01".to_string(),
            end_date: "2026-09-27".to_string(),
            target_ctl: Some(65.0),
            coach_message: Some("Let's build your aerobic base first.".to_string()),
        };
        create_macrocycle(pool, &input)
            .await
            .expect("create test macrocycle")
    }

    async fn create_test_mesocycle(pool: &SqlitePool, macrocycle_id: i64) -> Mesocycle {
        let input = CreateMesocycle {
            macrocycle_id,
            sequence_number: 1,
            phase: "capacity".to_string(),
            focus: "aerobic_capacity".to_string(),
            load_weeks: 3,
            recovery_weeks: 1,
            target_volume_km: Some(160.0),
            start_date: "2026-03-01".to_string(),
            end_date: "2026-03-28".to_string(),
        };
        create_mesocycle(pool, &input)
            .await
            .expect("create test mesocycle")
    }

    // -----------------------------------------------------------------------
    // Macrocycle tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_create_macrocycle() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;

        let input = CreateMacrocycle {
            user_id,
            race_goal_id,
            start_date: "2026-03-01".to_string(),
            end_date: "2026-09-27".to_string(),
            target_ctl: Some(65.0),
            coach_message: Some("Building your aerobic base.".to_string()),
        };

        let mc = create_macrocycle(&pool, &input)
            .await
            .expect("create_macrocycle should succeed");

        assert_eq!(mc.user_id, user_id);
        assert_eq!(mc.race_goal_id, race_goal_id);
        assert_eq!(mc.start_date, "2026-03-01");
        assert_eq!(mc.end_date, "2026-09-27");
        assert_eq!(mc.target_ctl, Some(65.0));
        assert_eq!(mc.status, "active");
        assert_eq!(mc.coach_message.as_deref(), Some("Building your aerobic base."));
        assert!(mc.id > 0);
    }

    #[tokio::test]
    async fn test_get_current_macrocycle() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;

        // No macrocycle yet
        let result = get_current_macrocycle(&pool, user_id)
            .await
            .expect("should not error");
        assert!(result.is_none());

        // Create one
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;

        let found = get_current_macrocycle(&pool, user_id)
            .await
            .expect("should not error")
            .expect("should find active macrocycle");

        assert_eq!(found.id, mc.id);
        assert_eq!(found.status, "active");
    }

    #[tokio::test]
    async fn test_get_current_macrocycle_ignores_non_active() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;

        // Create a macrocycle and mark it as completed
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
        sqlx::query("UPDATE macrocycles SET status = 'completed' WHERE id = ?")
            .bind(mc.id)
            .execute(&pool)
            .await
            .expect("update status");

        let result = get_current_macrocycle(&pool, user_id)
            .await
            .expect("should not error");
        assert!(result.is_none());
    }

    // -----------------------------------------------------------------------
    // Mesocycle tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_create_mesocycle() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;

        let input = CreateMesocycle {
            macrocycle_id: mc.id,
            sequence_number: 1,
            phase: "capacity".to_string(),
            focus: "aerobic_capacity".to_string(),
            load_weeks: 3,
            recovery_weeks: 1,
            target_volume_km: Some(160.0),
            start_date: "2026-03-01".to_string(),
            end_date: "2026-03-28".to_string(),
        };

        let meso = create_mesocycle(&pool, &input)
            .await
            .expect("create_mesocycle should succeed");

        assert_eq!(meso.macrocycle_id, mc.id);
        assert_eq!(meso.sequence_number, 1);
        assert_eq!(meso.phase, "capacity");
        assert_eq!(meso.focus, "aerobic_capacity");
        assert_eq!(meso.load_weeks, 3);
        assert_eq!(meso.recovery_weeks, 1);
        assert_eq!(meso.target_volume_km, Some(160.0));
        assert_eq!(meso.status, "pending");
        assert!(meso.evaluation_summary.is_none());
        assert!(meso.id > 0);
    }

    #[tokio::test]
    async fn test_get_mesocycles() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;

        // No mesocycles yet
        let empty = get_mesocycles(&pool, mc.id)
            .await
            .expect("should not error");
        assert!(empty.is_empty());

        // Create two mesocycles (out of order to test sorting)
        let input2 = CreateMesocycle {
            macrocycle_id: mc.id,
            sequence_number: 2,
            phase: "utilization".to_string(),
            focus: "aerobic_utilization".to_string(),
            load_weeks: 2,
            recovery_weeks: 1,
            target_volume_km: Some(140.0),
            start_date: "2026-03-29".to_string(),
            end_date: "2026-04-18".to_string(),
        };
        create_mesocycle(&pool, &input2).await.expect("create meso 2");

        let input1 = CreateMesocycle {
            macrocycle_id: mc.id,
            sequence_number: 1,
            phase: "capacity".to_string(),
            focus: "aerobic_capacity".to_string(),
            load_weeks: 3,
            recovery_weeks: 1,
            target_volume_km: Some(160.0),
            start_date: "2026-03-01".to_string(),
            end_date: "2026-03-28".to_string(),
        };
        create_mesocycle(&pool, &input1).await.expect("create meso 1");

        let mesocycles = get_mesocycles(&pool, mc.id)
            .await
            .expect("should not error");
        assert_eq!(mesocycles.len(), 2);
        // Should be ordered by sequence_number ASC
        assert_eq!(mesocycles[0].sequence_number, 1);
        assert_eq!(mesocycles[1].sequence_number, 2);
        assert_eq!(mesocycles[0].phase, "capacity");
        assert_eq!(mesocycles[1].phase, "utilization");
    }

    // -----------------------------------------------------------------------
    // PlannedWorkout tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_create_planned_workout() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
        let meso = create_test_mesocycle(&pool, mc.id).await;

        let input = CreatePlannedWorkout {
            mesocycle_id: meso.id,
            user_id,
            scheduled_date: "2026-03-03".to_string(),
            workout_type: "easy_run".to_string(),
            duration_min: Some(45),
            duration_category: Some("medium".to_string()),
            target_hr_zones: Some("Z2".to_string()),
            target_pace_zones: Some("Z1-Z2".to_string()),
            expected_tss: Some(35.0),
            description: Some("Easy recovery run, keep it conversational.".to_string()),
            coach_notes: Some("Focus on form, not pace.".to_string()),
            target_distance_km: None,
        };

        let workout = create_planned_workout(&pool, &input)
            .await
            .expect("create_planned_workout should succeed");

        assert_eq!(workout.mesocycle_id, meso.id);
        assert_eq!(workout.user_id, user_id);
        assert_eq!(workout.scheduled_date, "2026-03-03");
        assert_eq!(workout.workout_type, "easy_run");
        assert_eq!(workout.duration_min, Some(45));
        assert_eq!(workout.duration_category.as_deref(), Some("medium"));
        assert_eq!(workout.target_hr_zones.as_deref(), Some("Z2"));
        assert_eq!(workout.target_pace_zones.as_deref(), Some("Z1-Z2"));
        assert_eq!(workout.expected_tss, Some(35.0));
        assert_eq!(workout.is_completed, 0);
        assert!(workout.completed_workout_id.is_none());
        assert!(workout.target_distance_km.is_none());
        assert!(workout.id > 0);
    }

    #[tokio::test]
    async fn test_create_planned_workout_minimal() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
        let meso = create_test_mesocycle(&pool, mc.id).await;

        let input = CreatePlannedWorkout {
            mesocycle_id: meso.id,
            user_id,
            scheduled_date: "2026-03-04".to_string(),
            workout_type: "rest".to_string(),
            duration_min: None,
            duration_category: None,
            target_hr_zones: None,
            target_pace_zones: None,
            expected_tss: None,
            description: None,
            coach_notes: None,
            target_distance_km: None,
        };

        let workout = create_planned_workout(&pool, &input)
            .await
            .expect("create minimal workout should succeed");

        assert_eq!(workout.workout_type, "rest");
        assert!(workout.duration_min.is_none());
        assert!(workout.description.is_none());
        assert!(workout.target_distance_km.is_none());
    }

    #[tokio::test]
    async fn test_create_planned_workout_with_distance() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
        let meso = create_test_mesocycle(&pool, mc.id).await;

        let input = CreatePlannedWorkout {
            mesocycle_id: meso.id,
            user_id,
            scheduled_date: "2026-03-03".to_string(),
            workout_type: "easy_run".to_string(),
            duration_min: Some(45),
            duration_category: Some("medium".to_string()),
            target_hr_zones: Some("Z2".to_string()),
            target_pace_zones: Some("Z1-Z2".to_string()),
            expected_tss: Some(35.0),
            description: Some("Easy recovery run.".to_string()),
            coach_notes: Some("Keep it easy.".to_string()),
            target_distance_km: Some(8.5),
        };

        let workout = create_planned_workout(&pool, &input)
            .await
            .expect("create workout with distance should succeed");

        assert_eq!(workout.target_distance_km, Some(8.5));
        assert_eq!(workout.workout_type, "easy_run");
        assert_eq!(workout.duration_min, Some(45));

        // Verify it persists correctly via get_planned_workouts
        let workouts = get_planned_workouts(&pool, meso.id)
            .await
            .expect("get_planned_workouts should succeed");
        assert_eq!(workouts.len(), 1);
        assert_eq!(workouts[0].target_distance_km, Some(8.5));
    }

    #[tokio::test]
    async fn test_get_planned_workouts() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
        let meso = create_test_mesocycle(&pool, mc.id).await;

        // No workouts yet
        let empty = get_planned_workouts(&pool, meso.id)
            .await
            .expect("should not error");
        assert!(empty.is_empty());

        // Create workouts (out of date order to test sorting)
        let w2 = CreatePlannedWorkout {
            mesocycle_id: meso.id,
            user_id,
            scheduled_date: "2026-03-04".to_string(),
            workout_type: "tempo_run".to_string(),
            duration_min: Some(50),
            duration_category: Some("medium".to_string()),
            target_hr_zones: None,
            target_pace_zones: None,
            expected_tss: Some(70.0),
            description: Some("Tempo at threshold pace.".to_string()),
            coach_notes: None,
            target_distance_km: None,
        };
        create_planned_workout(&pool, &w2).await.expect("create w2");

        let w1 = CreatePlannedWorkout {
            mesocycle_id: meso.id,
            user_id,
            scheduled_date: "2026-03-03".to_string(),
            workout_type: "easy_run".to_string(),
            duration_min: Some(40),
            duration_category: Some("short".to_string()),
            target_hr_zones: None,
            target_pace_zones: None,
            expected_tss: Some(30.0),
            description: None,
            coach_notes: None,
            target_distance_km: None,
        };
        create_planned_workout(&pool, &w1).await.expect("create w1");

        let w3 = CreatePlannedWorkout {
            mesocycle_id: meso.id,
            user_id,
            scheduled_date: "2026-03-05".to_string(),
            workout_type: "rest".to_string(),
            duration_min: None,
            duration_category: None,
            target_hr_zones: None,
            target_pace_zones: None,
            expected_tss: None,
            description: None,
            coach_notes: None,
            target_distance_km: None,
        };
        create_planned_workout(&pool, &w3).await.expect("create w3");

        let workouts = get_planned_workouts(&pool, meso.id)
            .await
            .expect("should not error");
        assert_eq!(workouts.len(), 3);
        // Should be ordered by scheduled_date ASC
        assert_eq!(workouts[0].scheduled_date, "2026-03-03");
        assert_eq!(workouts[1].scheduled_date, "2026-03-04");
        assert_eq!(workouts[2].scheduled_date, "2026-03-05");
        assert_eq!(workouts[0].workout_type, "easy_run");
        assert_eq!(workouts[1].workout_type, "tempo_run");
        assert_eq!(workouts[2].workout_type, "rest");
    }

    // -----------------------------------------------------------------------
    // get_current_plan tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_get_current_plan_none() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;

        let result = get_current_plan(&pool, user_id)
            .await
            .expect("should not error");
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_current_plan_with_mesocycles() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;

        // Create two mesocycles
        let m1 = CreateMesocycle {
            macrocycle_id: mc.id,
            sequence_number: 1,
            phase: "capacity".to_string(),
            focus: "aerobic_capacity".to_string(),
            load_weeks: 3,
            recovery_weeks: 1,
            target_volume_km: Some(160.0),
            start_date: "2026-03-01".to_string(),
            end_date: "2026-03-28".to_string(),
        };
        create_mesocycle(&pool, &m1).await.expect("create m1");

        let m2 = CreateMesocycle {
            macrocycle_id: mc.id,
            sequence_number: 2,
            phase: "utilization".to_string(),
            focus: "aerobic_utilization".to_string(),
            load_weeks: 2,
            recovery_weeks: 1,
            target_volume_km: Some(140.0),
            start_date: "2026-03-29".to_string(),
            end_date: "2026-04-18".to_string(),
        };
        create_mesocycle(&pool, &m2).await.expect("create m2");

        let (found_mc, mesocycles) = get_current_plan(&pool, user_id)
            .await
            .expect("should not error")
            .expect("should find current plan");

        assert_eq!(found_mc.id, mc.id);
        assert_eq!(found_mc.status, "active");
        assert_eq!(mesocycles.len(), 2);
        assert_eq!(mesocycles[0].sequence_number, 1);
        assert_eq!(mesocycles[1].sequence_number, 2);
    }

    // -----------------------------------------------------------------------
    // complete_workout tests
    // -----------------------------------------------------------------------

    async fn create_test_workout(pool: &SqlitePool, mesocycle_id: i64, user_id: i64) -> PlannedWorkout {
        let input = CreatePlannedWorkout {
            mesocycle_id,
            user_id,
            scheduled_date: "2026-03-03".to_string(),
            workout_type: "easy_run".to_string(),
            duration_min: Some(45),
            duration_category: Some("medium".to_string()),
            target_hr_zones: Some("Z2".to_string()),
            target_pace_zones: Some("Z1-Z2".to_string()),
            expected_tss: Some(35.0),
            description: Some("Easy recovery run.".to_string()),
            coach_notes: Some("Focus on form.".to_string()),
            target_distance_km: None,
        };
        create_planned_workout(pool, &input)
            .await
            .expect("create test workout")
    }

    #[tokio::test]
    async fn test_complete_workout_all_fields() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
        let meso = create_test_mesocycle(&pool, mc.id).await;
        let workout = create_test_workout(&pool, meso.id, user_id).await;

        let completed = complete_workout(
            &pool,
            workout.id,
            user_id,
            Some(7),
            Some("felt strong"),
            Some(48),
        )
        .await
        .expect("complete_workout should succeed");

        assert_eq!(completed.id, workout.id);
        assert_eq!(completed.is_completed, 1);
        assert_eq!(completed.rpe, Some(7));
        assert_eq!(completed.athlete_notes.as_deref(), Some("felt strong"));
        assert_eq!(completed.actual_duration_min, Some(48));
        assert!(completed.completed_at.is_some());
    }

    #[tokio::test]
    async fn test_complete_workout_no_optional_fields() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
        let meso = create_test_mesocycle(&pool, mc.id).await;
        let workout = create_test_workout(&pool, meso.id, user_id).await;

        let completed = complete_workout(&pool, workout.id, user_id, None, None, None)
            .await
            .expect("complete_workout should succeed");

        assert_eq!(completed.is_completed, 1);
        assert!(completed.rpe.is_none());
        assert!(completed.athlete_notes.is_none());
        assert!(completed.actual_duration_min.is_none());
        assert!(completed.completed_at.is_some());
    }

    #[tokio::test]
    async fn test_complete_workout_wrong_user() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
        let meso = create_test_mesocycle(&pool, mc.id).await;
        let workout = create_test_workout(&pool, meso.id, user_id).await;

        let result = complete_workout(&pool, workout.id, 99999, Some(5), None, None).await;
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // get_previous_mesocycle_workouts tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_get_previous_mesocycle_workouts_first_mesocycle() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;
        let meso = create_test_mesocycle(&pool, mc.id).await;

        // First mesocycle — no previous
        let result = get_previous_mesocycle_workouts(&pool, user_id, meso.id)
            .await
            .expect("should not error");
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_get_previous_mesocycle_workouts_returns_previous() {
        let pool = setup_pool().await;
        let user_id = create_test_user(&pool).await;
        let race_goal_id = create_test_race_goal(&pool, user_id).await;
        let mc = create_test_macrocycle(&pool, user_id, race_goal_id).await;

        // Create mesocycle 1 with workouts
        let meso1 = create_test_mesocycle(&pool, mc.id).await;
        let _w1 = create_test_workout(&pool, meso1.id, user_id).await;

        let w2_input = CreatePlannedWorkout {
            mesocycle_id: meso1.id,
            user_id,
            scheduled_date: "2026-03-04".to_string(),
            workout_type: "rest".to_string(),
            duration_min: None,
            duration_category: None,
            target_hr_zones: None,
            target_pace_zones: None,
            expected_tss: None,
            description: None,
            coach_notes: None,
            target_distance_km: None,
        };
        create_planned_workout(&pool, &w2_input).await.expect("create w2");

        // Create mesocycle 2
        let meso2_input = CreateMesocycle {
            macrocycle_id: mc.id,
            sequence_number: 2,
            phase: "utilization".to_string(),
            focus: "aerobic_utilization".to_string(),
            load_weeks: 3,
            recovery_weeks: 1,
            target_volume_km: Some(50.0),
            start_date: "2026-03-29".to_string(),
            end_date: "2026-04-25".to_string(),
        };
        let meso2 = create_mesocycle(&pool, &meso2_input).await.expect("create meso2");

        // Get previous workouts for meso2 — should be meso1's workouts
        let prev = get_previous_mesocycle_workouts(&pool, user_id, meso2.id)
            .await
            .expect("should not error");
        assert_eq!(prev.len(), 2);
        assert_eq!(prev[0].scheduled_date, "2026-03-03");
        assert_eq!(prev[1].scheduled_date, "2026-03-04");
    }
}
