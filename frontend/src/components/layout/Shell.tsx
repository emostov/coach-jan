import { Outlet } from 'react-router';
import Nav from './Nav';

export default function Shell() {
  return (
    <div className="min-h-screen bg-cream">
      <Nav />

      {/* Main content area */}
      <main className="md:ml-16 pb-16 md:pb-0">
        <div className="max-w-4xl mx-auto px-4 sm:px-6 py-6 sm:py-8">
          <Outlet />
        </div>
      </main>
    </div>
  );
}
