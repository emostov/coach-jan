import { Link, useLocation } from 'react-router';
import { useLogout } from '../../hooks/useAuth';

interface NavItem {
  label: string;
  path: string;
  icon: string;
  disabled?: boolean;
}

const NAV_ITEMS: NavItem[] = [
  { label: 'Dashboard', path: '/', icon: '\u25A0' },
  { label: 'Plan', path: '/plan', icon: '\u25B6' },
  { label: 'Performance', path: '/performance', icon: '\u25B2', disabled: true },
  { label: 'Chat', path: '/chat', icon: '\u25CF', disabled: true },
  { label: 'Profile', path: '/profile', icon: '\u25CB' },
];

export default function Nav() {
  const location = useLocation();
  const logoutMutation = useLogout();

  const handleLogout = () => {
    logoutMutation.mutate(undefined, {
      onSuccess: () => {
        window.location.href = '/login';
      },
    });
  };

  return (
    <>
      {/* Desktop sidebar */}
      <nav className="hidden md:flex fixed left-0 top-0 bottom-0 w-16 bg-forest flex-col items-center py-6 z-50">
        {/* Brand */}
        <div className="mb-8">
          <span className="text-cream font-serif font-bold text-lg leading-none">J</span>
        </div>

        {/* Nav items */}
        <div className="flex-1 flex flex-col items-center gap-1">
          {NAV_ITEMS.map((item) => {
            const isActive = location.pathname === item.path;
            return (
              <div key={item.path} className="relative group">
                {item.disabled ? (
                  <div
                    className="w-10 h-10 rounded-lg flex items-center justify-center text-forest-light/40 cursor-not-allowed"
                    title={`${item.label} (coming soon)`}
                  >
                    <span className="text-base">{item.icon}</span>
                  </div>
                ) : (
                  <Link
                    to={item.path}
                    className={`w-10 h-10 rounded-lg flex items-center justify-center transition-colors ${
                      isActive
                        ? 'bg-cream/15 text-cream'
                        : 'text-cream/60 hover:text-cream hover:bg-cream/10'
                    }`}
                    title={item.label}
                  >
                    <span className="text-base">{item.icon}</span>
                  </Link>
                )}
                {/* Tooltip */}
                <div className="absolute left-full ml-2 top-1/2 -translate-y-1/2 px-2 py-1 bg-charcoal text-cream text-xs rounded opacity-0 group-hover:opacity-100 pointer-events-none transition-opacity whitespace-nowrap">
                  {item.label}
                  {item.disabled && ' (coming soon)'}
                </div>
              </div>
            );
          })}
        </div>

        {/* Logout */}
        <button
          onClick={handleLogout}
          className="w-10 h-10 rounded-lg flex items-center justify-center text-cream/40 hover:text-cream hover:bg-cream/10 transition-colors"
          title="Log out"
        >
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            strokeWidth={1.5}
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15m3 0l3-3m0 0l-3-3m3 3H9"
            />
          </svg>
        </button>
      </nav>

      {/* Mobile bottom nav */}
      <nav className="md:hidden fixed bottom-0 left-0 right-0 bg-forest border-t border-forest-light/20 z-50">
        <div className="flex items-center justify-around h-14">
          {NAV_ITEMS.map((item) => {
            const isActive = location.pathname === item.path;
            return item.disabled ? (
              <div
                key={item.path}
                className="flex flex-col items-center gap-0.5 text-forest-light/30 cursor-not-allowed px-3 py-1"
              >
                <span className="text-sm">{item.icon}</span>
                <span className="text-[10px]">{item.label}</span>
              </div>
            ) : (
              <Link
                key={item.path}
                to={item.path}
                className={`flex flex-col items-center gap-0.5 px-3 py-1 transition-colors ${
                  isActive ? 'text-cream' : 'text-cream/50'
                }`}
              >
                <span className="text-sm">{item.icon}</span>
                <span className="text-[10px]">{item.label}</span>
              </Link>
            );
          })}
        </div>
      </nav>
    </>
  );
}
