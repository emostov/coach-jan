# CoachJan — Future Features Roadmap

Features to add after the initial product design is complete. The overarching goal is to **make CoachJan useful for non-elite athletes who just want to be fit**, not only competitive racers.

---
TODO(claude): schedule and goal flexibility should be a part of onboarding. We should also have a way to let the user redo the onboarding physical assessment.

## Schedule Flexibility

These features let athletes fit training around real life — work, family, and variable schedules.

- **Available training days**: Let athletes specify which days of the week they can work out (e.g., "only weekdays" or "Tue/Thu/Sat")
- **Max training days per week**: Cap the number of sessions per week (e.g., "no more than 4 days")
- **Max workout duration per day**: Allow specifying a time ceiling per session (e.g., "45 min on weekdays, 90 min on weekends")
- **Ad-hoc schedule adjustments**: Ask Jan to update the plan for just the next few days to handle scheduling conflicts (work trip, sick kid, etc.) without replanning the whole macrocycle

## Goal Flexibility

Not every athlete is training for a race. Support a wider range of motivations.

- **Non-race goals**: Support goals like general fitness, injury prevention, injury recovery, and "just want to run consistently"
- **First-timer support**: Better onboarding and plan generation for athletes attempting their first 5K, 10K, half marathon, or marathon
- **Multiple concurrent goals**: Allow specifying more than one goal — e.g., a 5K race in April plus general fitness through summer, or two races at different distances
- **Cross-sport scheduling**: Let athletes specify other activities (basketball, swimming, volleyball, etc.) and integrate them into the training schedule. Ask during onboarding if they want to balance running with other sports
- **Fitness-focused athletes**: Some athletes are primarily focused on other sports or general life fitness, with running as a complement — support that use case

## Training Plan Customization

Give athletes more control over the shape of their training.

- **Target/max weekly mileage**: Allow specifying a range or ceiling for weekly volume (e.g., "30-40 km/week" or "never more than 50 km/week")
- **Athlete plan editing**: Allow athletes to modify their training plan directly (swap days, adjust workouts, etc.)
- **Longer workout support**: Support runs up to 30 miles — some athletes use long runs to accumulate volume in fewer sessions

## Data & History

Richer data input means better coaching decisions.

- **Running history notes**: Add an onboarding section where athletes can describe their running history in free text (injuries, past training blocks, what worked/didn't)
- **Workout history sync**: Add an onboarding step to import past workout data
- **Strava integration**: Sync workout data from Strava
- **Historical workout analysis**: Ask Jan to analyze past workout data, find patterns of what has worked and what hasn't, and produce a detailed write-up

## Workout Library Expansion

Broader workout variety for different race distances.

- **5K-specific workouts**: Research and add 5K-targeted sessions to the workout registry and Claude prompts (VO2max intervals, race-pace work, short hill repeats, etc.)
- **1-mile-specific workouts**: Same for the mile — speed development, neuromuscular work, race-specific pacing

## Infrastructure

Backend improvements to support growth and sustainability.

- **Claude API rate limiting**: Add rate limits on API calls to manage cost
- **Paid tier rate increases**: Higher rate limits for athletes on a paid plan
