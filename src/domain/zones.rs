use super::types::{HrZone, HrZones, PaceZone, PaceZones};

// ---------------------------------------------------------------------------
// HR Zones — 7-zone model based on LTHR
// ---------------------------------------------------------------------------
//
// Zone 1 (Recovery):       < 82% LTHR
// Zone 2 (Aerobic Base):   82-88% LTHR
// Zone 3 (Tempo):          89-93% LTHR
// Zone 4 (SubThreshold):   94-99% LTHR
// Zone 5 (Threshold):      100-104% LTHR
// Zone 6 (VO2max):         105-110% LTHR
// Zone 7 (Anaerobic):      > 110% LTHR
//
// Boundaries are inclusive on both ends. Adjacent zones share no overlap
// because we use integer BPM: zone N max_bpm = zone N+1 min_bpm - 1.

/// Calculate 7 heart-rate zones from a Lactate Threshold Heart Rate (LTHR).
pub fn calculate_hr_zones(lthr: u16) -> HrZones {
    let lthr_f = lthr as f64;

    // Helper: percentage of LTHR → integer BPM (floor for min, floor for max).
    let pct = |p: f64| -> u16 { (lthr_f * p).floor() as u16 };

    // Zone boundary BPMs (inclusive on both sides).
    // Zone 1: 0  ..  81% (i.e. < 82% means up to floor(82%) - 1)
    // Zone 2: 82% ..  88%
    // Zone 3: 89% ..  93%
    // Zone 4: 94% ..  99%
    // Zone 5: 100% .. 104%
    // Zone 6: 105% .. 110%
    // Zone 7: 111% .. open

    let z2_min = pct(0.82);
    let z2_max = pct(0.88);
    let z3_min = z2_max + 1;
    let z3_max = pct(0.93);
    let z4_min = z3_max + 1;
    let z4_max = pct(0.99);
    let z5_min = z4_max + 1;
    let z5_max = pct(1.04);
    let z6_min = z5_max + 1;
    let z6_max = pct(1.10);
    let z7_min = z6_max + 1;
    let z1_max = z2_min - 1;

    HrZones {
        zones: vec![
            HrZone {
                zone: 1,
                min_bpm: 0,
                max_bpm: Some(z1_max),
                name: "Recovery".into(),
            },
            HrZone {
                zone: 2,
                min_bpm: z2_min,
                max_bpm: Some(z2_max),
                name: "Aerobic Base".into(),
            },
            HrZone {
                zone: 3,
                min_bpm: z3_min,
                max_bpm: Some(z3_max),
                name: "Tempo".into(),
            },
            HrZone {
                zone: 4,
                min_bpm: z4_min,
                max_bpm: Some(z4_max),
                name: "SubThreshold".into(),
            },
            HrZone {
                zone: 5,
                min_bpm: z5_min,
                max_bpm: Some(z5_max),
                name: "Threshold".into(),
            },
            HrZone {
                zone: 6,
                min_bpm: z6_min,
                max_bpm: Some(z6_max),
                name: "VO2max".into(),
            },
            HrZone {
                zone: 7,
                min_bpm: z7_min,
                max_bpm: None,
                name: "Anaerobic".into(),
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// Pace Zones — 6-zone model based on FTPace (m/s)
// ---------------------------------------------------------------------------
//
// Zone 1 (Recovery):     < 75% FTPace   →  0  .. 0.7499*FTPace
// Zone 2 (Easy):         75-85% FTPace
// Zone 3 (Tempo):        86-95% FTPace
// Zone 4 (Threshold):    96-105% FTPace
// Zone 5 (VO2max):       106-120% FTPace
// Zone 6 (Sprint):       > 120% FTPace
//
// Higher m/s = faster. Boundaries are rounded to two decimal places.
// We use an epsilon offset (0.01 m/s) to avoid overlap between zones.

/// Calculate 6 pace zones from Functional Threshold Pace (m/s).
pub fn calculate_pace_zones(ftpace_m_per_s: f64) -> PaceZones {
    let fp = ftpace_m_per_s;

    // Helper: round to 2 decimal places.
    let r2 = |v: f64| -> f64 { (v * 100.0).round() / 100.0 };

    let z1_max = r2(fp * 0.75 - 0.01);
    let z2_min = r2(fp * 0.75);
    let z2_max = r2(fp * 0.85);
    let z3_min = r2(fp * 0.86);
    let z3_max = r2(fp * 0.95);
    let z4_min = r2(fp * 0.96);
    let z4_max = r2(fp * 1.05);
    let z5_min = r2(fp * 1.06);
    let z5_max = r2(fp * 1.20);
    let z6_min = r2(fp * 1.20 + 0.01);

    PaceZones {
        zones: vec![
            PaceZone {
                zone: 1,
                min_pace_m_per_s: 0.0,
                max_pace_m_per_s: Some(z1_max),
                name: "Recovery".into(),
            },
            PaceZone {
                zone: 2,
                min_pace_m_per_s: z2_min,
                max_pace_m_per_s: Some(z2_max),
                name: "Easy".into(),
            },
            PaceZone {
                zone: 3,
                min_pace_m_per_s: z3_min,
                max_pace_m_per_s: Some(z3_max),
                name: "Tempo".into(),
            },
            PaceZone {
                zone: 4,
                min_pace_m_per_s: z4_min,
                max_pace_m_per_s: Some(z4_max),
                name: "Threshold".into(),
            },
            PaceZone {
                zone: 5,
                min_pace_m_per_s: z5_min,
                max_pace_m_per_s: Some(z5_max),
                name: "VO2max".into(),
            },
            PaceZone {
                zone: 6,
                min_pace_m_per_s: z6_min,
                max_pace_m_per_s: None,
                name: "Sprint".into(),
            },
        ],
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // HR zone tests
    // -----------------------------------------------------------------------

    #[test]
    fn hr_zones_lthr_165() {
        let zones = calculate_hr_zones(165);
        assert_eq!(zones.len(), 7);

        // 82% of 165 = 135.3 → floor = 135
        assert_eq!(zones.zones[0].min_bpm, 0);
        assert_eq!(zones.zones[0].max_bpm, Some(134)); // z2_min - 1

        assert_eq!(zones.zones[1].min_bpm, 135);
        // 88% of 165 = 145.2 → floor = 145
        assert_eq!(zones.zones[1].max_bpm, Some(145));

        // z3: 89-93% → 146..153
        assert_eq!(zones.zones[2].min_bpm, 146);
        // 93% of 165 = 153.45 → floor = 153
        assert_eq!(zones.zones[2].max_bpm, Some(153));

        // z4: 94-99% → 154..163
        assert_eq!(zones.zones[3].min_bpm, 154);
        // 99% of 165 = 163.35 → floor = 163
        assert_eq!(zones.zones[3].max_bpm, Some(163));

        // z5: 100-104% → 164..171
        assert_eq!(zones.zones[4].min_bpm, 164);
        // 104% of 165 = 171.6 → floor = 171
        assert_eq!(zones.zones[4].max_bpm, Some(171));

        // z6: 105-110% → 172..181
        assert_eq!(zones.zones[5].min_bpm, 172);
        // 110% of 165 = 181.5 → floor = 181
        assert_eq!(zones.zones[5].max_bpm, Some(181));

        // z7: > 110% → 182+
        assert_eq!(zones.zones[6].min_bpm, 182);
        assert_eq!(zones.zones[6].max_bpm, None);
    }

    #[test]
    fn hr_zones_lthr_180() {
        let zones = calculate_hr_zones(180);
        assert_eq!(zones.len(), 7);

        // 82% of 180 = 147.6 → floor = 147
        assert_eq!(zones.zones[1].min_bpm, 147);
        // 88% of 180 = 158.4 → floor = 158
        assert_eq!(zones.zones[1].max_bpm, Some(158));

        // z1 max = 146
        assert_eq!(zones.zones[0].max_bpm, Some(146));

        // 93% of 180 = 167.4 → floor = 167
        assert_eq!(zones.zones[2].max_bpm, Some(167));

        // 99% of 180 = 178.2 → floor = 178
        assert_eq!(zones.zones[3].max_bpm, Some(178));

        // 104% of 180 = 187.2 → floor = 187
        assert_eq!(zones.zones[4].max_bpm, Some(187));

        // 110% of 180 = 198 → floor = 198
        assert_eq!(zones.zones[5].max_bpm, Some(198));

        // z7 starts at 199
        assert_eq!(zones.zones[6].min_bpm, 199);
    }

    #[test]
    fn hr_zones_no_overlap() {
        for lthr in [150, 160, 165, 170, 175, 180, 190, 200] {
            let zones = calculate_hr_zones(lthr);
            for i in 0..zones.len() - 1 {
                let current_max = zones.zones[i]
                    .max_bpm
                    .expect("all zones except last should have max_bpm");
                let next_min = zones.zones[i + 1].min_bpm;
                assert_eq!(
                    current_max + 1,
                    next_min,
                    "Gap or overlap between zone {} and {} for LTHR={}",
                    zones.zones[i].zone,
                    zones.zones[i + 1].zone,
                    lthr,
                );
            }
        }
    }

    #[test]
    fn hr_zones_cover_full_range() {
        let zones = calculate_hr_zones(165);
        // Zone 1 starts at 0
        assert_eq!(zones.zones[0].min_bpm, 0);
        // Zone 7 has no upper bound
        assert_eq!(zones.zones[6].max_bpm, None);
    }

    #[test]
    fn hr_zones_names() {
        let zones = calculate_hr_zones(165);
        let names: Vec<&str> = zones.zones.iter().map(|z| z.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "Recovery",
                "Aerobic Base",
                "Tempo",
                "SubThreshold",
                "Threshold",
                "VO2max",
                "Anaerobic",
            ]
        );
    }

    #[test]
    fn hr_zones_zone_numbers() {
        let zones = calculate_hr_zones(170);
        for (i, z) in zones.zones.iter().enumerate() {
            assert_eq!(z.zone, (i + 1) as u8);
        }
    }

    // -----------------------------------------------------------------------
    // Pace zone tests
    // -----------------------------------------------------------------------

    #[test]
    fn pace_zones_ftpace_4_0() {
        let zones = calculate_pace_zones(4.0);
        assert_eq!(zones.len(), 6);

        // Zone 1: 0 .. <75% = 0 .. 2.99
        assert_eq!(zones.zones[0].min_pace_m_per_s, 0.0);
        assert_eq!(zones.zones[0].max_pace_m_per_s, Some(2.99));

        // Zone 2: 75-85% = 3.00 .. 3.40
        assert_eq!(zones.zones[1].min_pace_m_per_s, 3.0);
        assert_eq!(zones.zones[1].max_pace_m_per_s, Some(3.4));

        // Zone 3: 86-95% = 3.44 .. 3.80
        assert_eq!(zones.zones[2].min_pace_m_per_s, 3.44);
        assert_eq!(zones.zones[2].max_pace_m_per_s, Some(3.8));

        // Zone 4: 96-105% = 3.84 .. 4.20
        assert_eq!(zones.zones[3].min_pace_m_per_s, 3.84);
        assert_eq!(zones.zones[3].max_pace_m_per_s, Some(4.2));

        // Zone 5: 106-120% = 4.24 .. 4.80
        assert_eq!(zones.zones[4].min_pace_m_per_s, 4.24);
        assert_eq!(zones.zones[4].max_pace_m_per_s, Some(4.8));

        // Zone 6: >120% = 4.81+
        assert_eq!(zones.zones[5].min_pace_m_per_s, 4.81);
        assert_eq!(zones.zones[5].max_pace_m_per_s, None);
    }

    #[test]
    fn pace_zones_ftpace_3_5() {
        let zones = calculate_pace_zones(3.5);
        assert_eq!(zones.len(), 6);

        // Zone 1: 0 .. <75% of 3.5 = 0 .. 2.6249 → r2 = 2.62 (3.5*0.75 - 0.01 = 2.625 - 0.01 = 2.615 → r2 = 2.62)
        assert_eq!(zones.zones[0].max_pace_m_per_s, Some(2.62));

        // Zone 2: 75% = 2.625 → r2 = 2.63; 85% = 2.975 → r2 = 2.98
        assert_eq!(zones.zones[1].min_pace_m_per_s, 2.63);
        assert_eq!(zones.zones[1].max_pace_m_per_s, Some(2.98));

        // Zone 3: 86% = 3.01 → r2 = 3.01; 95% = 3.325 → r2 = 3.33
        assert_eq!(zones.zones[2].min_pace_m_per_s, 3.01);
        assert_eq!(zones.zones[2].max_pace_m_per_s, Some(3.33));

        // Zone 4: 96% = 3.36 → r2 = 3.36; 105% = 3.675 → r2 = 3.68
        assert_eq!(zones.zones[3].min_pace_m_per_s, 3.36);
        assert_eq!(zones.zones[3].max_pace_m_per_s, Some(3.68));

        // Zone 5: 106% = 3.71 → r2 = 3.71; 120% = 4.2 → r2 = 4.20
        assert_eq!(zones.zones[4].min_pace_m_per_s, 3.71);
        assert_eq!(zones.zones[4].max_pace_m_per_s, Some(4.2));

        // Zone 6: >120% = 4.21+
        assert_eq!(zones.zones[5].min_pace_m_per_s, 4.21);
        assert_eq!(zones.zones[5].max_pace_m_per_s, None);
    }

    #[test]
    fn pace_zones_no_overlap() {
        for fp in [3.0, 3.5, 4.0, 4.5, 5.0] {
            let zones = calculate_pace_zones(fp);
            for i in 0..zones.len() - 1 {
                let current_max = zones.zones[i]
                    .max_pace_m_per_s
                    .expect("all zones except last should have max");
                let next_min = zones.zones[i + 1].min_pace_m_per_s;
                assert!(
                    next_min > current_max,
                    "Overlap between zone {} (max={}) and zone {} (min={}) for FTPace={}",
                    zones.zones[i].zone,
                    current_max,
                    zones.zones[i + 1].zone,
                    next_min,
                    fp,
                );
            }
        }
    }

    #[test]
    fn pace_zones_cover_full_range() {
        let zones = calculate_pace_zones(4.0);
        // Zone 1 starts at 0
        assert_eq!(zones.zones[0].min_pace_m_per_s, 0.0);
        // Zone 6 has no upper bound
        assert_eq!(zones.zones[5].max_pace_m_per_s, None);
    }

    #[test]
    fn pace_zones_names() {
        let zones = calculate_pace_zones(4.0);
        let names: Vec<&str> = zones.zones.iter().map(|z| z.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["Recovery", "Easy", "Tempo", "Threshold", "VO2max", "Sprint"]
        );
    }

    #[test]
    fn pace_zones_zone_numbers() {
        let zones = calculate_pace_zones(4.0);
        for (i, z) in zones.zones.iter().enumerate() {
            assert_eq!(z.zone, (i + 1) as u8);
        }
    }
}
