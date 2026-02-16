import { useState, useEffect, useRef } from 'react';
import { useNavigate } from 'react-router';
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useCreateProfile } from '../hooks/useAthlete';
import { useGeneratePlan, useConfirmPlan } from '../hooks/usePlan';
import { formatPace } from '../utils/formatting';
import type { CreateProfileInput } from '../api/athlete';
import type { RaceGoal, MacrocycleSkeleton } from '../api/types';

// ────────────────────────────────────────────────────
// Schemas per step
// ────────────────────────────────────────────────────

const step1Schema = z.object({
  name: z.string().min(1, 'Name is required'),
  age: z.number().int().min(12, 'Must be at least 12').max(100, 'Must be 100 or less'),
  weight_kg: z.number().min(30, 'Must be at least 30 kg').max(200, 'Must be 200 kg or less'),
});

const step2Schema = z
  .object({
    resting_hr: z.number().int().min(30, 'Min 30 bpm').max(100, 'Max 100 bpm'),
    max_hr: z.number().int().min(120, 'Min 120 bpm').max(230, 'Max 230 bpm'),
    lthr: z.number().int().min(100, 'Min 100 bpm').max(220, 'Max 220 bpm'),
  })
  .refine((data) => data.lthr < data.max_hr, {
    message: 'LTHR must be less than max HR',
    path: ['lthr'],
  })
  .refine((data) => data.resting_hr < data.lthr, {
    message: 'Resting HR must be less than LTHR',
    path: ['resting_hr'],
  });

const step3Schema = z.object({
  pace_minutes: z.string().optional(),
  pace_seconds: z.string().optional(),
  current_weekly_volume_km: z.number().min(0, 'Min 0 km').max(300, 'Max 300 km'),
  experience_level: z.enum(['beginner', 'intermediate', 'advanced'], {
    message: 'Please select an experience level',
  }),
  sports_background: z.string().optional(),
});

const RACE_PRESETS: Record<string, number> = {
  '5K': 5000,
  '10K': 10000,
  Half: 21097,
  Marathon: 42195,
  Ultra: 0,
};

const step4Schema = z.object({
  race_name: z.string().optional(),
  race_distance_preset: z.string().min(1, 'Select a distance'),
  race_distance_custom_m: z.string().optional(),
  race_date: z.string().min(1, 'Race date is required'),
  target_hours: z.string().optional(),
  target_minutes: z.string().optional(),
  target_seconds: z.string().optional(),
});

type Step1Data = z.infer<typeof step1Schema>;
type Step2Data = z.infer<typeof step2Schema>;
type Step3Data = z.infer<typeof step3Schema>;
type Step4Data = z.infer<typeof step4Schema>;

// ────────────────────────────────────────────────────
// Field component helpers
// ────────────────────────────────────────────────────

function FieldLabel({ htmlFor, children }: { htmlFor: string; children: React.ReactNode }) {
  return (
    <label htmlFor={htmlFor} className="block text-sm font-medium text-charcoal mb-1.5">
      {children}
    </label>
  );
}

function FieldError({ message }: { message?: string }) {
  if (!message) return null;
  return <p className="mt-1 text-sm text-terra">{message}</p>;
}

const inputClass =
  'w-full px-3.5 py-2.5 rounded-lg border border-cream-dark bg-cream/50 text-charcoal placeholder:text-slate-light focus:outline-none focus:ring-2 focus:ring-forest/30 focus:border-forest transition-colors';

const selectClass =
  'w-full px-3.5 py-2.5 rounded-lg border border-cream-dark bg-cream/50 text-charcoal focus:outline-none focus:ring-2 focus:ring-forest/30 focus:border-forest transition-colors appearance-none';

// ────────────────────────────────────────────────────
// Coach Jan Avatar (reusable)
// ────────────────────────────────────────────────────

function CoachAvatar({ size = 'sm' }: { size?: 'sm' | 'md' | 'lg' }) {
  const sizeClass = size === 'lg' ? 'w-16 h-16' : size === 'md' ? 'w-10 h-10' : 'w-7 h-7';
  const textClass = size === 'lg' ? 'text-2xl' : size === 'md' ? 'text-sm' : 'text-xs';
  return (
    <div className={`${sizeClass} rounded-full bg-forest flex items-center justify-center`}>
      <span className={`text-cream ${textClass} font-bold font-serif`}>J</span>
    </div>
  );
}

// ────────────────────────────────────────────────────
// Steps 1-4 (form steps)
// ────────────────────────────────────────────────────

function StepBasicInfo({
  onNext,
  defaultValues,
}: {
  onNext: (data: Step1Data) => void;
  defaultValues: Partial<Step1Data>;
}) {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<Step1Data>({
    resolver: zodResolver(step1Schema),
    defaultValues,
  });

  return (
    <form onSubmit={handleSubmit(onNext)} className="space-y-4">
      <div>
        <FieldLabel htmlFor="name">Full name</FieldLabel>
        <input id="name" type="text" {...register('name')} className={inputClass} placeholder="e.g. Sarah Chen" />
        <FieldError message={errors.name?.message} />
      </div>
      <div className="grid grid-cols-2 gap-4">
        <div>
          <FieldLabel htmlFor="age">Age</FieldLabel>
          <input id="age" type="number" {...register('age', { valueAsNumber: true })} className={inputClass} placeholder="32" />
          <FieldError message={errors.age?.message} />
        </div>
        <div>
          <FieldLabel htmlFor="weight_kg">Weight (kg)</FieldLabel>
          <input
            id="weight_kg"
            type="number"
            step="0.1"
            {...register('weight_kg', { valueAsNumber: true })}
            className={inputClass}
            placeholder="65"
          />
          <FieldError message={errors.weight_kg?.message} />
        </div>
      </div>
      <StepButtons />
    </form>
  );
}

function StepPhysiology({
  onNext,
  onBack,
  defaultValues,
}: {
  onNext: (data: Step2Data) => void;
  onBack: () => void;
  defaultValues: Partial<Step2Data>;
}) {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<Step2Data>({
    resolver: zodResolver(step2Schema),
    defaultValues,
  });

  return (
    <form onSubmit={handleSubmit(onNext)} className="space-y-4">
      <p className="text-sm text-slate mb-2">
        These heart rate values anchor your training zones. If you don't know your exact numbers, use
        estimates and we'll refine them over time.
      </p>

      <div>
        <FieldLabel htmlFor="resting_hr">Resting heart rate (bpm)</FieldLabel>
        <input
          id="resting_hr"
          type="number"
          {...register('resting_hr', { valueAsNumber: true })}
          className={inputClass}
          placeholder="50"
        />
        <FieldError message={errors.resting_hr?.message} />
      </div>

      <div>
        <FieldLabel htmlFor="max_hr">Max heart rate (bpm)</FieldLabel>
        <input
          id="max_hr"
          type="number"
          {...register('max_hr', { valueAsNumber: true })}
          className={inputClass}
          placeholder="185"
        />
        <FieldError message={errors.max_hr?.message} />
      </div>

      <div>
        <FieldLabel htmlFor="lthr">Lactate threshold HR (bpm)</FieldLabel>
        <input
          id="lthr"
          type="number"
          {...register('lthr', { valueAsNumber: true })}
          className={inputClass}
          placeholder="168"
        />
        <p className="mt-1 text-xs text-slate">Roughly your average HR during a 1-hour all-out effort</p>
        <FieldError message={errors.lthr?.message} />
      </div>

      <StepButtons onBack={onBack} />
    </form>
  );
}

function StepRunning({
  onNext,
  onBack,
  defaultValues,
}: {
  onNext: (data: Step3Data) => void;
  onBack: () => void;
  defaultValues: Partial<Step3Data>;
}) {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<Step3Data>({
    resolver: zodResolver(step3Schema),
    defaultValues,
  });

  return (
    <form onSubmit={handleSubmit(onNext)} className="space-y-4">
      <div>
        <FieldLabel htmlFor="pace_minutes">Functional threshold pace (optional)</FieldLabel>
        <p className="text-xs text-slate mb-2">Your ~1-hour race pace, in min:sec per km</p>
        <div className="flex items-center gap-2">
          <input
            id="pace_minutes"
            type="number"
            {...register('pace_minutes')}
            className={`${inputClass} w-20 text-center`}
            placeholder="4"
            min={2}
            max={15}
          />
          <span className="text-charcoal font-medium">:</span>
          <input
            id="pace_seconds"
            type="number"
            {...register('pace_seconds')}
            className={`${inputClass} w-20 text-center`}
            placeholder="30"
            min={0}
            max={59}
          />
          <span className="text-sm text-slate">/km</span>
        </div>
        {(errors.pace_minutes || errors.pace_seconds) && (
          <FieldError message="Enter a valid pace (e.g. 4:30)" />
        )}
      </div>

      <div>
        <FieldLabel htmlFor="current_weekly_volume_km">Current weekly volume (km)</FieldLabel>
        <input
          id="current_weekly_volume_km"
          type="number"
          step="0.1"
          {...register('current_weekly_volume_km', { valueAsNumber: true })}
          className={inputClass}
          placeholder="40"
        />
        <FieldError message={errors.current_weekly_volume_km?.message} />
      </div>

      <div>
        <FieldLabel htmlFor="experience_level">Experience level</FieldLabel>
        <select id="experience_level" {...register('experience_level')} className={selectClass}>
          <option value="">Select...</option>
          <option value="beginner">Beginner (0-2 years)</option>
          <option value="intermediate">Intermediate (2-5 years)</option>
          <option value="advanced">Advanced (5+ years)</option>
        </select>
        <FieldError message={errors.experience_level?.message} />
      </div>

      <div>
        <FieldLabel htmlFor="sports_background">Sports background (optional)</FieldLabel>
        <input
          id="sports_background"
          type="text"
          {...register('sports_background')}
          className={inputClass}
          placeholder="e.g. cycling, swimming, soccer"
        />
      </div>

      <StepButtons onBack={onBack} />
    </form>
  );
}

function StepGoal({
  onNext,
  onBack,
  defaultValues,
}: {
  onNext: (data: Step4Data) => void;
  onBack: () => void;
  defaultValues: Partial<Step4Data>;
}) {
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<Step4Data>({
    resolver: zodResolver(step4Schema),
    defaultValues,
  });

  const selectedPreset = watch('race_distance_preset');

  return (
    <form onSubmit={handleSubmit(onNext)} className="space-y-4">
      <div>
        <FieldLabel htmlFor="race_name">Race name (optional)</FieldLabel>
        <input
          id="race_name"
          type="text"
          {...register('race_name')}
          className={inputClass}
          placeholder="e.g. Berlin Marathon 2026"
        />
      </div>

      <div>
        <FieldLabel htmlFor="race_distance_preset">Race distance</FieldLabel>
        <select id="race_distance_preset" {...register('race_distance_preset')} className={selectClass}>
          <option value="">Select...</option>
          {Object.keys(RACE_PRESETS).map((key) => (
            <option key={key} value={key}>
              {key === 'Half' ? 'Half Marathon (21.1 km)' : key === 'Ultra' ? 'Ultra (custom distance)' : `${key} (${(RACE_PRESETS[key] / 1000).toFixed(1)} km)`}
            </option>
          ))}
          <option value="custom">Custom distance</option>
        </select>
        <FieldError message={errors.race_distance_preset?.message} />
      </div>

      {(selectedPreset === 'Ultra' || selectedPreset === 'custom') && (
        <div>
          <FieldLabel htmlFor="race_distance_custom_m">Distance (meters)</FieldLabel>
          <input
            id="race_distance_custom_m"
            type="number"
            {...register('race_distance_custom_m')}
            className={inputClass}
            placeholder="50000"
          />
          <FieldError message={errors.race_distance_custom_m?.message} />
        </div>
      )}

      <div>
        <FieldLabel htmlFor="race_date">Race date</FieldLabel>
        <input id="race_date" type="date" {...register('race_date')} className={inputClass} />
        <FieldError message={errors.race_date?.message} />
      </div>

      <div>
        <FieldLabel htmlFor="target_hours">Target time (optional)</FieldLabel>
        <div className="flex items-center gap-2">
          <input
            id="target_hours"
            type="number"
            {...register('target_hours')}
            className={`${inputClass} w-16 text-center`}
            placeholder="3"
            min={0}
            max={24}
          />
          <span className="text-sm text-slate">h</span>
          <input
            type="number"
            {...register('target_minutes')}
            className={`${inputClass} w-16 text-center`}
            placeholder="30"
            min={0}
            max={59}
          />
          <span className="text-sm text-slate">m</span>
          <input
            type="number"
            {...register('target_seconds')}
            className={`${inputClass} w-16 text-center`}
            placeholder="0"
            min={0}
            max={59}
          />
          <span className="text-sm text-slate">s</span>
        </div>
      </div>

      <StepButtons onBack={onBack} />
    </form>
  );
}

// ────────────────────────────────────────────────────
// Step 5: Review
// ────────────────────────────────────────────────────

interface AllData {
  step1: Step1Data;
  step2: Step2Data;
  step3: Step3Data;
  step4: Step4Data;
}

function StepReview({
  data,
  onBack,
  onSubmit,
  isSubmitting,
  apiError,
}: {
  data: AllData;
  onBack: () => void;
  onSubmit: () => void;
  isSubmitting: boolean;
  apiError: string | null;
}) {
  const { step1, step2, step3, step4 } = data;
  const hasPace = !!step3.pace_minutes && !!step3.pace_seconds;
  const paceDisplay = hasPace
    ? `${step3.pace_minutes}:${(step3.pace_seconds ?? '').padStart(2, '0')} /km`
    : 'Not set';

  // Calculate m/s for display
  const ftpaceMs = hasPace
    ? 1000 / (Number(step3.pace_minutes) * 60 + Number(step3.pace_seconds))
    : null;

  const raceDistanceM = resolveDistance(step4);
  const raceDistanceKm = raceDistanceM ? (raceDistanceM / 1000).toFixed(1) : '?';

  const hasTarget = !!step4.target_hours || !!step4.target_minutes || !!step4.target_seconds;

  const targetDisplay = hasTarget
    ? `${step4.target_hours || '0'}:${(step4.target_minutes || '0').padStart(2, '0')}:${(step4.target_seconds || '0').padStart(2, '0')}`
    : 'Not set';

  return (
    <div className="space-y-6">
      {apiError && (
        <div className="p-3 rounded-lg bg-terra/10 border border-terra/20 text-terra text-sm">
          {apiError}
        </div>
      )}

      <div className="space-y-4">
        <ReviewSection title="Basic info">
          <ReviewRow label="Name" value={step1.name} />
          <ReviewRow label="Age" value={String(step1.age)} />
          <ReviewRow label="Weight" value={`${step1.weight_kg} kg`} />
        </ReviewSection>

        <ReviewSection title="Physiology">
          <ReviewRow label="Resting HR" value={`${step2.resting_hr} bpm`} />
          <ReviewRow label="Max HR" value={`${step2.max_hr} bpm`} />
          <ReviewRow label="LTHR" value={`${step2.lthr} bpm`} />
        </ReviewSection>

        <ReviewSection title="Running">
          <ReviewRow label="FT Pace" value={paceDisplay} />
          {ftpaceMs && <ReviewRow label="" value={`(${formatPace(ftpaceMs)} /km = ${ftpaceMs.toFixed(3)} m/s)`} />}
          <ReviewRow label="Weekly volume" value={`${step3.current_weekly_volume_km} km`} />
          <ReviewRow label="Experience" value={step3.experience_level} />
          {step3.sports_background && (
            <ReviewRow label="Background" value={step3.sports_background} />
          )}
        </ReviewSection>

        <ReviewSection title="Race goal">
          {step4.race_name && <ReviewRow label="Race" value={step4.race_name} />}
          <ReviewRow label="Distance" value={`${raceDistanceKm} km`} />
          <ReviewRow label="Date" value={step4.race_date} />
          <ReviewRow label="Target" value={targetDisplay} />
        </ReviewSection>
      </div>

      <div className="flex gap-3 pt-2">
        <button
          type="button"
          onClick={onBack}
          className="flex-1 py-2.5 px-4 rounded-lg border border-cream-dark text-charcoal font-medium hover:bg-cream-dark/50 transition-colors"
        >
          Back
        </button>
        <button
          type="button"
          onClick={onSubmit}
          disabled={isSubmitting}
          className="flex-1 py-2.5 px-4 rounded-lg bg-terra text-cream font-medium hover:bg-terra-light transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isSubmitting ? 'Creating profile...' : 'Create Profile'}
        </button>
      </div>
    </div>
  );
}

function ReviewSection({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="rounded-lg bg-cream-dark/40 border border-cream-dark p-4">
      <h4 className="text-sm font-semibold text-forest uppercase tracking-wider mb-2">{title}</h4>
      <div className="space-y-1">{children}</div>
    </div>
  );
}

function ReviewRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between text-sm">
      {label && <span className="text-slate">{label}</span>}
      <span className="text-charcoal font-medium">{value}</span>
    </div>
  );
}

// ────────────────────────────────────────────────────
// Step 6: Plan Generation Loading
// ────────────────────────────────────────────────────

const COACHING_MESSAGES = [
  'Analyzing your physiology...',
  'Mapping your race timeline...',
  'Designing your periodization...',
  'Building your training blocks...',
];

function StepPlanGenerating({
  raceGoalId,
  onSuccess,
}: {
  raceGoalId: number;
  onSuccess: (skeleton: MacrocycleSkeleton) => void;
}) {
  const [messageIndex, setMessageIndex] = useState(0);
  const generatePlan = useGeneratePlan();
  const hasFired = useRef(false);

  // Rotate coaching messages every 3 seconds
  useEffect(() => {
    const interval = setInterval(() => {
      setMessageIndex((prev) => (prev + 1) % COACHING_MESSAGES.length);
    }, 3000);
    return () => clearInterval(interval);
  }, []);

  // Auto-fire plan generation on mount
  useEffect(() => {
    if (hasFired.current) return;
    hasFired.current = true;

    generatePlan.mutate(raceGoalId, {
      onSuccess: (skeleton) => onSuccess(skeleton),
    });
  }, [raceGoalId]); // eslint-disable-line react-hooks/exhaustive-deps

  const handleRetry = () => {
    generatePlan.mutate(raceGoalId, {
      onSuccess: (skeleton) => onSuccess(skeleton),
    });
  };

  return (
    <div className="flex flex-col items-center py-8">
      {/* Pulsing Coach Jan avatar */}
      <div className="relative mb-8">
        <div className="absolute inset-0 w-16 h-16 rounded-full bg-forest/20 animate-ping" />
        <div className="relative">
          <CoachAvatar size="lg" />
        </div>
      </div>

      {/* Rotating message */}
      <p className="text-lg text-charcoal font-medium text-center mb-2 min-h-[1.75rem] transition-opacity duration-300">
        {COACHING_MESSAGES[messageIndex]}
      </p>
      <p className="text-sm text-slate text-center">
        Coach Jan is designing your training plan
      </p>

      {/* Error state */}
      {generatePlan.isError && (
        <div className="mt-6 w-full">
          <div className="p-3 rounded-lg bg-terra/10 border border-terra/20 text-terra text-sm mb-4">
            {generatePlan.error?.message ?? 'Failed to generate plan. Please try again.'}
          </div>
          <button
            type="button"
            onClick={handleRetry}
            className="w-full py-2.5 px-4 rounded-lg bg-terra text-cream font-medium hover:bg-terra-light transition-colors"
          >
            Try Again
          </button>
        </div>
      )}
    </div>
  );
}

// ────────────────────────────────────────────────────
// Step 7: Skeleton Review
// ────────────────────────────────────────────────────

function StepSkeletonReview({
  skeleton,
  onConfirm,
}: {
  skeleton: MacrocycleSkeleton;
  onConfirm: () => void;
}) {
  const confirmPlan = useConfirmPlan();
  const [isConfirming, setIsConfirming] = useState(false);

  const handleConfirm = () => {
    setIsConfirming(true);
    confirmPlan.mutate(skeleton, {
      onSuccess: () => {
        onConfirm();
      },
      onError: () => {
        setIsConfirming(false);
      },
    });
  };

  return (
    <div className="space-y-6">
      {/* Coach message */}
      <div className="rounded-xl bg-forest/5 border border-forest/10 p-5">
        <div className="flex gap-3">
          <div className="flex-shrink-0 mt-0.5">
            <CoachAvatar size="sm" />
          </div>
          <div>
            <p className="text-sm text-charcoal leading-relaxed">
              <span className="font-medium">Coach Jan says:</span>{' '}
              {skeleton.coach_message}
            </p>
          </div>
        </div>
      </div>

      {/* Training blocks */}
      <div className="space-y-3">
        {skeleton.mesocycles.map((meso, i) => (
          <div
            key={meso.sequence_number}
            className="rounded-xl bg-white border border-cream-dark p-5"
          >
            <div className="flex items-start justify-between mb-2">
              <div>
                <p className="text-xs text-slate uppercase tracking-wider font-medium">
                  Training Block {i + 1}
                </p>
                <h4 className="font-serif text-lg text-charcoal font-semibold mt-0.5">
                  {meso.phase.charAt(0).toUpperCase() + meso.phase.slice(1)}
                </h4>
              </div>
            </div>
            <p className="text-sm text-slate leading-relaxed mb-3">
              {meso.focus}
            </p>
            <div className="flex flex-wrap gap-x-4 gap-y-1 text-xs text-slate-light">
              <span>
                {meso.load_weeks} week{meso.load_weeks !== 1 ? 's' : ''} loading + {meso.recovery_weeks} week{meso.recovery_weeks !== 1 ? 's' : ''} recovery
              </span>
              <span>
                ~{Math.round(meso.target_volume_km)} km/week
              </span>
            </div>
          </div>
        ))}
      </div>

      {/* Confirming state */}
      {isConfirming && (
        <div className="flex flex-col items-center py-4">
          <div className="relative mb-4">
            <div className="absolute inset-0 w-10 h-10 rounded-full bg-forest/20 animate-ping" />
            <div className="relative">
              <CoachAvatar size="md" />
            </div>
          </div>
          <p className="text-sm text-charcoal font-medium">Generating your workouts...</p>
        </div>
      )}

      {/* Error state */}
      {confirmPlan.isError && (
        <div className="p-3 rounded-lg bg-terra/10 border border-terra/20 text-terra text-sm">
          {confirmPlan.error?.message ?? 'Failed to confirm plan. Please try again.'}
        </div>
      )}

      {/* Actions */}
      {!isConfirming && (
        <button
          type="button"
          onClick={handleConfirm}
          disabled={confirmPlan.isPending}
          className="w-full py-2.5 px-4 rounded-lg bg-terra text-cream font-medium hover:bg-terra-light transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Confirm Plan
        </button>
      )}
    </div>
  );
}

// ────────────────────────────────────────────────────
// Navigation buttons (shared)
// ────────────────────────────────────────────────────

function StepButtons({ onBack }: { onBack?: () => void }) {
  return (
    <div className="flex gap-3 pt-2">
      {onBack && (
        <button
          type="button"
          onClick={onBack}
          className="flex-1 py-2.5 px-4 rounded-lg border border-cream-dark text-charcoal font-medium hover:bg-cream-dark/50 transition-colors"
        >
          Back
        </button>
      )}
      <button
        type="submit"
        className="flex-1 py-2.5 px-4 rounded-lg bg-forest text-cream font-medium hover:bg-forest-light transition-colors"
      >
        Next
      </button>
    </div>
  );
}

// ────────────────────────────────────────────────────
// Progress indicator
// ────────────────────────────────────────────────────

const STEP_LABELS = ['Basics', 'Physiology', 'Running', 'Goal', 'Review', 'Building', 'Your Plan'];

// Steps 1-5 are "profile setup", steps 6-7 are "plan generation"
const PROFILE_STEP_COUNT = 5;

function StepProgress({ current }: { current: number }) {
  return (
    <div className="flex items-center justify-center gap-2 mb-8">
      {STEP_LABELS.map((label, i) => {
        const step = i + 1;
        const isActive = step === current;
        const isCompleted = step < current;
        const isPlanPhase = step > PROFILE_STEP_COUNT;

        return (
          <div key={label} className="flex items-center gap-2">
            {/* Separator gap between profile and plan phases */}
            {step === PROFILE_STEP_COUNT + 1 && (
              <div className="w-px h-6 bg-cream-dark mx-0.5" />
            )}
            <div className="flex flex-col items-center gap-1">
              <div
                className={`w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium transition-colors ${
                  isActive
                    ? isPlanPhase
                      ? 'bg-terra text-cream'
                      : 'bg-forest text-cream'
                    : isCompleted
                      ? isPlanPhase
                        ? 'bg-terra/20 text-terra'
                        : 'bg-forest/20 text-forest'
                      : 'bg-cream-dark text-slate-light'
                }`}
              >
                {isCompleted ? (
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" strokeWidth={2.5}>
                    <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                  </svg>
                ) : (
                  step
                )}
              </div>
              <span
                className={`text-[10px] ${
                  isActive
                    ? isPlanPhase
                      ? 'text-terra font-medium'
                      : 'text-forest font-medium'
                    : 'text-slate-light'
                }`}
              >
                {label}
              </span>
            </div>
            {i < STEP_LABELS.length - 1 && step !== PROFILE_STEP_COUNT && (
              <div
                className={`w-6 h-px mb-4 ${
                  step < current ? (isPlanPhase ? 'bg-terra/30' : 'bg-forest/30') : 'bg-cream-dark'
                }`}
              />
            )}
          </div>
        );
      })}
    </div>
  );
}

// ────────────────────────────────────────────────────
// Helpers
// ────────────────────────────────────────────────────

function resolveDistance(step4: Step4Data): number {
  const preset = step4.race_distance_preset;
  if (preset === 'custom' || preset === 'Ultra') {
    return step4.race_distance_custom_m ? Number(step4.race_distance_custom_m) : 50000;
  }
  return RACE_PRESETS[preset] ?? 0;
}

function buildPayload(data: AllData): CreateProfileInput {
  const { step1, step2, step3, step4 } = data;

  // Convert pace to m/s
  let ftpace_m_per_s: number | undefined;
  if (step3.pace_minutes && step3.pace_seconds) {
    const totalSeconds = Number(step3.pace_minutes) * 60 + Number(step3.pace_seconds);
    if (totalSeconds > 0) {
      ftpace_m_per_s = 1000 / totalSeconds;
    }
  }

  // Convert target time to seconds
  let target_time_seconds: number | undefined;
  if (step4.target_hours || step4.target_minutes || step4.target_seconds) {
    target_time_seconds =
      (Number(step4.target_hours) || 0) * 3600 +
      (Number(step4.target_minutes) || 0) * 60 +
      (Number(step4.target_seconds) || 0);
  }

  return {
    name: step1.name,
    age: step1.age,
    weight_kg: step1.weight_kg,
    resting_hr: step2.resting_hr,
    max_hr: step2.max_hr,
    lthr: step2.lthr,
    ftpace_m_per_s,
    current_weekly_volume_km: step3.current_weekly_volume_km,
    experience_level: step3.experience_level,
    sports_background: step3.sports_background || undefined,
    race_name: step4.race_name || undefined,
    race_distance_m: resolveDistance(step4),
    race_date: step4.race_date,
    target_time_seconds,
  };
}

// ────────────────────────────────────────────────────
// Main component
// ────────────────────────────────────────────────────

export default function Onboarding() {
  const [step, setStep] = useState(1);
  const [step1Data, setStep1Data] = useState<Step1Data | null>(null);
  const [step2Data, setStep2Data] = useState<Step2Data | null>(null);
  const [step3Data, setStep3Data] = useState<Step3Data | null>(null);
  const [step4Data, setStep4Data] = useState<Step4Data | null>(null);
  const [apiError, setApiError] = useState<string | null>(null);
  const [raceGoal, setRaceGoal] = useState<RaceGoal | null>(null);
  const [skeleton, setSkeleton] = useState<MacrocycleSkeleton | null>(null);

  const navigate = useNavigate();
  const createProfile = useCreateProfile();

  const handleSubmit = () => {
    if (!step1Data || !step2Data || !step3Data || !step4Data) return;
    setApiError(null);

    const payload = buildPayload({
      step1: step1Data,
      step2: step2Data,
      step3: step3Data,
      step4: step4Data,
    });

    createProfile.mutate(payload, {
      onSuccess: (data) => {
        // Save race goal from response, transition to plan generation
        if (data.race_goal) {
          setRaceGoal(data.race_goal);
          setStep(6);
        } else {
          setApiError('Profile created but no race goal found. Please try again.');
        }
      },
      onError: (error) => setApiError(error.message),
    });
  };

  // Determine header text based on phase
  const isProfilePhase = step <= PROFILE_STEP_COUNT;
  const headerTitle = isProfilePhase ? "Let's get started" : 'Building your plan';
  const headerSubtitle = isProfilePhase
    ? 'Tell us about yourself so we can build your training plan'
    : 'Coach Jan is crafting your personalized training';

  // For steps 6-7, don't show the step title inside the card
  const showStepTitle = step <= PROFILE_STEP_COUNT;

  return (
    <div className="min-h-screen bg-cream flex items-center justify-center px-4 py-8">
      <div className="w-full max-w-lg">
        {/* Brand */}
        <div className="text-center mb-2">
          <h1 className="font-serif text-3xl text-forest font-bold tracking-tight">
            {headerTitle}
          </h1>
          <p className="mt-1 text-slate text-sm">
            {headerSubtitle}
          </p>
        </div>

        <StepProgress current={step} />

        {/* Card */}
        <div className="bg-white rounded-2xl shadow-sm border border-cream-dark p-6 sm:p-8">
          {showStepTitle && (
            <h2 className="font-serif text-xl text-charcoal mb-4">
              {STEP_LABELS[step - 1]}
            </h2>
          )}

          {step === 1 && (
            <StepBasicInfo
              onNext={(data) => {
                setStep1Data(data);
                setStep(2);
              }}
              defaultValues={step1Data ?? {}}
            />
          )}

          {step === 2 && (
            <StepPhysiology
              onNext={(data) => {
                setStep2Data(data);
                setStep(3);
              }}
              onBack={() => setStep(1)}
              defaultValues={step2Data ?? {}}
            />
          )}

          {step === 3 && (
            <StepRunning
              onNext={(data) => {
                setStep3Data(data);
                setStep(4);
              }}
              onBack={() => setStep(2)}
              defaultValues={step3Data ?? {}}
            />
          )}

          {step === 4 && (
            <StepGoal
              onNext={(data) => {
                setStep4Data(data);
                setStep(5);
              }}
              onBack={() => setStep(3)}
              defaultValues={step4Data ?? {}}
            />
          )}

          {step === 5 && step1Data && step2Data && step3Data && step4Data && (
            <StepReview
              data={{ step1: step1Data, step2: step2Data, step3: step3Data, step4: step4Data }}
              onBack={() => setStep(4)}
              onSubmit={handleSubmit}
              isSubmitting={createProfile.isPending}
              apiError={apiError}
            />
          )}

          {step === 6 && raceGoal && (
            <StepPlanGenerating
              raceGoalId={raceGoal.id}
              onSuccess={(skel) => {
                setSkeleton(skel);
                setStep(7);
              }}
            />
          )}

          {step === 7 && skeleton && (
            <StepSkeletonReview
              skeleton={skeleton}
              onConfirm={() => {
                navigate('/');
              }}
            />
          )}
        </div>

        {step <= PROFILE_STEP_COUNT && (
          <p className="mt-4 text-center text-xs text-slate-light">
            You can update these details any time from your profile
          </p>
        )}
      </div>
    </div>
  );
}
