import { useCurrentPlan } from '../hooks/usePlan';
import MacrocycleTimeline from '../components/plan/MacrocycleTimeline';
import Calendar from '../components/plan/Calendar';

export default function Plan() {
  const { data: plan, isLoading, isError } = useCurrentPlan();

  if (isLoading) {
    return (
      <div className="space-y-6">
        <h1 className="font-serif text-3xl text-charcoal font-bold">Training Plan</h1>
        <div className="rounded-xl bg-white border border-cream-dark p-8 text-center">
          <div className="w-8 h-8 border-2 border-forest border-t-transparent rounded-full animate-spin mx-auto" />
          <p className="text-slate text-sm mt-3">Loading your plan...</p>
        </div>
      </div>
    );
  }

  if (isError || !plan) {
    return (
      <div className="space-y-6">
        <h1 className="font-serif text-3xl text-charcoal font-bold">Training Plan</h1>
        <div className="rounded-xl bg-white border border-cream-dark p-8 text-center">
          <p className="text-slate">No training plan found. Complete onboarding to generate your plan.</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <h1 className="font-serif text-3xl text-charcoal font-bold">Training Plan</h1>

      <MacrocycleTimeline
        mesocycles={plan.mesocycles}
        macrocycleStart={plan.macrocycle.start_date}
        macrocycleEnd={plan.macrocycle.end_date}
      />

      <Calendar plan={plan} />
    </div>
  );
}
