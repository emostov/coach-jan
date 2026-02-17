import { useRef, useCallback, useMemo, useState } from 'react';
import {
  startOfISOWeek, endOfISOWeek, addDays, isSameDay, parseISO,
  format, eachWeekOfInterval, isSameWeek,
} from 'date-fns';
import type { PlanResponse, PlannedWorkout, MesocycleWithWorkouts } from '../../api/types';
import WorkoutCard from './WorkoutCard';
import WeekSummary from './WeekSummary';
import WorkoutDrawer from './WorkoutDrawer';
import { getAllWorkouts, getMesocycleForDate, getWeekOfMesocycle, formatPhase, formatWorkoutType } from '../../utils/planHelpers';

interface Props {
  plan: PlanResponse;
}

export default function Calendar({ plan }: Props) {
  const todayRef = useRef<HTMLDivElement>(null);
  const [selectedWorkout, setSelectedWorkout] = useState<PlannedWorkout | null>(null);
  const [selectedMesocycle, setSelectedMesocycle] = useState<MesocycleWithWorkouts | null>(null);

  const allWorkouts = useMemo(() => getAllWorkouts(plan), [plan]);
  const macroStart = parseISO(plan.macrocycle.start_date);
  const macroEnd = parseISO(plan.macrocycle.end_date);

  // Generate all ISO weeks in the macrocycle
  const weeks = useMemo(() => {
    return eachWeekOfInterval(
      { start: macroStart, end: macroEnd },
      { weekStartsOn: 1 }
    );
  }, [macroStart, macroEnd]);

  // Group workouts by week start date
  const workoutsByWeek = useMemo(() => {
    const map = new Map<string, PlannedWorkout[]>();
    for (const w of allWorkouts) {
      const weekStart = startOfISOWeek(parseISO(w.scheduled_date));
      const key = format(weekStart, 'yyyy-MM-dd');
      if (!map.has(key)) map.set(key, []);
      map.get(key)!.push(w);
    }
    return map;
  }, [allWorkouts]);

  // Precompute mesocycle boundaries: for each week, whether it starts a new mesocycle
  const mesoBoundaries = useMemo(() => {
    const boundaries = new Map<string, MesocycleWithWorkouts | null>();
    let lastMesoId: number | null = null;
    for (const weekStart of weeks) {
      const currentMeso = getMesocycleForDate(plan, weekStart);
      const mesoId = currentMeso?.id ?? null;
      if (lastMesoId !== null && mesoId !== lastMesoId && currentMeso) {
        boundaries.set(format(weekStart, 'yyyy-MM-dd'), currentMeso);
      }
      lastMesoId = mesoId;
    }
    return boundaries;
  }, [weeks, plan]);

  const scrollToToday = useCallback(() => {
    todayRef.current?.scrollIntoView({ behavior: 'smooth', block: 'center' });
  }, []);

  const handleWorkoutClick = (workout: PlannedWorkout) => {
    const meso = plan.mesocycles.find(m => m.id === workout.mesocycle_id) ?? null;
    setSelectedWorkout(workout);
    setSelectedMesocycle(meso);
  };

  const today = new Date();
  const dayHeaders = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

  return (
    <div>
      {/* Navigation */}
      <div className="flex items-center justify-between mb-4">
        <button
          onClick={scrollToToday}
          className="text-xs font-medium text-forest border border-forest/30 rounded-lg px-3 py-1.5 hover:bg-forest/5 transition-colors"
        >
          Today
        </button>
      </div>

      {/* Desktop calendar grid */}
      <div className="hidden md:block">
        {/* Day headers */}
        <div className="grid grid-cols-[repeat(7,1fr)_120px] gap-px mb-1">
          {dayHeaders.map(d => (
            <div key={d} className="text-center text-[10px] font-medium text-slate uppercase tracking-wider py-1">
              {d}
            </div>
          ))}
          <div className="text-center text-[10px] font-medium text-slate uppercase tracking-wider py-1">
            Summary
          </div>
        </div>

        {/* Week rows */}
        {weeks.map((weekStart) => {
          const weekKey = format(weekStart, 'yyyy-MM-dd');
          const weekWorkouts = workoutsByWeek.get(weekKey) ?? [];
          const isThisWeek = isSameWeek(weekStart, today, { weekStartsOn: 1 });
          const currentMeso = getMesocycleForDate(plan, weekStart);

          // Check for mesocycle boundary from precomputed map
          const boundaryMeso = mesoBoundaries.get(weekKey) ?? null;

          const weekLabel = currentMeso
            ? `Wk ${getWeekOfMesocycle(currentMeso, weekStart)}`
            : format(weekStart, 'MMM d');

          return (
            <div key={weekKey}>
              {/* Mesocycle boundary divider */}
              {boundaryMeso && (
                <div className="flex items-center gap-2 py-2">
                  <div className="flex-1 h-px bg-terra/30" />
                  <span className="text-[10px] font-medium text-terra">
                    {formatPhase(boundaryMeso.phase)} — {formatWorkoutType(boundaryMeso.focus)}
                  </span>
                  <div className="flex-1 h-px bg-terra/30" />
                </div>
              )}

              <div
                ref={isThisWeek ? todayRef : undefined}
                className={`grid grid-cols-[repeat(7,1fr)_120px] gap-px ${
                  isThisWeek ? 'bg-forest/5 rounded-lg' : ''
                }`}
              >
                {/* 7 day cells */}
                {Array.from({ length: 7 }, (_, i) => {
                  const cellDate = addDays(weekStart, i);
                  const cellDateStr = format(cellDate, 'yyyy-MM-dd');
                  const workout = weekWorkouts.find(w => w.scheduled_date === cellDateStr);
                  const isToday = isSameDay(cellDate, today);

                  return (
                    <div
                      key={i}
                      className={`min-h-[80px] p-1 border-b border-cream-dark/50 ${
                        isToday ? 'bg-forest/5' : ''
                      }`}
                    >
                      <p className={`text-[10px] mb-0.5 ${isToday ? 'font-bold text-forest' : 'text-slate-light'}`}>
                        {format(cellDate, 'd')}
                      </p>
                      {workout && (
                        <WorkoutCard
                          workout={workout}
                          onClick={() => handleWorkoutClick(workout)}
                        />
                      )}
                    </div>
                  );
                })}

                {/* Weekly summary column */}
                <div className="border-b border-cream-dark/50 border-l border-cream-dark/30">
                  <WeekSummary workouts={weekWorkouts} weekLabel={weekLabel} />
                </div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Mobile stacked list */}
      <div className="md:hidden space-y-6">
        {weeks.map((weekStart) => {
          const weekKey = format(weekStart, 'yyyy-MM-dd');
          const weekWorkouts = workoutsByWeek.get(weekKey) ?? [];
          if (weekWorkouts.length === 0) return null;
          const currentMeso = getMesocycleForDate(plan, weekStart);
          const weekLabel = currentMeso
            ? `Week ${getWeekOfMesocycle(currentMeso, weekStart)}`
            : format(weekStart, 'MMM d');

          return (
            <div key={weekKey}>
              <div className="flex items-center justify-between mb-2">
                <h3 className="text-sm font-medium text-charcoal">
                  {weekLabel} · {format(weekStart, 'MMM d')} – {format(endOfISOWeek(weekStart), 'MMM d')}
                </h3>
              </div>
              <WeekSummary workouts={weekWorkouts} weekLabel={weekLabel} />
              <div className="space-y-1 mt-2">
                {weekWorkouts
                  .sort((a, b) => a.scheduled_date.localeCompare(b.scheduled_date))
                  .map(w => (
                    <div key={w.id} className="flex items-center gap-2">
                      <span className="text-[10px] text-slate-light w-8">
                        {format(parseISO(w.scheduled_date), 'EEE')}
                      </span>
                      <div className="flex-1">
                        <WorkoutCard workout={w} onClick={() => handleWorkoutClick(w)} />
                      </div>
                    </div>
                  ))}
              </div>
            </div>
          );
        })}
      </div>

      {/* Workout detail drawer */}
      <WorkoutDrawer
        workout={selectedWorkout}
        mesocycle={selectedMesocycle}
        onClose={() => setSelectedWorkout(null)}
      />
    </div>
  );
}
