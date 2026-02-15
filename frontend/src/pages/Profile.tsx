import { Link } from 'react-router';
import { useProfile } from '../hooks/useAthlete';
import { formatPace } from '../utils/formatting';
import ZoneTable from '../components/shared/ZoneTable';

export default function Profile() {
  const { data, isLoading, error } = useProfile();

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-20">
        <div className="w-6 h-6 border-2 border-forest border-t-transparent rounded-full animate-spin" />
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="text-center py-20">
        <p className="text-slate">Could not load your profile.</p>
        <Link to="/" className="text-forest font-medium hover:text-forest-light mt-2 inline-block">
          Back to dashboard
        </Link>
      </div>
    );
  }

  const { profile, hr_zones, pace_zones } = data;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-start justify-between">
        <div>
          <h1 className="font-serif text-3xl text-charcoal font-bold">{profile.name}</h1>
          <p className="text-slate mt-1">
            {profile.experience_level.charAt(0).toUpperCase() + profile.experience_level.slice(1)} runner
            {profile.sports_background && ` / ${profile.sports_background}`}
          </p>
        </div>
        <Link
          to="/profile/edit"
          className="px-4 py-2 rounded-lg border border-cream-dark text-charcoal text-sm font-medium hover:bg-cream-dark/50 transition-colors"
        >
          Edit Profile
        </Link>
      </div>

      {/* Key stats */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <StatCard label="Age" value={String(profile.age)} />
        <StatCard label="Weight" value={`${profile.weight_kg} kg`} />
        <StatCard label="LTHR" value={`${profile.lthr} bpm`} />
        <StatCard
          label="FT Pace"
          value={profile.ftpace_m_per_s ? `${formatPace(profile.ftpace_m_per_s)} /km` : '--'}
        />
      </div>

      {/* Physiological details */}
      <div className="rounded-xl bg-white border border-cream-dark p-5">
        <h3 className="font-serif text-lg text-charcoal font-semibold mb-4">Physiological Profile</h3>
        <div className="grid grid-cols-2 sm:grid-cols-3 gap-y-3 gap-x-6 text-sm">
          <DetailRow label="Resting HR" value={`${profile.resting_hr} bpm`} />
          <DetailRow label="Max HR" value={`${profile.max_hr} bpm`} />
          <DetailRow label="LTHR" value={`${profile.lthr} bpm`} />
          <DetailRow
            label="FT Pace"
            value={
              profile.ftpace_m_per_s
                ? `${formatPace(profile.ftpace_m_per_s)} /km (${profile.ftpace_m_per_s.toFixed(2)} m/s)`
                : 'Not set'
            }
          />
          <DetailRow label="Weekly volume" value={`${profile.current_weekly_volume_km} km`} />
          <DetailRow
            label="Experience"
            value={profile.experience_level.charAt(0).toUpperCase() + profile.experience_level.slice(1)}
          />
        </div>
      </div>

      {/* HR Zones */}
      <ZoneTable title="Heart Rate Zones" type="hr" zones={hr_zones.zones} />

      {/* Pace Zones */}
      {pace_zones && pace_zones.zones.length > 0 && (
        <ZoneTable title="Pace Zones" type="pace" zones={pace_zones.zones} />
      )}
    </div>
  );
}

function StatCard({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-xl bg-white border border-cream-dark p-4 text-center">
      <p className="text-xs text-slate uppercase tracking-wider">{label}</p>
      <p className="mt-1 text-lg font-semibold text-charcoal">{value}</p>
    </div>
  );
}

function DetailRow({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <span className="text-slate">{label}</span>
      <p className="text-charcoal font-medium">{value}</p>
    </div>
  );
}
