import type { HrZone, PaceZone } from '../../api/types';
import { formatPace } from '../../utils/formatting';

interface ZoneTableProps {
  title: string;
  type: 'hr' | 'pace';
  zones: HrZone[] | PaceZone[];
}

const ZONE_COLORS: Record<number, string> = {
  1: 'bg-zone-1',
  2: 'bg-zone-2',
  3: 'bg-zone-3',
  4: 'bg-zone-4',
  5: 'bg-zone-5',
  6: 'bg-zone-6',
  7: 'bg-zone-7',
};

const ZONE_TEXT_COLORS: Record<number, string> = {
  1: 'text-zone-1',
  2: 'text-zone-2',
  3: 'text-zone-3',
  4: 'text-zone-4',
  5: 'text-zone-5',
  6: 'text-zone-6',
  7: 'text-zone-7',
};

const ZONE_BORDER_COLORS: Record<number, string> = {
  1: 'border-zone-1',
  2: 'border-zone-2',
  3: 'border-zone-3',
  4: 'border-zone-4',
  5: 'border-zone-5',
  6: 'border-zone-6',
  7: 'border-zone-7',
};

function isHrZone(zone: HrZone | PaceZone): zone is HrZone {
  return 'min_bpm' in zone;
}

function getHrRange(zones: HrZone[]): { min: number; max: number } {
  const min = Math.min(...zones.map((z) => z.min_bpm));
  const max = Math.max(...zones.map((z) => z.max_bpm ?? z.min_bpm + 20));
  return { min, max };
}

function getPaceRange(zones: PaceZone[]): { min: number; max: number } {
  // For pace zones, faster pace = higher m/s value
  const min = Math.min(...zones.map((z) => z.min_pace_m_per_s));
  const max = Math.max(...zones.map((z) => z.max_pace_m_per_s ?? z.min_pace_m_per_s + 0.5));
  return { min, max };
}

function getBarWidth(
  zone: HrZone | PaceZone,
  totalRange: number,
): number {
  if (isHrZone(zone)) {
    const width = (zone.max_bpm ?? zone.min_bpm + 20) - zone.min_bpm;
    return Math.max((width / totalRange) * 100, 8);
  }
  const paceZone = zone;
  const width = (paceZone.max_pace_m_per_s ?? paceZone.min_pace_m_per_s + 0.5) - paceZone.min_pace_m_per_s;
  return Math.max((width / totalRange) * 100, 8);
}

function formatRange(zone: HrZone | PaceZone, type: 'hr' | 'pace'): string {
  if (type === 'hr' && isHrZone(zone)) {
    if (zone.max_bpm === null) return `${zone.min_bpm}+ bpm`;
    return `${zone.min_bpm}–${zone.max_bpm} bpm`;
  }
  if (!isHrZone(zone)) {
    // Pace: lower m/s = slower = higher min:sec/km
    // Show as "slower pace – faster pace" in min/km
    if (zone.max_pace_m_per_s === null) {
      return `< ${formatPace(zone.min_pace_m_per_s)} /km`;
    }
    // Note: higher m/s = faster = lower min/km number
    const slowPace = formatPace(zone.min_pace_m_per_s);
    const fastPace = formatPace(zone.max_pace_m_per_s);
    return `${slowPace}–${fastPace} /km`;
  }
  return '';
}

export default function ZoneTable({ title, type, zones }: ZoneTableProps) {
  const totalRange =
    type === 'hr'
      ? (() => {
          const range = getHrRange(zones as HrZone[]);
          return range.max - range.min;
        })()
      : (() => {
          const range = getPaceRange(zones as PaceZone[]);
          return range.max - range.min;
        })();

  return (
    <div className="rounded-xl bg-cream-dark/60 border border-cream-dark overflow-hidden">
      <div className="px-5 py-3.5 border-b border-cream-dark">
        <h3 className="text-lg font-semibold text-charcoal font-serif">{title}</h3>
      </div>

      <div className="divide-y divide-cream-dark/80">
        {zones.map((zone) => {
          const zoneNum = zone.zone;
          const barWidth = getBarWidth(zone, totalRange);

          return (
            <div
              key={zoneNum}
              className="flex items-center gap-4 px-5 py-3 hover:bg-cream-dark/40 transition-colors"
            >
              {/* Zone badge */}
              <div
                className={`flex-shrink-0 w-8 h-8 rounded-full ${ZONE_COLORS[zoneNum]} flex items-center justify-center`}
              >
                <span className="text-sm font-bold text-white">{zoneNum}</span>
              </div>

              {/* Zone name */}
              <div className="w-28 flex-shrink-0">
                <span className={`text-sm font-semibold ${ZONE_TEXT_COLORS[zoneNum]}`}>
                  {zone.name}
                </span>
              </div>

              {/* Bar visualization */}
              <div className="flex-1 min-w-0">
                <div className="h-3 bg-cream rounded-full overflow-hidden">
                  <div
                    className={`h-full ${ZONE_COLORS[zoneNum]} rounded-full opacity-70`}
                    style={{ width: `${barWidth}%` }}
                  />
                </div>
              </div>

              {/* Range */}
              <div className="w-36 flex-shrink-0 text-right">
                <span
                  className={`text-sm font-mono ${ZONE_BORDER_COLORS[zoneNum]} border-l-2 pl-3 text-charcoal/80`}
                >
                  {formatRange(zone, type)}
                </span>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
