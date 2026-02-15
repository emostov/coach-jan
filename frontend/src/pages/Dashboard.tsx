import { Link } from 'react-router';
import { differenceInDays } from 'date-fns';
import { useProfile } from '../hooks/useAthlete';
import { useAuthStore } from '../hooks/useAuth';

export default function Dashboard() {
  const user = useAuthStore((s) => s.user);
  const { data: profileData } = useProfile();

  const profile = profileData?.profile;
  const athleteName = profile?.name?.split(' ')[0] ?? user?.email?.split('@')[0] ?? 'Athlete';

  // We'll check for race goal from the profile data if available
  // For now, race goal info is embedded in the profile creation but not directly on the profile object
  // This will be enhanced when race goals get their own endpoint

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

      {/* Race countdown placeholder */}
      <RaceCountdown />

      {/* Training plan placeholder */}
      <div className="rounded-xl bg-white border border-cream-dark p-6">
        <div className="flex items-start gap-4">
          <div className="w-10 h-10 rounded-lg bg-forest/10 flex items-center justify-center flex-shrink-0">
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
          </div>
          <div>
            <h2 className="font-serif text-xl text-charcoal font-semibold">Your Training Plan</h2>
            <p className="text-slate mt-1 text-sm leading-relaxed">
              Your personalized training plan will appear here once we generate it.
              This is where you'll see your daily workouts, weekly structure, and
              mesocycle progression.
            </p>
            <p className="text-slate-light mt-3 text-xs italic">
              Coming in Phase 3
            </p>
          </div>
        </div>
      </div>

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
      <div className="rounded-xl bg-forest/5 border border-forest/10 p-5">
        <div className="flex gap-3">
          <div className="flex-shrink-0 mt-0.5">
            <div className="w-7 h-7 rounded-full bg-forest flex items-center justify-center">
              <span className="text-cream text-xs font-bold font-serif">J</span>
            </div>
          </div>
          <div>
            <p className="text-sm text-charcoal leading-relaxed">
              <span className="font-medium">Coach Jan says:</span> "We start by building
              your aerobic capacity. Capacity is for training, power is for racing. Trust
              the process and let's develop a strong foundation together."
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

function RaceCountdown() {
  // Placeholder: in Phase 1 we don't have a race goals API endpoint separate from profile creation.
  // For now, show a generic motivational countdown card.
  // This will be wired up properly when race goals become queryable.
  const today = new Date();
  const sampleRaceDate = new Date(today.getFullYear(), today.getMonth() + 4, 1);
  const daysUntil = differenceInDays(sampleRaceDate, today);

  return (
    <div className="rounded-xl bg-gradient-to-br from-forest to-forest-light p-5 text-cream">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-cream/70 text-xs uppercase tracking-wider">Race Day</p>
          <p className="font-serif text-2xl font-bold mt-0.5">
            {daysUntil} days to go
          </p>
        </div>
        <div className="text-right">
          <p className="text-cream/70 text-xs">Your goal</p>
          <p className="text-sm font-medium mt-0.5">Set up in your profile</p>
        </div>
      </div>
    </div>
  );
}

function getTimeOfDay(): string {
  const hour = new Date().getHours();
  if (hour < 12) return 'morning';
  if (hour < 17) return 'afternoon';
  return 'evening';
}
