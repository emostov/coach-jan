import { apiFetch } from './client';
import type { ProfileResponse } from './types';

export interface CreateProfileInput {
  name: string;
  age: number;
  weight_kg: number;
  resting_hr: number;
  max_hr: number;
  lthr: number;
  ftpace_m_per_s?: number;
  current_weekly_volume_km: number;
  experience_level: string;
  sports_background?: string;
  race_name?: string;
  race_distance_m: number;
  race_date: string;
  target_time_seconds?: number;
}

export interface UpdateProfileInput {
  name?: string;
  age?: number;
  weight_kg?: number;
  resting_hr?: number;
  max_hr?: number;
  lthr?: number;
  ftpace_m_per_s?: number;
  current_weekly_volume_km?: number;
  experience_level?: string;
  sports_background?: string;
}

export function createProfile(data: CreateProfileInput): Promise<ProfileResponse> {
  return apiFetch('/athlete/profile', {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export function getProfile(): Promise<ProfileResponse> {
  return apiFetch('/athlete/profile');
}

export function updateProfile(data: UpdateProfileInput): Promise<ProfileResponse> {
  return apiFetch('/athlete/profile', {
    method: 'PUT',
    body: JSON.stringify(data),
  });
}
