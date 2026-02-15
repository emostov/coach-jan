import { useState } from 'react';
import { useNavigate } from 'react-router';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useLogin, useRegister } from '../hooks/useAuth';

const loginSchema = z.object({
  email: z.string().email('Please enter a valid email address'),
  password: z.string().min(8, 'Password must be at least 8 characters'),
});

const registerSchema = z
  .object({
    email: z.string().email('Please enter a valid email address'),
    password: z.string().min(8, 'Password must be at least 8 characters'),
    confirmPassword: z.string(),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: 'Passwords do not match',
    path: ['confirmPassword'],
  });

type LoginFormData = z.infer<typeof loginSchema>;
type RegisterFormData = z.infer<typeof registerSchema>;

export default function Login() {
  const [isRegister, setIsRegister] = useState(false);
  const [apiError, setApiError] = useState<string | null>(null);
  const navigate = useNavigate();
  const loginMutation = useLogin();
  const registerMutation = useRegister();

  const loginForm = useForm<LoginFormData>({
    resolver: zodResolver(loginSchema),
    defaultValues: { email: '', password: '' },
  });

  const registerForm = useForm<RegisterFormData>({
    resolver: zodResolver(registerSchema),
    defaultValues: { email: '', password: '', confirmPassword: '' },
  });

  const onLogin = (data: LoginFormData) => {
    setApiError(null);
    loginMutation.mutate(
      { email: data.email, password: data.password },
      {
        onSuccess: () => {
          // Auth guards will redirect to /onboarding if no profile
          navigate('/');
        },
        onError: (error) => {
          setApiError(error.message);
        },
      },
    );
  };

  const onRegister = (data: RegisterFormData) => {
    setApiError(null);
    registerMutation.mutate(
      { email: data.email, password: data.password },
      {
        onSuccess: () => {
          navigate('/onboarding');
        },
        onError: (error) => {
          setApiError(error.message);
        },
      },
    );
  };

  const isLoading = loginMutation.isPending || registerMutation.isPending;

  return (
    <div className="min-h-screen bg-cream flex items-center justify-center px-4">
      <div className="w-full max-w-md">
        {/* Brand */}
        <div className="text-center mb-8">
          <h1 className="font-serif text-4xl text-forest font-bold tracking-tight">CoachJan</h1>
          <p className="mt-2 text-slate text-sm">Your AI running coach</p>
        </div>

        {/* Card */}
        <div className="bg-white rounded-2xl shadow-sm border border-cream-dark p-8">
          <h2 className="font-serif text-2xl text-charcoal mb-6">
            {isRegister ? 'Create account' : 'Welcome back'}
          </h2>

          {/* API Error */}
          {apiError && (
            <div className="mb-4 p-3 rounded-lg bg-terra/10 border border-terra/20 text-terra text-sm">
              {apiError}
            </div>
          )}

          {isRegister ? (
            <form onSubmit={registerForm.handleSubmit(onRegister)} className="space-y-4">
              <div>
                <label htmlFor="reg-email" className="block text-sm font-medium text-charcoal mb-1.5">
                  Email
                </label>
                <input
                  id="reg-email"
                  type="email"
                  autoComplete="email"
                  {...registerForm.register('email')}
                  className="w-full px-3.5 py-2.5 rounded-lg border border-cream-dark bg-cream/50 text-charcoal placeholder:text-slate-light focus:outline-none focus:ring-2 focus:ring-forest/30 focus:border-forest transition-colors"
                  placeholder="you@example.com"
                />
                {registerForm.formState.errors.email && (
                  <p className="mt-1 text-sm text-terra">
                    {registerForm.formState.errors.email.message}
                  </p>
                )}
              </div>

              <div>
                <label htmlFor="reg-password" className="block text-sm font-medium text-charcoal mb-1.5">
                  Password
                </label>
                <input
                  id="reg-password"
                  type="password"
                  autoComplete="new-password"
                  {...registerForm.register('password')}
                  className="w-full px-3.5 py-2.5 rounded-lg border border-cream-dark bg-cream/50 text-charcoal placeholder:text-slate-light focus:outline-none focus:ring-2 focus:ring-forest/30 focus:border-forest transition-colors"
                  placeholder="Min. 8 characters"
                />
                {registerForm.formState.errors.password && (
                  <p className="mt-1 text-sm text-terra">
                    {registerForm.formState.errors.password.message}
                  </p>
                )}
              </div>

              <div>
                <label htmlFor="reg-confirm" className="block text-sm font-medium text-charcoal mb-1.5">
                  Confirm password
                </label>
                <input
                  id="reg-confirm"
                  type="password"
                  autoComplete="new-password"
                  {...registerForm.register('confirmPassword')}
                  className="w-full px-3.5 py-2.5 rounded-lg border border-cream-dark bg-cream/50 text-charcoal placeholder:text-slate-light focus:outline-none focus:ring-2 focus:ring-forest/30 focus:border-forest transition-colors"
                  placeholder="Repeat password"
                />
                {registerForm.formState.errors.confirmPassword && (
                  <p className="mt-1 text-sm text-terra">
                    {registerForm.formState.errors.confirmPassword.message}
                  </p>
                )}
              </div>

              <button
                type="submit"
                disabled={isLoading}
                className="w-full py-2.5 px-4 rounded-lg bg-forest text-cream font-medium hover:bg-forest-light transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading ? 'Creating account...' : 'Create account'}
              </button>
            </form>
          ) : (
            <form onSubmit={loginForm.handleSubmit(onLogin)} className="space-y-4">
              <div>
                <label htmlFor="login-email" className="block text-sm font-medium text-charcoal mb-1.5">
                  Email
                </label>
                <input
                  id="login-email"
                  type="email"
                  autoComplete="email"
                  {...loginForm.register('email')}
                  className="w-full px-3.5 py-2.5 rounded-lg border border-cream-dark bg-cream/50 text-charcoal placeholder:text-slate-light focus:outline-none focus:ring-2 focus:ring-forest/30 focus:border-forest transition-colors"
                  placeholder="you@example.com"
                />
                {loginForm.formState.errors.email && (
                  <p className="mt-1 text-sm text-terra">
                    {loginForm.formState.errors.email.message}
                  </p>
                )}
              </div>

              <div>
                <label htmlFor="login-password" className="block text-sm font-medium text-charcoal mb-1.5">
                  Password
                </label>
                <input
                  id="login-password"
                  type="password"
                  autoComplete="current-password"
                  {...loginForm.register('password')}
                  className="w-full px-3.5 py-2.5 rounded-lg border border-cream-dark bg-cream/50 text-charcoal placeholder:text-slate-light focus:outline-none focus:ring-2 focus:ring-forest/30 focus:border-forest transition-colors"
                  placeholder="Your password"
                />
                {loginForm.formState.errors.password && (
                  <p className="mt-1 text-sm text-terra">
                    {loginForm.formState.errors.password.message}
                  </p>
                )}
              </div>

              <button
                type="submit"
                disabled={isLoading}
                className="w-full py-2.5 px-4 rounded-lg bg-forest text-cream font-medium hover:bg-forest-light transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading ? 'Signing in...' : 'Sign in'}
              </button>
            </form>
          )}

          {/* Toggle */}
          <p className="mt-6 text-center text-sm text-slate">
            {isRegister ? 'Already have an account?' : "Don't have an account?"}{' '}
            <button
              type="button"
              onClick={() => {
                setIsRegister(!isRegister);
                setApiError(null);
              }}
              className="text-forest font-medium hover:text-forest-light transition-colors"
            >
              {isRegister ? 'Sign in' : 'Create one'}
            </button>
          </p>
        </div>

        {/* Footer */}
        <p className="mt-6 text-center text-xs text-slate-light">
          Built on Jan Olbrecht's training philosophy
        </p>
      </div>
    </div>
  );
}
