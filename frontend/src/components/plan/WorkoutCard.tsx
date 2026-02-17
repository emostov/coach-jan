import type { PlannedWorkout } from '../../api/types';
import { getWorkoutBorderColor } from '../../utils/workoutColors';
import { formatWorkoutType } from '../../utils/planHelpers';
import { isSameDay, parseISO, isPast, startOfDay } from 'date-fns';

interface Props {
  workout: PlannedWorkout;
  onClick: () => void;
}

export default function WorkoutCard({ workout, onClick }: Props) {
  const borderColor = getWorkoutBorderColor(workout.workout_type);
  const isToday = isSameDay(parseISO(workout.scheduled_date), new Date());
  const isPastDate = isPast(startOfDay(parseISO(workout.scheduled_date))) && !isToday;
  const isCompleted = workout.is_completed === 1;
  const isRest = workout.workout_type === 'rest';

  return (
    <button
      onClick={onClick}
      className={`w-full text-left p-1.5 rounded-lg transition-colors text-xs
        ${isToday ? 'ring-2 ring-forest bg-forest/5' : ''}
        ${isPastDate && !isCompleted && !isRest ? 'opacity-50' : ''}
        ${isRest ? 'bg-cream-dark/30' : 'hover:bg-cream/80'}
      `}
      style={{ borderLeft: isRest ? 'none' : `3px solid ${borderColor}` }}
    >
      <p className={`font-medium truncate ${isRest ? 'text-slate-light' : 'text-charcoal'}`}>
        {formatWorkoutType(workout.workout_type)}
      </p>
      {!isRest && (
        <div className="flex items-center gap-1 mt-0.5 text-slate-light">
          {workout.duration_min && <span>{workout.duration_min}m</span>}
          {workout.target_hr_zones && <span className="truncate">{workout.target_hr_zones}</span>}
        </div>
      )}
      {isCompleted && (
        <span className="text-forest font-medium">Done</span>
      )}
    </button>
  );
}
