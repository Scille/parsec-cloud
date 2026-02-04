// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, TestInfo, test as base } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { MockBms, MockClientAreaOverload, MockRouteOptions } from '@tests/e2e/helpers/bms';
import { CRYPTPAD_SERVER } from '@tests/e2e/helpers/cryptpad';
import { DEFAULT_USER_INFORMATION, generateDefaultOrganizationInformation, generateDefaultUserData } from '@tests/e2e/helpers/data';
import { mockExternalWebsites } from '@tests/e2e/helpers/externalWebsites';
import { mockLibParsec } from '@tests/e2e/helpers/libparsec';
import { dropTestbed, initTestBed } from '@tests/e2e/helpers/testbed';
import { DisplaySize, MsContext, MsPage, SetupOptions } from '@tests/e2e/helpers/types';
import { createWorkspace, fillInputModal, fillIonInput, importDefaultFiles, logout } from '@tests/e2e/helpers/utils';

const DEV_TOOLS_OFFSET = 400;
const DEFAULT_INIT_TIMEOUT = 5000;

export async function setupNewPage(page: MsPage, opts: SetupOptions = {}): Promise<void> {
  const TESTBED_SERVER = process.env.TESTBED_SERVER;
  if (TESTBED_SERVER === undefined) {
    throw new Error('Environ variable `TESTBED_SERVER` must be defined to use testbed');
  }

  const expectTimeout = opts.expectTimeout ?? DEFAULT_INIT_TIMEOUT;

  page.on('console', (msg) => console.log('> ', msg.text()));

  await page.addInitScript(
    (options: SetupOptions & { testbedServer: string }) => {
      async function createMockWritableStream(): Promise<FileSystemWritableFileStream> {
        return {
          write: async (data: any): Promise<any> => {
            if ((window as any).__downloadedFiles === undefined) {
              (window as any).__downloadedFiles = {
                default: data,
              };
            } else {
              (window as any).__downloadedFiles.default = data;
            }
          },
          close: async (): Promise<any> => {},
          abort: async (): Promise<any> => {
            console.log('Stream aborted');
          },
          writable: {
            size: 0,
            getWriter: (): WritableStreamDefaultWriter => {
              return {
                ready: new Promise<void>((resolve) => resolve()),
                close: async (): Promise<void> => {},
                releaseLock: (): void => {},
                write: async (chunk: any): Promise<void> => {
                  if ((window as any).__downloadedFiles === undefined) {
                    (window as any).__downloadedFiles = {
                      default: chunk,
                    };
                  } else {
                    (window as any).__downloadedFiles.default = new Uint8Array([...(window as any).__downloadedFiles.default, ...chunk]);
                  }
                },
              } as WritableStreamDefaultWriter;
            },
          },
        } as unknown as FileSystemWritableFileStream;
      }

      (window as any).TESTING = true;
      (window as any).TESTING_DISABLE_STRIPE = !options.enableStripe;
      if (options.enableUpdateEvent) {
        (window as any).TESTING_ENABLE_UPDATE_EVENT = options.enableUpdateEvent;
      }
      if (options.mockPki) {
        (window as any).TESTING_PKI = true;
      }
      if (options.openBaoServer) {
        (window as any).TESTING_OPEN_BAO_SERVER = options.openBaoServer;
      }
      if (options.parsecAccountAutoLogin) {
        options.withParsecAccount = true;
        (window as any).TESTING_ACCOUNT_AUTO_LOGIN = true;
      }
      if (options.withParsecAccount) {
        (window as any).TESTING_ENABLE_ACCOUNT = true;
        (window as any).TESTING_ACCOUNT_SERVER = options.testbedServer;
      }
      if (options.withEditics) {
        (window as any).TESTING_ENABLE_EDITICS = true;
        (window as any).TESTING_CRYPTPAD_SERVER = `https://${options.cryptpadServer}`;
        (window as any).TESTING_EDITICS_SAVE_TIMEOUT = 1;
      }
      if (options.withCustomBranding) {
        (window as any).TESTING_ENABLE_CUSTOM_BRANDING = true;
      }
      if (options.mockBrowser) {
        (window as any).TESTING_MOCK_BROWSER = options.mockBrowser;
      }
      if (options.saasServers) {
        (window as any).TESTING_SAAS_SERVERS = options.saasServers;
      }
      if (options.trialServers) {
        (window as any).TESTING_TRIAL_SERVERS = options.trialServers;
      }
      (window as any).showSaveFilePicker = (opts?: { suggestedName?: string }): Promise<FileSystemFileHandle> => {
        return new Promise((resolve) => {
          resolve({
            kind: 'file',
            name: opts?.suggestedName ?? 'downloadedFile.tmp',
            createWritable: createMockWritableStream,
            remove: async (): Promise<undefined> => {
              if ((window as any).__downloadedFiles) {
                (window as any).__downloadedFiles.default = undefined;
              }
            },
          } as unknown as FileSystemFileHandle);
        });
      };
      (window as any).showDirectoryPicker = async (): Promise<FileSystemDirectoryHandle> => {
        return {
          kind: 'directory',
          name: 'folder',
          getFileHandle: async (name: string, _options?: FileSystemGetFileOptions): Promise<FileSystemFileHandle> => {
            return {
              kind: 'file',
              name: name,
              createWritable: async () => ({
                write: async (data: Uint8Array): Promise<any> => {
                  if ((window as any).__downloadedFiles === undefined) {
                    (window as any).__downloadedFiles = {
                      [name]: data,
                    };
                  } else {
                    (window as any).__downloadedFiles[name] = new Uint8Array([...(window as any).__downloadedFiles[name], ...data]);
                  }
                },
                close: async (): Promise<any> => {},
              }),
            } as FileSystemFileHandle;
          },
          getDirectoryHandle: async (_name: string, _options?: FileSystemGetDirectoryOptions): Promise<FileSystemDirectoryHandle> => {
            return {} as FileSystemDirectoryHandle;
          },
          removeEntry: async (_name: string, _options?: FileSystemRemoveOptions): Promise<void> => {},
          resolve: async (_descendant: FileSystemHandle): Promise<string[] | null> => null,
          isSameEntry: async (_other: FileSystemHandle): Promise<boolean> => false,
        };
      };
    },
    { ...opts, testbedServer: TESTBED_SERVER },
  );
  if (!opts.skipGoto) {
    await page.goto(opts.location ?? '/');
  }
  await page.waitForLoadState('domcontentloaded');

  // Wait for app to initialize with longer timeout for CI stability
  await expect(page.locator('#app')).toHaveAttribute('app-state', 'initializing', { timeout: expectTimeout });

  if (opts.libparsecMockFunctions) {
    await mockLibParsec(page, opts.libparsecMockFunctions);
  }

  let testbed: string | undefined;
  if (!opts.skipTestbed) {
    testbed = await initTestBed(page, opts.testbedPath);
  } else {
    await page.evaluate(async () => {
      const [, nextStage] = window.nextStageHook();
      await nextStage(undefined, 'en-US');
    });
  }

  page.userData = generateDefaultUserData();
  page.orgInfo = generateDefaultOrganizationInformation();
  page.isReleased = false;
  if (!page.skipTestbedRelease && opts.skipTestbed) {
    page.skipTestbedRelease = true;
  }
  page.openNewTab = async (newOpts?: SetupOptions): Promise<MsPage> => {
    const newTab = (await page.context().newPage()) as MsPage;
    newTab.skipTestbedRelease = true;
    await setupNewPage(newTab, newOpts !== undefined ? { ...newOpts, testbedPath: testbed } : { ...opts, testbedPath: testbed });
    return newTab;
  };
  page.release = async (): Promise<void> => {
    if (!page.isReleased) {
      if (!page.skipTestbedRelease) {
        await dropTestbed(page);
      }
      await page.close();
    }
    page.isReleased = true;
  };

  page.isDebugEnabled = (): boolean => {
    return process.env.PWDEBUG === 'true';
  };
  page.defaultLargeSize = [1600, 900];
  page.defaultSmallSize = [700, 700];
  page.setDisplaySize = async (displaySize: DisplaySize): Promise<void> => {
    page.displaySize = displaySize;
    const width =
      (displaySize === DisplaySize.Large ? page.defaultLargeSize[0] : page.defaultSmallSize[0]) +
      (page.isDebugEnabled() ? DEV_TOOLS_OFFSET : 0);
    const height = displaySize === DisplaySize.Large ? page.defaultLargeSize[1] : page.defaultSmallSize[1];
    await page.setViewportSize({ width: width, height: height });
  };
  page.getDisplaySize = async (): Promise<DisplaySize> => {
    return page.displaySize;
  };
  await page.setDisplaySize(opts.displaySize ?? DisplaySize.Large);
}

const debugTest = base.extend({
  ...(process.env.PWDEBUG && {
    browser: [
      async ({ playwright, browserName }, use): Promise<any> => {
        const args = [];
        // cspell:disable-next-line
        if (browserName.toLowerCase().includes('chrom')) {
          args.push('--auto-open-devtools-for-tabs');
        }
        const browser = await playwright[browserName].launch({
          headless: false,
          args: args,
        });
        await use(browser);
        await browser.close();
      },
      {
        scope: 'worker',
        timeout: 0,
      },
    ],
  }),
});

interface ClientAreaInitialParams {
  mocks?: Partial<
    Record<
      keyof typeof MockBms,
      {
        overload?: MockClientAreaOverload;
        options?: MockRouteOptions;
      }
    >
  >;
  rememberMe?: boolean;
}

export const msTest = debugTest.extend<{
  context: MsContext;
  home: MsPage;
  secondTab: MsPage;
  connected: MsPage;
  workspacesStandard: MsPage;
  workspacesExternal: MsPage;
  workspaces: MsPage;
  documentsOptions: { empty?: boolean };
  documents: MsPage;
  documentsReadOnly: MsPage;
  usersPage: MsPage;
  organizationPage: MsPage;
  organizationPageStandard: MsPage;
  invitationsPage: MsPage;
  myProfilePage: MsPage;
  userJoinModal: Locator;
  userGreetModal: Locator;
  createOrgModal: Locator;
  deviceGreetModal: Locator;
  workspaceSharingModal: Locator;
  clientArea: MsPage;
  clientAreaCustomOrder: MsPage;
  clientAreaInitialParams: ClientAreaInitialParams;
  parsecAccount: MsPage;
  parsecAccountLoggedIn: MsPage;
  parsecEditics: MsPage;
}>({
  context: async ({ browser }, use) => {
    const context = (await browser.newContext()) as MsContext;
    await context.grantPermissions(['clipboard-read']);
    await mockExternalWebsites(context);
    await use(context);
    await context.close();
  },

  home: async ({ context }, use) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page);
    await use(page);
    await page.release();
  },

  connected: async ({ home }, use) => {
    await home.locator('.organization-card').first().click();
    await expect(home.locator('#password-input')).toBeVisible();

    await expect(home.locator('.login-button')).toHaveDisabledAttribute();

    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
    await expect(home).toBeWorkspacePage();

    await use(home);
  },

  workspacesStandard: async ({ home }, use) => {
    await home.locator('.organization-card').nth(1).click();
    await expect(home.locator('#password-input')).toBeVisible();

    await expect(home.locator('.login-button')).toHaveDisabledAttribute();

    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home.locator('.topbar-right').locator('.text-content-name')).toHaveText('Boby McBobFace');
    await expect(home).toBeWorkspacePage();

    await use(home);
  },

  workspacesExternal: async ({ home }, use) => {
    await home.locator('.organization-card').nth(2).click();
    await expect(home.locator('#password-input')).toBeVisible();

    await expect(home.locator('.login-button')).toHaveDisabledAttribute();

    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home.locator('.topbar-right').locator('.text-content-name')).toHaveText('Malloryy McMalloryFace');
    await expect(home).toBeWorkspacePage();

    await use(home);
  },

  workspaces: async ({ connected }, use) => {
    use(connected);
  },

  documentsOptions: [
    async ({}, use: any): Promise<void> => {
      await use({ empty: false });
    },
    { option: true },
  ],

  documents: async ({ workspaces, documentsOptions }, use, testInfo: TestInfo) => {
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(workspaces).toHaveHeader(['wksp1'], true, true);
    await expect(workspaces.locator('.folder-container').locator('.no-files')).toBeVisible();
    if (!documentsOptions.empty) {
      await importDefaultFiles(workspaces, testInfo);
      await expect(workspaces.locator('.folder-container').locator('.no-files')).toBeHidden();
      await expect(workspaces.locator('.file-list-item')).toHaveCount(9);
      await expect(workspaces.locator('.file-list-item').nth(0).locator('.label-name')).toHaveText('Dir_Folder');
    }
    use(workspaces);
  },

  documentsReadOnly: async ({ home }, use, testInfo: TestInfo) => {
    // Log in with Bob
    await home.locator('.organization-card').nth(1).click();
    await expect(home.locator('#password-input')).toBeVisible();
    await expect(home.locator('.login-button')).toHaveDisabledAttribute();
    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home).toBeWorkspacePage();

    await createWorkspace(home, 'wksp2-readonly');
    await home.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1).click();
    await expect(home).toBeDocumentPage();
    await expect(home).toHaveHeader(['wksp2-readonly'], true, true);
    await expect(home.locator('.folder-container').locator('.no-files')).toBeVisible();
    await importDefaultFiles(home, testInfo);
    await expect(home.locator('.folder-container').locator('.no-files')).toBeHidden();

    await home.locator('.topbar-left').locator('.back-button').click();
    await expect(home).toBeWorkspacePage();

    await home
      .locator('.workspaces-container-grid')
      .locator('.workspace-card-item')
      .nth(1)
      .locator('.workspace-card-bottom__icons')
      .locator('.icon-share-container')
      .nth(0)
      .click();
    const sharingModal = home.locator('.workspace-sharing-modal');
    await expect(sharingModal).toBeVisible();
    // Share the workspace with Alice
    const content = sharingModal.locator('.ms-modal-content');
    const users = content.locator('.user-list-suggestions').locator('.user-list-suggestions-item');
    await users.nth(0).locator('.dropdown-button').click();
    const roleDropdown = sharingModal.page().locator('.dropdown-popover');
    const roles = roleDropdown.getByRole('listitem');
    // Set reader
    await roles.nth(3).click();
    await expect(sharingModal.page()).toShowToast("Alicey McAliceFace's role has been updated to Reader.", 'Success');
    await sharingModal.locator('.closeBtn').click();
    await expect(sharingModal).toBeHidden();
    await logout(home);

    await expect(home).toBeHomePage();
    await home.locator('.organization-card').nth(0).click();
    await expect(home.locator('#password-input')).toBeVisible();
    await expect(home.locator('.login-button')).toHaveDisabledAttribute();
    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home).toBeWorkspacePage();

    await home.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1).click();
    await expect(home).toBeDocumentPage();
    await expect(home).toHaveHeader(['wksp2-readonly'], true, true);
    await expect(home.locator('.folder-container').locator('.no-files-content')).toBeHidden();
    use(home);
  },

  usersPage: async ({ connected }, use) => {
    await connected.locator('.sidebar').locator('#sidebar-users').click();
    await expect(connected).toHavePageTitle('Users');
    await expect(connected).toBeUserPage();
    use(connected);
  },

  organizationPage: async ({ connected }, use) => {
    await connected.locator('.sidebar').locator('#sidebar-organization-information').click();
    await expect(connected).toHavePageTitle('Information');
    await expect(connected).toBeOrganizationPage();
    use(connected);
  },

  organizationPageStandard: async ({ workspacesStandard }, use) => {
    await workspacesStandard.locator('.sidebar').locator('#sidebar-organization-information').click();
    await expect(workspacesStandard).toHavePageTitle('Information');
    await expect(workspacesStandard).toBeOrganizationPage();
    use(workspacesStandard);
  },

  invitationsPage: async ({ connected }, use) => {
    await connected.locator('.sidebar').locator('#sidebar-invitations').click();
    await expect(connected).toHavePageTitle('Invitations & Requests');
    await expect(connected).toBeInvitationPage();
    use(connected);
  },

  myProfilePage: async ({ connected }, use) => {
    await connected.locator('.topbar').locator('.profile-header').click();
    const myProfileButton = connected.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(0);
    await expect(myProfileButton).toHaveText('Settings');
    await myProfileButton.click();
    await expect(connected).toHavePageTitle('My profile');
    await expect(connected).toBeMyProfilePage();
    use(connected);
  },

  userJoinModal: async ({ home }, use) => {
    // cspell:disable-next-line
    const INVITATION_LINK = 'parsec3://parsec.cloud/Test?a=claim_user&p=xBBHJlEjlpxNZYTCvBWWDPIS';

    await home.locator('#create-organization-button').click();
    await expect(home.locator('.homepage-popover')).toBeVisible();
    await home.locator('.homepage-popover').getByRole('listitem').nth(1).click();
    await fillInputModal(home, INVITATION_LINK);
    const modal = home.locator('.join-organization-modal');
    await expect(home.locator('.join-organization-modal')).toBeVisible();
    await use(modal);
  },

  createOrgModal: async ({ home }, use) => {
    await home.locator('#create-organization-button').click();
    await expect(home.locator('.homepage-popover')).toBeVisible();
    await home.locator('.homepage-popover').getByRole('listitem').nth(0).click();
    const modal = home.locator('.create-organization-modal');
    await expect(home.locator('.create-organization-modal')).toBeVisible();
    await use(modal);
  },

  userGreetModal: async ({ connected }, use) => {
    await connected.locator('.topbar').locator('#invitations-button').click();
    const greetButton = connected.locator('.invitations-list-popover ').getByRole('listitem').nth(0).locator('ion-button').nth(2);
    await expect(greetButton).toHaveText('Greet');
    await greetButton.click();
    const modal = connected.locator('.greet-organization-modal');
    await use(modal);
  },

  deviceGreetModal: async ({ connected }, use) => {
    await connected.locator('.topbar').locator('.profile-header').click();
    const myProfileButton = connected.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(1);
    await expect(myProfileButton).toHaveText('My devices');
    await myProfileButton.click();
    await expect(connected).toHavePageTitle('My profile');
    await expect(connected).toBeMyProfilePage();
    await expect(connected.locator('.menu-list__item').nth(1)).toHaveText('My devices');
    await connected.locator('.menu-list__item').nth(1).click();
    await connected.locator('#add-device-button').click();
    const modal = connected.locator('.greet-organization-modal');
    await expect(modal).toBeVisible();
    await use(modal);
  },

  workspaceSharingModal: async ({ workspaces }, use) => {
    await workspaces
      .locator('.workspaces-container-grid')
      .locator('.workspace-card-item')
      .nth(0)
      .locator('.workspace-card-bottom__icons')
      .locator('.icon-share-container')
      .nth(0)
      .click();
    const modal = workspaces.locator('.workspace-sharing-modal');
    await expect(modal).toBeVisible();
    await use(modal);
  },

  clientArea: async ({ context, clientAreaInitialParams }, use) => {
    const home = (await context.newPage()) as MsPage;
    await setupNewPage(home, { enableStripe: true });

    home.userData.reset();
    await MockBms.mockUserRoute(home);
    for (const key of Object.keys(MockBms) as (keyof typeof MockBms)[]) {
      if (key !== 'mockUserRoute') {
        const overload = clientAreaInitialParams?.mocks?.[key]?.overload;
        const options = clientAreaInitialParams?.mocks?.[key]?.options;
        await MockBms[key](home, overload as any, options);
      }
    }

    const button = home.locator('.homepage-menu-secondary').locator('#trigger-customer-area-button');
    await expect(button).toHaveText('Customer area');
    await button.click();
    const saasContainer = home.locator('.saas-login');
    await expect(saasContainer).toBeVisible();
    await expect(saasContainer.locator('.saas-login__title')).toHaveText('Log in to your customer account');
    const loginButton = saasContainer.locator('.saas-login-button').locator('ion-button');
    await expect(loginButton).toBeTrulyDisabled();
    await fillIonInput(saasContainer.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(saasContainer.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);

    // Check "remember me" checkbox so tokens persist across page reloads
    if (clientAreaInitialParams?.rememberMe) {
      const rememberMeCheckbox = saasContainer.locator('.ms-checkbox');
      await expect(rememberMeCheckbox).toBeVisible();
      await rememberMeCheckbox.check();
      await expect(rememberMeCheckbox).toBeChecked();
    }

    await expect(loginButton).toHaveText('Log in');
    await expect(loginButton).toBeTrulyEnabled();
    await loginButton.click();
    await expect(home).toHaveURL(/.+\/clientArea$/);

    // Switch to first org
    const orgSwitchButton = home.locator('.sidebar-header').locator('.card-header-title');
    await expect(orgSwitchButton).toBeVisible();
    const popover = home.locator('.popover-switch');
    await expect(popover).toBeHidden();
    await orgSwitchButton.click();
    await expect(popover).toBeVisible();
    const orgs = popover.locator('.organization-list').getByRole('listitem');
    await orgs.nth(0).click();
    await expect(popover).toBeHidden();
    await expect(orgSwitchButton).toHaveText(home.orgInfo.name);

    await use(home);
    await home.release();
  },

  clientAreaCustomOrder: async ({ context, clientAreaInitialParams }, use) => {
    const home = (await context.newPage()) as MsPage;
    await setupNewPage(home, { enableStripe: true });

    home.userData.reset();
    await MockBms.mockUserRoute(home, { billingSystem: 'CUSTOM_ORDER' });
    for (const key of Object.keys(MockBms) as (keyof typeof MockBms)[]) {
      if (key !== 'mockUserRoute') {
        const overload = clientAreaInitialParams?.mocks?.[key]?.overload;
        const options = clientAreaInitialParams?.mocks?.[key]?.options;
        await MockBms[key](home, overload as any, options);
      }
    }

    const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
    await expect(button).toHaveText('Customer area');
    await button.click();
    const saasContainer = home.locator('.saas-login');
    await expect(saasContainer).toBeVisible();
    await expect(saasContainer.locator('.saas-login__title')).toHaveText('Log in to your customer account');
    const loginButton = saasContainer.locator('.saas-login-button').locator('ion-button');
    await expect(loginButton).toBeTrulyDisabled();
    await fillIonInput(saasContainer.locator('.saas-login-content').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(saasContainer.locator('.saas-login-content').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);

    // Check "remember me" checkbox so tokens persist across page reloads
    if (clientAreaInitialParams?.rememberMe) {
      const rememberMeCheckbox = saasContainer.locator('.ms-checkbox');
      await expect(rememberMeCheckbox).toBeVisible();
      await rememberMeCheckbox.check();
      await expect(rememberMeCheckbox).toBeChecked();
    }

    await expect(loginButton).toHaveText('Log in');
    await expect(loginButton).toBeTrulyEnabled();
    await loginButton.click();
    await expect(home).toHaveURL(/.+\/clientArea$/);

    await use(home);
    await home.release();
  },

  clientAreaInitialParams: [
    async ({}, use): Promise<void> => {
      await use({
        mocks: undefined,
        rememberMe: false,
      });
    },
    { option: true },
  ],

  parsecAccount: async ({ context }, use) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page, { withParsecAccount: true });
    await expect(page).toHaveURL(/.+\/account$/);
    await use(page);
    await page.release();
  },

  parsecAccountLoggedIn: async ({ context }, use) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page, { withParsecAccount: true, parsecAccountAutoLogin: true, location: '/home' });
    await expect(page).toHaveURL(/.+\/home$/);
    await use(page);
    await page.release();
  },

  parsecEditics: async ({ context, documentsOptions }, use, testInfo: TestInfo) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page, {
      withParsecAccount: false,
      withEditics: true,
      location: '/home',
      cryptpadServer: CRYPTPAD_SERVER,
      expectTimeout: 15000,
    });
    await page.locator('.organization-card').first().click();
    await expect(page.locator('#password-input')).toBeVisible();

    await expect(page.locator('.login-button')).toHaveDisabledAttribute();

    await page.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(page.locator('.login-button')).toBeEnabled();
    await page.locator('.login-button').click();
    await expect(page.locator('#connected-header')).toContainText('My workspaces');
    await expect(page.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
    await expect(page).toBeWorkspacePage();
    await expect(page).toHaveURL(/.+\/workspaces$/);
    await page.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(page).toHaveHeader(['wksp1'], true, true);
    await expect(page.locator('.folder-container').locator('.no-files')).toBeVisible();
    if (!documentsOptions.empty) {
      await importDefaultFiles(page, testInfo);
      await expect(page.locator('.folder-container').locator('.no-files')).toBeHidden();
    }
    await use(page);
  },
});
