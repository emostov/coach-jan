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
  race_goal: RaceGoal | null;
}

export interface RaceGoal {
  id: number;
  race_name: string | null;
  distance_m: number;
  race_date: string;
  target_time_seconds: number | null;
}

export interface MacrocycleSkeleton {
  target_ctl: number;
  coach_message: string;
  mesocycles: MesocycleSkeleton[];
}

export interface MesocycleSkeleton {
  sequence_number: number;
  phase: string;
  focus: string;
  load_weeks: number;
  recovery_weeks: number;
  target_volume_km: number;
}

export interface Macrocycle {
  id: number;
  user_id: number;
  race_goal_id: number;
  start_date: string;
  end_date: string;
  target_ctl: number | null;
  status: string;
  coach_message: string | null;
  created_at: string;
}

export interface Mesocycle {
  id: number;
  macrocycle_id: number;
  sequence_number: number;
  phase: string;
  focus: string;
  load_weeks: number;
  recovery_weeks: number;
  target_volume_km: number | null;
  start_date: string;
  end_date: string;
  status: string;
  evaluation_summary: string | null;
  created_at: string;
}

export interface PlannedWorkout {
  id: number;
  mesocycle_id: number;
  user_id: number;
  scheduled_date: string;
  workout_type: string;
  duration_min: number | null;
  duration_category: string | null;
  target_hr_zones: string | null;
  target_pace_zones: string | null;
  expected_tss: number | null;
  description: string | null;
  coach_notes: string | null;
  is_completed: number;
  completed_workout_id: number | null;
  created_at: string;
}

export interface GeneratedPlan {
  macrocycle: Macrocycle;
  mesocycles: Mesocycle[];
  workouts: PlannedWorkout[];
}

export interface PlanResponse {
  macrocycle: Macrocycle;
  mesocycles: Mesocycle[];
  workouts: PlannedWorkout[];
}
