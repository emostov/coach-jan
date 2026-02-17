-- Add target_distance_km to planned_workouts for weekly mileage summaries
ALTER TABLE planned_workouts ADD COLUMN target_distance_km REAL;
