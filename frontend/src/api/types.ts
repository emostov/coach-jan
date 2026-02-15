export interface User {
  id: number;
  email: string;
}

export interface AuthResponse {
  user: User;
}

export interface MeResponse {
  user: User;
  has_profile: boolean;
}

export interface HrZone {
  zone: number;
  min_bpm: number;
  max_bpm: number | null;
  name: string;
}

export interface PaceZone {
  zone: number;
  min_pace_m_per_s: number;
  max_pace_m_per_s: number | null;
  name: string;
}

export interface HrZones {
  zones: HrZone[];
}

export interface PaceZones {
  zones: PaceZone[];
}

export interface AthleteProfile {
  id: number;
  user_id: number;
  name: string;
  age: number;
  weight_kg: number;
  resting_hr: number;
  max_hr: number;
  lthr: number;
  ftpace_m_per_s: number | null;
  current_weekly_volume_km: number;
  experience_level: string;
  sports_background: string | null;
  created_at: string;
  updated_at: string;
}

export interface ProfileResponse {
  profile: AthleteProfile;
  hr_zones: HrZones;
  pace_zones: PaceZones | null;
}

export interface RaceGoal {
  id: number;
  race_name: string | null;
  distance_m: number;
  race_date: string;
  target_time_seconds: number | null;
}
