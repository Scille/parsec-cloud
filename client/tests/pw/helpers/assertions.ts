// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect as baseExpect, Locator, Page } from '@playwright/test';

interface AssertReturnType {
  message: () => string;
  pass: boolean;
}

export const expect = baseExpect.extend({
  async toHaveDisabledAttribute(locator: Locator): Promise<AssertReturnType> {
    const attr1 = await locator.getAttribute('disabled');
    const attr2 = await locator.getAttribute('aria-disabled');
    return {
      message: () => 'failed',
      pass: attr1 !== null || (attr2 !== null && attr2 === 'true'),
    };
  },

  async toHaveTheClass(locator: Locator, className: string): Promise<AssertReturnType> {
    const classList = await locator.evaluate((node) => Array.from(node.classList.values()));
    const pass = classList.includes(className);
    return {
      message: () => `Does not have class '${className}'. Found classes: ${classList}`,
      pass: pass,
    };
  },

  async toShowToast(page: Page, message: string, theme: 'Success' | 'Warning' | 'Error' | 'Info'): Promise<AssertReturnType> {
    const toast = page.locator('.notification-toast');
    let errorMessage = '';
    let pass = await toast.evaluate((node, theme: string) => {
      const expectedClass = `ms-${theme.toLowerCase()}`;
      return node.classList.contains(expectedClass);
    }, theme);

    if (pass) {
      try {
        await baseExpect(toast.locator('.toast-message')).toHaveText(message);
      } catch (error: any) {
        errorMessage = `Toast does not contain the text '${message}'. Found: '${error.matcherResult.actual}' instead.`;
        pass = false;
      }
    } else {
      errorMessage = `Toast does not have the theme '${theme}'`;
    }

    return {
      message: () => errorMessage,
      pass: pass,
    };
  },

  async toHaveHeader(page: Page, breadcrumbs: string[], backButtonVisible: boolean): Promise<AssertReturnType> {
    const header = page.locator('#connected-header').locator('.topbar-left');
    const backButton = header.locator('.back-button');
    let pass = true;
    let errorMessage = '';
    try {
      if (backButtonVisible) {
        await baseExpect(backButton).toBeVisible();
      } else {
        await baseExpect(backButton).toBeHidden();
      }
    } catch (error: any) {
      pass = false;
      errorMessage = `Back button is ${backButtonVisible ? 'hidden' : 'visible'}`;
    }
    try {
      const bcs = header.locator('ion-breadcrumb');
      await expect(bcs).toHaveCount(breadcrumbs.length);
      await expect(bcs).toHaveText(breadcrumbs);
    } catch (error: any) {
      pass = false;
      errorMessage = `Invalid breadcrumbs. Expected '[${breadcrumbs}]', got '[${error.matcherResult.actual}]' instead.`;
    }
    return {
      message: () => errorMessage,
      pass: pass,
    };
  },

  async toHaveWizardStepper(locator: Locator, titles: string[], currentIndex: number): Promise<AssertReturnType> {
    const wizard = locator.locator('.ms-wizard-stepper');
    const steps = wizard.locator('.ms-wizard-stepper__step');
    let pass = true;
    let errorMessage = '';
    try {
      await expect(steps).toHaveCount(titles.length);
      await expect(steps.locator('.step-title')).toHaveText(titles);
    } catch (error: any) {
      pass = false;
      errorMessage = `Invalid stepper titles. Expected '${error.matcherResult.expected}', got '${error.matcherResult.actual}' instead.`;
    }
    if (pass) {
      try {
        for (let i = 0; i < titles.length; i++) {
          if (i < currentIndex) {
            await expect(steps.nth(i).locator('.ms-wizard-stepper-step').locator('.done')).toBeVisible();
          } else if (i === currentIndex) {
            await expect(steps.nth(i).locator('.ms-wizard-stepper-step').locator('.active')).toBeVisible();
          } else {
            await expect(steps.nth(i).locator('.ms-wizard-stepper-step').locator('.default')).toBeVisible();
          }
        }
      } catch (error: any) {
        console.log(error);
        pass = false;
        errorMessage = `Invalid step. ${currentIndex} should be active.`;
      }
    }
    return {
      message: () => errorMessage,
      pass: pass,
    };
  },
});
