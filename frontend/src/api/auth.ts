import { apiFetch } from './client';
import type { AuthResponse, MeResponse } from './types';

export function register(email: string, password: string): Promise<AuthResponse> {
  return apiFetch('/auth/register', {
    method: 'POST',
    body: JSON.stringify({ email, password }),
  });
}

export function login(email: string, password: string): Promise<AuthResponse> {
  return apiFetch('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ email, password }),
  });
}

export function logout(): Promise<{ message: string }> {
  return apiFetch('/auth/logout', { method: 'POST' });
}

export function getMe(): Promise<MeResponse> {
  return apiFetch('/auth/me');
}
