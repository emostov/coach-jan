import { useEffect, useRef } from 'react';
import type { PlannedWorkout, MesocycleWithWorkouts } from '../../api/types';
import { getWorkoutBorderColor } from '../../utils/workoutColors';
import { formatWorkoutType, formatPhase } from '../../utils/planHelpers';
import { format, parseISO } from 'date-fns';

interface Props {
  workout: PlannedWorkout | null;
  mesocycle: MesocycleWithWorkouts | null;
  onClose: () => void;
}

export default function WorkoutDrawer({ workout, mesocycle, onClose }: Props) {
  const drawerRef = useRef<HTMLDivElement>(null);

  // Close on Escape
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    };
    document.addEventListener('keydown', handler);
    return () => document.removeEventListener('keydown', handler);
  }, [onClose]);

  if (!workout) return null;

  const borderColor = getWorkoutBorderColor(workout.workout_type);
  const date = parseISO(workout.scheduled_date);

  return (
    <>
      {/* Backdrop */}
      <div className="fixed inset-0 bg-charcoal/20 z-40" onClick={onClose} />

      {/* Drawer */}
      <div
        ref={drawerRef}
        className="fixed right-0 top-0 bottom-0 w-full max-w-md bg-white shadow-xl z-50 overflow-y-auto"
      >
        <div className="p-6">
          {/* Header */}
          <div className="flex items-start justify-between mb-6">
            <div>
              <p className="text-xs text-slate uppercase tracking-wider">
                {format(date, 'EEEE, MMMM d, yyyy')}
              </p>
              <h2
                className="font-serif text-2xl text-charcoal font-bold mt-1"
                style={{ borderLeft: `4px solid ${borderColor}`, paddingLeft: '12px' }}
              >
                {formatWorkoutType(workout.workout_type)}
              </h2>
              {mesocycle && (
                <p className="text-xs text-slate mt-1 pl-4">
                  {formatPhase(mesocycle.phase)} — {formatWorkoutType(mesocycle.focus)}
                </p>
              )}
            </div>
            <button
              onClick={onClose}
              className="text-slate hover:text-charcoal transition-colors p-1"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" strokeWidth={1.5}>
                <path strokeLinecap="round" strokeLinejoin="round" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          {/* Metrics row */}
          <div className="grid grid-cols-3 gap-3 mb-6">
            {workout.duration_min && (
              <div className="rounded-lg bg-cream p-3 text-center">
                <p className="text-lg font-bold text-charcoal">{workout.duration_min}</p>
                <p className="text-[10px] text-slate uppercase">Minutes</p>
              </div>
            )}
            {workout.expected_tss != null && (
              <div className="rounded-lg bg-cream p-3 text-center">
                <p className="text-lg font-bold text-charcoal">{Math.round(workout.expected_tss)}</p>
                <p className="text-[10px] text-slate uppercase">TSS</p>
              </div>
            )}
            {workout.target_distance_km != null && (
              <div className="rounded-lg bg-cream p-3 text-center">
                <p className="text-lg font-bold text-charcoal">{workout.target_distance_km.toFixed(1)}</p>
                <p className="text-[10px] text-slate uppercase">km</p>
              </div>
            )}
          </div>

          {/* Target zones */}
          {(workout.target_hr_zones || workout.target_pace_zones) && (
            <div className="mb-6">
              <h3 className="text-xs font-medium text-slate uppercase tracking-wider mb-2">Target Zones</h3>
              <div className="space-y-1">
                {workout.target_hr_zones && (
                  <p className="text-sm text-charcoal">HR: {workout.target_hr_zones}</p>
                )}
                {workout.target_pace_zones && (
                  <p className="text-sm text-charcoal">Pace: {workout.target_pace_zones}</p>
                )}
              </div>
            </div>
          )}

          {/* Description */}
          {workout.description && (
            <div className="mb-6">
              <h3 className="text-xs font-medium text-slate uppercase tracking-wider mb-2">Description</h3>
              <p className="text-sm text-charcoal leading-relaxed">{workout.description}</p>
            </div>
          )}

          {/* Coach notes */}
          {workout.coach_notes && (
            <div className="rounded-xl bg-forest/5 border border-forest/10 p-4">
              <div className="flex gap-2">
                <div className="w-6 h-6 rounded-full bg-forest flex items-center justify-center flex-shrink-0">
                  <span className="text-cream text-[10px] font-bold font-serif">J</span>
                </div>
                <p className="text-sm text-charcoal leading-relaxed">
                  {workout.coach_notes}
                </p>
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  );
}
