// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect as baseExpect, Locator, Page } from '@playwright/test';
import { dismissToast, DisplaySize, MsPage } from '@tests/e2e/helpers';

interface AssertReturnType {
  message: () => string;
  pass: boolean;
}

// Helper function for file handler page assertions
async function checkFileHandlerPage(page: MsPage, mode?: 'view' | 'edit'): Promise<AssertReturnType> {
  try {
    if (mode) {
      await baseExpect(page).toHaveURL(new RegExp(`\\/\\d+\\/fileHandler\\/${mode}\\??.*$`));
    } else {
      await baseExpect(page).toHaveURL(/\/\d+\/fileHandler\/(view|edit)\??.*$/);
    }

    if ((await page.getDisplaySize()) === DisplaySize.Large) {
      // Ensure header is visible before checking title
      const isTopbarVisible = await page.locator('#connected-header .topbar').isVisible();
      const fileViewerButton = page.locator('.file-handler-topbar-buttons__item.toggle-menu');
      let topbarToggled = false;
      if (!isTopbarVisible && fileViewerButton) {
        await fileViewerButton.click();
        topbarToggled = true;
      }

      if (topbarToggled) {
        await fileViewerButton.click();
      }
    }
  } catch (error: any) {
    const modeText = mode ? ` in ${mode} mode` : '';
    return {
      message: () => `Page is not file handler page${modeText} : '${error.matcherResult?.expected}' VS '${error.matcherResult?.actual}')`,
      pass: false,
    };
  }
  return {
    message: () => '',
    pass: true,
  };
}

export const expect = baseExpect.extend({
  async toHaveDisabledAttribute(locator: Locator): Promise<AssertReturnType> {
    try {
      await baseExpect(locator).toHaveAttribute('disabled');
      return {
        message: () => '',
        pass: true,
      };
    } catch (_error: any) {
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
    } catch (_error: any) {
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
    } catch (_error: any) {
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
      } catch (_error: any) {
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
    } catch (_error: any) {
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
      await expect(page.locator('.topbar-left').locator('.topbar-left-text__title')).toHaveText(title);
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
    const STATUSES: Record<string, string> = {
      indeterminate: 'mixed',
      checked: 'true',
      unchecked: 'false',
    };
    await baseExpect(checkbox).toHaveAttribute('aria-checked', STATUSES[state]);
    return {
      pass: true,
      message: () => '',
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

  async toBeInvitationPage(page: Page): Promise<AssertReturnType> {
    try {
      await expect(page).toHaveURL(/\/\d+\/invitations\??.*$/);
    } catch (error: any) {
      return {
        message: () => `Page is not invitations page (url is '${error.matcherResult.actual}')`,
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

  async toBeViewerPage(page: MsPage): Promise<AssertReturnType> {
    return await checkFileHandlerPage(page, 'view');
  },

  async toBeEditorPage(page: MsPage): Promise<AssertReturnType> {
    return await checkFileHandlerPage(page, 'edit');
  },

  async toBeFileHandlerPage(page: MsPage, mode?: 'view' | 'edit'): Promise<AssertReturnType> {
    return await checkFileHandlerPage(page, mode);
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
    } catch (_error: any) {
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
    } catch (_error: any) {
      errorMessage = "Element has a 'disabled' attribute.";
      pass = false;
    }
    return {
      message: () => errorMessage,
      pass: pass,
    };
  },

  async toShowInformationModal(
    page: Page,
    message: string,
    theme: 'Success' | 'Warning' | 'Error' | 'Info',
    title?: string,
  ): Promise<AssertReturnType> {
    const modal = page.locator('.information-modal');
    let errorMessage = '';
    let pass = true;

    try {
      await baseExpect(modal).toBeVisible();
    } catch (_error: any) {
      errorMessage = 'Modal is not visible';
      pass = false;
    }

    if (pass && title) {
      try {
        await baseExpect(modal.locator('.ms-modal-header__title')).toHaveText(title);
      } catch (error: any) {
        errorMessage = `Modal does not contain the title '${title}'. Found: '${error.matcherResult.actual}' instead.`;
        pass = false;
      }
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

    // Close modal
    await modal.locator('#next-button').click();
    await expect(modal).toBeHidden();

    return {
      message: () => errorMessage,
      pass: pass,
    };
  },

  async toHaveNotification(page: MsPage, expectedText): Promise<AssertReturnType> {
    const header = page.locator('#connected-header');
    const notifButton = header.locator('#trigger-notifications-button');
    await baseExpect(header).toBeVisible();
    await baseExpect(notifButton).toBeVisible();
    const classList = await notifButton.evaluate((node) => Array.from(node.classList.values()));
    baseExpect(classList).toContain('unread');
    await notifButton.click();
    const notifCenter = page.locator('.notification-center-popover');
    await baseExpect(notifCenter).toBeVisible();
    const items = notifCenter.locator('.notification-container');
    const messages = await items.locator('.notification-details__message').allInnerTexts();
    baseExpect(messages.some((m) => m === expectedText)).toBeTruthy();
    return {
      message: () => '',
      pass: true,
    };
  },

  async toHaveAuthentication(
    authRadio: Locator,
    state?: { passwordDisabled?: boolean; ssoDisabled?: boolean; pkiDisabled?: boolean; keyringDisabled?: boolean },
  ): Promise<AssertReturnType> {
    await baseExpect(authRadio).toHaveCount(4);
    await baseExpect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('Password');
    await baseExpect(authRadio.nth(1).locator('.authentication-card-text__title')).toHaveText('System authentication');
    await baseExpect(authRadio.nth(2).locator('.authentication-card-text__title')).toHaveText('Smartcard');
    await baseExpect(authRadio.nth(3).locator('.authentication-card-text__title')).toHaveText('Single Sign-On');

    if (state?.passwordDisabled) {
      await baseExpect(authRadio.nth(0)).toHaveClass(/radio-disabled/);
    } else {
      await baseExpect(authRadio.nth(0)).not.toHaveClass(/radio-disabled/);
    }
    if (state?.keyringDisabled) {
      await baseExpect(authRadio.nth(1)).toHaveClass(/radio-disabled/);
      await baseExpect(authRadio.nth(1).locator('.authentication-card-text__description')).toHaveText('Unavailable on web');
    } else {
      await baseExpect(authRadio.nth(1)).not.toHaveClass(/radio-disabled/);
    }
    if (state?.pkiDisabled) {
      await baseExpect(authRadio.nth(2)).toHaveClass(/radio-disabled/);
      await baseExpect(authRadio.nth(2).locator('.authentication-card-text__description')).toHaveText(
        'Smartcard authentication is unavailable.',
      );
    } else {
      await baseExpect(authRadio.nth(2)).not.toHaveClass(/radio-disabled/);
      await baseExpect(authRadio.nth(2).locator('.authentication-card-text__description')).toHaveText('Login with an external account');
    }
    if (state?.ssoDisabled) {
      await baseExpect(authRadio.nth(3)).toHaveClass(/radio-disabled/);
      await baseExpect(authRadio.nth(3).locator('.authentication-card-text__description')).toHaveText(
        'This method is not allowed by this server.',
      );
    } else {
      await baseExpect(authRadio.nth(3)).not.toHaveClass(/radio-disabled/);
      await baseExpect(authRadio.nth(3).locator('.authentication-card-text__description')).toHaveText('Login with an external account');
    }
    return {
      message: () => '',
      pass: true,
    };
  },
});
