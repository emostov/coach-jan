import type { PlannedWorkout } from '../../api/types';
import { getWeekSummary, formatMinutes } from '../../utils/planHelpers';

interface Props {
  workouts: PlannedWorkout[];
  weekLabel: string;
}

export default function WeekSummary({ workouts, weekLabel }: Props) {
  const { targetKm, targetTss, targetMinutes, totalSessions, completedSessions } =
    getWeekSummary(workouts);

  return (
    <div className="text-xs space-y-1 p-2">
      <p className="font-medium text-charcoal truncate">{weekLabel}</p>
      {targetKm > 0 && (
        <p className="text-slate">{targetKm.toFixed(1)} km</p>
      )}
      {targetTss > 0 && (
        <p className="text-slate">{Math.round(targetTss)} TSS</p>
      )}
      {targetMinutes > 0 && (
        <p className="text-slate">{formatMinutes(targetMinutes)}</p>
      )}
      <p className="text-slate">{completedSessions}/{totalSessions}</p>
    </div>
  );
}
