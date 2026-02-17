import { apiFetch } from './client';
import type { MacrocycleSkeleton, GeneratedPlan, PlanResponse, WorkoutDetailResponse } from './types';

export function generatePlan(raceGoalId: number): Promise<MacrocycleSkeleton> {
  return apiFetch('/plan/generate', {
    method: 'POST',
    body: JSON.stringify({ race_goal_id: raceGoalId }),
  });
}

export function confirmPlan(skeleton: MacrocycleSkeleton): Promise<GeneratedPlan> {
  return apiFetch('/plan/confirm', {
    method: 'POST',
    body: JSON.stringify(skeleton),
  });
}

export function getCurrentPlan(): Promise<PlanResponse> {
  return apiFetch('/plan');
}

export function getWorkout(workoutId: number): Promise<WorkoutDetailResponse> {
  return apiFetch(`/plan/workout/${workoutId}`);
}
