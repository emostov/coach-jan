use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Experience Level
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExperienceLevel {
    Beginner,
    Intermediate,
    Advanced,
}

impl ExperienceLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Beginner => "beginner",
            Self::Intermediate => "intermediate",
            Self::Advanced => "advanced",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "beginner" => Some(Self::Beginner),
            "intermediate" => Some(Self::Intermediate),
            "advanced" => Some(Self::Advanced),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Heart-Rate Zone (7-zone model)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HrZone {
    pub zone: u8,
    pub min_bpm: u16,
    pub max_bpm: Option<u16>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HrZones {
    pub zones: Vec<HrZone>,
}

impl HrZones {
    /// Return the zone that contains the given heart-rate value.
    pub fn zone_for_bpm(&self, bpm: u16) -> Option<&HrZone> {
        self.zones.iter().find(|z| {
            bpm >= z.min_bpm && match z.max_bpm {
                Some(max) => bpm <= max,
                None => true, // open-ended top zone
            }
        })
    }

    /// Number of zones.
    pub fn len(&self) -> usize {
        self.zones.len()
    }

    /// Whether the zone list is empty.
    pub fn is_empty(&self) -> bool {
        self.zones.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Pace Zone (6-zone model)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaceZone {
    pub zone: u8,
    pub min_pace_m_per_s: f64,
    pub max_pace_m_per_s: Option<f64>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaceZones {
    pub zones: Vec<PaceZone>,
}

impl PaceZones {
    /// Return the zone that contains the given pace (m/s).
    pub fn zone_for_pace(&self, pace: f64) -> Option<&PaceZone> {
        self.zones.iter().find(|z| {
            pace >= z.min_pace_m_per_s && match z.max_pace_m_per_s {
                Some(max) => pace <= max,
                None => true,
            }
        })
    }

    /// Number of zones.
    pub fn len(&self) -> usize {
        self.zones.len()
    }

    /// Whether the zone list is empty.
    pub fn is_empty(&self) -> bool {
        self.zones.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn experience_level_round_trip() {
        for level in [
            ExperienceLevel::Beginner,
            ExperienceLevel::Intermediate,
            ExperienceLevel::Advanced,
        ] {
            let s = level.as_str();
            let back = ExperienceLevel::from_str(s).expect("should parse back");
            assert_eq!(back, level);
        }
    }

    #[test]
    fn experience_level_invalid() {
        assert_eq!(ExperienceLevel::from_str("elite"), None);
    }

    #[test]
    fn experience_level_serde_json() {
        let level = ExperienceLevel::Intermediate;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"intermediate\"");
        let parsed: ExperienceLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, ExperienceLevel::Intermediate);
    }

    #[test]
    fn hr_zones_lookup() {
        let zones = HrZones {
            zones: vec![
                HrZone { zone: 1, min_bpm: 0, max_bpm: Some(134), name: "Recovery".into() },
                HrZone { zone: 2, min_bpm: 135, max_bpm: Some(145), name: "Aerobic Base".into() },
                HrZone { zone: 3, min_bpm: 146, max_bpm: None, name: "Tempo".into() },
            ],
        };

        assert_eq!(zones.zone_for_bpm(100).unwrap().zone, 1);
        assert_eq!(zones.zone_for_bpm(140).unwrap().zone, 2);
        assert_eq!(zones.zone_for_bpm(200).unwrap().zone, 3);
        assert_eq!(zones.len(), 3);
        assert!(!zones.is_empty());
    }

    #[test]
    fn pace_zones_lookup() {
        let zones = PaceZones {
            zones: vec![
                PaceZone { zone: 1, min_pace_m_per_s: 0.0, max_pace_m_per_s: Some(2.99), name: "Recovery".into() },
                PaceZone { zone: 2, min_pace_m_per_s: 3.0, max_pace_m_per_s: None, name: "Easy".into() },
            ],
        };

        assert_eq!(zones.zone_for_pace(2.5).unwrap().zone, 1);
        assert_eq!(zones.zone_for_pace(3.5).unwrap().zone, 2);
        assert_eq!(zones.len(), 2);
        assert!(!zones.is_empty());
    }
}
