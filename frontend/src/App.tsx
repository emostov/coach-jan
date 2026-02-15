import { BrowserRouter, Routes, Route, Navigate } from 'react-router';
import { useMe, useAuthStore } from './hooks/useAuth';
import Login from './pages/Login';
import Onboarding from './pages/Onboarding';
import Dashboard from './pages/Dashboard';
import Profile from './pages/Profile';
import Shell from './components/layout/Shell';
import type { ReactNode } from 'react';

function LoadingSpinner() {
  return (
    <div className="min-h-screen bg-cream flex items-center justify-center">
      <div className="flex flex-col items-center gap-3">
        <div className="w-8 h-8 border-2 border-forest border-t-transparent rounded-full animate-spin" />
        <p className="text-slate text-sm">Loading...</p>
      </div>
    </div>
  );
}

function AuthGuard({ children, requireProfile }: { children: ReactNode; requireProfile?: boolean }) {
  const { isLoading, isError } = useMe();
  const user = useAuthStore((s) => s.user);
  const hasProfile = useAuthStore((s) => s.hasProfile);

  if (isLoading) return <LoadingSpinner />;
  if (isError || !user) return <Navigate to="/login" replace />;
  if (requireProfile && !hasProfile) return <Navigate to="/onboarding" replace />;

  return <>{children}</>;
}

function GuestGuard({ children }: { children: ReactNode }) {
  const { isLoading, isError } = useMe();
  const user = useAuthStore((s) => s.user);
  const hasProfile = useAuthStore((s) => s.hasProfile);

  if (isLoading) return <LoadingSpinner />;
  if (!isError && user) {
    return <Navigate to={hasProfile ? '/' : '/onboarding'} replace />;
  }

  return <>{children}</>;
}

function OnboardingGuard({ children }: { children: ReactNode }) {
  const { isLoading, isError } = useMe();
  const user = useAuthStore((s) => s.user);
  const hasProfile = useAuthStore((s) => s.hasProfile);

  if (isLoading) return <LoadingSpinner />;
  if (isError || !user) return <Navigate to="/login" replace />;
  if (hasProfile) return <Navigate to="/" replace />;

  return <>{children}</>;
}

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* Public route */}
        <Route
          path="/login"
          element={
            <GuestGuard>
              <Login />
            </GuestGuard>
          }
        />

        {/* Onboarding (auth required, no profile yet) */}
        <Route
          path="/onboarding"
          element={
            <OnboardingGuard>
              <Onboarding />
            </OnboardingGuard>
          }
        />

        {/* Authenticated routes with shell */}
        <Route
          element={
            <AuthGuard requireProfile>
              <Shell />
            </AuthGuard>
          }
        >
          <Route path="/" element={<Dashboard />} />
          <Route path="/profile" element={<Profile />} />
        </Route>

        {/* Catch all */}
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}
