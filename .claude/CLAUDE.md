# CoachJan — AI Running Coach

## Project Overview

CoachJan is an AI running coach built on Jan Olbrecht's training philosophy. It creates periodized training plans, ingests workout data (FIT files), and provides ongoing coaching feedback. The canonical design spec is `PRODUCT_DESIGN.md` — read it before making changes.

## Key Design Principles

1. **Stability over reactivity**: The coach errs on the side of NOT adjusting the plan. Missed days → keep plan as-is. Only adjust for sustained patterns (3+ sessions) or extended absences (1+ weeks). No reactive daily tweaking.

2. **Full athlete picture**: Every plan decision must consider all athlete data — profile, sports background, all stress scores (rTSS, TRIMP, aerobic/anaerobic effect), ATL/CTL/TSB trends, strength load, and macrocycle timeline.

3. **Olbrecht's framework**: Training develops capacity first (aerobic/anaerobic), then utilization. "Capacity is for training, power is for racing."

4. **Strength from Running Rewired**: Strength/mobility sessions are drawn from Jay Dicharry's *Running Rewired*, programmed based on athlete's sports background, injury history, and training phase.

## Architecture

- **Status**: Design phase. `PRODUCT_DESIGN.md` is the source of truth.
- **Target**: Local-first app, no cloud dependencies beyond Claude API. Future: hostable on Railway.
- **Data flow**: Athlete profile → Plan generation → FIT upload → Workout analysis → Coach feedback → (rarely) Plan adjustment

## Domain Concepts

- **Macrocycle**: Full preparation period through race day
- **Mesocycle**: Load phase + recovery phase; duration varies by athlete level
- **TSS/rTSS/TRIMP**: Training stress scores — primary load metrics
- **ATL/CTL/TSB**: Acute/Chronic training load and Training Stress Balance (fatigue/fitness/form)
- **Aerobic/Anaerobic Effect**: 0-5 scores measuring training stimulus type
- **FTPace**: Functional Threshold Pace (~1hr race pace), anchors pace zones
- **LTHR**: Lactate Threshold Heart Rate, anchors HR zones

## Working on This Project

- When doing research, spin up multiple agents do parrellelize research as much as possible
- You have 3 key agents: software application developer, Jan Olbrecht the Running Coach who will use this, and code reviewer. When building features use all 3 agents or more
- Whenever doing a task thats more then 2 steps, make a plan and a todo list
- Ask questions whenever there is ambiguity
- Always read `PRODUCT_DESIGN.md` before proposing design changes
- When adding features, update `PRODUCT_DESIGN.md` to reflect the change
- Coach persona is "Jan" — direct, knowledgeable, uses "we" language, explains physiology
- The 7-zone HR model and 6-zone pace model are intentional — don't change without discussion
- Stress score algorithms should be based on research of existing algorithms (Firstbeat, EPOC, etc.) but our own implementation
- Anytime you make a mistake and get corrected, update this document to ensure you don't make the same mistake again.
- Use Test Driven Development (TDD) - validate your code against tests