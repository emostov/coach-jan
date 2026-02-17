import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import * as planApi from '../api/plan';
import type { MacrocycleSkeleton, WorkoutDetailResponse } from '../api/types';

export function useGeneratePlan() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (raceGoalId: number) => planApi.generatePlan(raceGoalId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['plan'] });
    },
  });
}

export function useConfirmPlan() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (skeleton: MacrocycleSkeleton) => planApi.confirmPlan(skeleton),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['plan'] });
    },
  });
}

export function useCurrentPlan() {
  return useQuery({
    queryKey: ['plan'],
    queryFn: planApi.getCurrentPlan,
    retry: false,
  });
}

export function useWorkout(workoutId: number | null) {
  return useQuery({
    queryKey: ['workout', workoutId],
    queryFn: () => planApi.getWorkout(workoutId!),
    enabled: workoutId !== null,
  });
}
