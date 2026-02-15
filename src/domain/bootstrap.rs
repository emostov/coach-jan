use super::types::ExperienceLevel;

// ---------------------------------------------------------------------------
// CTL Bootstrap from weekly volume
// ---------------------------------------------------------------------------
//
// Formula:
//   avg_pace_factor = { beginner: 0.65, intermediate: 0.75, advanced: 0.85 }
//   estimated_weekly_tss = weekly_km * avg_pace_factor * 5
//   estimated_daily_tss  = estimated_weekly_tss / 7
//   initial_CTL = estimated_daily_tss
//   initial_ATL = estimated_daily_tss
//

/// Returns the average pace factor for a given experience level.
fn avg_pace_factor(level: &ExperienceLevel) -> f64 {
    match level {
        ExperienceLevel::Beginner => 0.65,
        ExperienceLevel::Intermediate => 0.75,
        ExperienceLevel::Advanced => 0.85,
    }
}

/// Bootstrap initial CTL and ATL from weekly running volume and experience level.
///
/// Returns `(CTL, ATL)` — both values are identical at bootstrap.
pub fn bootstrap_ctl(weekly_km: f64, level: &ExperienceLevel) -> (f64, f64) {
    let factor = avg_pace_factor(level);
    let estimated_weekly_tss = weekly_km * factor * 5.0;
    let estimated_daily_tss = estimated_weekly_tss / 7.0;
    (estimated_daily_tss, estimated_daily_tss)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.01;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn beginner_30km() {
        // 30 * 0.65 * 5 / 7 = 97.5 / 7 ≈ 13.928..
        let (ctl, atl) = bootstrap_ctl(30.0, &ExperienceLevel::Beginner);
        let expected = 30.0 * 0.65 * 5.0 / 7.0;
        assert!(
            approx_eq(ctl, expected),
            "CTL {ctl} != expected {expected}"
        );
        assert!(
            approx_eq(atl, expected),
            "ATL {atl} != expected {expected}"
        );
        // Verify the approximate value from the spec
        assert!(approx_eq(ctl, 13.93), "CTL {ctl} != ~13.93");
    }

    #[test]
    fn intermediate_50km() {
        // 50 * 0.75 * 5 / 7 = 187.5 / 7 ≈ 26.786..
        let (ctl, atl) = bootstrap_ctl(50.0, &ExperienceLevel::Intermediate);
        let expected = 50.0 * 0.75 * 5.0 / 7.0;
        assert!(
            approx_eq(ctl, expected),
            "CTL {ctl} != expected {expected}"
        );
        assert!(
            approx_eq(atl, expected),
            "ATL {atl} != expected {expected}"
        );
        assert!(approx_eq(ctl, 26.79), "CTL {ctl} != ~26.79");
    }

    #[test]
    fn advanced_80km() {
        // 80 * 0.85 * 5 / 7 = 340.0 / 7 ≈ 48.571..
        let (ctl, atl) = bootstrap_ctl(80.0, &ExperienceLevel::Advanced);
        let expected = 80.0 * 0.85 * 5.0 / 7.0;
        assert!(
            approx_eq(ctl, expected),
            "CTL {ctl} != expected {expected}"
        );
        assert!(
            approx_eq(atl, expected),
            "ATL {atl} != expected {expected}"
        );
        assert!(approx_eq(ctl, 48.57), "CTL {ctl} != ~48.57");
    }

    #[test]
    fn ctl_equals_atl() {
        for (km, level) in [
            (20.0, ExperienceLevel::Beginner),
            (40.0, ExperienceLevel::Intermediate),
            (100.0, ExperienceLevel::Advanced),
        ] {
            let (ctl, atl) = bootstrap_ctl(km, &level);
            assert_eq!(ctl, atl, "CTL and ATL must be equal at bootstrap");
        }
    }

    #[test]
    fn zero_volume() {
        let (ctl, atl) = bootstrap_ctl(0.0, &ExperienceLevel::Beginner);
        assert_eq!(ctl, 0.0);
        assert_eq!(atl, 0.0);
    }

    #[test]
    fn pace_factor_values() {
        assert_eq!(avg_pace_factor(&ExperienceLevel::Beginner), 0.65);
        assert_eq!(avg_pace_factor(&ExperienceLevel::Intermediate), 0.75);
        assert_eq!(avg_pace_factor(&ExperienceLevel::Advanced), 0.85);
    }
}
