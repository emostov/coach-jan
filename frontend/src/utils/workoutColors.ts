export const WORKOUT_COLORS: Record<string, string> = {
  // Easy / recovery / long / aerobic → forest green
  easy_run: '#1b4332',
  recovery_run: '#1b4332',
  long_run: '#1b4332',
  long_run_progression: '#1b4332',
  long_run_moderate: '#1b4332',
  aerobic_development: '#1b4332',
  moderate_run: '#1b4332',
  shakeout_run: '#1b4332',
  mixed_energy: '#1b4332',
  // Tempo / threshold / steady / utilization → terra
  tempo_run: '#c4572a',
  cruise_intervals: '#c4572a',
  steady_run: '#c4572a',
  progression_run: '#c4572a',
  race_specific: '#c4572a',
  lactate_clearance: '#c4572a',
  under_over: '#c4572a',
  time_trial: '#c4572a',
  // Intervals / track / VO2max / anaerobic / speed → zone-5 red
  vo2max_intervals: '#ef4444',
  track_200m: '#ef4444',
  track_400m: '#ef4444',
  track_800m: '#ef4444',
  anaerobic_hills: '#ef4444',
  anaerobic_flat: '#ef4444',
  anaerobic_power: '#ef4444',
  hill_sprints: '#ef4444',
  fartlek_structured: '#ef4444',
  // Strength / drills / cross-training → slate
  strength_precision: '#64748b',
  strength_performance: '#64748b',
  strength_power: '#64748b',
  form_drills: '#64748b',
  plyo_running: '#64748b',
  // Rest → cream-dark
  rest: '#f0ede5',
};

export function getWorkoutBorderColor(workoutType: string): string {
  return WORKOUT_COLORS[workoutType] ?? '#94a3b8';
}
