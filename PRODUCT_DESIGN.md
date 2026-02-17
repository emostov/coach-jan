# CoachJan — AI Running Coach
## Product Design Document

See [Future Features Roadmap](docs/FUTURE_FEATURES.md) for post-launch feature ideas.

---

## 1. Vision

An AI running coach application that embodies Jan Olbrecht's training philosophy. The coach analyzes athlete physiology, creates periodized training plans based on capacity vs. utilization principles, ingests workout data from GPS watches (Coros API integration + FIT file upload from any device), and provides ongoing feedback — adjusting the plan based on actual training response.

**Core principle**: Training should develop aerobic and anaerobic capacity to *optimum*, not maximum, levels. The AI coach "steers" the training process through a continuous cycle of **Train → Measure → Adjust**.

---

## 2. User Personas

### Athlete (Primary User)
- Recreational to competitive runner
- Owns a GPS running watch (Coros, Garmin, Wahoo, Suunto, Polar, etc.)
- Wants structured, science-backed training
- Syncs workouts via Coros API (primary) or manual FIT file upload (any watch)
- Interacts with the coach through a chat-like interface

### Coach Persona (AI — "Jan")
- Embodies Jan Olbrecht's training methodology
- Speaks with authority on periodization, energy systems, and training load
- Provides workout analysis, plan adjustments, and physiological insights
- Communicates directly — explains the "why" behind every prescription

---

## 3. Core Features

### 3.1 Athlete Profile Setup

The athlete provides baseline information that drives all calculations and plan generation.

**Required inputs:**
| Field | Purpose |
|-------|---------|
| Name | Personalization |
| Age | Zone calculation, recovery modeling |
| Weight (kg) | Load calculations |
| Resting Heart Rate | HR zone anchoring, TRIMP calculation |
| Max Heart Rate | HR zone ceiling |
| Lactate Threshold HR | Zone boundary, intensity factor |
| Current weekly volume (km) | Starting point for plan generation |
| Goal race | Target event (distance + date) |
| Running experience level | Mesocycle structure (beginner/intermediate/advanced) |
| Sports background | Prior/concurrent sports, injury history, movement patterns — informs strength programming and training capacity |
| Recent race results (optional) | Estimate functional threshold pace |

**Derived on setup:**
| Metric | How |
|--------|-----|
| Functional Threshold Pace (FTPace) | From recent race result or time trial, or manual entry |
| Heart Rate Zones | Calculated from LTHR (see §4.1) |
| Pace Zones | Calculated from FTPace (see §4.2) |

### 3.2 Training Plan Generation

The coach creates a periodized training plan based on Olbrecht's framework.

**Plan structure:**

```
Macrocycle (full preparation → race)
  └── Mesocycle 1 (capacity focus)
        └── Training Block (load phase)
        └── Recovery Block (super-compensation)
  └── Mesocycle 2 (capacity focus)
        └── ...
  └── Mesocycle N (competition/utilization focus)
        └── ...
  └── Race Week (taper)
```

**Mesocycle duration by athlete level:**
| Level | Load Phase | Recovery Phase |
|-------|-----------|---------------|
| Beginner | 1 week | 1 week |
| Intermediate | 2 weeks | 1 week |
| Advanced | 3 weeks | 2 weeks |

**Workout types prescribed:**

| Workout Type | Olbrecht Category | Description | Typical Structure |
|-------------|-------------------|-------------|-------------------|
| Easy Run | Aerobic Capacity | Low intensity, conversational | Continuous, Zone 1-2 HR |
| Long Run | Aerobic Capacity | Extended duration at easy pace | Continuous, Zone 1-2 HR |
| Aerobic Development | Aerobic Capacity | "High and Low" — mostly easy with short fast bursts | 3/5 easy + 2/5 as short fast strides/surges |
| Tempo Run | Aerobic Utilization | Sustained threshold effort | Continuous or cruise intervals at Zone 3-4 HR |
| VO2max Intervals | Aerobic Utilization | Hard intervals with recovery | 3-5min intervals at Zone 5, jog recovery |
| Speed/Sprint Work | Anaerobic Capacity | Short, explosive repetitions | 10-30s all-out efforts with full recovery |
| Race-Specific | Anaerobic Utilization | Goal-pace work | Intervals at goal race pace |
| Recovery Run | Recovery | Very easy movement | Zone 1 HR, short duration |
| Strength / Mobility | Injury Prevention & Performance | Exercises from *Running Rewired* (Jay Dicharry) | Programmed based on athlete's sports background, movement patterns, and training phase |

**Strength workout programming (Running Rewired):**

Strength and mobility sessions are prescribed alongside running workouts, drawn from Jay Dicharry's *Running Rewired* framework. The system is built on Dicharry's core thesis: runners need four essential movement skills — pelvic stability, counter-rotation of the trunk, hip extension propulsion, and spring mechanics (elastic energy storage/return).

#### Self-Assessment Protocol

During onboarding, the athlete completes a movement self-assessment based on Running Rewired's screen-and-correct model. Results, combined with the athlete's sports background and injury history, determine which exercises to prioritize.

| Assessment | What It Tests | Method |
|-----------|--------------|--------|
| Hip Flexor Length | Tight hip flexors limiting stride | Modified Thomas Test (kneeling in doorjamb) |
| Ankle Dorsiflexion | Ankle mobility affecting foot strike | Knee-to-wall test (~3.6° per cm) |
| Single-Leg Squat | Pelvic stability, knee tracking | 6 reps, observe for hip drop/knee valgus/trunk lean |
| Single-Leg Balance | Proprioceptive control | Single-leg stance, proactive vs. reactive control |
| Posture / Spinal Alignment | Standing posture, pelvic tilt | Natural curve and alignment check |
| Hip Rotation | Internal/external rotation mobility | Prone or seated rotation |
| Trunk Rotation | Thoracic rotation capacity | Seated rotation assessment |
| Big Toe Extension | Big toe function (needs 50-90°) | Toe yoga (isolate big toe from little toes) |
| Foot Intrinsics | Arch control | Short foot exercise |
| Core Stability | Anti-extension/anti-rotation control | Dynamic stability tests |
| Alignment Under Load | How posture holds during movement | Observed during functional movements |

**Assessment-to-exercise mapping**: Failed assessments direct the athlete to targeted corrective exercises before progressing to the full workout program. The athlete's sports background further refines selection (e.g., a former soccer player may need different hip stability work than a former swimmer; a former cyclist may have tight hip flexors and weak foot intrinsics).

#### Exercise Taxonomy

Exercises are organized along three dimensions:

**Tier 1 — Movement Domain** (maps to Running Rewired Chapters 5-9):
1. Pelvic Stability ("Pivot Point")
2. Trunk Rotation ("Counter-Rotation")
3. Hip Extension / Propulsion
4. Alignment / Posture
5. Spring Mechanics / Elastic Recoil

**Tier 2 — Training Modality:**

| Modality | Purpose | Duration | Equipment | When |
|----------|---------|----------|-----------|------|
| Precision (Workouts 1-6) | Neuromuscular coordination, proprioception, motor pattern correction | 15-20 min | Mat, band, sling trainer, Swiss ball | Before runs as warm-up, or standalone |
| Performance Strength (Workouts 7-10) | Force production, muscle recruitment | 20-30 min | Dumbbells, kettlebells, barbells | On hard training days |
| Performance Power (Workouts 11-15) | Rate of force development, elastic energy, plyometrics | 20-30 min | Loaded implements + bodyweight explosive | On hard training days |

**Tier 3 — Body Region:**
- Foot / Ankle (intrinsics, dorsiflexion, big toe, calf)
- Hip / Glute (activation, abduction, extension, rotation)
- Core / Trunk (anti-rotation, anti-extension, rotation control)
- Posterior Chain (hamstrings, glute-ham complex)
- Full Body / Integrated (carries, loaded movements, compound lifts)

#### Key Exercises

| Exercise | Modality | Target |
|----------|----------|--------|
| Banded Hip Jacks | Precision | Hip abduction, glute med activation |
| Banded Hip Drag | Precision | Hip extension, glute activation |
| Banded Hip Twist | Precision | Rotational hip control |
| Frog Bridge | Precision | Glute activation in external rotation |
| Pigeon Hip Extension | Precision | Hip extension from stretched position |
| Rotisserie Chicken | Precision | Trunk rotation control |
| Toe Yoga | Precision | Big toe isolation, intrinsic foot muscles |
| Short Foot Exercise | Precision | Arch control, foot intrinsics |
| Chair of Death Squat | Strength | Single-leg squat pattern, quad/glute |
| Single-Leg Deadlift with Dowel | Strength | Single-leg hip hinge, balance |
| Sling Pistol Squat | Strength | Advanced single-leg strength |
| Kneeling Banded Deadlift | Strength | Hip hinge pattern |
| Swiss Curls | Strength | Hamstring activation |
| Kettlebell Swing | Power | Hip extension power, posterior chain |
| Suitcase Carry | Strength | Anti-lateral flexion core |
| Calf Raises with Big Toe Focus | Strength | Calf + big toe propulsion |

#### Progression Model

| Level | Timing | Workouts | Focus | Frequency |
|-------|--------|----------|-------|-----------|
| 1 — Corrective/Foundation | Weeks 1-3 | Precision 1-3 + corrective exercises for failed assessments | Address assessment failures, basic motor patterns | 3x/week, 15-20 min, before runs |
| 2 — Precision Mastery | Weeks 3-6 | Precision 4-6 | Movement patterns stabilized under controlled conditions | 3x/week, 15-20 min |
| 3 — Strength Building | Weeks 6-12 | Performance Strength 7-10 | Loaded movements, progressive overload | 2-3x/week, 20-30 min, on hard running days |
| 4 — Power Development | Weeks 12-18+ | Performance Power 11-15 | Explosive movements, plyometrics, elastic energy | 2-3x/week, 20-30 min, on hard running days |
| Maintenance (Taper) | Race-specific | Precision only | Maintain neuromuscular activation, reduce volume 50-70% | 1-2x/week |

#### Phase Alignment with Running Macrocycle

| Running Phase | Strength Focus | Rationale |
|--------------|---------------|-----------|
| Base Building / Capacity | Precision + Performance Strength | Correct movement deficits, build foundational strength. Heaviest gym loading here. |
| Build / Specific Prep | Performance Strength + Performance Power | Transition from strength to power. Increase rate of force development. |
| Competition / Race-Specific | Performance Power + Precision maintenance | Emphasis on spring mechanics, neuromuscular sharpness. Reduce volume/load. |
| Taper / Race Week | Light Precision only | Maintain activation without adding fatigue. |
| Recovery / Transition | Precision + re-assessment | Reassess, address new limiters, rebuild base patterns. |

#### Strength Load Quantification

Strength sessions contribute to overall training load using an estimated TSS:

| Session Type | Estimated TSS | Rationale |
|-------------|--------------|-----------|
| Precision (15-20 min) | 15-25 | Low stress, neuromuscular focus, minimal fatigue |
| Performance Strength (20-30 min) | 30-50 | Moderate muscular stress, some systemic fatigue |
| Performance Power (20-30 min) | 35-55 | High neuromuscular demand, moderate systemic fatigue |

These estimates are added to the day's running TSS for ATL/CTL/TSB calculations. The coach may adjust estimates based on athlete feedback (RPE) over time.

**Intensity distribution:**
- **Capacity phases**: ~85-90% easy / 10-15% high intensity / minimal threshold
- **Competition phases**: Introduce threshold work, shift toward utilization
- **Taper**: Reduce volume 40-60%, maintain intensity

**Weekly plan output:**
- Day-by-day schedule with workout type, target duration, target pace/HR zones
- Weekly volume target (km and minutes)
- Expected TSS for the week
- Phase context ("Week 2 of Mesocycle 1 — Aerobic Capacity Building")

### 3.3 Workout Upload & Parsing

**Input methods (in priority order):**
1. **Coros API sync** (primary) — automatic workout sync for Coros watch users via OAuth. The athlete connects their Coros account once and workouts flow in automatically.
2. **Manual FIT file upload** — works with any GPS watch (Garmin, Wahoo, Suunto, Polar, Coros, etc.). The FIT file format is an ANT+ standard; all watches produce compatible files.

**Parsed data from FIT file:**
- Timestamp (per-second recording)
- Heart rate (bpm)
- Pace / speed
- GPS position (lat/long)
- Elevation / altitude
- Cadence (steps/min)
- Running power (watts)
- Distance (cumulative)
- Temperature

**Computed from parsed data:**
- Duration
- Total distance
- Average / max heart rate
- Average / max pace
- Normalized Graded Pace (NGP) — adjusts for elevation changes (see §3.4 for algorithm)
- Elevation gain / loss
- Time in each HR zone
- Time in each pace zone
- Pace distribution (splits per km/mile)
- Cadence average

### 3.4 Workout Analysis & Scoring

After parsing, the coach computes training load metrics and provides analysis.

#### Normalized Graded Pace (NGP)

NGP adjusts actual pace for elevation changes so that hilly runs produce comparable load metrics to flat runs. The algorithm converts uphill/downhill pace to flat-equivalent pace using the cost-of-running model:

```
For each second t:
  grade = elevation_change / horizontal_distance  (as decimal, e.g., 0.05 = 5%)
  cost_factor = 1 + (15.3 × grade) + (4.2 × grade²)
    (positive grade = uphill costs more, negative grade = downhill costs less but not linearly)
    For downhill: cost_factor = max(0.6, 1 + (15.3 × grade) + (4.2 × grade²))
    (floor at 0.6 — steep downhill is never "free" due to eccentric braking)

  adjusted_speed(t) = actual_speed(t) × cost_factor

NGP = rolling_average(adjusted_speed, 30s window)  →  then take weighted average across session
```

*Units note: NGP and FTPace are expressed as speed (m/s) in all formulas below. To convert from pace (min/km): speed_m_s = 1000 / (pace_min_km × 60).*

#### Training Load (per workout)

**Running TSS (rTSS):**

All values in speed units (m/s):
```
IF = NGP / FTPace
rTSS = (duration_seconds × NGP × IF) / (FTPace × 3600) × 100
```

**TRIMP (heart-rate based, per-second accumulation, Coros-compatible):**
```
For each second t:
  ΔHR(t) = (HR(t) - resting_HR) / (max_HR - resting_HR)
  TRIMP += (1/60) × ΔHR(t) × 0.2445 × e^(3.411 × ΔHR(t))
```
*Note: Per-second accumulation is critical for interval workouts. Using avg_HR underestimates TRIMP when HR swings widely.*

Both are calculated; rTSS is primary for load tracking, TRIMP is shown for Coros comparison.

#### Aerobic Effect Score (0–5 scale)

Our aerobic effect algorithm is inspired by Firstbeat's EPOC-based Training Effect but uses a simplified, implementable model based on zone-duration weighting and fitness-level normalization. The key insight from Firstbeat's research: aerobic training effect is primarily determined by **peak EPOC** during the session, which is driven by intensity and duration, then normalized by the athlete's fitness level.

**Step 1: Estimate %VO2max from heart rate**

For each second of the workout, estimate exercise intensity:
```
%HRR = (current_HR - resting_HR) / (max_HR - resting_HR)
%VO2max ≈ %HRR  (HR reserve maps approximately 1:1 to VO2 reserve)
```

**Step 2: Accumulate EPOC**

EPOC accumulates as a function of intensity and time, with an exponential relationship to intensity (research shows intensity explains 5x more EPOC variance than duration):
```
For each second t:
  intensity = %VO2max(t)
  if intensity < 0.30:
    accumulation_rate = 0  (below aerobic threshold, no meaningful EPOC accumulation)
  else:
    accumulation_rate = k1 × e^(k2 × intensity)

  decay_rate = k3 × EPOC(t-1)
  EPOC(t) = EPOC(t-1) + accumulation_rate - decay_rate

Where:
  k1 = 0.1   (base accumulation coefficient)
  k2 = 2.5   (exponential intensity scaling — makes high intensity disproportionately costly)
  k3 = 0.002 (EPOC decay rate per second during exercise — slow decay while exercising)
```

Track `peak_EPOC` = max(EPOC(t)) across the session.

**Step 3: Calculate aerobic EPOC contribution**

Only time spent below ~104% LTHR (Zones 1-4, 5a) contributes to aerobic effect. Time above this is counted toward anaerobic effect instead.
```
aerobic_EPOC = peak EPOC calculated using only HR samples ≤ 104% LTHR
```

**Step 4: Normalize to 0–5 scale by fitness level**

Fitter athletes need higher EPOC for the same training effect. The normalization uses the athlete's estimated fitness level (derived from CTL or weekly volume):

```
fitness_factor = 1.0 + (athlete_CTL / 100) × 0.5
  (CTL 0 → factor 1.0, CTL 50 → 1.25, CTL 100 → 1.5)

normalized_EPOC = aerobic_EPOC / fitness_factor

Aerobic Effect Score = min(5.0, normalized_EPOC / EPOC_scale_factor)
  where EPOC_scale_factor maps the expected EPOC range to 0-5
```

**Reference EPOC-to-score mapping** (calibrated against known workout types):

| Workout Example | Expected peak EPOC (ml/kg) | Expected Aerobic Effect |
|----------------|---------------------------|------------------------|
| 30-min easy run (Zone 1-2) | 10-20 | 1.0-1.9 |
| 60-min easy run (Zone 2) | 25-40 | 2.0-2.9 |
| 90-min long run (Zone 2) | 45-70 | 3.0-3.9 |
| 60-min tempo run (Zone 3-4) | 60-90 | 3.5-4.5 |
| Hard VO2max intervals (Zone 5) | 80-120 | 4.0-4.9 |

Scoring interpretation:
| Score | Meaning |
|-------|---------|
| 0.0–0.9 | No aerobic effect |
| 1.0–1.9 | Minor / recovery |
| 2.0–2.9 | Maintaining aerobic fitness |
| 3.0–3.9 | Improving aerobic fitness |
| 4.0–4.9 | Highly improving |
| 5.0 | Overloading |

#### Anaerobic Effect Score (0–5 scale)

The anaerobic effect algorithm detects high-intensity intervals with metabolic significance and scores their impact on anaerobic capacity. Inspired by Firstbeat's approach: the basis is to **detect high-intensity intervals that have metabolic meaning** and analyze their characteristics.

**Step 1: Detect high-intensity intervals**

Scan the workout for intervals where intensity exceeds threshold:
```
For each second t:
  is_high_intensity = (current_HR > 104% LTHR) OR (current_pace > FTPace)

Cluster consecutive high-intensity seconds into intervals.
Filter out intervals < 5 seconds (noise).
```

**Step 2: Score each interval's anaerobic contribution**

Each detected interval contributes based on its intensity, duration, and recovery context:
```
For each interval i:
  avg_intensity_i = average %VO2max during interval
  duration_i = interval duration in seconds

  # Short intervals (10-120s) have highest anaerobic impact
  # Long intervals (>180s) shift toward aerobic contribution
  duration_weight = 1.0 if duration_i <= 120
                  = max(0.3, 1.0 - (duration_i - 120) / 300) if duration_i > 120

  # Higher intensity = exponentially more anaerobic stress
  intensity_score = (avg_intensity_i - 0.85)^2 × 100  (only meaningful above ~85% VO2max)

  interval_anaerobic_score = intensity_score × duration_weight × (duration_i / 60)
```

**Step 3: Aggregate and normalize**

```
raw_anaerobic = sum(interval_anaerobic_score for all intervals)

# Normalize by fitness level (same approach as aerobic)
fitness_factor = 1.0 + (athlete_CTL / 100) × 0.5
normalized_anaerobic = raw_anaerobic / fitness_factor

Anaerobic Effect Score = min(5.0, normalized_anaerobic / anaerobic_scale_factor)
```

**Reference calibration:**

| Workout Example | Expected Anaerobic Effect |
|----------------|--------------------------|
| Easy run (Zone 1-2, no bursts) | 0.0-0.5 |
| Long run with strides | 0.5-1.5 |
| Tempo run (sustained threshold) | 1.0-2.0 |
| VO2max intervals (3-5 min @ Zone 5) | 2.5-3.5 |
| Sprint repeats (10-30s all-out, full recovery) | 3.5-4.5 |
| Race-pace intervals + sprint finish | 4.0-5.0 |

Same 0–5 scale interpretation as aerobic effect.

**Implementation note**: The exact coefficients (k1, k2, k3, scale factors) should be tuned by running the algorithm against a set of known workouts with expected scores. Start with the values above, then calibrate against 20-30 representative workouts across all types. The algorithm should produce scores consistent with Coros/Garmin for the same workout data.

#### Workout Classification (Deterministic — Server-Side)

Workout classification is **deterministic code, not an AI decision**. Claude receives the classification as a pre-computed fact and may comment on it, but never determines it.

Classification rules based on zone-time percentages and effect scores:

```
zone_1_3_pct = time_in_zones(1,2,3) / total_duration
zone_4_5_pct = time_in_zones(4,5) / total_duration
zone_6_7_pct = time_in_zones(6,7) / total_duration

IF zone_1_3_pct >= 0.80 AND anaerobic_effect < 1.5:
  classification = "aerobic_capacity"

ELIF zone_4_5_pct >= 0.20 AND aerobic_effect >= 2.5:
  classification = "aerobic_utilization"

ELIF zone_6_7_pct >= 0.10 AND anaerobic_effect >= 2.5:
  classification = "anaerobic_capacity"

ELIF zone_6_7_pct >= 0.05 AND aerobic_effect >= 2.0 AND anaerobic_effect >= 2.0:
  classification = "anaerobic_utilization"

ELSE:
  classification = "mixed"  (coach commentary should interpret)
```

**Date arithmetic is also deterministic.** The app computes all temporal context and passes it to Claude as facts:
- "Week 8 of 15"
- "Mesocycle 3 of 5, load phase, week 2 of 2"
- "52 days until race"
- "Last workout: 2 days ago"
- "Consecutive off-target sessions: 4"

Claude never performs date calculations.

### 3.5 Daily Load Tracking (ATL / CTL / TSB)

Calculated daily using exponentially weighted moving averages:

**Chronic Training Load (CTL — "Fitness"):**
```
CTL_today = CTL_yesterday + (TSS_today − CTL_yesterday) / 42
```
- 42-day time constant
- Represents accumulated fitness

**Acute Training Load (ATL — "Fatigue"):**
```
ATL_today = ATL_yesterday + (TSS_today − ATL_yesterday) / 7
```
- 7-day time constant
- Represents recent fatigue

**Training Stress Balance (TSB — "Form"):**
```
TSB = CTL_yesterday − ATL_yesterday
```
- Negative = fatigued/loading
- Positive = fresh/tapered
- Race-day target: +15 to +25

**Display**: Chart showing CTL, ATL, and TSB over time (Performance Management Chart)

### 3.6 Coach Feedback & Commentary

After each workout upload, the AI coach provides:

1. **Workout summary** — key metrics, what type of workout it was
2. **Comparison to prescription** — did the athlete hit the intended zones/paces?
3. **Training effect analysis** — aerobic/anaerobic scores and what they mean
4. **Load context** — how this workout affects ATL/CTL/TSB, current fatigue state
5. **Adjustment recommendations** — the coach considers the full picture before recommending any change:
   - All stress scores (rTSS, TRIMP, aerobic/anaerobic effect) and their trends
   - ATL/CTL/TSB current values and trajectory
   - Athlete's full profile (experience, sports background, injury history, training capacity)
   - Whether the signal is sustained (3+ sessions) vs. a one-off
   - The coach defaults to keeping the plan unchanged unless there is a clear, sustained reason to adjust (see §3.7)

**Tone**: Direct, knowledgeable, encouraging. Explains physiology in accessible terms. References Olbrecht's principles when relevant ("Remember — capacity is for training, power is for racing").

### 3.7 Plan Adjustment

**Philosophy: Stability over reactivity.** The coach errs on the side of *not* adjusting the plan. A well-designed periodized plan should be followed consistently — the coach should not chase day-to-day variance. Small deviations are normal and expected; the plan accounts for them. The coach only intervenes when there is a clear, sustained signal that the plan is misaligned with the athlete's actual state.

**Default behavior for common situations:**
- **Missed workout(s)**: Keep the plan as-is. Do not attempt to "make up" or redistribute missed work. The athlete simply picks up the plan where it is. If multiple consecutive days are missed, the coach may acknowledge it and reassure the athlete, but the plan remains unchanged unless the absence is prolonged (1+ weeks).
- **Workout slightly harder/easier than prescribed**: No adjustment. Note it, move on. The plan is designed with variance in mind.
- **Single bad day**: No adjustment. Could be sleep, stress, weather, nutrition. The plan stays.

**The coach adjusts the plan only when:**

| Trigger | Response |
|---------|----------|
| Extended absence (1+ weeks illness/injury/life) | Reassess readiness, potentially extend current mesocycle or add recovery week before resuming |
| Sustained pattern of workouts much harder than prescribed (3+ sessions) | Add recovery, reduce next session intensity |
| Sustained pattern of workouts much easier than prescribed (3+ sessions) | Consider increasing targets |
| ATL/CTL ratio > 1.5 (sustained) | Warn about overtraining risk, suggest recovery |
| ATL/CTL ratio < 0.8 (sustained) | Suggest increasing training stimulus |
| Approaching race | Transition to taper mesocycle (Workflow 4) |
| Illness / injury reported | Switch to recovery protocol |
| Mesocycle boundary | Evaluate against macrocycle goal, plan next mesocycle (Workflow 3) |

**When generating or adjusting the plan, the coach must consider the full athlete picture:**
- All athlete profile data (age, weight, experience level, sports background, injury history)
- All stress scores (rTSS, TRIMP, aerobic/anaerobic effect scores)
- Current ATL/CTL/TSB and their trends over time
- The athlete's sports background and how it affects training capacity, injury risk, and strength needs
- Strength/mobility programming load from Running Rewired sessions
- The macrocycle timeline and where the athlete is relative to race day

---

## 4. Zone Systems

### 4.1 Heart Rate Zones

7-zone model based on Lactate Threshold Heart Rate (LTHR), aligned with Olbrecht's energy system framework:

| Zone | Name | % of LTHR | Purpose | Olbrecht Category |
|------|------|-----------|---------|-------------------|
| 1 | Recovery | < 82% | Active recovery, blood flow | Recovery |
| 2 | Aerobic Easy | 82–88% | Aerobic capacity building | Aerobic Capacity |
| 3 | Aerobic Development | 89–93% | Upper aerobic, "high & low" work | Aerobic Capacity |
| 4 | Tempo / Threshold | 94–99% | Lactate threshold development | Aerobic Utilization |
| 5 | Supra-Threshold | 100–104% | VO2max development | Aerobic Utilization |
| 6 | Anaerobic | 105–110% | Anaerobic capacity | Anaerobic Capacity |
| 7 | Sprint / Max | > 110% | Neuromuscular power, max efforts | Anaerobic Capacity |

### 4.2 Pace Zones

Based on Functional Threshold Pace (FTPace = ~1hr race pace):

| Zone | Name | % of FTPace | Typical Use |
|------|------|-------------|-------------|
| 1 | Recovery | < 75% FTPace | Recovery runs |
| 2 | Endurance | 75–85% FTPace | Easy/long runs |
| 3 | Tempo | 86–95% FTPace | Tempo runs |
| 4 | Threshold | 96–105% FTPace | Threshold intervals |
| 5 | VO2max | 106–120% FTPace | VO2max intervals |
| 6 | Speed | > 120% FTPace | Sprint/speed work |

*Note: Pace zones are inverted — faster pace = higher zone, so percentages represent speed, not time.*

---

## 5. User Interface

**Platform: Responsive web application.** The app must work well on both desktop and mobile browsers. Runners engage with their training data on their phones immediately post-run — this is the highest-engagement moment and we cannot miss it. No native mobile app needed; a responsive web UI is sufficient. Data is stored in SQLite on the server.

### 5.1 Screens / Views

**Dashboard**
- Current training phase and mesocycle info
- Today's prescribed workout
- CTL / ATL / TSB current values with trend arrows
- Weekly volume completed vs. target
- Next race countdown (if set)

**Chat / Coach Interaction**
- Conversational interface with Coach Jan
- Workout analysis appears here after upload
- Athlete can ask questions about their training
- Plan adjustments are discussed and confirmed here

**Training Plan**
- Calendar view of upcoming workouts
- Each workout shows: type, duration, target zones, expected TSS
- Past workouts show: completed metrics, compliance color (green/yellow/red)
- Weekly and mesocycle summaries

**Workout Detail**
- Full metrics from parsed FIT file
- HR over time chart with zone coloring
- Pace over time chart with zone coloring
- Splits table (per km)
- Aerobic / Anaerobic effect scores
- rTSS and TRIMP values
- Coach commentary

**Performance Chart**
- PMC (Performance Management Chart) showing CTL, ATL, TSB over time
- Workout dots on timeline colored by workout type
- Annotations for mesocycle boundaries and races

**Athlete Profile**
- Personal details and current zones
- Zone tables (HR and pace)
- FTPace history
- Race results

### 5.2 Interaction Patterns

- **Upload workflow**: Athlete clicks upload → selects FIT file → parsing happens → coach commentary appears in chat → plan adjusts if needed
- **Plan review**: Athlete views calendar → taps a workout → sees details and coach notes
- **Ask the coach**: Free-form text input in chat → Claude responds as Coach Jan
- **Zone updates**: Athlete can manually update LTHR or FTPace → zones recalculate → coach acknowledges and adjusts plan if needed

---

## 6. Coach Persona: Jan

### Personality
- Knowledgeable and confident, rooted in exercise physiology
- Direct communicator — gives clear prescriptions with reasoning
- Patient teacher — explains complex concepts accessibly
- Pragmatic — adapts to the athlete's life constraints
- Encouraging but honest — celebrates progress, flags concerns early

### Communication Style
- Uses "we" language ("Let's focus on building your aerobic capacity this mesocycle")
- References the science ("Your lactate system needs steady-state work right now, not more threshold")
- Gives context for every prescription ("I'm keeping today easy because your ATL is 20% above your CTL — you need to absorb the work from this week")
- Celebrates consistency over heroics ("Three solid easy runs are worth more than one spectacular workout followed by three days off")

### Key Phrases / Philosophy
- "Capacity is for training, power is for racing."
- "We develop your engine first, then we teach you to use it."
- "The aerobic and anaerobic systems are dance partners — we can't train one without affecting the other."
- "Your body adapts during recovery, not during the workout itself."
- "I'd rather you finish a workout feeling like you could do more than have you crawling to the finish."

---

## 7. AI Interaction Layer

### How Claude Powers the Coach

The app uses Claude as the reasoning engine for plan generation, workout analysis, mesocycle evaluation, and coaching conversation. All AI interactions follow a structured pattern: the app assembles context, calls Claude with a system prompt + structured data via **tool_use (function calling)**, and validates the response.

**Key boundary**: Claude reasons and communicates. The app computes. All numeric work (rTSS, TRIMP, EPOC, effect scores, ATL/CTL/TSB, zones, NGP, workout classification, date arithmetic) is deterministic server-side code. Claude never computes numbers — it receives pre-computed values and interprets them.

### System Prompt Architecture

Every Claude call includes a **base system prompt** (cached via Anthropic's prompt caching for cost/latency optimization), followed by **context-specific data** injected per-request.

**Base system prompt** (see §7.1 for full specification):
- Coach Jan persona, communication style, key phrases (§6)
- Olbrecht's training philosophy as concrete decision rules
- Plan stability philosophy as conditional logic (§3.7)
- Zone definitions and workout type taxonomy (enum: `easy_run`, `long_run`, `aerobic_development`, `tempo_run`, `vo2max_intervals`, `speed_sprint`, `race_specific`, `recovery_run`, `strength_precision`, `strength_performance`, `strength_power`)
- Scoring interpretation guidelines
- Explicit guardrails (no plan changes via chat, no off-topic responses, no date arithmetic)

**Per-request context** (assembled by the app, injected into each call):
- Current athlete profile (all fields from §3.1, including sports background)
- Current zones (HR and pace tables)
- Pre-computed temporal context: "Week 8 of 15, Mesocycle 3 of 5, 52 days until race" (never ask Claude to compute dates)
- Current ATL/CTL/TSB values and 7-day trend
- Recent workout history (last 7-14 days of completed workouts with scores)
- The current training plan (upcoming 1-2 weeks of scheduled workouts)
- App-computed flags: `consecutive_off_target_sessions: 4`, `days_since_last_workout: 2`, `adjustment_eligible: true`
- Strength programming state (current level, recent sessions, assessment results) — included only when relevant

**Model selection per interaction type:**
| Interaction | Model | Rationale |
|-------------|-------|-----------|
| Plan Generation | Sonnet | Complex reasoning with good cost/speed balance. Upgrade to Opus if quality insufficient |
| Mesocycle Transition | Sonnet | Needs evaluation + generation |
| Workout Analysis | Sonnet | Needs compliance reasoning, pattern detection |
| Free-form Chat | Haiku | Fast, cheap, sufficient with good system prompt |

### Structured Output via Tool Use

All structured interactions use Claude's **tool_use (function calling)** instead of raw JSON generation. This provides:
- Schema validation at the API level
- Enum constraints on workout types, phase types, adjustment operations
- Reliable parsing without JSON formatting errors
- Natural decomposition of complex outputs into sequential tool calls

**Retry policy**: On tool_use validation failure, retry up to 2 times with the validation error included in the next message. If all retries fail, fall back to displaying computed metrics only (all deterministic values are available without Claude) and queue a background retry. The athlete always gets immediate feedback.

### Interaction Types

**1. Plan Generation** (onboarding)

Two-phase approach for reliability:

**Phase 1 — Macrocycle Skeleton** (`generate_macrocycle_skeleton` tool):

Input to Claude:
- Full athlete profile
- Goal race details
- Current CTL (bootstrapped or actual)
- Pre-computed: weeks until race, recommended mesocycle count and durations for athlete level

Output (tool_use):
```
generate_macrocycle_skeleton(
  target_ctl: 65,
  mesocycles: [
    { phase: "capacity", focus: "aerobic_capacity", load_weeks: 2, recovery_weeks: 1, target_volume_km: 40 },
    { phase: "capacity", focus: "aerobic_capacity", load_weeks: 2, recovery_weeks: 1, target_volume_km: 45 },
    { phase: "utilization", focus: "aerobic_utilization", load_weeks: 2, recovery_weeks: 1, target_volume_km: 45 },
    { phase: "taper", focus: "race_specific", load_weeks: 1, recovery_weeks: 1, target_volume_km: 28 }
  ],
  coach_message: "Let's build your aerobic engine over the next 15 weeks..."
)
```

The app presents the skeleton to the athlete for review. Athlete can request modifications ("I can't train on Tuesdays," "that's too much volume") before proceeding.

**Phase 2 — Weekly Detail** (`generate_weekly_plan` tool, called per-week):

Input: mesocycle context + week number + volume target
Output (tool_use):
```
generate_weekly_plan(
  week_number: 1,
  target_volume_km: 40,
  target_tss: 250,
  workouts: [
    { day: "monday", type: "easy_run", duration_min: 45, target_hr_zone: [1, 2], target_pace_zone: [1, 2], expected_tss: 35, notes: "Conversational pace" },
    { day: "tuesday", type: "rest", duration_min: 0, expected_tss: 0 },
    ...
  ],
  strength_sessions: [
    { day: "monday", type: "strength_precision", workout_number: 1, duration_min: 20, estimated_tss: 20 },
    ...
  ]
)
```

Each weekly plan is validated by the server-side plan validation layer (§7.2) before being committed to the database.

**2. Workout Analysis** (after FIT file upload)

Input to Claude:
- Parsed workout summary (all metrics from §3.3)
- Computed scores (rTSS, TRIMP, aerobic/anaerobic effect)
- Computed workout classification (deterministic, see §3.4)
- The prescribed workout for that day
- Current ATL/CTL/TSB before and after this workout
- App-computed flags: `consecutive_off_target_count`, `off_target_direction` (harder/easier), `adjustment_eligible`

Output (`analyze_workout` tool):
```
analyze_workout(
  summary: "60-minute easy run, well-executed",
  prescription_compliance: "on_target",  // enum: on_target | harder_than_prescribed | easier_than_prescribed | different_type | unplanned
  training_effect_commentary: "Solid aerobic stimulus...",
  load_commentary: "Your ATL is at 45, CTL at 38...",
  plan_adjustment: null,  // null in most cases — see adjustment schema below
  coach_message: "Nice work today. You held Zone 2 consistently..."
)
```

**Plan adjustment schema** (when non-null, must use a closed set of operations):
```
plan_adjustment: {
  trigger_reason: "sustained_harder_than_prescribed",  // must map to §3.7 trigger
  operations: [
    { type: "swap_workout", target_date: "2024-03-15", new_type: "recovery_run", new_duration_min: 30 },
    { type: "reduce_intensity", target_date: "2024-03-16", new_target_hr_zone: [1, 2] }
  ],
  explanation: "You've been running harder than prescribed for 4 sessions in a row..."
}
```

Allowed operation types: `swap_workout`, `reduce_intensity`, `increase_intensity`, `add_recovery_day`, `extend_mesocycle`, `skip_workout`.

**All plan adjustments are proposals.** The app presents a diff view to the athlete: "Coach Jan proposes the following changes: [list]. Accept / Reject." Adjustments are never auto-applied.

**3. Mesocycle Transition** (at mesocycle boundary)

This is a distinct interaction type from Plan Generation. It evaluates the completed mesocycle and generates only the next one.

Input to Claude:
- Completed mesocycle summary: total volume, compliance rate, average weekly TSS, CTL trend over the mesocycle
- Pre-computed: CTL trajectory vs. race-day target (on track / behind / ahead), weeks remaining, mesocycles remaining
- Current athlete state: ATL/CTL/TSB, any reported issues
- Macrocycle skeleton (what was originally planned for the next mesocycle)
- FTPace/LTHR re-validation results if available (Workflow 3a)

Output (`evaluate_mesocycle` tool, then `generate_weekly_plan` per-week):
```
evaluate_mesocycle(
  assessment: "Mesocycle 2 completed successfully. Volume compliance 92%, CTL increased from 38 to 45.",
  ctl_vs_target: "on_track",  // enum: on_track | behind | ahead
  next_phase_recommendation: "capacity",  // enum: capacity | utilization | taper
  next_focus: "aerobic_capacity",
  volume_adjustment: "maintain",  // enum: increase_10pct | maintain | decrease
  strength_level_progression: true,  // advance to next strength level?
  coach_message: "Great block of training. Your aerobic base is building nicely..."
)
```

After the athlete reviews the evaluation, the app calls `generate_weekly_plan` for each week of the next mesocycle (same as Plan Generation Phase 2).

**4. Free-form Chat** (athlete asks a question)

Input to Claude:
- The athlete's message
- Full context (profile, plan, recent workouts, current metrics)
- Conversation history (last 20 messages or 4,000 tokens, whichever is smaller)

Output: Free-text response as Coach Jan. No structured data — purely conversational. The app stores the message in the chat history.

**Guardrails**: The system prompt explicitly instructs: "You never modify the training plan during chat conversations. Plan changes only happen through the formal workout analysis or mesocycle transition process. If the athlete asks to change their plan, explain that you'll evaluate it at the next appropriate time. You only discuss running, training, exercise physiology, and related topics. If asked about anything outside your domain, redirect to training."

### Context Window Management

The app is responsible for assembling context that fits within Claude's context window. Strategy:
- **Always include**: athlete profile, current zones, current ATL/CTL/TSB, pre-computed temporal context, current week's plan, today's prescribed workout, app-computed flags
- **Include when relevant**: last 7-14 days of workout summaries (not raw time-series data), mesocycle objectives, recent chat history
- **Include conditionally**: strength programming context (only for plan generation, mesocycle transitions, or when athlete asks about strength)
- **Summarize when needed**: older workout history is pre-summarized into weekly aggregates (total volume, average TSS, compliance rate)
- **Never send**: raw FIT file time-series data to Claude. All parsing and scoring is done server-side; Claude receives computed summaries only.
- **Chat history limit**: last 20 messages or 4,000 tokens, whichever is smaller. Older messages remain in the database for UI display but are dropped from context.

### 7.1 System Prompt Specification

The base system prompt is the most critical artifact in the AI layer. It must be concrete, rule-based, and tested against scenarios.

**Structure:**

```
You are Coach Jan, an AI running coach built on Jan Olbrecht's training philosophy.

## Your Identity
[Persona from §6 — personality, communication style, key phrases, "we" language]

## Your Training Philosophy
- Capacity is for training, power is for racing
- During capacity phases: prescribe 85-90% of volume in Zones 1-3. No more than 2 high-intensity sessions per week for beginners, 2-3 for intermediate/advanced
- During utilization phases: introduce threshold work (Zone 4-5), shift toward race-specific intervals
- During taper: reduce volume 40-60%, maintain 1-2 intensity sessions, monitor TSB trending toward +15 to +25
- Recovery weeks: reduce volume 30-50% from the preceding load week, maintain one intensity session

## Plan Stability Rules
You are conservative about plan changes. Most deviations are normal and expected.

- IF athlete missed 1-2 workouts: DO NOT adjust. Acknowledge, reassure, continue the plan as-is.
- IF athlete missed 3-6 consecutive days (but <7): Acknowledge, DO NOT adjust the plan. The athlete picks up where the plan is.
- IF athlete missed 7+ days: PROPOSE extending the current mesocycle or adding a recovery week. Never auto-apply.
- IF a single workout was harder/easier than prescribed: DO NOT adjust. Note it, move on.
- IF 3+ consecutive workouts were harder than prescribed (app will flag this): PROPOSE adding recovery or reducing intensity.
- IF 3+ consecutive workouts were easier than prescribed (app will flag this): PROPOSE increasing targets.
- IF ATL/CTL ratio > 1.5 (sustained, app will flag): Warn about overtraining risk, PROPOSE recovery.

## What You Never Do
- Never compute numbers (TSS, EPOC, ATL/CTL/TSB, zones, dates, paces). All numbers are provided to you pre-computed.
- Never modify the training plan during chat conversations.
- Never discuss topics outside running, training, and exercise physiology.
- Never recommend adjustments based on a single off-target workout.
- Never generate workout types outside the allowed set: [enum list]

## How You Analyze Workouts
[2-3 few-shot examples at different compliance levels — see below]
```

**Few-shot examples** (included in system prompt):

Example 1 — On-target easy run:
```
Workout: 50 min easy run. Avg HR Zone 2 (84% LTHR). rTSS 38. Aerobic effect 2.3.
Prescribed: 45 min easy run, Zone 1-2.
→ "Good session. You stayed right in Zone 2 for the full run — exactly what we want for aerobic capacity building. The extra 5 minutes is fine. Your aerobic effect of 2.3 confirms this was a maintaining-to-building stimulus. Keep this consistency up."
Plan adjustment: null
```

Example 2 — Single workout harder than prescribed (NO adjustment):
```
Workout: 45 min easy run. Avg HR Zone 3 (92% LTHR). rTSS 52. Aerobic effect 2.8.
Prescribed: 45 min easy run, Zone 1-2.
→ "Your heart rate crept into Zone 3 today — a bit harder than intended. Could be heat, stress, or just an off day. Not a concern from a single session. Tomorrow's workout stays as planned. Just try to keep it conversational next time."
Plan adjustment: null
```

Example 3 — Sustained pattern requiring adjustment:
```
Workout: 50 min easy run. Avg HR Zone 3 (93% LTHR). rTSS 55. Aerobic effect 3.1.
App flags: consecutive_off_target_count: 4, off_target_direction: harder, adjustment_eligible: true
→ "This is the fourth session in a row where your heart rate has been above the prescribed zone. Your body is telling us something — either the zones need recalibration or you need more recovery. I'd like to swap tomorrow's tempo run for an easy recovery run so you can absorb the accumulated load."
Plan adjustment: { trigger_reason: "sustained_harder_than_prescribed", operations: [...] }
```

### 7.2 Server-Side Plan Validation

Every Claude-generated plan (from Plan Generation or Mesocycle Transition) must pass deterministic validation before being committed to the database. These are hard rules that reject or flag plans regardless of what Claude produces.

**Validation rules:**

| Rule | Check | Action on Failure |
|------|-------|-------------------|
| Intensity frequency | No more than 3 high-intensity sessions (tempo, VO2max, speed, race-specific) per week | Reject, retry with constraint in prompt |
| Volume progression | Weekly volume increase ≤ 10% over previous week (except recovery weeks) | Reject, retry |
| Recovery week volume | Recovery weeks must reduce volume by 30-60% from preceding load week | Reject, retry |
| TSS bounds | Weekly TSS must be within 0.5x–2.0x of athlete's current CTL × 7 | Reject, retry |
| Workout type enum | Every workout `type` must be in the allowed enum | Reject, retry |
| Date consistency | No duplicate workouts on same day, all dates within macrocycle range | Reject, retry |
| Rest days | Minimum 1 rest or recovery-only day per week | Reject, retry |
| Long run frequency | Maximum 1 long run per week | Reject, retry |
| Strength scheduling | Strength performance/power sessions not on same day as VO2max/speed running sessions | Flag as warning |

**On validation failure**: The app retries the Claude call (up to 2 retries) with the specific validation error included as context: "Your previous plan was rejected because [rule]. Please regenerate with this constraint." If all retries fail, the app alerts the athlete that plan generation needs manual review.

---

## 8. Data Model (Conceptual)

### Athlete
- Profile (name, age, weight, experience level, sports background)
- Physiological markers (resting HR, max HR, LTHR, FTPace)
- FTPace history (date, value, source — race/time trial/estimate)
- LTHR history (date, value, source)
- HR zones (derived from current LTHR)
- Pace zones (derived from current FTPace)
- Goal race (distance, date)

### Training Plan
- Goal (race distance, race date, target time or effort level)
- Macrocycle (start date, end date, goal race)
  - Target CTL at race day
  - Planned mesocycle sequence with phase focus progression (capacity → utilization → taper)
  - Expected peak timing (when TSB should hit +15 to +25)
- Mesocycles (phase type, load/recovery structure, objectives relative to macrocycle goal)
- Planned workouts (date, type, duration, target zones, expected TSS)

### Workout (Completed)
- Raw FIT file reference
- Parsed summary metrics
- Time-series data (HR, pace, cadence, elevation per second)
- Computed scores (rTSS, TRIMP, aerobic effect, anaerobic effect)
- Workout classification
- Coach commentary

### Daily Metrics
- Date
- TSS for the day (sum of all workouts)
- ATL
- CTL
- TSB

---

## 9. Key Workflows

### Workflow 1: Onboarding
```
Athlete opens app
  → Fills out profile form
  → Coach Jan introduces himself, explains his philosophy
  → Coach asks about current training and goals
  → Zones are calculated and displayed
  → Coach generates initial training plan
  → Athlete reviews and confirms plan
```

### Workflow 2: Daily Training
```
Athlete views dashboard
  → Sees today's prescribed workout with details
  → Goes for run with GPS watch
  → Workout syncs automatically via Coros API, or athlete uploads FIT file manually
  → Coach analyzes and provides feedback
  → ATL/CTL/TSB update
  → Plan adjusts if needed
```

### Workflow 3: Mesocycle Transition

Every mesocycle transition is evaluated against the overarching macrocycle plan. The goal race and its date are the anchor — each mesocycle exists to move the athlete closer to peaking on race day.

```
End of mesocycle load phase
  → Coach reviews mesocycle summary (volume, compliance, load trends)
  → Coach evaluates progress against macrocycle goal:
    → Is CTL on track to hit race-day target?
    → Is the capacity/utilization phase progression correct for time remaining?
    → Are there signs that the plan needs to accelerate or back off?
  → Recovery phase begins
  → At start of next mesocycle:
    → Coach reviews super-compensation (are CTL/performance improving?)
    → Coach determines next mesocycle's focus based on:
      → Where we are in the macrocycle timeline
      → What the athlete still needs to develop before race day
      → Whether to stay in capacity work or shift toward utilization
    → Coach sets objectives for next mesocycle (tied to macrocycle goal)
    → New mesocycle workouts generated
    → If the next mesocycle is the final one before race day → transition to Workflow 4
```

### Workflow 3a: FTPace Re-Validation

Zones are only as good as the FTPace they're derived from. Over a macrocycle, fitness changes can shift FTPace significantly. The coach schedules periodic validation to keep zones accurate.

**When to validate:**
- At every mesocycle boundary (during the recovery phase, when the athlete is relatively fresh)
- After a race result that can serve as a data point
- If the coach detects a sustained pattern of workouts feeling too easy or too hard relative to prescribed zones (cardiac drift analysis, pace-to-HR decoupling trending in one direction)

**Validation methods (in order of preference):**
1. **Race result**: If the athlete races during the macrocycle, use the result to recalculate FTPace (e.g., a 10K race result can estimate 1-hour threshold pace using standard equivalency tables)
2. **Structured time trial**: Coach prescribes a 30-minute time trial (best effort for 30 min, FTPace ≈ 95% of average pace) during the recovery phase of a mesocycle transition
3. **Workout-derived estimation**: If the athlete has done multiple tempo or threshold workouts, the coach can estimate FTPace from the pace-to-HR relationship at threshold — specifically, the pace at which HR stabilizes near LTHR

**What happens after re-validation:**
```
New FTPace established (or confirmed unchanged)
  → Pace zones recalculate automatically
  → Coach explains what changed and why
  → Upcoming workouts in the plan update target paces to new zones
  → Coach notes the FTPace progression in athlete history
  → If FTPace improved significantly → confirms training is working
  → If FTPace stagnated or declined → coach considers whether to adjust mesocycle focus
```

**LTHR re-validation**: LTHR is more stable than FTPace but should also be checked. If a time trial or race includes HR data, the coach can estimate LTHR from the average HR during the effort. If LTHR shifts, HR zones recalculate.

### Workflow 4: Race Preparation (Taper Mesocycle)

The taper is the final mesocycle in the macrocycle — not a separate protocol. It is the planned culmination that all prior mesocycles have been building toward. The coach has been steering the athlete's CTL and phase progression specifically so the taper begins at the right time with the right fitness base.

```
Final mesocycle before goal race (typically 2-3 weeks out)
  → Coach confirms athlete is ready for taper:
    → CTL has reached or is near race-day target
    → Capacity work is sufficient — time to shift to utilization/sharpening
  → Taper mesocycle begins:
    → Volume drops 40-60%, intensity maintained
    → Shift from capacity to utilization work (race-pace intervals, tempo)
    → Monitor TSB trending positive toward +15 to +25 on race day
  → Race week:
    → Minimal volume, race-pace activation runs
    → Coach gives race strategy and final advice
  → Race day:
    → Coach gives final pacing/fueling guidance
  → Post-race:
    → Recovery protocol
    → Coach debrief: review macrocycle, what worked, what to adjust
    → Decision point: set next goal → triggers new macrocycle (back to Workflow 1)
```

---

## 9. Authentication & Accounts

**MVP: Email + password auth.** Simple registration and login. No email verification, no emails sent.

- Single athlete per account (no multi-athlete support in MVP)
- Passwords hashed with Argon2id
- Session-based auth with secure HTTP-only cookies
- No OAuth/social login in MVP (except Coros OAuth for API sync, which is a data connection, not a login method)
- Account data is isolated in SQLite — each athlete's data is only accessible to them
- No email sending infrastructure required

---

## 10. Scope & Constraints

### In Scope (MVP)
- Athlete profile with HR/pace zones
- Training plan generation (one macrocycle for a goal race)
- FIT file upload and parsing (any GPS watch)
- Coros API integration for automatic workout sync
- Workout analysis with rTSS, aerobic/anaerobic effect scores
- Daily ATL/CTL/TSB tracking
- Coach chat with workout feedback
- Plan adjustment based on actual training
- SQLite database for all data storage
- Deployable as a single server (e.g., Railway) with minimal infrastructure

### Out of Scope (MVP)
- Multi-athlete support
- Lactate test data input
- Running power-based training zones
- Nutrition / sleep / HRV integration
- Native mobile app (responsive web covers mobile use)
- Workout export to Coros/Garmin
- Social features
- Payment / subscription

### Future Considerations
- Lactate test integration for true Olbrecht-style "steering"
- HRV-based readiness scores
- Running power zones (from Coros or Stryd)
- Multi-sport support (cycling, swimming)
- Workout builder with visual drag-and-drop

---

## 11. Success Criteria

- Athlete can go from zero to a generated training plan in under 10 minutes
- FIT file upload to coach feedback in under 30 seconds
- Training plans follow Olbrecht's periodization principles correctly
- ATL/CTL/TSB calculations match TrainingPeaks within 5% for same data
- Coach commentary is specific, actionable, and grounded in the athlete's data
- The app runs as a single server with SQLite and the Claude API as its only dependencies
- Deployable on Railway or similar platforms at minimal cost

---

## 12. Open Questions

1. **Threshold estimation**: If the athlete doesn't have a recent race result, how should we estimate FTPace? Options: guided time trial protocol, conservative estimate from easy pace, or ask-and-adjust.
Ask and adjust for now. In the future we can also add time trial protocol option

2. **Zone model**: Should we use a 5-zone or 7-zone HR model?
Resolved: 7-zone model in §4.1, mapped to Olbrecht's four energy system categories. Zones 1-3 cover recovery and aerobic capacity, Zones 4-5 cover utilization/threshold, Zones 6-7 cover anaerobic capacity.

3. **Aerobic/anaerobic scoring algorithm**: The Firstbeat algorithm is proprietary. We need to define our own EPOC-based or simplified model. Should we document the exact algorithm in the technical design?
Yes — fully specified in §3.4. Our algorithm uses EPOC accumulation (exponential intensity scaling + linear duration) normalized by fitness level (CTL-derived). Aerobic effect is based on peak EPOC from sub-threshold work. Anaerobic effect detects high-intensity intervals and scores them by intensity, duration (10-120s intervals weighted highest), and recovery context. Coefficients should be calibrated against 20-30 representative workouts.

4. **Plan rigidity**: How strictly should the coach follow the generated plan? Should it auto-adjust daily based on the previous day's data, or only at mesocycle boundaries?
The coach errs on the side of NOT adjusting. Missed days → keep plan as-is, athlete picks up where they are. Only adjust for sustained patterns (3+ sessions off-target) or extended absences (1+ weeks). No reactive daily tweaking.

5. **FIT file access**: Coros requires manual export through their app. Should we document the export steps for the athlete, or build a helper that watches a directory for new FIT files?
Resolved: Coros API integration is in MVP scope (primary sync method). Manual FIT upload supports all GPS watch brands as fallback. Include instructions for historical upload via FIT files.

6. **Initial CTL bootstrapping**: New users have no history. Should we estimate initial CTL from their reported weekly volume, or start from zero and let it build?
Resolved: Estimate initial CTL from reported weekly volume and experience level. Starting at 0 would produce nonsense metrics for 6+ weeks for any runner with an existing training base.

**Bootstrap formula:**
```
# Estimate average daily TSS from reported weekly volume
avg_pace_factor = {beginner: 0.65, intermediate: 0.75, advanced: 0.85}  (IF estimate)
estimated_weekly_tss = weekly_km × avg_pace_factor[level] × 5  (rough TSS-per-km heuristic)
estimated_daily_tss = estimated_weekly_tss / 7
initial_CTL = estimated_daily_tss  (CTL ≈ average daily TSS at steady state)
initial_ATL = estimated_daily_tss  (assume current load matches chronic load)
```
This produces a reasonable starting point: a 40km/week intermediate runner gets CTL ≈ ~21, which is plausible. The values self-correct within 2-3 weeks as real workout data flows in.
