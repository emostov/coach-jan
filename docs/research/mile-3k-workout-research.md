# Mile & 3K Workout Research: Missing Track Interval Types

## Research Date: 2026-02-16

This document identifies workout types missing from CoachJan's registry that are essential for athletes targeting the 1-mile (1500m/1600m) and 3K (3000m) race distances. These events demand a unique blend of anaerobic power, VO2max utilization, and speed endurance that the current registry under-serves.

---

## Table of Contents

1. [Race Demands: 1 Mile vs 3K](#1-race-demands-1-mile-vs-3k)
2. [Olbrecht Framework for Short Distances](#2-olbrecht-framework-for-short-distances)
3. [Missing Workout Type: Track 1200m Repeats](#3-track-1200m-repeats)
4. [Missing Workout Type: Track 1600m (Mile) Repeats](#4-track-1600m-mile-repeats)
5. [Missing Workout Type: Track Mixed (Ladders/Cut-Downs)](#5-track-mixed-ladderscut-downs)
5b. [Mile-Focused Short Track Workouts](#5b-mile-focused-short-track-workouts)
6. [Missing Workout Type: Speed Endurance (300-600m)](#6-speed-endurance-300-600m)
7. [Coaching References: Daniels, Canova, Magness](#7-coaching-references)
8. [Implementation Recommendations](#8-implementation-recommendations)
9. [Sources](#9-sources)

---

## 1. Race Demands: 1 Mile vs 3K

### Energy System Contributions

| Race | Duration (Competitive) | Duration (Recreational) | Aerobic % | Anaerobic % | Primary Energy System |
|------|----------------------|------------------------|-----------|-------------|----------------------|
| 1 Mile / 1500m | 3:25 - 4:30 | 5:00 - 8:00 | 75-80% | 20-25% | VO2max + glycolytic |
| 3K / 3000m | 7:20 - 9:00 | 10:00 - 15:00 | 86-90% | 10-14% | VO2max + aerobic power |
| 5K (comparison) | 12:30 - 15:00 | 18:00 - 30:00 | 90-95% | 5-10% | Aerobic power + threshold |

### What Makes Mile/3K Training Distinct

1. **Higher anaerobic tolerance needed**: Both races operate at or above VO2max pace. The mile is run at ~98-102% vVO2max, the 3K at ~93-97% vVO2max. Athletes must tolerate higher blood lactate levels (8-12 mmol/L for mile, 6-10 for 3K) vs 5K (4-8 mmol/L).

2. **Speed reserve matters more**: The ability to run FASTER than race pace for short segments (surges, kicks) directly correlates with performance. A miler needs a significant 400m speed reserve; a 3K runner needs 800m speed reserve.

3. **Race-pace work uses shorter intervals**: While a 10K runner trains race pace via 1600m-2000m repeats, a miler's race-pace work uses 400m-800m repeats and a 3K runner uses 600m-1200m repeats.

4. **Mixed-distance sessions are essential**: Unlike longer events where uniform-distance repeats dominate, middle-distance training relies heavily on sessions that mix interval lengths within a single workout — ladders, cut-downs, and alternating-distance formats.

5. **Utilization and capacity overlap**: For events this short (~4-10 min), the distinction between capacity and utilization work blurs. A 200m rep at mile pace takes ~30s — exactly Olbrecht's prescribed burst duration for capacity building, yet it's also race-specific neuromuscular training.

---

## 2. Olbrecht Framework for Short Distances

### Ideal AEC/ANC Balance by Distance

| Race Distance | Ideal AEC (Aerobic Capacity) | Ideal ANC (Anaerobic Capacity) | VLamax Direction |
|---------------|------|------|------|
| 1 Mile / 1500m | Very High | Moderate-High | Higher than 5K+ runners |
| 3K / 3000m | Very High | Moderate | Slightly higher than 5K |
| 5K | Very High | Moderate | Moderate |
| 10K+ | Very High | Low-Moderate | Suppress |

**Key insight**: For the mile, Olbrecht would NOT suppress VLamax as aggressively as for longer distances. The miler needs glycolytic power to sustain speeds above VO2max. However, they still need massive aerobic capacity — the 75-80% aerobic contribution means VO2max is the foundation.

### Modified "High and Low" for Milers

The capacity phase for a miler looks similar to longer distances but with:
- **Slightly longer bursts**: 30-45s at high intensity (vs 30-35s for distance runners) to better target the glycolytic system
- **More frequent anaerobic stimuli**: 2x/week of some form of short-fast work vs 1x for marathon trainers
- **Track-specific neuromuscular patterns**: Bursts done ON the track at target race paces, not just generic fast running

### Utilization Phase Differences

For milers/3K athletes, the utilization phase:
- **Starts earlier**: Race-specific work can be introduced 4-6 weeks before competition (vs 2-3 weeks for longer events)
- **Uses shorter, faster intervals**: 200-800m at race pace or faster, with more recovery than threshold work
- **Includes mixed-distance sessions**: Ladders and cut-downs that simulate the metabolic demands of changing pace during a race
- **Prioritizes speed endurance**: The ability to hold near-max speed while lactate accumulates

### Where New Workout Types Fit in Olbrecht's Model

| Proposed Type | Capacity Phase | Utilization Phase | Category |
|---|---|---|---|
| `track_1200m` | Late capacity (3K-specific aerobic power) | Primary utilization workout | Aerobic Utilization |
| `track_1600m` | Yes (aerobic power at threshold+) | Secondary utilization | Aerobic Utilization |
| `track_mixed` | Late capacity (the short reps = capacity) | Primary utilization (race simulation) | Mixed Capacity + Utilization |
| `speed_endurance` | No (too specific) | Primary utilization (race-pace+ work) | Anaerobic Utilization |

---

## 3. Track 1200m Repeats

### Rationale

1200m repeats sit in a critical gap between the existing 800m (2-2.5 min effort, highly anaerobic) and VO2max intervals (time-based, 1-3 min at generic VO2max effort). For 3K athletes, 1200m is 40% of race distance — the sweet spot for race-specific VO2max development. Each rep takes 3:30-5:00 depending on level, which is long enough to reach and sustain VO2max but short enough for quality accumulation.

### Coaching Consensus

**Daniels**: 1200m at I (Interval) pace, 3-3.5 min jog recovery, 3-5 reps. Considers 1200m the upper limit for I-pace reps — beyond this, runners can't maintain true VO2max intensity for enough reps. Work:rest ratio ~1:0.85-1:1.

**Canova**: 1200m at 95-100% of 3K race pace in the special period. Part of descending mixed sessions (e.g., 2000+1600+1200+800+(4x400)).

**General**: A bread-and-butter workout for collegiate 1500m/3K training programs. Ron Warhurst (Michigan) used 1200m reps extensively.

### Proposed Template

**Key**: `track_1200m`

**Description**: Track 1200m repeats at VO2max/3K-race effort. Sustained high-intensity intervals targeting aerobic power for 3K and mile preparation.

**Olbrecht Category**: Aerobic Utilization (primary), Anaerobic Utilization (secondary)

**When Used**: Utilization phase for 3K athletes. Late capacity phase as VO2max stimulus. Can be used year-round for general aerobic power development.

**Target Zones**: HR Zone 5, Pace Zone 5

| Duration | Structure | TSS Range |
|----------|-----------|-----------|
| Short | WU + 3x1200m @ Zone 5 / 400m jog (~3 min) + CD (~40 min total) | 50-65 |
| Medium | WU + 4x1200m @ Zone 5 / 400m jog (~3.5 min) + CD (~50 min total) | 60-80 |
| Long | WU + 5x1200m @ Zone 5 / 400m jog (~3.5 min) + CD (~55 min total) | 70-95 |

**Progressive Adjustment**: Add reps (3→4→5). Can also reduce recovery (400m jog → 300m jog → 200m jog) as utilization phase progresses. Do not increase pace — pace is fixed at I/VO2max effort.

**Recovery**: 400m jog between reps (~3-3.5 min). This provides approximately 1:1 work:rest ratio, which keeps VO2 elevated between reps.

---

## 4. Track 1600m (Mile) Repeats

### Rationale

Mile repeats are THE definitive aerobic power workout for 3K-10K athletes, but they also serve a specific purpose for milers: running at 98-102% of race distance at goal pace. Currently there's no distance-based workout between 800m and the continuous tempo formats. Mile repeats occupy a unique niche — they're long enough to be primarily aerobic (4:00-7:00 per rep) but fast enough (Zone 4-5) to develop VO2max utilization.

### Coaching Consensus

**Daniels**: Mile repeats at I pace for 5K-10K athletes (3-4 reps with 3-4 min rest). For milers, mile repeats can be done at goal race pace with 4-5 min recovery as a "broken race" format.

**Canova**: Uses 1600m reps in descending sets. Also uses 1600m at 92-95% of 5K race pace as "specific endurance" development.

**General**: The collegiate standard for aerobic power development. "If you can only do one workout, make it mile repeats" is a common coaching maxim for 3K-10K athletes.

### Proposed Template

**Key**: `track_1600m`

**Description**: Track mile (1600m) repeats at VO2max/threshold-plus effort. The classic aerobic power workout for middle-distance and distance athletes.

**Olbrecht Category**: Aerobic Utilization

**When Used**: Utilization phase. Also appropriate during late capacity for sustained aerobic power development. For milers, this IS race-simulation work.

**Target Zones**: HR Zone 4-5, Pace Zone 4-5

| Duration | Structure | TSS Range |
|----------|-----------|-----------|
| Short | WU + 3x1600m @ Zone 4-5 / 400m jog (~4 min) + CD (~50 min total) | 60-80 |
| Medium | WU + 4x1600m @ Zone 4-5 / 400m jog (~4 min) + CD (~60 min total) | 75-95 |
| Long | WU + 5x1600m @ Zone 4-5 / 400m jog (~4 min) + CD (~70 min total) | 85-110 |

**Progressive Adjustment**: Add reps (3→4→5). For milers approaching race, shift pace from I pace toward race pace. For 3K athletes, maintain I pace throughout.

**Recovery**: 400m jog (~3.5-4 min). Slightly longer than 1200m repeats because the rep duration is longer and the athlete needs more time to partially clear lactate.

**Distinction from VO2max intervals**: `vo2max_intervals` uses time-based intervals (1-3 min) at a higher intensity. `track_1600m` uses distance-based intervals at a slightly lower intensity (I pace vs faster VO2max effort) but accumulates more total quality volume. They target similar adaptations from different angles.

---

## 5. Track Mixed (Ladders/Cut-Downs)

### Rationale

Mixed-distance track sessions are the most important missing workout category for mile/3K athletes. Every major middle-distance coaching tradition — Daniels, Canova, Michigan (Warhurst), Oregon (Bowerman/Wetmore), Peter Coe — uses some form of mixed-distance session. These workouts:

1. **Train multiple energy systems in one session**: Short reps (200-400m) hit the glycolytic system; long reps (800-1200m) hit VO2max/aerobic power. This IS Olbrecht's "high and low" applied to track-specific paces.

2. **Simulate race demands**: Real races aren't run at constant pace. Surges, tactical moves, and kicks require changing pace. Mixed sessions train this.

3. **Provide psychological variety**: Instead of 8x800m (monotonous), a 200-400-800-1200-800-400-200 pyramid breaks the effort into mentally manageable chunks while accumulating the same total volume.

4. **Bridge capacity and utilization**: The short reps serve as capacity-like stimuli (30-60s bursts) while the longer reps serve as utilization work (race-specific sustained effort). This dual purpose is why mixed sessions can be used from late capacity through the utilization phase.

### Classic Formats

#### A. Ladder (Ascending)
Distances increase rep-by-rep. Pace generally stays constant (at 3K-5K race effort) or slows slightly as distance increases.

**Examples**:
- 200-400-600-800-1000-1200 with 200m jog between reps
- 400-800-1200-1600 with 400m jog between reps

**Purpose**: Progressively longer sustained efforts teach the body to hold pace as fatigue accumulates. Mentally "easier" because each rep is the longest you'll do.

#### B. Descending Ladder (Cut-Down)
Distances decrease; pace INCREASES. The most iconic middle-distance format.

**Examples**:
- 1200-800-400-200 getting 2-3 sec/400m faster each step
- 1600-1200-800-400-200 starting at I pace, finishing at R pace
- McMillan's "Mile Cut-Down": 1600m at 5K pace → 1200m at 3K pace → 800m at mile pace → 400m at 800m pace → 200m all-out

**Purpose**: Develops the ability to ACCELERATE when fatigued. Simulates the closing stages of a race where you must find another gear. The shorter reps at faster pace become increasingly anaerobic while the athlete is already carrying fatigue from the longer reps.

#### C. Pyramid (Up and Down)
Ascend then descend through distances.

**Examples**:
- 200-400-800-1200-800-400-200 (the "classic pyramid")
- 400-800-1200-800-400 (shorter pyramid)
- 200-400-600-800-600-400-200 (smoother progression)

**Pace guidance**: Shorter reps at R/speed pace, longer reps at I/VO2max pace. The descending half is the most valuable — finding speed again after the longest rep simulates race finishing.

**Recovery**: Equal distance jog between reps (200m jog after 200m rep, 400m jog after longer reps) or timed recovery (90s-3 min).

#### D. Alternating Distance Sets
Pair a long rep with short rep(s) in a set, then repeat.

**Examples**:
- 3x(1200m at I pace + 2x200m at R pace), 2-3 min between reps, 5 min between sets (Daniels combo)
- 4x(800m at 3K pace + 400m at mile pace), 200m jog between reps, 3 min between sets
- 3x(1000m at I pace + 2x300m at R pace), 3 min between reps, 5 min between sets

**Purpose**: The long rep builds aerobic stress; the short reps maintain speed and recruit fast-twitch fibers. This is the most Olbrecht-aligned mixed format — it literally implements "sustained effort + short explosive bursts" within a structured track session.

#### E. Michigan (Warhurst)
The legendary workout mixing tempo segments with track intervals of decreasing distance:
- 1600m at 10K pace → 2K tempo → 1200m at 5K pace → 2K tempo → 800m at 3K pace → 2K tempo → 400m all-out

**Purpose**: Builds race toughness across the full aerobic-anaerobic spectrum. The tempo segments between track reps prevent full recovery, creating progressive fatigue.

#### F. Oregon Classic
400-600-400-300 at 1500m race pace with equal-distance jog recovery. Total volume near race distance.

**Purpose**: Race-simulation at goal pace with the volume distributed across distances that target different aspects of the race (start, middle, kick).

#### G. Canova Change-of-Rhythm
6x500m alternating 100m at 103% race pace and 100m at 94% race pace, with 6 min rest between reps.

**Purpose**: Teaches the athlete to handle pace changes within a single rep. Each 500m contains both faster-than-race and slower-than-race segments, training lactate production AND clearance simultaneously. This is a sophisticated utilization workout.

### Proposed Template

**Key**: `track_mixed`

**Description**: Mixed-distance track intervals combining different rep lengths in a single session. Formats include pyramids, cut-downs, ladders, and alternating-distance sets. Targets multiple energy systems simultaneously — short reps build anaerobic capacity/speed, long reps build aerobic power/utilization.

**Olbrecht Category**: Mixed Capacity + Utilization (unique — the only workout type that explicitly serves both)

**When Used**: Late capacity phase through utilization phase. Transitions well from capacity to utilization because the short reps maintain the "high" stimulus while the longer reps introduce race-specific pacing.

**Target Zones**: HR Zones 4-6 (varies by rep length), Pace Zones 4-6 (short reps faster, long reps at VO2max/I pace)

| Duration | Structure | TSS Range |
|----------|-----------|-----------|
| Short | WU + 200-400-800-400-200 @ descending pace (R→I→R), 200m jog between + CD (~40 min total) | 45-60 |
| Medium | WU + 400-800-1200-800-400 @ I pace (short reps faster), 400m jog between + CD (~50 min total) | 60-80 |
| Long | WU + 200-400-800-1200-800-400-200 full pyramid, 200-400m jog between + CD (~60 min total) | 70-95 |

**Alternative structures** (Claude can select based on athlete goals and race distance):

For **milers**:
- Cut-down: 1200-800-600-400-200 getting progressively faster (I→R pace)
- Combo: 3x(800m at I pace + 2x200m at R pace)
- Oregon: 400-600-400-300 at mile pace

For **3K athletes**:
- Pyramid: 400-800-1200-800-400 at I pace
- Cut-down: 1600-1200-800-400 at I→R pace progression
- Combo: 2x(1200m at I pace + 2x400m at R pace)

**Recovery within session**:
- Between reps: 200m jog (short reps) to 400m jog (long reps), ~90s-3 min
- Between sets (for set-based formats): 4-5 min jog
- Recovery is SHORTER than for uniform-distance repeats because the varying lengths provide built-in intensity modulation

**Progressive Adjustment**: Progress through formats: pyramid → cut-down → race-specific combos. Can also increase total volume (add reps to the pattern) or shift pace targets toward race pace.

**Why this is a single workout type** (vs separate types for ladder, pyramid, cut-down): The AI coach (Claude) selects the specific format based on:
- Race distance (mile vs 3K)
- Training phase (capacity → utilization)
- Athlete level (simpler pyramids for beginners, complex combos for advanced)
- Recent training history

The `structure` field in the plan output specifies the exact format. This avoids registry bloat while giving Claude maximum flexibility.

---

## 5b. Mile-Focused Short Track Workouts

### The Case for Mile-Specific Track Types

The current track workout types (200m, 400m, 800m) are described as generic "hard effort" or "near-max." But a miler's track session is fundamentally different:

1. **Paced, not all-out**: Reps are at GOAL MILE PACE, not maximum effort. The miler runs a controlled 67s 400m, not an all-out 60s 400m.
2. **Low total volume**: The race is only 1600m. Total quality volume in a session is often 1200-2400m — much less than 10x200m or 10x400m.
3. **Mixed distances within a session**: Milers routinely combine different rep lengths (e.g., 3x400 + 2x200 + 1x800) to hit different aspects of the race.
4. **Generous recovery**: Work:rest ratio of 1:2 to 1:3 because the goal is PACE QUALITY, not fatigue accumulation.

### Catalog of Mile-Focused Track Sessions

These are organized by primary intent. All paces reference mile race pace (Daniels' R pace) unless noted.

#### A. Uniform Short Reps at Mile Pace

| Workout | Pace | Recovery | Total Work | Purpose |
|---------|------|----------|------------|---------|
| 6x300m | Mile pace | 300m jog (~2 min) | 1800m | Speed endurance foundation; comfort at race pace |
| 8x300m | Mile pace | 200m jog (~90s) | 2400m | Higher volume speed endurance |
| 4x400m | Mile pace | 400m jog (~3 min) | 1600m | "Broken mile" — total volume = race distance |
| 6x400m | Mile pace | 400m jog (~3 min) | 2400m | Extended broken mile — builds race-pace stamina |
| 3x500m | Mile pace | 3-4 min rest | 1500m | Longer speed endurance; glycolytic stress |
| 3x600m | Mile pace | 4-5 min rest | 1800m | Upper limit of mile-pace speed endurance |
| 2x800m | Mile pace | 5-6 min rest | 1600m | Broken mile race simulation; closest to race feel |

**Key**: Total work volume for a miler's speed session is typically 1200-2400m (75-150% of race distance). More than that and quality drops.

#### B. Mixed-Distance Combos at Mile Pace

These are the bread-and-butter of mile training. They combine different distances to target different race segments (start, middle, kick) within one session.

| Workout | Structure | Recovery | Total Work | Purpose |
|---------|-----------|----------|------------|---------|
| 3x400 + 2x200 | 3x400m @ mile pace, then 2x200m slightly faster | 400m jog between 400s, 200m jog between 200s | 1600m | Race distance as a combo; 200s sharpen finishing speed |
| 800 + 2x400 + 2x200 | 800m @ mile pace → 2x400m @ mile pace → 2x200m faster | 4 min / 3 min / 200m jog | 2000m | Full spectrum — sustained + repeated + speed |
| 2x600 + 2x300 | 2x600m @ mile pace → 2x300m at mile pace or faster | 4 min / 2 min | 1800m | Medium-length + short speed endurance |
| 600 + 400 + 300 + 200 | Descending cut-down, each rep slightly faster | 3 min / 2 min / 90s | 1500m | Near race distance; practices accelerating when tired |
| 2x(400 + 200) | 2 sets of (400m mile pace + 200m fast), set rest 4-5 min | 200m jog between reps | 1200m | Set-based; short and sharp |
| 3x(400 + 200) | 3 sets of (400m mile pace + 200m fast), set rest 4-5 min | 200m jog between reps | 1800m | Higher volume version |
| 500 + 400 + 300 + 200 | Cut-down, getting faster each rep | 3 min / 2.5 min / 2 min | 1400m | Progressive speed build |
| 800 + 600 + 400 + 200 | Full cut-down from VO2max to speed | 4 min / 3 min / 2 min | 2000m | Classic miler's descending ladder |
| 2x(300 + 300 + 200) | 2 sets of (300, 300, 200) all at mile pace or faster | 200m jog between reps, 5 min between sets | 1600m | High rep frequency, set-based |
| 400 + 600 + 400 + 200 | Oregon-style at mile pace | 3 min / 3 min / 2 min | 1600m | Race distance with varying rep lengths |

#### C. Mile Pace with Built-in Warm-Up Set

The user's concept of including a warm-up rep (e.g., 800m at moderate effort before the main set). This is practical because milers often need a "blowout" rep to find their gear before the quality reps.

| Workout | Structure | Recovery | Total Work | Purpose |
|---------|-----------|----------|------------|---------|
| 800 WU set + 3x400 + 2x200 | 800m @ 3K pace (warm-up), then 3x400m @ mile pace + 2x200m fast | 3 min after 800, then 400m/200m jog | 2400m | Warm-up rep primes the system; main set at race pace |
| 800 WU set + 4x400 | 800m @ 3K pace, then 4x400m @ mile pace | 3 min after 800, 3 min between 400s | 2400m | "Broken 3K start + broken mile" |
| 1200 WU set + 3x300 + 200 | 1200m @ I pace, then 3x300m @ mile pace + 200m fast | 4 min after 1200, 2 min between reps | 2300m | Aerobic prime → speed endurance |
| 600 WU set + 2x500 + 2x200 | 600m @ 3K pace, then 2x500m @ mile pace + 2x200m sprint | 3 min after 600, 3 min / 200m jog | 2000m | Progressive intensity build |

#### D. Race Simulation / "Broken Mile" Formats

These simulate the race by totaling approximately 1600m of work at goal pace, but breaking it into manageable pieces with brief recovery.

| Workout | Structure | Recovery | Notes |
|---------|-----------|----------|-------|
| 4x400 | 4x400m @ goal mile pace | 60-90s jog (incomplete recovery) | The classic Daniels broken mile; short rest forces race-like lactate |
| 2x800 | 2x800m @ goal mile pace | 3-4 min rest | Two halves of a mile; tests race-pace stamina |
| 800 + 2x400 | 800m at goal pace → 2x400m at goal pace | 3 min / 60-90s | Simulates holding pace after a strong first half |
| 600 + 600 + 400 | 2x600m + 400m at goal pace | 90s / 60s | Near-race conditions; minimal recovery |
| 1200 + 400 | 1200m @ goal pace, 400m @ goal pace or faster | 2-3 min | "3/4 race + kick" simulation |
| 500 + 500 + 600 | Getting slightly longer while holding pace | 90s / 90s | Teaches patience early, strength late |

**Recovery key**: For race simulations, recovery is SHORT (60-120s) — deliberately incomplete. This is what distinguishes race simulation from speed endurance work (where recovery is full). The short rest creates race-like lactate accumulation.

### How These Map to Olbrecht

All of these mile-focused sessions serve **anaerobic utilization** — training the athlete to USE their anaerobic capacity at a controlled, sustained pace. In Olbrecht's framework:

- **Capacity phase**: None of these belong here. During capacity, milers use `mixed_energy` (30s bursts within easy runs) and `anaerobic_flat`/`hill_sprints` for short explosive work.
- **Late capacity / transition**: The uniform short reps (6x300m, 4x400m) can be introduced as "race-pace familiarization" — low volume, full recovery.
- **Utilization phase**: All formats belong here. This is where the miler learns to race.
- **Final sharpening**: Race simulations (broken miles with short rest) are the last 2 weeks before the goal race.

The short reps (200-400m at 30-60s duration) are interesting from Olbrecht's perspective because they sit right at the boundary of his "high" stimulus (~30-35s). When done at controlled race pace with full recovery, they function more as neuromuscular patterning than metabolic stress — teaching the body the exact motor patterns and pacing it needs for the race. This is distinct from "high and low" capacity work where the bursts are at uncontrolled maximal effort.

### Proposed Template Structure

Rather than creating a separate workout type for each distance/combo, I recommend **two new types** that cover all mile-focused track work:

**1. `track_mile_pace`** — Uniform-distance reps at mile/R pace with generous recovery:

| Duration | Structure | TSS Range |
|----------|-----------|-----------|
| Short | WU + 6x300m @ R/mile pace / 300m jog + CD (~35 min) | 35-50 |
| Medium | WU + 4x400m @ R/mile pace / 400m jog + CD (~40 min) | 40-55 |
| Long | WU + 2x800m @ mile pace / 5 min rest + CD (~45 min) | 45-65 |

**2. `track_race_combo`** — Mixed-distance combos at race pace, potentially with a warm-up set:

| Duration | Structure | TSS Range |
|----------|-----------|-----------|
| Short | WU + 3x400 + 2x200 @ mile pace, 400m/200m jog + CD (~35 min) | 35-50 |
| Medium | WU + 800m @ 3K pace + 3x400 + 2x200 @ mile pace + CD (~45 min) | 45-65 |
| Long | WU + 800-600-400-200 cut-down @ mile→faster + CD (~40 min) | 50-70 |

Both types are intensity sessions. Claude selects the specific format and rep scheme based on:
- Where in the utilization phase (early → uniform reps, late → race simulations)
- Athlete level (fewer reps for beginners)
- Target race (mile vs 3K affects distance selection)
- Recent training load

---

## 6. Speed Endurance (300-600m)

### Rationale

The 300-600m range sits in a critical physiological gap:
- **200m and shorter**: Pure speed, ATP-PCr dominant, <30s effort. Already covered by `track_200m` and `anaerobic_flat`.
- **800m and longer**: VO2max dominant, 2+ min effort. Already covered by `track_800m` and `vo2max_intervals`.
- **300-600m**: The "speed endurance" zone. 40-90 seconds of effort. Glycolytic system is primary, but VO2max contribution is growing. Lactate rises rapidly. This is the zone that separates a miler from a 5K runner.

For a miler, 300m reps AT race pace take ~45-55s. For a 3K runner, 600m reps at race pace take ~90-120s. These durations are where the anaerobic system is maximally stressed while the aerobic system is also heavily engaged — the exact metabolic state they'll experience during their race.

### Coaching Consensus

**Daniels**: 300m at R (Repetition) pace, 6-10 reps, 200-300m jog recovery. R pace ≈ current mile race pace per 400m. Work:rest ratio 1:2 to 1:3. 600m at I pace, 6-8 reps, 2-2.5 min jog. These are "short VO2max intervals" useful for milers who find 1000s/1200s too grinding.

**Canova**: 4x2x400 with 30 sec between reps and 5 min between sets, each 400m faster than 1500m pace. Also: 600m at slightly faster than 1500m pace, rest 30-45s, then sprint all-out 200m, 4-5 min rest between sets.

**Magness**: Sprint progression — hill sprints → flat sprints (60-100m) → speed endurance (150-300m). Speed endurance reps are 95-105% of race pace with generous recovery (work:rest 1:2-1:3). Magness emphasizes that these should be run RELAXED, not straining.

**Australian/NZ tradition**: 3x600m is a classic miler's workout. 600m reps are long enough to build lactate substantially but short enough for quality pace. Also: 1000m + 600m + 400m with 5 min / 3 min recovery — average pace right around 1500m race pace.

### Proposed Template

**Key**: `speed_endurance`

**Description**: Track repeats in the 300-600m range at mile/3K race pace or faster. Develops the ability to sustain near-maximal speed while lactate accumulates. The critical "speed endurance" zone between pure speed work (200m) and sustained VO2max intervals (800m+).

**Olbrecht Category**: Anaerobic Utilization (primary), Anaerobic Capacity (secondary for shorter reps with full recovery)

**When Used**: Utilization phase, 3-6 weeks before target race. For milers, this IS the primary race-specific workout alongside broken-race formats. For 3K athletes, this develops the finishing speed and surge capacity needed for the final 600-800m.

**Target Zones**: HR Zones 5-6, Pace Zones 5-6

| Duration | Structure | TSS Range |
|----------|-----------|-----------|
| Short | WU + 6x300m at R/mile pace / 300m jog (~2 min) + CD (~35 min total) | 40-55 |
| Medium | WU + 5x500m at mile-3K pace / 400m jog (~3 min) + CD (~45 min total) | 50-65 |
| Long | WU + 4x600m at 3K pace / 400m jog (~3 min) + CD (~50 min total) | 55-75 |

**Alternative structures**:

For **milers**:
- 8x300m at mile pace / 300m jog (pure speed endurance)
- 3x(2x400m at mile pace / 200m jog), 4 min between sets (set-based)
- 3x600m at mile pace / 4 min rest (longer speed endurance)
- 600m fast + 200m sprint, 5 min rest, 3 sets (Canova-style)

For **3K athletes**:
- 6x500m at 3K pace / 300m jog (3K-specific speed endurance)
- 4x600m at 3K pace / 400m jog (slightly longer)
- 1000m + 600m + 400m at 3K→mile pace / 5 min + 3 min rest (cut-down)

**Recovery**:
- 300m reps: 200-300m jog (work:rest ~1:2-1:3) — FULL recovery to maintain speed quality
- 400-500m reps: 300-400m jog (work:rest ~1:1.5-1:2)
- 600m reps: 400m jog or 3-4 min (work:rest ~1:1.5-1:2)
- Recovery is LONGER relative to rep time than for 800m+ intervals because the purpose is speed QUALITY, not lactate accumulation

**Progressive Adjustment**:
- Add reps (6→8 for 300m, 3→4→5 for 600m)
- Extend rep distance (300→400→500m)
- Reduce recovery ratio slightly (1:3 → 1:2)
- Do NOT increase pace — pace is already at or near race pace

**Distinction from existing types**:
- vs `track_200m`: 200m is pure speed (ATP-PCr), <30s. Speed endurance is 40-90s with significant glycolytic stress.
- vs `track_400m`: Current 400m template is at "hard effort" (Zone 5-6). Speed endurance 400s are at MILE RACE PACE specifically — more structured pacing, potentially with set-based format.
- vs `track_800m`: 800m is VO2max-dominant (2+ min). Speed endurance is glycolytic-dominant (<90s).
- vs `anaerobic_flat`: Anaerobic flat uses 20-45s all-out efforts. Speed endurance uses 40-90s at controlled race pace — not all-out, but sustained near-max.

---

## 7. Coaching References

### Jack Daniels (VDOT/Running Formula)

**Training pace taxonomy for mile/3K athletes:**

| Pace | Definition | Typical Distances | Recovery | Purpose |
|------|-----------|-------------------|----------|---------|
| R (Repetition) | ~Current mile race pace per 400m | 200m, 300m, 400m | Full (1:2-1:3 work:rest) | Speed, economy, anaerobic power |
| I (Interval) | ~3K-5K race pace | 600m, 800m, 1000m, 1200m | 1:1 work:rest | VO2max development |
| T (Threshold) | ~15K-half marathon pace | 1000m+, continuous 15-25 min | 60-90s jog | Lactate threshold |

**VDOT pace relationships:**

| VDOT | Mile Time | 3K Time | R Pace (/400m) | I Pace (/km) |
|------|-----------|---------|----------------|--------------|
| 50 | 5:14 | 11:10 | 82s | 4:08 |
| 55 | 4:49 | 10:16 | 76s | 3:49 |
| 60 | 4:27 | 9:30 | 70s | 3:33 |
| 65 | 4:08 | 8:50 | 65s | 3:19 |

**Key insight**: R pace ≈ current mile race pace. I pace ≈ current 3K-5K race pace. For a miler, R pace IS the race and I pace builds the aerobic engine. For a 3K runner, I pace approaches race pace and R pace builds speed reserve.

**Daniels' mixed sessions (Q sessions)**:
- I+R Combo: 3x1000m at I pace (3 min jog) + 4x200m at R pace (200m jog)
- Race simulation: 2x800m at goal mile pace with 5 min rest + 4x200m at R pace
- Broken race: 4x400m at goal mile pace with 1 min rest
- Sandwich: 2x(1200 at I + 4x200 at R), 5 min between sets
- Ladder: 200-400-600-800-600-400-200, short reps at R, long reps at I

**Mile/3K weekly structure (Phase III, peak training):**
- 2-3 quality days: 1-2 R-pace sessions, 1-2 I-pace sessions
- R-pace work emphasized MORE than for 5K+ athletes
- T-pace (threshold) work DE-emphasized compared to 5K+ athletes
- Long run shorter proportionally (22-25% of weekly volume vs 25-30% for distance)

### Renato Canova

**Key concepts for middle distance:**

1. **Extension principle**: Every event is a matter of extending pace. Training progresses from short reps at race pace → longer reps at race pace → near-race-distance at race pace.

2. **Special period workouts for 1500m**:
   - 8x400m at 1500m race pace with 2 min recovery (conservative but effective)
   - 6x500m alternating 100m at 103% race pace and 100m at 94% race pace, 6 min rest
   - Fundamental period uses longer intervals at slower pace: 8x400m in 62s with 2 min recovery (for a 1:44 800m runner — deliberately slow aerobic endurance work)

3. **Change of rhythm workouts**: Alternating pace within a single rep trains lactate dynamics — producing lactate at faster pace, clearing it at slower pace. Critical for the mile/3K where tactical pace changes are constant.

4. **Descending mixed sessions**: 2km + 1600m + 1200m + 800m + (4x400m) with 200m jog recovery between all reps, starting at 10K pace and increasing to 5K pace or faster.

5. **Recovery philosophy**: For race-pace sessions, recoveries are GENEROUS because pace quality is paramount. Between workout days, typically 2-3 days of easy regeneration running.

### Steve Magness

**Key concepts:**

1. **Sprint progression for middle distance**: Hill sprints → flat sprints (60-100m) → speed endurance (150-300m). Teaching RELAXED sprinting — distance runners tend to "bear down and force it."

2. **Interval manipulation**: Rather than fixed "do 4x4 min to develop VO2max," Magness manipulates length, intensity, and rest to target specific adaptations. The same workout structure can serve different purposes by adjusting these variables.

3. **VO2max skepticism**: Stop trying to improve VO2max — it levels off much earlier than performance. Focus on speed, economy, and lactate dynamics instead. This aligns with Olbrecht's utilization concept.

---

## 8. Implementation Recommendations

### Priority Order

1. **`track_mixed`** — HIGHEST PRIORITY. This is the biggest gap. No existing workout type supports mixed-distance track sessions. Every competitive mile/3K program uses them extensively. This is also the most Olbrecht-aligned of the new types because it inherently blends capacity (short reps) and utilization (long reps) stimuli. Covers pyramids, ladders, cut-downs, and longer alternating-distance formats (e.g., 400-800-1200-800-400).

2. **`track_race_combo`** — HIGHEST PRIORITY. Short mixed-distance combos at race pace, often with a warm-up set. Examples: 3x400+2x200, 800+3x400+2x200, 600+400+300+200 cut-down. These are the bread-and-butter of mile/3K race preparation. Distinct from `track_mixed` because total volume is lower (1200-2400m), all reps are at race pace or faster, and the focus is race simulation rather than multi-system development.

3. **`track_mile_pace`** — HIGH PRIORITY. Uniform-distance reps at mile/R pace with generous recovery. Examples: 6x300m, 4x400m, 3x500m, 2x800m. The missing "speed endurance" zone between pure speed (track_200m) and VO2max work (track_800m). Recovery is full (1:2-1:3 work:rest) — these are about pace quality, not fatigue accumulation.

4. **`track_1200m`** — MODERATE PRIORITY. 1200m repeats at VO2max/3K-race effort. The bread-and-butter collegiate middle-distance workout. 40% of 3K race distance. Daniels considers 1200m the upper limit for I-pace reps.

5. **`track_1600m`** — MODERATE PRIORITY. Mile repeats at I pace / threshold-plus. Classic aerobic power workout. For milers, this IS race simulation. 3-5 x 1600m with 400m jog recovery.

6. **`speed_endurance`** — CONSIDER MERGING with `track_mile_pace`. The original `speed_endurance` proposal (Section 6) covers the same 300-600m range. The difference was in framing — `speed_endurance` was described for both mile and 3K athletes, while `track_mile_pace` is explicitly mile-focused. **Recommendation**: Merge into a single type called `track_mile_pace` that covers uniform reps at R/race pace from 300m to 800m. The AI coach adjusts rep distance based on target race (300-400m for milers, 500-600m for 3K athletes).

### Existing Types That Need Updates

**`race_specific`**: Currently uses generic 5-12 min reps at "race pace." For mile/3K athletes, this should support much shorter race-pace intervals:
- Mile: 4x400m at goal pace / 2 min rest
- 3K: 3x1000m at goal pace / 3 min rest
- 3K: 6x600m at goal pace / 2 min rest

Consider either updating `race_specific` to support these shorter formats or ensuring the new `speed_endurance` and `track_mixed` types cover this need.

**`track_400m`**: Currently described as "hard effort" at Zone 5-6. For mile athletes, 400m at mile pace is Zone 5 (not max effort). The current template works but the description could note that milers may use 400m reps at controlled race pace rather than near-max.

### How Claude Should Select Formats

For `track_mixed`, the AI coach needs guidance on format selection:

| Race Target | Training Phase | Recommended Format |
|---|---|---|
| Mile | Late capacity | Pyramid: 200-400-800-400-200 |
| Mile | Early utilization | Cut-down: 1200-800-400-200 at progressive pace |
| Mile | Late utilization | Combo: 3x(800m I + 2x200m R) or Oregon: 400-600-400-300 |
| 3K | Late capacity | Ascending ladder: 400-800-1200 at I pace |
| 3K | Early utilization | Pyramid: 400-800-1200-800-400 |
| 3K | Late utilization | Cut-down: 1600-1200-800-400 at I→R pace |

### `is_intensity()` Classification

All five new types are intensity sessions:
- `track_1200m`: Yes — VO2max-level effort
- `track_1600m`: Yes — VO2max/threshold-plus effort
- `track_mixed`: Yes — includes reps at Zone 5-6
- `track_mile_pace`: Yes — at race pace (Zone 5-6)
- `track_race_combo`: Yes — at race pace or faster (Zone 5-6)

### Workout Selection Matrix (Athlete Limitation)

| Workout Type | Aerobically Limited | Anaerobically Limited | Balanced |
|---|---|---|---|
| `track_1200m` | AVOID (capacity phase) | Standard | Util phase |
| `track_1600m` | AVOID (capacity phase) | Standard | Util phase |
| `track_mixed` | Late capacity OK (short reps = capacity) | HIGH priority | Util phase |
| `track_mile_pace` | AVOID | HIGH priority | Util phase |
| `track_race_combo` | AVOID | HIGH priority | Late util phase |

**Note**: For anaerobically limited athletes targeting mile/3K, `track_mile_pace` and `track_race_combo` are HIGH priority because these athletes need glycolytic development and race-pace neuromuscular patterning. The short reps (200-400m at race pace) serve as anaerobic utilization builders.

### How Claude Selects Format Within Each Type

**`track_mile_pace`**: Claude picks the rep distance based on training phase and target:
- Early utilization: 6x300m (shorter, more manageable)
- Mid utilization: 4x400m or 3x500m (building)
- Late utilization: 2x800m or 3x600m (race-simulation intensity)
- Mile target: 300-400m reps preferred (match race-pace motor patterns)
- 3K target: 500-600m reps preferred (match 3K race demands)

**`track_race_combo`**: Claude selects combo structure based on:
- Early utilization: Simple combos (3x400+2x200)
- Mid utilization: With warm-up set (800+3x400+2x200)
- Late utilization / sharpening: Race simulations with short rest (4x400 / 60s rest)
- Cut-downs when building toward race peak (800-600-400-200 getting faster)

---

## 9. Sources

### Jack Daniels / VDOT

- [Jack Daniels' Running Formula](https://www.amazon.com/Daniels-Running-Formula-Jack/dp/1450431836) — Canonical text on training paces, R/I/T system
- [VDOT O2 Running Calculator](https://vdoto2.com/calculator) — Official VDOT pace calculator
- [VDOT Training Definitions](https://vdoto2.com/learn-more/training-definitions) — R, I, T, E pace definitions

### Renato Canova

- [Something New in Training: The Methods of Renato Canova (PDF)](https://runningscience.co.za/wp-content/uploads/2017/01/The-Methods-of-Renato-Canova.pdf) — Full methodology overview
- [Preparing for Championship Races with Renato Canova — Running Writings](https://runningwritings.com/2011/09/peaking-with-renato-canova_28.html) — 1500m, 5K, 10K peaking strategies
- [Canova-style "Full-Spectrum" Percentage-Based Training — Running Writings](https://runningwritings.com/2023/12/percentage-based-training.html) — Comprehensive overview of Canova's system
- [Canova Mile/1500m Training — LetsRun Forum](https://www.letsrun.com/forum/flat_read.php?thread=3676972) — Community discussion of Canova's 1500m methods
- [5 Speed Workouts for 5km/10km Runners (Canova Style) — SweatElite](https://www.sweatelite.co/5-speed-workouts-for-5km-10km-runners/)

### Steve Magness

- [Sprint Training Part 2 — Science of Running](https://www.scienceofrunning.com/2009/05/sprint-training-part-2.html) — Sprint-to-speed-endurance progression
- [The Science of Running (Book)](https://www.amazon.com/Science-Running-limit-maximize-performance/dp/0615942946) — VO2max critique, interval manipulation
- [Magness Speaks: Interval Training, Why It's Misunderstood — High Performance West](https://www.highperformancewest.com/blog/2018/6/22/magness-speaks-interval-training-why-its-misunderstood)

### Mixed-Distance / Pyramid / Ladder Workouts

- [The Mile Cut-Down — McMillan Running](https://www.mcmillanrunning.com/the-mile-cut-down/) — Descending ladder for milers
- [Middle Distance Training Guide — McMillan Running](https://www.mcmillanrunning.com/middle-distance-training-guide/)
- [How to Run the Michigan — InsideHook](https://www.insidehook.com/wellness/michigan-track-workout) — Ron Warhurst's legendary workout
- [Try This Fun Inverted Pyramid Workout to Build Miler Speed — Outside Online](https://www.outsideonline.com/health/running/training-advice/workouts/try-this-fun-inverted-pyramid-workout-to-build-miler-speed/)
- [Pyramid Running Sessions — Momentum Sports](https://www.momentumsports.co.uk/TtPyramid.asp)
- [What Is the Logic Behind Ladder Workouts? — LetsRun Forum](https://www.letsrun.com/forum/flat_read.php?thread=4240694)

### Speed Endurance

- [Multi-Pace Training Speeds for 800m and 1500m — SpeedEndurance.com](https://speedendurance.com/2015/08/04/multi-pace-training-speeds-for-800m-and-1500m/)
- [Boost Your Kick: Finishing Speed Workouts for 800m-1500m — Track and Field Forever](https://www.trackandfieldforever.com/news/boost-your-kick)
- [Speed Endurance Workout Examples 1500m/mile — LetsRun Forum](https://www.letsrun.com/forum/flat_read.php?thread=6546028)
- [1500m — 5 Race Indication Workouts — SweatElite](https://www.sweatelite.co/1500m-5-race-indication-workouts/)
- [Struggling With Your Mile or 1500? Two Key Workouts — Working Class Runner](https://misterstevengomez.com/2021/05/26/struggling-with-your-mile-or-1500-two-key-workouts/)

### Middle Distance Training (General)

- [Crossing the Golden Training Divide: Science and Practice of Training 800m and 1500m Runners — PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC8363530/)
- [Joe Rubio's Fundamental Training Principles for the Competitive 1500m Runner (PDF)](https://img.runningwarehouse.com/pdf/middle_distance_guide.pdf)
- [4 Key Middle Distance Workouts from Aussie Elites — Runner's Tribe](https://runnerstribe.com/features/4-key-middle-distance-workouts-aussie-elites/)
- [Middle-Distance Intervals — Champions Everywhere](https://www.championseverywhere.com/middle-distance-intervals/)
- [Planning Ideas for 800m and 1500m Athletes — Steve Bennett / OzTrack](https://medium.com/oztrackathletics/planning-ideas-for-800m-and-1500m-athletes-f05089194ada)
- [Getting the Interval Workout Recovery Right — Running Writings](https://runningwritings.com/2024/05/getting-interval-workout-recovery-right.html)

### Recovery Intervals

- [How Much Recovery Should You Take Between Speed Intervals? — Outside Online](https://run.outsideonline.com/training/getting-started/how-much-recovery-should-you-take-between-speed-intervals/)
- [Mastering Workout Paces, Rests, and Recoveries — Run Baldwin](https://www.runbaldwin.com/intervals-rests-recoveries/)
