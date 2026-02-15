import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { create } from 'zustand';
import * as authApi from '../api/auth';
import type { User } from '../api/types';

interface AuthStore {
  user: User | null;
  hasProfile: boolean;
  setAuth: (user: User | null, hasProfile: boolean) => void;
  clear: () => void;
}

export const useAuthStore = create<AuthStore>((set) => ({
  user: null,
  hasProfile: false,
  setAuth: (user, hasProfile) => set({ user, hasProfile }),
  clear: () => set({ user: null, hasProfile: false }),
}));

export function useMe() {
  const setAuth = useAuthStore((s) => s.setAuth);
  return useQuery({
    queryKey: ['me'],
    queryFn: async () => {
      const data = await authApi.getMe();
      setAuth(data.user, data.has_profile);
      return data;
    },
    retry: false,
  });
}

export function useLogin() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ email, password }: { email: string; password: string }) =>
      authApi.login(email, password),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['me'] }),
  });
}

export function useRegister() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ email, password }: { email: string; password: string }) =>
      authApi.register(email, password),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['me'] }),
  });
}

export function useLogout() {
  const queryClient = useQueryClient();
  const clear = useAuthStore((s) => s.clear);
  return useMutation({
    mutationFn: authApi.logout,
    onSuccess: () => {
      clear();
      queryClient.clear();
    },
  });
}
