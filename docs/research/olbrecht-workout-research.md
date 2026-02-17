# Olbrecht Workout Registry Research

## Research Date: 2026-02-15

This document captures research on Jan Olbrecht's training methodology and how it applies to CoachJan's workout registry. It covers missing workout types, under/over interval variations, warmup/cooldown scaling, progressive adjustment patterns, and athlete limitation-based prescription.

---

## Table of Contents

1. [Olbrecht's Core Framework Recap](#1-olbrechts-core-framework-recap)
2. [Missing Workout Types](#2-missing-workout-types)
3. [Hill Sprint Specifics](#3-hill-sprint-specifics)
4. [Under/Over Workout Variations (Detailed)](#4-underover-workout-variations-detailed)
5. [Warmup/Cooldown as Distances](#5-warmupcooldown-as-distances)
6. [Progressive Adjustment Patterns](#6-progressive-adjustment-patterns)
7. [Athlete Limitation-Based Prescription](#7-athlete-limitation-based-prescription)
8. [Implementation Recommendations](#8-implementation-recommendations)
9. [Sources](#9-sources)

---

## 1. Olbrecht's Core Framework Recap

Before defining new workout types, it is essential to ground every decision in Olbrecht's framework. His key principles, as documented across his book "The Science of Winning" and various interviews/applications:

### Capacity vs. Power (Utilization)

- **Capacity** = the size of your engine (aerobic capacity = VO2max; anaerobic capacity = VLamax/ability to produce lactate)
- **Power/Utilization** = how much of that engine you can use during performance (e.g., %VO2max at lactate threshold)
- These are **zero-sum**: improving one tends to diminish the other. You cannot develop both simultaneously at maximum rate.
- **"Capacity is for training, power is for racing."**

### The Aerobic-Anaerobic Interplay

- Aerobic capacity (AEC) and anaerobic capacity (ANC) are "counterplayers" on the lactate curve
- AEC pulls the lactate curve to the right (better endurance)
- ANC pulls the lactate curve to the left (more glycolytic power)
- For distance runners: generally want HIGH AEC and MODERATE-TO-LOW ANC
- The right balance depends on race distance: shorter races tolerate higher ANC; marathon/ultra demand low ANC

### The "High and Low" Principle

Olbrecht's signature training approach:
- **Low intensity (60%+ of session)**: Easy running that develops slow-twitch fibers and speeds regeneration
- **High intensity (up to 40% of session)**: Short bursts (30-35 seconds) at speeds exceeding VO2max
- **What's deliberately avoided**: Sustained threshold/tempo work during the capacity-building phase
- The mixing of easy + short-hard activates mitochondria across both slow and fast-twitch fibers simultaneously

### Periodization Sequence

1. **Capacity phase** (majority of macrocycle): Build aerobic and anaerobic capacity through "high and low" training. Lots of easy volume + short explosive bursts. Minimal threshold work.
2. **Utilization/Power phase** (2-3 weeks before race): Shift to threshold and race-pace work. This deliberately trades some capacity for power.
3. **Taper** (1-1.5 weeks): Reduce volume, maintain sharpness.

### Mesocycle Structure

| Athlete Level | Load Phase | Recovery Phase |
|---------------|------------|----------------|
| Beginner      | 1 week     | 1 week         |
| Intermediate  | 2 weeks    | 1 week         |
| Advanced      | 3 weeks    | 2 weeks        |

---

## 2. Missing Workout Types

### 2.1 Fartlek (Structured)

**Key**: `fartlek_structured`

**Description**: Continuous run with predetermined hard/easy segments. Unlike track intervals, the runner never stops. The hard segments are run at varied intensities (5K effort to mile effort) with easy running between. This is the Kenyan-style approach where the workout structure is known before starting.

**Olbrecht Category**: Aerobic Utilization / Mixed (depends on segment intensities)

**When Used**: Utilization phase primarily. Can be introduced late in capacity phase as a transition workout. Also useful for race-specific phases to practice surge handling.

**Structure**:
- **Short**: WU + 8x1 min hard (Zone 4-5) / 1 min easy (Zone 2) + CD (~40 min total)
- **Medium**: WU + 12x1 min hard (Zone 4-5) / 1 min easy (Zone 2) + CD (~50 min total)
- **Long**: WU + 15x2 min hard (Zone 4-5) / 1 min easy (Zone 2) + CD (~65 min total)

**Target Zones**: HR Zones 2-5 (mixed), Pace Zones 2-5

**TSS Range**: Short 45-60, Medium 60-80, Long 80-105

**Progressive Adjustment**: Increase number of reps week-over-week, or shift from 1:1 to 2:1 work:recovery ratio. Can also increase intensity of hard segments from Zone 4 toward Zone 5.

**Rationale**: The Kenyan fartlek is a proven format (typically done weekly in Iten, Kenya). The three classic Kenyan variations are: 30x1min/1min, 20x2min/1min, 15x3min/1min for a one-hour session. This trains race-surge handling, lactate clearance during movement, and mental toughness. It differs from track intervals because the continuous nature prevents full recovery and teaches the body to clear lactate while still running.

**Warmup/Cooldown**: See Section 5.

---

### 2.2 Cruise Intervals

**Key**: `cruise_intervals`

**Description**: Repeated threshold-pace segments (1000m to 2-mile repeats) with very brief recovery (60-90 seconds jog). The brief recovery keeps lactate slightly elevated, training the body's lactate clearance mechanisms. Unlike tempo runs (sustained effort), cruise intervals break up the threshold work to accumulate more total time at threshold pace.

**Olbrecht Category**: Aerobic Utilization

**When Used**: Utilization phase. This is a KEY utilization workout. Olbrecht would use this sparingly and only in the final 2-6 weeks before a goal race. During capacity phases, this type of sustained near-threshold work actually diminishes aerobic capacity.

**Structure**:
- **Short**: WU + 3x1000m @ Zone 4 / 60s jog + CD (~35 min total)
- **Medium**: WU + 4x1200m @ Zone 4 / 75s jog + CD (~45 min total)
- **Long**: WU + 5x1 mile @ Zone 4 / 90s jog + CD (~55 min total)

**Target Zones**: HR Zone 4, Pace Zone 3-4

**TSS Range**: Short 45-55, Medium 55-70, Long 65-85

**Progressive Adjustment**: Add reps (3x1000m -> 4x1000m -> 5x1000m), increase distance of each rep (1000m -> 1200m -> 1600m), or reduce recovery (90s -> 75s -> 60s). Do not combine all three progressions simultaneously.

**Rationale**: Jack Daniels popularized cruise intervals, and Olbrecht's utilization phase aligns with this concept. The key distinction from tempo runs: cruise intervals accumulate more total threshold-pace volume because the brief rests prevent premature fatigue. The short recovery (1 min per 5-6 min of running) is critical -- it must be short enough that lactate doesn't fully clear, training the clearance mechanism.

---

### 2.3 Progression Run

**Key**: `progression_run`

**Description**: Start at easy effort and progressively increase pace throughout the run. Three variations exist: (a) "Thirds" progression -- each third of the run is faster, (b) "Fast finish" -- easy for 75-80% then pick up to tempo/threshold for the final 20-25%, (c) "Race simulation" -- easy start, build to race pace for final 10-15%. This workout trains finishing speed on fatigued legs and teaches pacing discipline.

**Olbrecht Category**: Aerobic Capacity / Aerobic Utilization (depends on how fast the finish is)

**When Used**: Late capacity phase and utilization phase. The "thirds" version works during capacity (since most of the run is easy). The "fast finish" version is better for utilization.

**Structure**:
- **Short**: 35 min total: 25 min Zone 2 -> 10 min Zone 3
- **Medium**: 50 min total: 20 min Zone 2 -> 15 min Zone 3 -> 15 min Zone 4
- **Long**: 60 min total: 25 min Zone 2 -> 20 min Zone 3 -> 15 min Zone 4

**Target Zones**: HR Zones 2-4, Pace Zones 2-4

**TSS Range**: Short 35-45, Medium 55-70, Long 65-85

**Progressive Adjustment**: Extend the time spent at higher intensity (e.g., final Zone 4 segment grows from 10 min to 15 min to 20 min). Or shift the final segment from Zone 3 to Zone 4.

**Rationale**: Progression runs cause less overall fatigue than tempo runs while still providing a meaningful aerobic stimulus. They teach the body to recruit faster motor units on tired legs -- a critical race skill. From Olbrecht's perspective, the bulk of the run is still at low intensity (capacity development), with the fast finish providing a controlled utilization stimulus.

**Note**: This is DIFFERENT from `long_run_progression` (which has a Zone 4-5 finish on a long run) and `long_run_moderate` (Zone 3-4 finish on a long run). The progression run is a shorter, midweek workout that builds pace throughout.

---

### 2.4 Shakeout Run

**Key**: `shakeout_run`

**Description**: Very short, very easy run for neuromuscular activation before a race or hard workout. The purpose is to keep muscles loose, promote blood flow, and calm pre-race nerves. Always at Zone 1, always brief.

**Olbrecht Category**: Recovery / Activation

**When Used**: Race week (day before race, or race morning 2-2.5 hours before start). Can also be used before key workouts.

**Structure**:
- **Short**: 10-15 min @ Zone 1 (pre-race, marathon distance)
- **Medium**: 15-20 min @ Zone 1 (pre-race, half marathon or shorter)
- **Long**: 20-30 min @ Zone 1 with 2-4 strides (pre-race, 5K-10K)

**Target Zones**: HR Zone 1, Pace Zone 1

**TSS Range**: Short 5-10, Medium 10-15, Long 12-20

**Progressive Adjustment**: N/A -- this is a fixed-purpose workout. No progression.

**Rationale**: Research shows shakeout runs maintain neuromuscular fitness and promote circulation of oxygen-rich blood without creating fatigue. The goal is to feel loose and refreshed, not tired. Longer races get shorter shakeouts (marathon = 10 min), shorter races get longer shakeouts with strides (5K = 20-30 min with strides).

---

### 2.5 Time Trial / Benchmark

**Key**: `time_trial`

**Description**: All-out effort over a predetermined distance (typically 3K or 5K) to measure current fitness and establish/validate training zones. Serves as a "field test" for FTPace and LTHR calibration.

**Olbrecht Category**: Assessment (not a training stimulus per se, but generates data for the "steering" process)

**When Used**: At mesocycle boundaries (during recovery phase when athlete is fresh). After a race result. When the coach detects zones may need recalibration. Typically every 4-8 weeks.

**Structure**:
- **Short**: WU (15 min easy + 4 strides) + 3K all-out effort + CD (10 min easy) (~35 min total)
- **Medium**: WU (15 min easy + 4 strides) + 5K all-out effort + CD (10 min easy) (~45 min total)
- **Long**: WU (15 min easy + 4 strides) + 30-min time trial (best effort) + CD (10 min easy) (~55 min total)

**Target Zones**: HR Zones 4-6 (during effort), Pace Zone 4-5

**TSS Range**: Short 40-55, Medium 55-75, Long 65-85

**Progressive Adjustment**: N/A -- this is an assessment, not a progressive workout. The result determines future training paces.

**Rationale**: Olbrecht's entire "steering" methodology depends on periodic measurement. In his original work, this means lactate testing every 6 weeks. Without lactate testing, a time trial is the best proxy for recalibrating FTPace and LTHR. A 5K TT provides enough data to estimate both threshold pace (FTPace ~= 95% of 30-min TT average pace) and LTHR (average HR during last 20 minutes of a 30-min effort).

**Special handling**: After this workout, the system should prompt the athlete to confirm zone recalculation if the result suggests FTPace/LTHR has changed.

---

### 2.6 Form Drills Session

**Key**: `form_drills`

**Description**: Standalone session of running-specific drills (A-skips, B-skips, high knees, butt kicks, carioca, bounding) designed to improve running economy, neuromuscular coordination, and elastic recoil. Can be combined with a short easy run or performed as a standalone session.

**Olbrecht Category**: Neuromuscular / Running Economy (aligns with Running Rewired's spring mechanics and alignment domains)

**When Used**: All phases, especially during capacity phase. Best performed 1-2x per week, often as a pre-run warmup drill sequence or as a standalone session on easy days.

**Structure**:
- **Short**: 10 min easy jog + 15 min drill circuit (A-skip, B-skip, high knees, butt kicks, bounding -- 2x30m each) + 5 min easy jog (~30 min total)
- **Medium**: 15 min easy jog + 20 min drill circuit (full drill set 3x30m each + 4 strides) + 5 min easy jog (~40 min total)
- **Long**: 15 min easy jog + 20 min drill circuit + 4x100m strides + 10 min easy jog (~45 min total)

**Target Zones**: HR Zone 1-2 (jog portions), Pace Zone 5-6 (during drills/strides)

**TSS Range**: Short 15-25, Medium 25-35, Long 30-45

**Progressive Adjustment**: Progress from 2 sets of each drill to 3, add more dynamic/explosive drills (bounding, single-leg hops), add strides.

**Rationale**: Running drills target neuromuscular patterns, coordination, and elastic energy storage/return. A-skips strengthen glutes and hip flexors, B-skips dynamically stretch hamstrings and emphasize pawing motion, high knees develop hip flexor strength and midfoot strike. These are effectively "micro-dosed plyometrics" that improve running economy. This overlaps somewhat with Running Rewired's spring mechanics but is specifically running-movement focused rather than gym-based.

**Note**: This workout type bridges the gap between the Running Rewired strength program and running-specific neural activation. It is NOT a replacement for strength sessions.

---

### 2.7 Lactate Clearance Fartlek

**Key**: `lactate_clearance`

**Description**: A workout specifically designed to train the lactate shuttle system. Alternates between surges that elevate blood lactate and moderate-pace segments that train the body to use lactate as fuel. The key distinction from regular fartlek: the hard segments are shorter and more intense (pushing into Zone 5-6), and the "recovery" segments are run at moderate pace (Zone 3) rather than easy -- deliberately keeping lactate elevated during recovery to train clearance.

**Olbrecht Category**: Aerobic Utilization / Anaerobic Utilization (mixed)

**When Used**: Late utilization phase, 3-5 weeks before race. This is a sophisticated workout that requires a strong aerobic base.

**Structure**:
- **Short**: WU + 6x(30s hard Zone 5-6 + 90s moderate Zone 3) + CD (~35 min total)
- **Medium**: WU + 8x(40s hard Zone 5-6 + 2 min moderate Zone 3) + CD (~45 min total)
- **Long**: WU + 10x(45s hard Zone 5-6 + 2 min moderate Zone 3) + CD (~55 min total)

**Target Zones**: HR Zones 3-6 (mixed), Pace Zones 3-5

**TSS Range**: Short 45-55, Medium 55-70, Long 65-85

**Progressive Adjustment**: Increase number of reps, extend hard segment duration, or reduce the "recovery" segment (from 2 min to 90s). The key is that recovery segments stay at moderate pace, not easy.

**Rationale**: Olbrecht emphasizes that lactate is not a waste product but a fuel source for the aerobic system. Training at fluctuating intensities -- creating lactate then using it as fuel -- develops the lactate shuttle. The moderate-pace recovery (rather than easy) is critical: it keeps the aerobic system working hard to clear the lactate generated during the surge. This is a "create more lactate, then use it as fuel" cycle.

---

### 2.8 Mixed Energy System Session

**Key**: `mixed_energy`

**Description**: A workout that deliberately combines different energy system stimuli in a single session, following Olbrecht's capacity-building model. The session structure is: easy running mixed with short explosive bursts (30-35s) that activate fast-twitch fibers, followed by easy running that trains slow-twitch fibers to process the metabolic byproducts. This IS Olbrecht's signature "high and low" workout applied as a structured session.

**Olbrecht Category**: Aerobic Capacity (primary purpose is to build aerobic capacity through the high-and-low mechanism)

**When Used**: Capacity phase -- this is a PRIMARY capacity-building workout. Can be used 1-2x per week throughout the capacity phase.

**Structure**:
- **Short**: 30 min total: 60% easy (18 min Zone 1-2) + 40% as 6x30s hard (Zone 6-7) with 2.5 min easy between
- **Medium**: 40 min total: 60% easy (24 min Zone 1-2) + 40% as 8x30s hard (Zone 6-7) with 2.5 min easy between
- **Long**: 50 min total: 60% easy (30 min Zone 1-2) + 40% as 10x30s hard (Zone 6-7) with 2.5 min easy between

**Target Zones**: HR Zones 1-2 (base), Zones 6-7 (bursts), Pace Zones 1-2 (base), Zone 6 (bursts)

**TSS Range**: Short 30-45, Medium 45-60, Long 55-75

**Progressive Adjustment**: Add more 30s bursts (6 -> 8 -> 10), extend total session duration, increase burst intensity.

**Rationale**: This is a direct implementation of Olbrecht's "high and low" principle, which he describes as: "Run easy, and spice it up for efforts of 30-35 seconds at high intensity." The short explosive bursts activate mitochondria in fast-twitch fibers, while the extensive easy running develops slow-twitch fibers. The combination builds aerobic capacity across ALL fiber types simultaneously. The session should maintain at least 60% easy / max 40% hard time distribution. Classic VO2max intervals (e.g., 5x3 min) are actually "power/utilization" exercises in Olbrecht's framework -- this workout is the true capacity builder.

**Note**: This workout is conceptually similar to `aerobic_development` (which already exists) but is more structured and uses slightly longer bursts (30s vs 20s strides). The `aerobic_development` type represents the lighter version (strides at the end of an easy run), while `mixed_energy` represents a more deliberate capacity-building session with the bursts distributed throughout.

---

### 2.9 Plyometric Running Circuit

**Key**: `plyo_running`

**Description**: A hybrid session combining easy running with bodyweight plyometric exercises (box jumps, squat jumps, bounding, single-leg hops, depth jumps). Designed to develop elastic energy storage and return -- the "spring" in running. This is essentially Jay Dicharry's Chapter 9 ("Spring Mechanics") performed as a running-circuit format rather than a gym session.

**Olbrecht Category**: Neuromuscular / Elastic Recoil

**When Used**: Build phase (after 6-8 weeks of strength foundation). 1x per week. Replace one strength session rather than adding on top.

**Structure**:
- **Short**: 10 min easy jog + 3 rounds of (6 box jumps + 8 squat jumps + 4x30m bounding + 200m easy jog between) + 10 min easy jog (~35 min total)
- **Medium**: 10 min easy jog + 4 rounds of (8 box jumps + 10 squat jumps + 6x30m bounding + 2x30m single-leg hops + 200m easy jog between) + 10 min easy jog (~45 min total)
- **Long**: 15 min easy jog + 5 rounds of (full plyo circuit + 400m easy jog between) + 10 min easy jog (~55 min total)

**Target Zones**: HR Zones 1-3 (mixed), Pace Zone 1-2 (jog portions)

**TSS Range**: Short 25-35, Medium 35-50, Long 45-60

**Progressive Adjustment**: Increase rounds, add more challenging exercises (depth jumps, single-leg hops), increase jump height.

**Rationale**: Running efficiency depends heavily on elastic energy return (tendons acting as springs). Plyometric training improves rate of force development and elastic recoil. This overlaps with `strength_power` but is performed in a running context rather than a gym setting. The easy jog between rounds keeps the heart rate elevated and provides active recovery.

**Note**: The decision whether to include this as a workout type vs. keeping it under the existing `strength_power` umbrella needs discussion. Recommend keeping this as a separate type because it has a running component (contributes to mileage) whereas strength sessions do not.

---

## 3. Hill Sprint Specifics

The existing `anaerobic_hills` workout type covers 30s-1:15 hill repeats (anaerobic capacity). However, hill-based training encompasses three distinct categories that serve completely different physiological purposes. This section expands on hill sprint specifics that the current registry under-represents.

### 3.1 The Three Categories of Hill Work

| Category | Duration | Gradient | Energy System | Lactate | Recovery Need | Olbrecht Mapping |
|----------|----------|----------|---------------|---------|---------------|------------------|
| **Hill Sprints** (neuromuscular) | 8-12s | 6-10% | ATP-PCr | Negligible | Minimal (next day OK) | Outside framework (neural) |
| **Hill Repeats** (anaerobic capacity) | 20-90s | 4-7% | Glycolytic | Very high | High (48h+) | Anaerobic capacity |
| **Hill Intervals** (aerobic power) | 3-5 min | 3-5% | Oxidative | Moderate | High (48h+) | Aerobic power ("high") |

The current `anaerobic_hills` type covers Category B (hill repeats). Categories A and C are not explicitly represented.

### 3.2 Short Hill Sprints (8-12s) -- The Missing Neuromuscular Tool

**Why they matter**: Very short hill sprints (8-12 seconds at maximal effort) are primarily a **neuromuscular** stimulus, not a metabolic one. Energy for efforts under ~10 seconds comes almost entirely from the ATP-PCr (creatine phosphate) system, which:
- Does **not** produce lactate
- Replenishes fully within ~2 minutes of rest
- Allows these sprints to be done without accumulating metabolic fatigue

This means short hill sprints can be performed **the day before a hard workout** without compromising the next day's session. Both Brad Hudson and Renato Canova found that athletes often feel *more* energized the day after hill sprints due to enhanced neuromuscular activation.

**Coaching consensus (Hudson / Canova / Magness)**:
- **Brad Hudson**: 6-8 second hill sprints the day before hard training sessions. After several weeks of consistent work, athletes develop stronger ability to recruit all available muscle fibers -- reporting a feeling of "pop" in their legs.
- **Renato Canova**: 6-8 reps of 10-12 seconds, ~2 minutes recovery, after an easy run. Up to 2x per week. Enhances ground contact reactivity and fast-twitch fiber recruitment.
- **Steve Magness**: Hill sprints as stage 1 of a sprint progression: hill sprints -> flat sprints (60-100m) -> speed endurance (150-250m). Emphasizes teaching *relaxed* sprinting -- distance runners tend to "bear down and force it," which is counterproductive.

**Fiber recruitment mechanism**: Fast-twitch fibers recruited during hill sprints can, over time, develop the ability to capture elastic energy at footfall. When combined with lactate recycling capacity, these newly recruited fibers enhance running economy without increasing blood acidity. Hill sprints expand the "recruitable fiber pool" -- so when Olbrecht's high-intensity capacity work is performed later, more fibers are available to receive the aerobic training stimulus.

### 3.3 Why Hills Over Flat Sprints

Hill sprints offer critical biomechanical advantages over flat sprinting for distance runners:

1. **Reduced eccentric loading**: During uphill sprinting, the gait shifts from eccentric (braking) to concentric (pushing). A 2025 review in *Frontiers in Bioengineering* found that at steep grades (~15%), impact force peaks occasionally **disappeared entirely**, while propulsive peaks increased ~75%.

2. **Lower injury risk**: Hamstring strains in flat sprinting occur because the hamstring must decelerate the leg at high velocities during late swing phase. Uphill sprinting mechanically limits top-end velocity, so the hamstring never reaches the dangerous lengthened-under-load position. Steve Magness describes them as "almost impossible to get hurt doing."

3. **Natural form enforcement**: The incline enforces proper hip extension mechanics, high knee drive, and midfoot/forefoot strike. It is "almost impossible to sprint wrong while doing them."

4. **Muscle activation increases**: Hip power contribution increases from 28% (level) to 36% (uphill). Vastus group activity +23%, soleus +14%, with general increases across glutes, hamstrings, and calves.

5. **Running-specific strength**: Jason Fitzgerald characterizes them as "just like heavy weight lifting except they're sport-specific" -- strengthening muscles, tendons, and connective tissue without gym equipment.

### 3.4 Gradient Recommendations

| Gradient | Purpose | Best For |
|----------|---------|----------|
| 3-4% | Sprint mechanics | Preserving sprint-like gait; speed emphasis over strength |
| 6-8% | General neuromuscular | Starting point for most runners; good power/cadence compromise |
| 8-10% | Power + strength | "Sweet spot" for experienced runners; primary development range |
| 10-15% | Maximum strength | Advanced progression; significant muscular loading |
| >15% | Diminishing returns | Gait shifts to "climbing" pattern; poor transfer to running |

**Key principle**: Gradient inversely correlates with duration. Steep (6-10%) for short neuromuscular sprints, moderate (4-7%) for anaerobic repeats, gentle (3-5%) for VO2max hill intervals.

**Caution**: Gradients above 6-7% place significant load on hamstrings, glutes, and associated tendons. Introduce steep grades gradually over 3-4 weeks. Above ~15%, the running gait fundamentally changes into climbing -- minimal transfer to flat-ground mechanics.

### 3.5 Progressive Programming

Hill sprint progression is **conservative and slow-building**. The consensus is emphatic: there is little benefit to more than 8-10 sprints, and increasing volume too rapidly raises injury risk dramatically.

**Progression order** (progress one variable at a time):
1. **Reps** (first): 2 -> 4 -> 6 -> 8 -> 10 (add 1-2 per week)
2. **Duration** (second): 8s -> 10s -> 12s (never exceed 12s for pure neuromuscular work)
3. **Gradient** (third): 6% -> 8% -> 10% -> 12%
4. **Recovery**: Always stays full -- 2-3 minutes minimum (walk back)

**Sample 8-week introduction**:

| Week | Reps | Duration | Gradient | Total Sprint Time |
|------|------|----------|----------|-------------------|
| 1 | 2 | 8s | 6% | 16s |
| 2 | 4 | 8s | 6% | 32s |
| 3 | 6 | 8s | 6% | 48s |
| 4 | 8 | 8s | 6% | 64s |
| 5 | 8 | 10s | 6% | 80s |
| 6 | 10 | 10s | 6% | 100s |
| 7 | 10 | 10s | 8% | 100s |
| 8 | 10 | 12s | 8% | 120s |

**Session placement**: After easy/maintenance runs (not before). 1-2x per week. Can be done the day before a hard workout (Hudson/Canova protocol). Never on the same day as other high-intensity work.

### 3.6 Hill Sprints in Olbrecht's Framework

Short hill sprints (8-12s) don't fit neatly into Olbrecht's "high and low" model:

- **Not "high" enough for aerobic stimulus**: At 8-12 seconds, VO2 barely rises. Olbrecht's prescribed 30-35 second bursts allow significant oxygen uptake. Hill sprints are purely ATP-PCr work.
- **Don't meaningfully tax anaerobic capacity**: With full recovery, the glycolytic system is barely touched.
- **Neural, not metabolic**: The primary adaptation is neuromuscular (fiber recruitment, rate coding, motor unit synchronization), not the mitochondrial/enzymatic adaptations Olbrecht targets.

**The synthesis**: Short hill sprints serve a **complementary "third dimension"** to Olbrecht's framework. They develop the neural infrastructure (fiber recruitment, coordination) that makes both "high" and "low" training more effective by expanding the recruitable fiber pool. They should **not** count toward the 10-15% high-intensity budget because they don't produce the metabolic stress that Olbrecht's model tracks via lactate.

**Practical classification for CoachJan**: Treat hill sprints as a **neuromuscular maintenance stimulus** -- prescribed alongside easy runs, not competing with intensity sessions for scheduling space.

### 3.7 Macrocycle Placement

| Macrocycle Phase | Hill Sprint Role | Volume |
|------------------|-----------------|--------|
| Base/General Prep | Primary introduction and progression period | Build from 2 -> 10 reps |
| Specific Prep | Transition to flat sprints / speed endurance (Magness progression) | Maintain 6-8 reps or replace |
| Competition | Maintain with reduced volume | 4-6 reps |
| Taper | Optional neural activation (day before race) | 2-4 reps |

### 3.8 Implementation Recommendation

**Option A (Recommended)**: Add a new workout type `hill_sprints` (8-12s, neuromuscular) distinct from `anaerobic_hills` (30s-1:15, glycolytic). This reflects the fundamentally different energy systems, recovery profiles, and scheduling logic.

**Option B**: Expand `anaerobic_hills` to cover the full spectrum. Problem: the scheduling logic differs -- hill sprints can go the day before intensity while anaerobic hills cannot. Lumping them obscures this critical distinction.

**If Option A**:

| Field | Value |
|-------|-------|
| Key | `hill_sprints` |
| Category | Neuromuscular / Neural Activation |
| Short | WU (10 min easy) + 4x8s max hill sprint / walk back (~2 min) + CD (5 min easy) (~25 min) |
| Medium | WU (10 min easy) + 6x10s max hill sprint / walk back (~2 min) + CD (5 min easy) (~30 min) |
| Long | WU (10 min easy) + 8-10x10s max hill sprint / walk back (~2 min) + CD (5 min easy) (~40 min) |
| Target Zones | HR Zone 6-7 (during sprint), Pace Zone 6 (during sprint) |
| TSS Range | Short 10-15, Medium 15-25, Long 20-30 |

Note the very low TSS -- this is intentional. Hill sprints produce minimal training load because the total sprint time is only 32-120 seconds per session.

---

## 4. Under/Over Workout Variations (Detailed)

The existing `under_over` workout type needs to be expanded into multiple sub-protocols. Here are the researched variations:

### 4.1 Billat 30/30 (VO2max Protocol)

**Current registry**: Already partially captured in `under_over` (short duration).

**Research findings**: Veronique Billat's research shows that alternating 30s at vVO2max pace with 30s at 50% vVO2max allows runners to accumulate 3x more time at VO2max pace than continuous running. Some runners amassed 18+ minutes at VO2max, with 5 of 8 subjects reaching VO2max with blood lactate below 4mM.

**Protocol**:
- Hard segment: 30s @ vVO2max (roughly Pace Zone 5, HR Zone 5-6)
- Easy segment: 30s @ 50% vVO2max (roughly Pace Zone 1, HR Zone 2)
- Volume: Start with 2x10 reps (with 3 min rest between sets), progress to 3x10 or 2x15
- Total hard time: 10-15 minutes

**Caveat**: Research suggests that once athletes are well-trained, 30/30s may only keep them at threshold rather than VO2max. At that point, longer intervals (3-5 min) or the 15/15 protocol may be more effective.

### 4.2 Billat 15/15 (Short VO2max Protocol)

**Research findings**: The 15/15 protocol (15s @ 90-100% vVO2max / 15s @ 70-80% vVO2max) allows middle-aged runners to maintain VO2max for 14 minutes. It appears to be MORE effective than 30/30 at sustaining time at VO2max.

**Protocol**:
- Hard segment: 15s @ 90-100% vVO2max
- Easy segment: 15s @ 70-80% vVO2max
- Volume: Sets of 20-30 reps (total 10-15 min of alternating), 2-3 sets with 3 min rest between
- This is more accessible for beginners and masters runners

### 4.3 60/60 Intervals

**Protocol**:
- Hard segment: 60s @ Zone 5 (95-100% vVO2max)
- Easy segment: 60s @ Zone 1-2 (50-60% vVO2max)
- Volume: Start with 10 reps, progress to 15-20
- This provides longer sustained efforts than 30/30, allowing HR to reach higher levels

### 4.4 Under/Over Threshold

**Research findings**: Alternates between 95% and 105% of threshold pace/effort. The "over" segments create lactate accumulation, and the "under" segments train lactate clearance at near-threshold effort.

**Protocol**:
- Over segment: 2-3 min @ 105% FTPace (Zone 4-5, ~10 sec/mile faster than threshold)
- Under segment: 2-3 min @ 95% FTPace (Zone 3-4, ~10 sec/mile slower than threshold)
- Volume: 3-5 complete cycles (18-30 min of work)
- Total session with warmup/cooldown: 45-65 min

### 4.5 Recommended Registry Changes

Rather than having a single `under_over` type that covers all variations, I recommend **keeping the single `under_over` type but enhancing the duration categories** to represent the different protocols:

| Duration Category | Protocol | Structure |
|-------------------|----------|-----------|
| Short | 30/30 Billat | WU + 2x(10x30s Zone 5-6 / 30s Zone 2, 3 min rest) + CD |
| Medium | 60/60 | WU + 12-15x(60s Zone 5 / 60s Zone 2) + CD |
| Long | Under/Over Threshold | WU + 4x(3 min @ 105% FTP / 3 min @ 95% FTP) + CD |

**Rationale**: The short/medium/long categories naturally map to increasing duration AND complexity. The 30/30 is the shortest and most accessible. The 60/60 is moderate duration. The under/over threshold is the longest and most demanding, appropriate for the utilization phase.

**Alternative**: If we want more granularity, we could split into:
- `under_over_3030` -- Billat 30/30 protocol
- `under_over_6060` -- 60/60 intervals
- `under_over_threshold` -- Under/over threshold protocol

This would add 2 new types but give the AI more precise selection. **Recommendation: Keep as one type for now, split later if Claude needs finer control.**

---

## 5. Warmup/Cooldown as Distances

### 5.1 General Framework

Warmup and cooldown distances should scale based on three factors:
1. **Athlete level** (primary factor)
2. **Workout type** (longer warmup before higher-intensity work)
3. **Weekly mileage context** (warmup/cooldown contribute to mileage goals)

### 5.2 Warmup Distance by Athlete Level

| Level | Base Warmup | Before Intensity Session | Before Long Run | Before Easy Run |
|-------|-------------|-------------------------|-----------------|-----------------|
| Beginner (<30 km/week) | 1.0 km (8-10 min) | 1.5 km (12-15 min) | 0.5 km (5 min) | N/A (run starts easy) |
| Intermediate (30-60 km/week) | 1.5 km (10-12 min) | 2.0 km (15-18 min) | 1.0 km (8 min) | N/A |
| Advanced (60+ km/week) | 2.0 km (12-15 min) | 2.5-3.0 km (18-22 min) | 1.5 km (10 min) | N/A |

**Notes**:
- Easy runs and recovery runs do NOT have a formal warmup -- the run itself starts easy and the first km serves as warmup
- Long runs start easy by definition; a brief warmup is optional
- Intensity sessions (VO2max, track, tempo, under/over, race-specific) ALWAYS have a dedicated warmup
- Warmup pace: Zone 1-2 with optional 2-4 strides in the final 2 minutes

### 5.3 Cooldown Distance by Athlete Level

| Level | Base Cooldown | After Intensity Session | After Long Run | After Easy Run |
|-------|---------------|------------------------|----------------|----------------|
| Beginner | 0.5 km (5 min) | 1.0 km (8-10 min) | N/A (run ends easy) | N/A |
| Intermediate | 1.0 km (8 min) | 1.5 km (10-12 min) | N/A | N/A |
| Advanced | 1.5 km (10 min) | 2.0 km (12-15 min) | N/A | N/A |

**Key principle**: Cooldowns are generally shorter than warmups (roughly 60-75% of warmup distance). The exception is after very hard anaerobic sessions (track 200m, anaerobic flat/hills), where a slightly longer cooldown aids lactate clearance.

### 5.4 Warmup/Cooldown by Workout Type

| Workout Type | Has Dedicated WU/CD? | WU Modifier | CD Modifier | Notes |
|--------------|---------------------|-------------|-------------|-------|
| easy_run | No | N/A | N/A | First/last km IS warmup/cooldown |
| long_run | Optional | -0.5 km | N/A | Brief optional jog to loosen up |
| long_run_progression | Optional | -0.5 km | N/A | Run ends fast; no formal CD |
| long_run_moderate | Optional | -0.5 km | N/A | Similar to long_run |
| aerobic_development | No | N/A | N/A | Run starts easy; strides at end |
| moderate_run | No | N/A | N/A | Run starts easy |
| steady_run | Yes | +0 | +0 | Standard WU/CD |
| progression_run | No | N/A | N/A | Run starts easy; ends fast |
| tempo_run | Yes | +0 | +0 | Standard WU/CD |
| cruise_intervals | Yes | +0 | +0 | Standard WU/CD |
| vo2max_intervals | Yes | +0.5 km | +0 | Longer WU before hard intervals |
| under_over | Yes | +0.5 km | +0 | Longer WU before hard work |
| track_200m | Yes | +0.5 km | +0.5 km | Full WU with strides; longer CD |
| track_400m | Yes | +0.5 km | +0.5 km | Full WU with strides; longer CD |
| track_800m | Yes | +0.5 km | +0 | Full WU with strides |
| anaerobic_hills | Yes | +0.5 km | +0.5 km | Thorough WU to prevent injury |
| anaerobic_flat | Yes | +0.5 km | +0.5 km | Thorough WU |
| anaerobic_power | Yes | +0 | +0 | Standard WU/CD |
| race_specific | Yes | +0.5 km | +0 | Race-rehearsal WU |
| fartlek_structured | Yes | +0 | +0 | Standard WU/CD |
| lactate_clearance | Yes | +0 | +0 | Standard WU/CD |
| mixed_energy | No | N/A | N/A | Starts easy; bursts distributed |
| time_trial | Yes | +0.5 km | +0 | Extended WU with strides |
| form_drills | No | N/A | N/A | Jog is part of workout |
| shakeout_run | No | N/A | N/A | Entire run is easy |
| recovery_run | No | N/A | N/A | Entire run is easy |

### 5.5 Warmup/Cooldown Contribution to Weekly Mileage

**All warmup and cooldown distance counts toward weekly mileage.** This is consistent with coaching practice -- every running step contributes to training load and mileage.

For a typical intermediate athlete doing 3 intensity sessions per week:
- 3 warmups x 2.0 km = 6.0 km
- 3 cooldowns x 1.5 km = 4.5 km
- Total WU/CD contribution: ~10.5 km/week

This means for a 50 km/week athlete, about 20% of their mileage comes from warmup/cooldown. The plan generator must account for this when setting weekly volume targets.

### 5.6 Implementation Approach

Add warmup/cooldown fields to the `WorkoutTemplate` struct:

```
struct WarmupCooldown {
    warmup_km_beginner: f64,
    warmup_km_intermediate: f64,
    warmup_km_advanced: f64,
    cooldown_km_beginner: f64,
    cooldown_km_intermediate: f64,
    cooldown_km_advanced: f64,
    includes_strides: bool,  // true for intensity sessions
}
```

The existing `total_duration_min` in `DurationParams` should be renamed or supplemented with `main_set_duration_min` since warmup/cooldown are now separate. Or we can keep `total_duration_min` as an estimate that includes warmup/cooldown.

---

## 6. Progressive Adjustment Patterns

### 6.1 Olbrecht's Mesocycle Progression Model

Olbrecht's progression within a mesocycle follows a "staircase" pattern:

**Load weeks**: Each week sees a slight increase in overall training stress (TSS), achieved by:
1. More or longer intervals (primary progression lever)
2. Slightly increased total session time
3. Maintained or slightly increased intensity

**Recovery weeks**: Volume drops 30-50%, intensity maintained for 1-2 sessions.

### 6.2 Parameters That Change Week-Over-Week

| Parameter | How It Changes | Example |
|-----------|---------------|---------|
| **Number of reps** | +1-2 reps per week | 4x3min -> 5x3min -> 6x3min |
| **Interval duration** | +30s-1min per week | 4x2min -> 4x2.5min -> 4x3min |
| **Total work volume** | Combined effect of reps x duration | 12 min -> 15 min -> 18 min total work |
| **Recovery duration** | Can decrease (harder) | 2.5 min jog -> 2 min jog -> 1.5 min jog |
| **Intensity (pace/HR)** | Generally stays constant within a mesocycle | Stay at Zone 5 throughout |
| **Session total time** | Increases with more/longer intervals | 45 min -> 50 min -> 55 min |

### 6.3 Duration Category as Progression Vehicle

Our Short/Medium/Long duration categories can serve as the week-over-week progression vehicle within a mesocycle:

**3-week load pattern (advanced athlete)**:
- Week 1: vo2max_intervals / short (4x2min)
- Week 2: vo2max_intervals / medium (5x2min or 4x3min)
- Week 3: vo2max_intervals / long (6x3min)

**2-week load pattern (intermediate athlete)**:
- Week 1: tempo_run / short (15 min threshold)
- Week 2: tempo_run / medium (2x12 min threshold)

**1-week load pattern (beginner athlete)**:
- Week 1: easy_run / medium

This means Claude's job is to assign progressively larger duration categories week-over-week for the same workout type.

### 6.4 Progression Patterns by Workout Type

#### Easy/Aerobic Runs
| Week | Progression | What Changes |
|------|-------------|--------------|
| 1 | easy_run / short (30 min) | Base duration |
| 2 | easy_run / medium (45 min) | +15 min |
| 3 | easy_run / long (60 min) | +15 min |

#### VO2max Intervals
| Week | Progression | What Changes |
|------|-------------|--------------|
| 1 | 5x1 min @ Z5 / 2 min jog (short) | Base reps and duration |
| 2 | 5x2 min @ Z5 / 2 min jog (medium) | +1 min per interval |
| 3 | 6x3 min @ Z5 / 2.5 min jog (long) | +1 rep and +1 min per interval |

#### Tempo Run
| Week | Progression | What Changes |
|------|-------------|--------------|
| 1 | 15 min sustained @ Z3-4 (short) | Base duration |
| 2 | 2x12 min @ Z3-4 / 3 min easy (medium) | More total threshold time, broken into repeats |
| 3 | 25 min sustained @ Z3-4 (long) | Continuous long threshold |

#### Track Work (200m, 400m, 800m)
| Week | Progression | What Changes |
|------|-------------|--------------|
| 1 | 6x200m / 200m walk (short) | Base reps |
| 2 | 8x200m / 200m walk (medium) | +2 reps |
| 3 | 10x200m / 200m walk (long) | +2 reps |

#### Under/Over
| Week | Progression | What Changes |
|------|-------------|--------------|
| 1 | 15x30/30 (short) | 30/30 protocol, base reps |
| 2 | 20x30/30 (medium) | +5 reps |
| 3 | 15x60/60 (long) | Longer intervals |

#### Anaerobic Work
| Week | Progression | What Changes |
|------|-------------|--------------|
| 1 | 6x30s / 2.5 min walk (short) | Base reps |
| 2 | 8x30s / 2.5 min walk (medium) | +2 reps |
| 3 | 10x45s / 2.5 min walk (long) | +2 reps and +15s per rep |

### 6.5 Recovery Interval Manipulation

Recovery interval length is a powerful training variable:

| Recovery Style | Duration | Effect | When to Use |
|----------------|----------|--------|-------------|
| Full recovery | 3-5 min or work:rest 1:3+ | More anaerobic, each rep is near-max quality | Anaerobic capacity phase, sprint work |
| Standard recovery | 2-3 min or work:rest 1:1.5 | Balanced aerobic/anaerobic | VO2max intervals, tempo |
| Short recovery | 60-90s or work:rest 1:0.5 | More aerobic, lactate accumulates | Cruise intervals, utilization phase |
| Active recovery (moderate pace) | Same duration, moderate effort | Lactate clearance training | Lactate clearance fartlek |

**Within a mesocycle**: Recovery can shorten week-over-week as a progression lever, but this is secondary to adding reps/duration. Shortening recovery makes the workout more aerobic but also more fatiguing.

### 6.6 Implementation Approach

The current `DurationCategory` (Short/Medium/Long) already provides the primary progression mechanism. To enable week-over-week progression, Claude assigns:
- Week 1 of load: workout_type / short
- Week 2 of load: workout_type / medium
- Week 3 of load: workout_type / long (advanced only)
- Recovery week: either workout_type / short or different (easier) workout type

For more granular progression (e.g., 4x2min -> 5x2min instead of jumping to the next duration category), we could introduce **interpolated parameters**. But this adds significant complexity. **Recommendation: Start with Short/Medium/Long as the progression mechanism, and only add interpolation if 3 discrete levels prove insufficient.**

---

## 7. Athlete Limitation-Based Prescription

### 7.1 Olbrecht's Classification Framework

Olbrecht classifies athletes along two independent axes:

**Aerobic Capacity (AEC)**: How much oxygen the body can use (VO2max equivalent)
**Anaerobic Capacity (ANC)**: How much lactate the body can produce (VLamax equivalent)

For distance runners, the ideal profile varies by race distance:

| Race Distance | Ideal AEC | Ideal ANC | Notes |
|---------------|-----------|-----------|-------|
| 5K | Very High | Moderate | Need anaerobic kick but still primarily aerobic |
| 10K | Very High | Moderate-Low | More aerobic, less tolerance for high lactate |
| Half Marathon | Very High | Low | Almost entirely aerobic |
| Marathon | Very High | Very Low | Glycolytic capacity must be suppressed |
| Ultra | High | Very Low | Economy and fat oxidation paramount |

### 7.2 Aerobically Limited Athletes

**Characteristics**:
- Low VO2max relative to their training age
- Poor aerobic endurance despite training volume
- Lactate curve shifted left (reach threshold at low intensities)
- Often undertrained in easy volume or over-trained at threshold

**Training Prescription**:

| Prescription Element | Approach |
|---------------------|----------|
| Volume | Increase gradually -- aerobic capacity responds to volume, but too much can "break" a weak aerobic engine |
| Easy running | Primary emphasis. 85-90% of volume in Zones 1-2 |
| "High and Low" sessions | 1-2x per week. Short bursts (30-35s) at high intensity within easy runs to activate fast-twitch mitochondria |
| Threshold work | AVOID during capacity phase. Threshold training trades capacity for power -- counterproductive for aerobically limited athletes |
| Anaerobic work | Small doses (1x per 2-3 weeks) to maintain minimum glycolytic capacity |
| Long runs | Progressively extend. The long run is the single best workout for aerobic capacity |
| Recovery | Emphasize. Aerobic adaptations happen during recovery |

**Workout type distribution (aerobically limited, capacity phase)**:

| Workout Type | Frequency | Purpose |
|--------------|-----------|---------|
| easy_run | 3-4x/week | Foundation aerobic development |
| long_run | 1x/week | Primary capacity builder |
| aerobic_development | 1x/week | Strides to activate fast-twitch fibers |
| mixed_energy | 1x/week | Olbrecht's "high and low" for fast-twitch mitochondria |
| recovery_run | 1-2x/week | Active recovery |
| tempo_run | 0x/week | AVOIDED during capacity phase |
| vo2max_intervals | 0x/week | AVOIDED during capacity phase |

### 7.3 Anaerobically Limited Athletes

**Characteristics** (rare in distance runners):
- Good aerobic base but poor kick / surge ability
- Can hold steady pace but cannot accelerate
- Low VLamax -- glycolytic system is weak
- Lactate curve shifted far right (high threshold but limited above-threshold capacity)

**Training Prescription**:

| Prescription Element | Approach |
|---------------------|----------|
| Volume | Maintain current level. Adding more easy volume will further suppress ANC |
| Easy running | Sufficient to maintain aerobic capacity but not excessive |
| Sprint/speed work | 2x/week during capacity phase. 10-30s all-out sprints with FULL recovery (2:1 to 3:1 rest) |
| Anaerobic intervals | 1x/week. Short explosive efforts that stimulate glycolytic enzymes |
| Threshold work | Can be introduced earlier than for aerobically limited athletes |
| Long runs | Maintain but do not increase beyond current level |

**Workout type distribution (anaerobically limited, capacity phase)**:

| Workout Type | Frequency | Purpose |
|--------------|-----------|---------|
| easy_run | 2-3x/week | Maintain aerobic base |
| long_run | 1x/week | Maintain (not extend) |
| anaerobic_flat or anaerobic_hills | 2x/week | Build glycolytic capacity |
| track_200m or track_400m | 1x/week | Speed and neuromuscular power |
| aerobic_development | 1x/week | Balanced development |
| recovery_run | 1x/week | Active recovery |

### 7.4 Balanced Athletes

Most well-trained distance runners fall into this category -- adequate AEC and ANC for their race distance.

**Training Prescription**: Follow the standard Olbrecht periodization:
- Capacity phase: 85-90% easy + high-and-low sessions
- Utilization phase: Introduce threshold and race-pace work
- Taper: Reduce volume, maintain sharpness

### 7.5 How to Classify Without Lactate Testing

Without lactate testing (which is out of scope for MVP), we can approximate classification using:

1. **Race performance vs. training volume**: If an athlete trains high volume but races poorly, likely aerobically limited (poor economy or genuine low AEC)
2. **Pace-HR relationship**: High HR at easy pace suggests low aerobic capacity
3. **Race distance performance comparison**: If the athlete's 5K performance far exceeds their half-marathon performance (more than expected from standard race equivalency tables), they may have high ANC / low AEC
4. **Training history**: Runners who have done mostly tempo/threshold work may have overdeveloped their utilization at the expense of capacity
5. **Sports background**: Former sprinters/team sport athletes (soccer, basketball) often have high ANC and need to develop AEC. Former swimmers/cyclists often have good AEC.

**For the MVP**: Default to "balanced" unless the athlete's profile data strongly suggests one direction. The AI coach can note observations in commentary but should not aggressively classify without data.

### 7.6 Workout Selection Matrix by Limitation Type

| Workout Type | Aerobically Limited | Anaerobically Limited | Balanced |
|--------------|--------------------|-----------------------|----------|
| easy_run | HIGH priority | Standard | Standard |
| long_run | HIGH priority | Maintain | Standard |
| long_run_progression | Late capacity/util | Moderate | Standard |
| long_run_moderate | Late capacity | Moderate | Standard |
| aerobic_development | HIGH priority | Standard | Standard |
| moderate_run | Standard | Standard | Standard |
| steady_run | AVOID (capacity phase) | Standard | Standard |
| tempo_run | AVOID (capacity phase) | Earlier introduction | Util phase only |
| cruise_intervals | AVOID (capacity phase) | Earlier introduction | Util phase only |
| vo2max_intervals | AVOID (capacity phase) | Standard | Util phase only |
| under_over | AVOID (capacity phase) | Standard | Util phase only |
| track_200m | LOW priority | HIGH priority | Standard |
| track_400m | LOW priority | HIGH priority | Standard |
| track_800m | LOW priority | Moderate | Standard |
| anaerobic_hills | LOW priority | HIGH priority | Standard |
| anaerobic_flat | LOW priority | HIGH priority | Standard |
| anaerobic_power | AVOID | Moderate | Standard |
| race_specific | Util phase only | Standard | Util phase only |
| mixed_energy | HIGH priority | Standard | Standard |
| fartlek_structured | Late capacity/util | Standard | Standard |
| lactate_clearance | Util phase only | Util phase only | Util phase only |
| progression_run | Late capacity | Standard | Standard |
| recovery_run | Standard | Standard | Standard |
| shakeout_run | Race week | Race week | Race week |
| time_trial | Mesocycle boundaries | Mesocycle boundaries | Mesocycle boundaries |
| form_drills | Standard | Standard | Standard |
| hill_sprints | Standard (neural) | Standard (neural) | Standard (neural) |

**Note**: `hill_sprints` is marked "Standard" for all limitation types because it serves a neuromuscular purpose orthogonal to the aerobic/anaerobic axis. It does not compete with intensity sessions and can be prescribed alongside easy runs in any phase.

---

## 8. Implementation Recommendations

### 8.1 New Workout Types to Add (Prioritized)

**Must Add (High Value, Clear Olbrecht Alignment)**:

1. **`fartlek_structured`** -- Fills a major gap. The continuous-effort format is fundamentally different from stop-start track intervals. Essential for utilization phase.

2. **`cruise_intervals`** -- Key utilization workout. More sustainable than sustained tempo for accumulating threshold volume. Critical for the 2-6 weeks before race.

3. **`progression_run`** -- Versatile midweek workout. Less fatiguing than tempo but still provides utilization stimulus. Bridges the gap between easy running and structured intervals.

4. **`shakeout_run`** -- Essential for race week. Currently no way to prescribe a pre-race activation run shorter than a recovery_run.

5. **`time_trial`** -- Required for zone recalibration (Workflow 3a). Without this, there is no structured way to reassess FTPace.

6. **`mixed_energy`** -- This IS Olbrecht's "high and low" signature workout, which is the single most important capacity-building format in his system. Currently, `aerobic_development` only partially captures this.

7. **`hill_sprints`** -- Short (8-12s) maximal hill sprints for neuromuscular development. Distinct from `anaerobic_hills` (30s-1:15, glycolytic). Different energy system (ATP-PCr vs glycolytic), different recovery profile (can precede hard workouts), different scheduling logic. See Section 3 for full details.

**Should Add (Good Value, Fills Gaps)**:

8. **`lactate_clearance`** -- Specialized utilization workout. Unique because recovery is at moderate pace, not easy. Could wait for v2.

9. **`form_drills`** -- Valuable for economy improvement, bridges strength and running. Could be handled as a variant of `strength_precision` initially.

**Consider Later (Nice to Have)**:

10. **`plyo_running`** -- Hybrid workout. Could be handled by existing `strength_power` type.

### 8.2 Updates to Existing Types

1. **`under_over`**: Refine the duration category descriptions to clearly map to 30/30, 60/60, and under/over threshold protocols (see Section 4.5).

2. **`aerobic_development`**: Keep as-is. It represents the lighter "strides at end of easy run" version. The new `mixed_energy` type handles the more structured Olbrecht "high and low" session.

### 8.3 Warmup/Cooldown Implementation

Add warmup/cooldown distance fields to `WorkoutTemplate` (see Section 5.6). This is a structural change to the registry that affects all workout types.

**Phase this in**: First add the fields and data, then update the plan generation to use distances instead of minutes for warmup/cooldown.

### 8.4 Progressive Adjustment Implementation

The Short/Medium/Long duration categories already provide the core progression mechanism. Claude should assign ascending duration categories week-over-week during load phases.

**No new data structures needed initially.** The progression logic lives in Claude's plan generation (it already knows mesocycle week numbers and should assign harder variants in later weeks).

**Future enhancement**: Add interpolated parameters for finer-grained progression (e.g., 4.5x3min as a midpoint between Short and Medium). This is only needed if the 3-level system proves too coarse.

---

## 9. Sources

### Primary

- [Jan Olbrecht, "The Science of Winning"](https://www.amazon.com/Science-Winning-Planning-Periodizing-Optimizing-ebook/dp/B009JTJ676) -- The canonical text
- [Scientific Triathlon Podcast #198 with Jan Olbrecht](https://scientifictriathlon.com/tts198/) -- Detailed interview covering capacity vs. power, mesocycle structure, "high and low" training, and athlete classification
- [Lactate.com - Steering](https://www.lactate.com/steering.html) -- Olbrecht's steering methodology
- [SDX Training - High and Low: A New Vision on Training](https://www.sdxtraining.com/articles/high-and-low-a-new-vision-on-training) -- Detailed explanation of the "high and low" approach

### Hill Sprints and Hill Training

- [A review of uphill and downhill running: biomechanics, physiology and modulating factors (PMC, 2025)](https://pmc.ncbi.nlm.nih.gov/articles/PMC12592170/) -- Ground reaction forces, muscle activation, gait changes on inclines
- [Sprint Training Part 2 -- Science of Running (Steve Magness)](https://www.scienceofrunning.com/2009/05/sprint-training-part-2.html?v=47e5dceea252) -- Sprint progression for distance runners: hills -> flat -> speed endurance
- [Hill Sprints for Injury Prevention and Speed -- Strength Running](https://strengthrunning.com/2014/08/hill-sprints-injury-prevention-speed/) -- Running-specific strength, safety profile, programming
- [Hill Sprints for Distance Runners -- Kenya Experience](https://www.traininkenya.com/2018/09/28/hill-sprints-for-distance-runners/) -- Kenyan approach to hill sprints, Canova protocol
- [Hill Sprints for Novice Distance Runners -- Runners Connect](https://runnersconnect.net/sprint-training-program-for-novice-runners/) -- Conservative introduction protocol (2 reps -> 10 reps over 8 weeks)
- [Hills: Incline and Interval Workouts -- Sifuentes Coaching](https://sifuentescoaching.com/resources/hills-inclinesandintervals) -- Gradient recommendations by workout type
- [What is the best gradient for hill reps? -- Tri Training Harder](https://tritrainingharder.com/blog/2022/06/what-is-the-best-gradient-for-hill-reps) -- Gradient taxonomy (3-15%)
- [Short Hill Repeats: Speed Endurance -- Training 4 Endurance](https://training4endurance.co.uk/short-hill-repeats-speed-endurance/) -- Category B hill repeats (20-90s)
- [Long Hill Repeats: Improve VO2max -- Training 4 Endurance](https://training4endurance.co.uk/long-hill-vo2max-running-intervals/) -- Category C hill intervals (3-5 min)
- [Hill Sprints -- Canute's Running Site](https://canute1.wordpress.com/2016/12/03/hill-sprints/) -- Fiber recruitment mechanism and elastic energy capture

### Billat 30/30 and 15/15 Research

- [Billat et al. (1999) - Interval training at VO2max](https://pubmed.ncbi.nlm.nih.gov/9927024/) -- Original Billat 30/30 study
- [Billat et al. (2000) - Intermittent runs at vVO2max](https://pubmed.ncbi.nlm.nih.gov/10638376/) -- 30/30 allows 3x more time at VO2max vs. continuous
- [Billat et al. (2001) - Very Short 15/15 Interval Training](https://pubmed.ncbi.nlm.nih.gov/11354523/) -- 15/15 protocol for sustained VO2max
- [GearJunkie - How to Boost VO2max: Billat 30-30](https://gearjunkie.com/endurance/running/how-to-boost-v02-max-billat-30-30-interval-workout) -- Practical application guide

### Under/Over Intervals

- [INSCYD - Over-Under Intervals](https://inscyd.com/article/over-under-intervals/) -- Benefits, structure, and common mistakes
- [TrainerRoad - Over-Under Intervals](https://www.trainerroad.com/blog/over-under-intervals-the-science-behind-them-and-tips-for-success/) -- Science and practical tips
- [Outside Online - Over/Under Intervals workout](https://run.outsideonline.com/training/workouts/workout-of-the-week-over-under-intervals/) -- Specific running protocol

### Cruise Intervals and Threshold Training

- [Running Writings - Getting the interval workout recovery right](https://runningwritings.com/2024/05/getting-interval-workout-recovery-right.html) -- Recovery intervals and lactate clearance
- [VDO2 - Training Definitions](https://vdoto2.com/learn-more/training-definitions) -- Jack Daniels' cruise interval definition
- [PMC - Lactate-Guided Threshold Interval Training](https://pmc.ncbi.nlm.nih.gov/articles/PMC10000870/) -- Research on Norwegian-style threshold intervals

### Progression Runs

- [McMillan Running - Start Slow Finish Fast](https://www.mcmillanrunning.com/start-slow-finish-fast-how-three-types-of-progression-runs-boost-your-fitness/) -- Three types of progression runs
- [Marathon Handbook - Progression Runs](https://marathonhandbook.com/progression-runs/) -- Structure and benefits

### Fartlek and Kenyan Training

- [Kenya Experience - Fartlek Part 2: Example Sessions](https://www.traininkenya.com/2018/06/30/run-the-kenyan-way-fartlek-part-2-example-sessions/) -- Classic Kenyan fartlek variations
- [Kenya Experience - The Famous Iten Thursday Fartlek](https://www.traininkenya.com/2025/12/03/run-the-kenyan-way-the-famous-iten-thursday-fartlek/) -- Iten fartlek structure

### Warmup/Cooldown

- [Canadian Running Magazine - Should warmup and cooldown count as mileage?](https://runningmagazine.ca/sections/training/should-your-warmup-and-cooldown-count-as-mileage/) -- Mileage counting
- [Laura Norris Running - Warm-Up and Cooldown Miles](https://lauranorrisrunning.com/why-and-how-to-run-warm-up-and-cooldown-miles/) -- Distance recommendations
- [Luke Humphrey Running - Warm Ups: Science and Art](https://lukehumphreyrunning.com/warm-ups-a-little-science-and-a-little-art/) -- Evidence-based warmup guidelines

### Shakeout Runs

- [Marathon Handbook - Shakeout Runs Explained](https://marathonhandbook.com/shakeout-runs-explained/) -- Duration and intensity guidelines
- [Runners Connect - Can a shakeout run improve performance?](https://runnersconnect.net/coach-corner/can-a-shakeout-run-improve-race-day-performance/) -- Research on shakeout effectiveness

### Athlete Classification

- [LetsRun Forum - Jan Olbrecht Training for Runners](https://www.letsrun.com/forum/flat_read.php?thread=9830107) -- Application of Olbrecht's framework to running
- [LetsRun Forum - Norwegian Training and Olbrecht's Principles](https://www.letsrun.com/forum/flat_read.php?thread=11193935) -- Comparison of Norwegian method and Olbrecht
- [Uphill Athlete - Capacity vs. Utilization Training](https://uphillathlete.com/aerobic-training/capacity-training-vs-utilization-training/) -- Practical guide to capacity/utilization distinction
- [Evoke Endurance - Capacity vs Utilization: A Deep Dive](https://evokeendurance.com/resources/capacity-vs-utilization-training/) -- Detailed framework explanation
- [Lactate.com - Aerobic and Anaerobic Capacities Animation](https://lactate.com/aerobic_anaerobic_animation.html) -- Visual explanation of AEC/ANC interplay

### Time Trials

- [Mile By Mile - 5K Time Trial](https://www.milebymileblog.com/why-and-how-to-run-a-5k-time-trial-to-test-your-fitness-2/) -- Protocol and benefits
- [Sifuentes Coaching - Time Trial 101](https://sifuentescoaching.com/resources/timetrial101) -- Comprehensive TT guide

### Running Drills

- [Training Peaks - Drills for Proper Running Form](https://www.trainingpeaks.com/blog/drills-for-proper-running-form/) -- Drill descriptions and benefits
- [Marathon Handbook - A Skips for Runners](https://marathonhandbook.com/a-skips/) -- Detailed A-skip guide

### General Periodization

- [TrainingPeaks - Macrocycles, Mesocycles, and Microcycles](https://www.trainingpeaks.com/blog/macrocycles-mesocycles-and-microcycles-understanding-the-3-cycles-of-periodization/) -- Periodization overview
- [Strength Running - Why Periodization Matters](https://strengthrunning.com/2019/08/periodization-training-for-runners/) -- Running-specific periodization
