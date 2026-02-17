import { parseISO, differenceInDays } from 'date-fns';
import type { MesocycleWithWorkouts } from '../../api/types';
import { formatPhase, formatWorkoutType } from '../../utils/planHelpers';

const PHASE_COLORS: Record<string, string> = {
  capacity: 'bg-forest',
  utilization: 'bg-terra',
  taper: 'bg-amber-500',
  recovery: 'bg-slate-400',
  transition: 'bg-slate-300',
};

interface Props {
  mesocycles: MesocycleWithWorkouts[];
  macrocycleStart: string;
  macrocycleEnd: string;
}

export default function MacrocycleTimeline({ mesocycles, macrocycleStart, macrocycleEnd }: Props) {
  const totalDays = differenceInDays(parseISO(macrocycleEnd), parseISO(macrocycleStart));
  if (totalDays <= 0) return null;

  const today = new Date();
  const todayOffset = differenceInDays(today, parseISO(macrocycleStart));
  const todayPct = Math.min(Math.max((todayOffset / totalDays) * 100, 0), 100);

  return (
    <div className="rounded-xl bg-white border border-cream-dark p-4">
      <div className="flex items-center justify-between mb-2">
        <h3 className="text-xs font-medium text-slate uppercase tracking-wider">Training Timeline</h3>
      </div>
      <div className="relative h-8 rounded-lg overflow-hidden flex">
        {mesocycles.map((m) => {
          const days = differenceInDays(parseISO(m.end_date), parseISO(m.start_date));
          const widthPct = (days / totalDays) * 100;
          const bgColor = PHASE_COLORS[m.phase] ?? 'bg-slate-300';
          return (
            <div
              key={m.id}
              className={`${bgColor} flex items-center justify-center`}
              style={{ width: `${widthPct}%` }}
              title={`${formatPhase(m.phase)} — ${formatWorkoutType(m.focus)}`}
            >
              {widthPct > 10 && (
                <span className="text-cream text-[10px] font-medium truncate px-1">
                  {formatPhase(m.phase)}
                </span>
              )}
            </div>
          );
        })}
        {/* Today marker */}
        <div
          className="absolute top-0 bottom-0 w-0.5 bg-charcoal"
          style={{ left: `${todayPct}%` }}
        />
      </div>
    </div>
  );
}
