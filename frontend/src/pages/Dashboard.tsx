import { Link } from 'react-router';
import { differenceInDays, format, parseISO } from 'date-fns';
import { useProfile } from '../hooks/useAthlete';
import { useCurrentPlan } from '../hooks/usePlan';
import { useAuthStore } from '../hooks/useAuth';
import type { RaceGoal, PlannedWorkout, Mesocycle, Macrocycle } from '../api/types';

export default function Dashboard() {
  const user = useAuthStore((s) => s.user);
  const { data: profileData } = useProfile();
  const { data: planData, isLoading: planLoading, isError: planError } = useCurrentPlan();

  const profile = profileData?.profile;
  const raceGoal = profileData?.race_goal ?? null;
  const athleteName = profile?.name?.split(' ')[0] ?? user?.email?.split('@')[0] ?? 'Athlete';

  const macrocycle = planData?.macrocycle ?? null;
  const mesocycles = planData?.mesocycles ?? [];
  const workouts = planData?.workouts ?? [];

  // Find the current mesocycle (first one whose end_date is in the future)
  const today = new Date();
  const currentMesocycle = mesocycles.find((m) => parseISO(m.end_date) >= today) ?? mesocycles[0] ?? null;

  // Filter workouts for the current mesocycle
  const currentWorkouts = currentMesocycle
    ? workouts
        .filter((w) => w.mesocycle_id === currentMesocycle.id)
        .sort((a, b) => a.scheduled_date.localeCompare(b.scheduled_date))
    : [];

  return (
    <div className="space-y-6">
      {/* Welcome header */}
      <div>
        <h1 className="font-serif text-3xl text-charcoal font-bold">
          Good {getTimeOfDay()}, {athleteName}
        </h1>
        <p className="text-slate mt-1">
          Let's build something great together.
        </p>
      </div>

      {/* Race countdown */}
      <RaceCountdown raceGoal={raceGoal} />

      {/* Training plan */}
      <TrainingPlanCard
        macrocycle={macrocycle}
        currentMesocycle={currentMesocycle}
        workouts={currentWorkouts}
        isLoading={planLoading}
        isError={planError}
      />

      {/* Quick links */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <Link
          to="/profile"
          className="rounded-xl bg-white border border-cream-dark p-5 hover:border-forest/30 transition-colors group"
        >
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-forest/10 flex items-center justify-center group-hover:bg-forest/20 transition-colors">
              <span className="text-forest text-sm">P</span>
            </div>
            <div>
              <p className="text-charcoal font-medium text-sm">View Profile</p>
              <p className="text-slate-light text-xs">Zones, stats & settings</p>
            </div>
          </div>
        </Link>

        <div className="rounded-xl bg-white border border-cream-dark p-5 opacity-50 cursor-not-allowed">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 rounded-lg bg-cream-dark flex items-center justify-center">
              <span className="text-slate-light text-sm">U</span>
            </div>
            <div>
              <p className="text-charcoal font-medium text-sm">Upload Workout</p>
              <p className="text-slate-light text-xs">FIT file analysis (coming soon)</p>
            </div>
          </div>
        </div>
      </div>

      {/* Coaching note */}
      <CoachNote coachMessage={macrocycle?.coach_message ?? null} />
    </div>
  );
}

// --- Race Countdown ---

function RaceCountdown({ raceGoal }: { raceGoal: RaceGoal | null }) {
  if (!raceGoal) {
    return (
      <div className="rounded-xl bg-gradient-to-br from-forest to-forest-light p-5 text-cream">
        <div className="flex items-center justify-between">
          <div>
            <p className="text-cream/70 text-xs uppercase tracking-wider">Race Day</p>
            <p className="font-serif text-2xl font-bold mt-0.5">No race set</p>
          </div>
          <div className="text-right">
            <p className="text-cream/70 text-xs">Your goal</p>
            <p className="text-sm font-medium mt-0.5">Set up in your profile</p>
          </div>
        </div>
      </div>
    );
  }

  const daysUntil = differenceInDays(parseISO(raceGoal.race_date), new Date());
  const raceName = raceGoal.race_name || 'Race Day';
  const distanceLabel = formatDistance(raceGoal.distance_m);

  return (
    <div className="rounded-xl bg-gradient-to-br from-forest to-forest-light p-5 text-cream">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-cream/70 text-xs uppercase tracking-wider">{raceName}</p>
          <p className="font-serif text-2xl font-bold mt-0.5">
            {daysUntil > 0 ? `${daysUntil} days to go` : daysUntil === 0 ? 'Race day!' : 'Race completed'}
          </p>
          <p className="text-cream/60 text-xs mt-1">
            {distanceLabel} &middot; {format(parseISO(raceGoal.race_date), 'MMMM d, yyyy')}
          </p>
        </div>
        {raceGoal.target_time_seconds != null && (
          <div className="text-right">
            <p className="text-cream/70 text-xs">Target time</p>
            <p className="font-serif text-xl font-bold mt-0.5">
              {formatSeconds(raceGoal.target_time_seconds)}
            </p>
          </div>
        )}
      </div>
    </div>
  );
}

// --- Training Plan Card ---

interface TrainingPlanCardProps {
  macrocycle: Macrocycle | null;
  currentMesocycle: Mesocycle | null;
  workouts: PlannedWorkout[];
  isLoading: boolean;
  isError: boolean;
}

function TrainingPlanCard({ currentMesocycle, workouts, isLoading, isError }: TrainingPlanCardProps) {
  // Loading state
  if (isLoading) {
    return (
      <div className="rounded-xl bg-white border border-cream-dark p-6">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-forest/10 flex items-center justify-center flex-shrink-0">
            <CalendarIcon />
          </div>
          <h2 className="font-serif text-xl text-charcoal font-semibold">Your Training Plan</h2>
        </div>
        <p className="text-slate mt-4 text-sm">Loading your plan...</p>
      </div>
    );
  }

  // No plan / error state
  if (isError || !currentMesocycle || workouts.length === 0) {
    return (
      <div className="rounded-xl bg-white border border-cream-dark p-6">
        <div className="flex items-start gap-4">
          <div className="w-10 h-10 rounded-lg bg-forest/10 flex items-center justify-center flex-shrink-0">
            <CalendarIcon />
          </div>
          <div>
            <h2 className="font-serif text-xl text-charcoal font-semibold">Your Training Plan</h2>
            <p className="text-slate mt-1 text-sm leading-relaxed">
              No training plan yet. Once you create a profile with a race goal, you can generate
              your personalized training plan from the Plan page.
            </p>
            <Link
              to="/plan"
              className="inline-block mt-3 text-sm font-medium text-forest hover:text-forest-light transition-colors"
            >
              Go to Plan &rarr;
            </Link>
          </div>
        </div>
      </div>
    );
  }

  // Phase display
  const phaseLabel = formatPhase(currentMesocycle.phase);
  const focusLabel = currentMesocycle.focus ? ` \u2014 ${formatWorkoutType(currentMesocycle.focus)}` : '';

  return (
    <div className="rounded-xl bg-white border border-cream-dark p-6">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-lg bg-forest/10 flex items-center justify-center flex-shrink-0">
            <CalendarIcon />
          </div>
          <div>
            <h2 className="font-serif text-xl text-charcoal font-semibold">Your Training Plan</h2>
            <p className="text-slate text-xs mt-0.5">
              {phaseLabel}{focusLabel}
            </p>
          </div>
        </div>
        <Link
          to="/plan"
          className="text-xs font-medium text-forest hover:text-forest-light transition-colors"
        >
          View full plan
        </Link>
      </div>

      <div className="space-y-1">
        {workouts.map((w) => (
          <WorkoutRow key={w.id} workout={w} />
        ))}
      </div>
    </div>
  );
}

function WorkoutRow({ workout }: { workout: PlannedWorkout }) {
  const borderColor = getWorkoutBorderColor(workout.workout_type);
  const date = parseISO(workout.scheduled_date);
  const dayLabel = format(date, 'EEE');
  const dateLabel = format(date, 'MMM d');
  const typeLabel = formatWorkoutType(workout.workout_type);
  const durationLabel = workout.duration_min ? `${workout.duration_min} min` : null;

  return (
    <div
      className="flex items-start gap-3 py-2.5 px-3 rounded-lg hover:bg-cream/50 transition-colors"
      style={{ borderLeft: `3px solid ${borderColor}` }}
    >
      <div className="w-14 flex-shrink-0 text-center">
        <p className="text-xs font-medium text-charcoal">{dayLabel}</p>
        <p className="text-[11px] text-slate-light">{dateLabel}</p>
      </div>
      <div className="flex-1 min-w-0">
        <div className="flex items-baseline gap-2">
          <p className="text-sm font-medium text-charcoal truncate">{typeLabel}</p>
          {durationLabel && (
            <span className="text-xs text-slate-light flex-shrink-0">{durationLabel}</span>
          )}
        </div>
        {workout.description && (
          <p className="text-xs text-slate mt-0.5 leading-relaxed">{workout.description}</p>
        )}
      </div>
      {workout.is_completed === 1 && (
        <span className="text-xs text-forest font-medium flex-shrink-0">Done</span>
      )}
    </div>
  );
}

// --- Coach Note ---

function CoachNote({ coachMessage }: { coachMessage: string | null }) {
  const fallbackMessage =
    'We start by building your aerobic capacity. Capacity is for training, power is for racing. Trust the process and let\'s develop a strong foundation together.';
  const message = coachMessage || fallbackMessage;

  return (
    <div className="rounded-xl bg-forest/5 border border-forest/10 p-5">
      <div className="flex gap-3">
        <div className="flex-shrink-0 mt-0.5">
          <div className="w-7 h-7 rounded-full bg-forest flex items-center justify-center">
            <span className="text-cream text-xs font-bold font-serif">J</span>
          </div>
        </div>
        <div>
          <p className="text-sm text-charcoal leading-relaxed">
            <span className="font-medium">Coach Jan says:</span> &ldquo;{message}&rdquo;
          </p>
        </div>
      </div>
    </div>
  );
}

// --- Helpers ---

function CalendarIcon() {
  return (
    <svg
      className="w-5 h-5 text-forest"
      fill="none"
      stroke="currentColor"
      viewBox="0 0 24 24"
      strokeWidth={1.5}
    >
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5"
      />
    </svg>
  );
}

function getTimeOfDay(): string {
  const hour = new Date().getHours();
  if (hour < 12) return 'morning';
  if (hour < 17) return 'afternoon';
  return 'evening';
}

function formatWorkoutType(snakeCase: string): string {
  return snakeCase
    .replace(/_/g, ' ')
    .replace(/\b\w/g, (c) => c.toUpperCase());
}

function formatPhase(phase: string): string {
  return formatWorkoutType(phase);
}

function formatDistance(meters: number): string {
  if (meters === 42195) return 'Marathon (42.2 km)';
  if (meters === 21098 || meters === 21097) return 'Half Marathon (21.1 km)';
  if (meters === 10000) return '10K';
  if (meters === 5000) return '5K';
  const km = meters / 1000;
  if (km >= 1) return `${km % 1 === 0 ? km.toFixed(0) : km.toFixed(1)} km`;
  return `${meters} m`;
}

function formatSeconds(totalSeconds: number): string {
  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  const seconds = totalSeconds % 60;
  if (hours > 0) {
    return `${hours}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
  }
  return `${minutes}:${String(seconds).padStart(2, '0')}`;
}

function getWorkoutBorderColor(workoutType: string): string {
  // Easy / recovery / long runs → forest green
  const easyTypes = ['easy_run', 'recovery_run', 'long_run_easy', 'long_run', 'warm_up', 'cool_down'];
  if (easyTypes.some((t) => workoutType.includes(t) || workoutType === t)) return '#1b4332';

  // Tempo / threshold → terra/amber
  const tempoTypes = ['tempo_run', 'threshold_intervals', 'cruise_intervals', 'tempo', 'threshold', 'steady_state'];
  if (tempoTypes.some((t) => workoutType.includes(t) || workoutType === t)) return '#c4572a';

  // Intervals / track / VO2max / anaerobic → zone-5 red
  const intervalTypes = [
    'vo2max_intervals', 'vo2max', 'track_200m', 'track_400m', 'track_800m',
    'anaerobic_flat', 'anaerobic_hills', 'interval', 'speed', 'fartlek', 'repetition',
  ];
  if (intervalTypes.some((t) => workoutType.includes(t) || workoutType === t)) return '#ef4444';

  // Strength → slate
  const strengthTypes = ['strength_general', 'strength_power', 'strength', 'mobility'];
  if (strengthTypes.some((t) => workoutType.includes(t) || workoutType === t)) return '#64748b';

  // Rest day → cream-dark
  if (workoutType.includes('rest') || workoutType === 'rest_day' || workoutType === 'off') return '#f0ede5';

  // Default → slate-light
  return '#94a3b8';
}
