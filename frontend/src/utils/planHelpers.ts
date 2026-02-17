import { startOfISOWeek, endOfISOWeek, isWithinInterval, parseISO, isSameDay } from 'date-fns';
import type { PlanResponse, MesocycleWithWorkouts, PlannedWorkout } from '../api/types';

/** Get all workouts from all mesocycles as a flat array. */
export function getAllWorkouts(plan: PlanResponse): PlannedWorkout[] {
  return plan.mesocycles.flatMap(m => m.workouts);
}

/** Get today's workout, if any. */
export function getTodaysWorkout(plan: PlanResponse): PlannedWorkout | null {
  const today = new Date();
  return getAllWorkouts(plan).find(w => isSameDay(parseISO(w.scheduled_date), today)) ?? null;
}

/** Get workouts for the ISO week containing the given date. */
export function getWeekWorkouts(plan: PlanResponse, date: Date): PlannedWorkout[] {
  const weekStart = startOfISOWeek(date);
  const weekEnd = endOfISOWeek(date);
  return getAllWorkouts(plan).filter(w =>
    isWithinInterval(parseISO(w.scheduled_date), { start: weekStart, end: weekEnd })
  );
}

/** Calculate weekly summary from a list of workouts. */
export function getWeekSummary(workouts: PlannedWorkout[]) {
  const targetKm = workouts.reduce((sum, w) => sum + (w.target_distance_km ?? 0), 0);
  const targetTss = workouts.reduce((sum, w) => sum + (w.expected_tss ?? 0), 0);
  const targetMinutes = workouts.reduce((sum, w) => sum + (w.duration_min ?? 0), 0);
  const totalSessions = workouts.filter(w => w.workout_type !== 'rest').length;
  const completedSessions = workouts.filter(w => w.is_completed === 1).length;

  return { targetKm, targetTss, targetMinutes, totalSessions, completedSessions };
}

/** Find the mesocycle that contains the given date. */
export function getMesocycleForDate(plan: PlanResponse, date: Date): MesocycleWithWorkouts | null {
  return plan.mesocycles.find(m => {
    const start = parseISO(m.start_date);
    const end = parseISO(m.end_date);
    return date >= start && date <= end;
  }) ?? null;
}

/** Get week number within a mesocycle for a given date. */
export function getWeekOfMesocycle(mesocycle: MesocycleWithWorkouts, date: Date): number {
  const start = startOfISOWeek(parseISO(mesocycle.start_date));
  const current = startOfISOWeek(date);
  return Math.floor((current.getTime() - start.getTime()) / (7 * 24 * 60 * 60 * 1000)) + 1;
}

/** Format workout type from snake_case to Title Case. */
export function formatWorkoutType(snakeCase: string): string {
  return snakeCase.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase());
}

/** Format phase name. */
export function formatPhase(phase: string): string {
  return formatWorkoutType(phase);
}

/** Format minutes to hours:minutes display. */
export function formatMinutes(minutes: number): string {
  const hrs = Math.floor(minutes / 60);
  const mins = minutes % 60;
  if (hrs > 0) return `${hrs}h ${mins > 0 ? `${mins}m` : ''}`.trim();
  return `${mins}m`;
}
