import { test, expect } from '@playwright/test';

const TEST_EMAIL = `e2e-${Date.now()}@test.com`;
const TEST_PASSWORD = 'test-password-123';

test.describe('Authentication', () => {
  test('register → onboarding → zones → profile → logout → login', async ({ page }) => {
    // ─── Step 1: Navigate to login page ───
    await page.goto('/login');
    await expect(page.getByText('CoachJan')).toBeVisible();
    await expect(page.getByText('Welcome back')).toBeVisible();

    // ─── Step 2: Switch to register mode ───
    await page.getByRole('button', { name: 'Create one' }).click();
    await expect(page.getByRole('heading', { name: 'Create account' })).toBeVisible();

    // ─── Step 3: Register ───
    await page.getByLabel('Email').fill(TEST_EMAIL);
    await page.getByLabel('Password', { exact: true }).fill(TEST_PASSWORD);
    await page.getByLabel('Confirm password').fill(TEST_PASSWORD);
    await page.getByRole('button', { name: 'Create account' }).click();

    // Should redirect to onboarding
    await expect(page).toHaveURL('/onboarding', { timeout: 10000 });

    // ─── Step 4: Complete onboarding Step 1 (Basic Info) ───
    await page.getByLabel('Full name').fill('E2E Test Runner');
    await page.getByLabel('Age').fill('30');
    await page.getByLabel('Weight (kg)').fill('70');
    await page.getByRole('button', { name: 'Next' }).click();

    // ─── Step 5: Complete onboarding Step 2 (Physiology) ───
    await page.getByLabel('Resting heart rate (bpm)').fill('50');
    await page.getByLabel('Max heart rate (bpm)').fill('190');
    await page.getByLabel('Lactate threshold HR (bpm)').fill('170');
    await page.getByRole('button', { name: 'Next' }).click();

    // ─── Step 6: Complete onboarding Step 3 (Running) ───
    await page.getByLabel('Current weekly volume (km)').fill('40');
    await page.getByLabel('Experience level').selectOption('intermediate');
    await page.getByRole('button', { name: 'Next' }).click();

    // ─── Step 7: Complete onboarding Step 4 (Goal) ───
    await page.getByLabel('Race date').fill('2026-09-27');
    // Select a race distance from the dropdown
    await page.getByLabel('Race distance').selectOption('Marathon');
    await page.getByRole('button', { name: 'Next' }).click();

    // ─── Step 8: Review and submit ───
    // Should see review step with the data we entered
    await expect(page.getByText('E2E Test Runner')).toBeVisible();
    await expect(page.getByText('170 bpm')).toBeVisible();
    await page.getByRole('button', { name: /create profile/i }).click();

    // Should redirect to dashboard
    await expect(page).toHaveURL('/', { timeout: 10000 });

    // ─── Step 9: Navigate to profile and check zones ───
    // Click profile nav item (desktop sidebar has tooltip "Profile")
    await page.getByTitle('Profile').click();
    await expect(page).toHaveURL('/profile');

    // Should see HR zone table
    await expect(page.getByText('Heart Rate Zones')).toBeVisible();

    // ─── Step 10: Logout ───
    await page.getByTitle('Log out').click();

    // Should redirect to login
    await expect(page).toHaveURL('/login', { timeout: 10000 });
    await expect(page.getByText('Welcome back')).toBeVisible();

    // ─── Step 11: Login again ───
    await page.getByLabel('Email').fill(TEST_EMAIL);
    await page.getByLabel('Password').fill(TEST_PASSWORD);
    await page.getByRole('button', { name: 'Sign in' }).click();

    // Should redirect to dashboard (has profile now, skips onboarding)
    await expect(page).toHaveURL('/', { timeout: 10000 });
  });
});
