import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import * as athleteApi from '../api/athlete';
import type { CreateProfileInput, UpdateProfileInput } from '../api/athlete';

export function useProfile() {
  return useQuery({
    queryKey: ['profile'],
    queryFn: athleteApi.getProfile,
    retry: false,
  });
}

export function useCreateProfile() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateProfileInput) => athleteApi.createProfile(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['profile'] });
      queryClient.invalidateQueries({ queryKey: ['me'] });
    },
  });
}

export function useUpdateProfile() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: UpdateProfileInput) => athleteApi.updateProfile(data),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['profile'] }),
  });
}
