// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect as baseExpect, Locator, Page } from '@playwright/test';
import { dismissToast } from '@tests/e2e/helpers/utils';

interface AssertReturnType {
  message: () => string;
  pass: boolean;
}

export const expect = baseExpect.extend({
  async toHaveDisabledAttribute(locator: Locator): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).toHaveAttribute('disabled');
      return {
        message: () => '',
        pass: true,
      };
    } catch (error: any) {
      return {
        message: () => 'Does not have disabled attribute',
        pass: false,
      };
    }
  },

  async toNotHaveDisabledAttribute(locator: Locator): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).not.toHaveAttribute('disabled');
      return {
        message: () => '',
        pass: true,
      };
    } catch (error: any) {
      return {
        message: () => 'Has disabled attribute',
        pass: false,
      };
    }
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
    let pass = true;
    let isVisible = true;

    try {
      await expect(toast).toHaveCount(1);
    } catch (error: any) {
      isVisible = false;
      errorMessage = 'Toast is not visible';
      pass = false;
    }

    if (pass) {
      try {
        await baseExpect(toast.locator('.toast-message')).toHaveText(message);
      } catch (error: any) {
        errorMessage = `Toast does not contain the text '${message}'. Found: '${error.matcherResult.actual}' instead.`;
        pass = false;
      }
    }

    if (pass) {
      try {
        await expect(toast).toHaveTheClass(`ms-${theme.toLocaleLowerCase()}`);
        if (!pass) {
          errorMessage = `Toast does not have the theme '${theme}'`;
        }
      } catch (error: any) {
        errorMessage = `Toast does not have the theme '${theme}'.`;
        pass = false;
      }
    }

    if (isVisible) {
      await dismissToast(page);
      await expect(toast).toHaveCount(0);
    }

    return {
      message: () => errorMessage,
      pass: pass,
    };
  },

  async toHaveHeader(
    page: Page,
    breadcrumbs: Array<string | RegExp>,
    backButtonVisible: boolean,
    hasHomeButton: boolean,
  ): Promise<AssertReturnType> {
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
      // Couldn't find how to skip the first matching `ion-breadcrumb` so
      // we add an empty element at the start of the provided array
      if (hasHomeButton) {
        breadcrumbs.unshift('');
      }
      const bcs = header.locator('ion-breadcrumbs').locator('ion-breadcrumb');
      expect(bcs).toHaveCount(breadcrumbs.length);
      await expect(bcs).toHaveText(breadcrumbs, { useInnerText: true });
    } catch (error: any) {
      pass = false;
      console.log(error);
      errorMessage = `Invalid breadcrumbs. Expected '[${breadcrumbs}]', got '[${error.matcherResult.actual}]' instead.`;
    }
    return {
      message: () => errorMessage,
      pass: pass,
    };
  },

  async toHavePageTitle(page: Page, title: string): Promise<AssertReturnType> {
    try {
      await expect(page.locator('.topbar-left').locator('.topbar-left__title')).toHaveText(title);
    } catch (error: any) {
      return {
        message: () => `Invalid page title, expected '${title}', got '${error.actual}'`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toHaveState(checkbox: Locator, state: 'indeterminate' | 'unchecked' | 'checked'): Promise<AssertReturnType> {
    try {
      if (state === 'indeterminate') {
        await expect(checkbox).toHaveTheClass('checkbox-indeterminate');
      } else if (state === 'checked') {
        await expect(checkbox).toHaveTheClass('checkbox-checked');
      } else {
        await expect(checkbox).not.toHaveTheClass('checkbox-checked');
        await expect(checkbox).not.toHaveTheClass('checkbox-indeterminate');
      }
    } catch (error: any) {
      return {
        message: () => `Checkbox is not '${state}'`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
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

  async toBeHomePage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/home\??.*$/);
    } catch (error: any) {
      return {
        message: () => `Page is not home page (url is '${error.matcherResult.actual}')`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toBeWorkspacePage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/\d+\/workspaces$/);
    } catch (error: any) {
      console.log(error);
      return {
        message: () => `Page is not workspace page (url is '${error.matcherResult.actual}')`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toBeDocumentPage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/\d+\/documents\?documentPath=\/.*&workspaceHandle=\d+$/);
    } catch (error: any) {
      return {
        message: () => `Page is not documents page (url is '${error.matcherResult.actual}')`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toBeUserPage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/\d+\/users\??.*$/);
    } catch (error: any) {
      return {
        message: () => `Page is not users page (url is '${error.matcherResult.actual}')`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toBeMyProfilePage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/\d+\/myProfile\??.*$/);
    } catch (error: any) {
      return {
        message: () => `Page is not profile page (url is '${error.matcherResult.actual}')`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toBeWorkspaceHistoryPage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/\d+\/history\??.*$/);
    } catch (error: any) {
      return {
        message: () => `Page is not workspace history page (url is '${error.matcherResult.actual}')`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toBeOrganizationPage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/\d+\/organization\??.*$/);
    } catch (error: any) {
      return {
        message: () => `Page is not organization page (url is '${error.matcherResult.actual}')`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toBeViewerPage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/\d+\/viewer\??.*$/);
    } catch (error: any) {
      return {
        message: () => `Page is not viewer page (url is '${error.matcherResult.actual}')`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toBeClientAreaPage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/clientArea\??.*$/);
    } catch (error: any) {
      return {
        message: () => `Page is not client area page (url is '${error.matcherResult.actual}')`,
        pass: false,
      };
    }
    return {
      message: () => '',
      pass: true,
    };
  },

  async toBeTrulyDisabled(locator: Locator): Promise<AssertReturnType> {
    let errorMessage = '';
    let pass = true;
    try {
      await baseExpect(locator).toHaveAttribute('aria-disabled');
    } catch (error: any) {
      errorMessage = "Element does not have a 'disabled' attribute.";
      pass = false;
    }
    return {
      message: () => errorMessage,
      pass: pass,
    };
  },

  async toBeTrulyEnabled(locator: Locator): Promise<AssertReturnType> {
    let errorMessage = '';
    let pass = true;
    try {
      await baseExpect(locator).not.toHaveAttribute('aria-disabled');
    } catch (error: any) {
      errorMessage = "Element has a 'disabled' attribute.";
      pass = false;
    }
    return {
      message: () => errorMessage,
      pass: pass,
    };
  },

  async toShowInformationModal(page: Page, message: string, theme: 'Success' | 'Warning' | 'Error' | 'Info'): Promise<AssertReturnType> {
    const modal = page.locator('.information-modal');
    let errorMessage = '';
    let pass = true;

    try {
      await baseExpect(modal).toBeVisible();
    } catch (error: any) {
      errorMessage = 'Modal is not visible';
      pass = false;
    }

    if (pass) {
      try {
        await baseExpect(modal.locator('.container-textinfo')).toHaveText(message);
      } catch (error: any) {
        errorMessage = `Modal does not contain the text '${message}'. Found: '${error.matcherResult.actual}' instead.`;
        pass = false;
      }
    }

    if (pass) {
      pass = await modal.locator('.container-textinfo').evaluate((node, theme: string) => {
        const expectedClass = `ms-${theme.toLowerCase()}`;
        return node.classList.contains(expectedClass);
      }, theme);
      if (!pass) {
        errorMessage = `Modal does not have the theme '${theme}'`;
      }
    }

    // Close toast
    await modal.locator('#next-button').click();
    await expect(modal).toBeHidden();

    return {
      message: () => errorMessage,
      pass: pass,
    };
  },
});
