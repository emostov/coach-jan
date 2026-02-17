import { Link } from 'react-router';
import { differenceInDays, format, parseISO } from 'date-fns';
import { useProfile } from '../hooks/useAthlete';
import { useCurrentPlan } from '../hooks/usePlan';
import { useAuthStore } from '../hooks/useAuth';
import { getWorkoutBorderColor } from '../utils/workoutColors';
import { formatWorkoutType, getTodaysWorkout, getWeekWorkouts, getWeekSummary, getMesocycleForDate, getWeekOfMesocycle, getAllWorkouts } from '../utils/planHelpers';
import type { RaceGoal, PlanResponse } from '../api/types';

export default function Dashboard() {
  const user = useAuthStore((s) => s.user);
  const { data: profileData } = useProfile();
  const { data: planData, isLoading: planLoading, isError: planError } = useCurrentPlan();

  const profile = profileData?.profile;
  const raceGoal = profileData?.race_goal ?? null;
  const athleteName = profile?.name?.split(' ')[0] ?? user?.email?.split('@')[0] ?? 'Athlete';

  const macrocycle = planData?.macrocycle ?? null;

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

      {/* Phase banner */}
      {planData && <PhaseBanner plan={planData} />}

      {/* Today's workout hero */}
      <TodaysWorkoutCard plan={planData ?? null} isLoading={planLoading} isError={planError} />

      {/* Weekly progress */}
      {planData && <WeeklyProgressBar plan={planData} />}

      {/* Metrics placeholder */}
      <MetricsPlaceholder />

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

// --- Phase Banner ---

function PhaseBanner({ plan }: { plan: PlanResponse }) {
  const today = new Date();
  const currentMeso = getMesocycleForDate(plan, today);
  if (!currentMeso) return null;

  const weekNum = getWeekOfMesocycle(currentMeso, today);
  const totalWeeks = currentMeso.load_weeks + currentMeso.recovery_weeks;

  return (
    <div className="rounded-xl bg-white border border-cream-dark px-5 py-3">
      <p className="text-sm text-charcoal">
        <span className="font-medium">Mesocycle {currentMeso.sequence_number}</span>
        {' — '}
        {formatWorkoutType(currentMeso.focus)}
        <span className="text-slate ml-2">Week {weekNum} of {totalWeeks}</span>
      </p>
    </div>
  );
}

// --- Today's Workout Card ---

function TodaysWorkoutCard({ plan, isLoading, isError }: { plan: PlanResponse | null; isLoading: boolean; isError: boolean }) {
  // Loading state
  if (isLoading) {
    return (
      <div className="rounded-xl bg-white border border-cream-dark p-6">
        <div className="flex items-center gap-3">
          <CalendarIcon />
          <h2 className="font-serif text-xl text-charcoal font-semibold">Today</h2>
        </div>
        <p className="text-slate mt-4 text-sm">Loading your plan...</p>
      </div>
    );
  }

  // No plan / error state
  if (isError || !plan) {
    return (
      <div className="rounded-xl bg-white border border-cream-dark p-6">
        <div className="flex items-start gap-4">
          <CalendarIcon />
          <div>
            <h2 className="font-serif text-xl text-charcoal font-semibold">Today</h2>
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

  const today = new Date();
  const todaysWorkout = getTodaysWorkout(plan);

  // Rest day
  if (todaysWorkout && todaysWorkout.workout_type === 'rest') {
    return (
      <div className="rounded-xl bg-white border border-cream-dark p-6">
        <div className="flex items-center gap-3 mb-2">
          <CalendarIcon />
          <h2 className="font-serif text-xl text-charcoal font-semibold">Today</h2>
        </div>
        <div className="text-center py-4">
          <p className="font-serif text-2xl text-charcoal font-bold">Rest Day</p>
          <p className="text-slate text-sm mt-1">Recovery is training too.</p>
        </div>
      </div>
    );
  }

  // Active workout today
  if (todaysWorkout) {
    const borderColor = getWorkoutBorderColor(todaysWorkout.workout_type);
    return (
      <div className="rounded-xl bg-white border border-cream-dark p-6">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-3">
            <CalendarIcon />
            <h2 className="font-serif text-xl text-charcoal font-semibold">Today</h2>
          </div>
          <Link to="/plan" className="text-xs font-medium text-forest hover:text-forest-light transition-colors">
            View full plan &rarr;
          </Link>
        </div>
        <div style={{ borderLeft: `4px solid ${borderColor}`, paddingLeft: '16px' }}>
          <p className="font-serif text-2xl text-charcoal font-bold">
            {formatWorkoutType(todaysWorkout.workout_type)}
          </p>
          <div className="flex items-center gap-3 mt-2 text-sm text-slate">
            {todaysWorkout.duration_min && <span>{todaysWorkout.duration_min} min</span>}
            {todaysWorkout.target_distance_km != null && <span>{todaysWorkout.target_distance_km} km</span>}
            {todaysWorkout.expected_tss != null && <span>{Math.round(todaysWorkout.expected_tss)} TSS</span>}
          </div>
          {todaysWorkout.target_hr_zones && (
            <p className="text-xs text-slate mt-1">HR: {todaysWorkout.target_hr_zones}</p>
          )}
          {todaysWorkout.description && (
            <p className="text-sm text-charcoal mt-3 leading-relaxed">{todaysWorkout.description}</p>
          )}
          {todaysWorkout.coach_notes && (
            <div className="mt-3 rounded-lg bg-forest/5 border border-forest/10 p-3">
              <p className="text-xs text-charcoal leading-relaxed">
                <span className="font-medium">Coach Jan:</span> {todaysWorkout.coach_notes}
              </p>
            </div>
          )}
        </div>
      </div>
    );
  }

  // No workout today — show next upcoming
  const allWorkouts = getAllWorkouts(plan);
  const nextWorkout = allWorkouts
    .filter(w => parseISO(w.scheduled_date) > today && w.workout_type !== 'rest')
    .sort((a, b) => a.scheduled_date.localeCompare(b.scheduled_date))[0];

  return (
    <div className="rounded-xl bg-white border border-cream-dark p-6">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-3">
          <CalendarIcon />
          <h2 className="font-serif text-xl text-charcoal font-semibold">Today</h2>
        </div>
        <Link to="/plan" className="text-xs font-medium text-forest hover:text-forest-light transition-colors">
          View full plan &rarr;
        </Link>
      </div>
      {nextWorkout ? (
        <div>
          <p className="text-xs text-slate uppercase tracking-wider mb-1">
            Next up: {format(parseISO(nextWorkout.scheduled_date), 'EEEE')}
          </p>
          <p className="font-serif text-xl text-charcoal font-bold">
            {formatWorkoutType(nextWorkout.workout_type)}
          </p>
          <div className="flex items-center gap-3 mt-1 text-sm text-slate">
            {nextWorkout.duration_min && <span>{nextWorkout.duration_min} min</span>}
          </div>
        </div>
      ) : (
        <p className="text-slate text-sm">No upcoming workouts scheduled.</p>
      )}
    </div>
  );
}

// --- Weekly Progress Bar ---

function WeeklyProgressBar({ plan }: { plan: PlanResponse }) {
  const today = new Date();
  const weekWorkouts = getWeekWorkouts(plan, today);
  const { targetKm, targetTss, totalSessions, completedSessions } = getWeekSummary(weekWorkouts);

  if (weekWorkouts.length === 0) return null;

  return (
    <div className="rounded-xl bg-white border border-cream-dark p-5">
      <h3 className="text-xs font-medium text-slate uppercase tracking-wider mb-3">This Week</h3>
      <div className="grid grid-cols-3 gap-4 text-center">
        {targetKm > 0 && (
          <div>
            <p className="text-lg font-bold text-charcoal">{targetKm.toFixed(1)}</p>
            <p className="text-[10px] text-slate uppercase">km planned</p>
          </div>
        )}
        {targetTss > 0 && (
          <div>
            <p className="text-lg font-bold text-charcoal">{Math.round(targetTss)}</p>
            <p className="text-[10px] text-slate uppercase">TSS planned</p>
          </div>
        )}
        <div>
          <p className="text-lg font-bold text-charcoal">{completedSessions}/{totalSessions}</p>
          <p className="text-[10px] text-slate uppercase">sessions</p>
        </div>
      </div>
    </div>
  );
}

// --- Metrics Placeholder ---

function MetricsPlaceholder() {
  return (
    <div className="grid grid-cols-3 gap-3">
      {['CTL', 'ATL', 'TSB'].map((metric) => (
        <div key={metric} className="rounded-xl bg-white border border-cream-dark p-4 text-center opacity-50">
          <p className="text-lg font-bold text-charcoal">0.0</p>
          <p className="text-[10px] text-slate uppercase">{metric}</p>
          <p className="text-[9px] text-slate-light mt-1">Phase 5</p>
        </div>
      ))}
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
