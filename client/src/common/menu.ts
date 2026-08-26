// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { openBugReportModal } from '@/components/misc';
import { UserProfile } from '@/parsec';
import { libparsec, Platform } from '@/plugins/libparsec';
import { getConnectionHandle, navigateTo, ProfilePages, Routes, watchRoute } from '@/router';
import { Env } from '@/services/environment';
import { Events } from '@/services/eventDistributor';
import { InjectionProvider } from '@/services/injectionProvider';
import { openAboutModal } from '@/views/about';
import { openSettingsModal } from '@/views/settings';
import { I18n } from 'megashark-lib';
import { ref } from 'vue';

// Keep in sync with Electron/menu.ts
interface ParsecMenuAction {
  type: 'action';
  label: string;
  accelerator?: string;
  action?: string;
  children?: ParsecMenuItem[];
  disabled?: boolean;
  hidden?: boolean;
}

interface ParsecMenuSeparator {
  type: 'separator';
}

export type ParsecMenuItem = ParsecMenuAction | ParsecMenuSeparator;

const modalOpened = ref(false);
const popoverOpened = ref(false);
const loggedIn = ref(false);

function _onModalClose(): void {
  modalOpened.value = false;
  setupAppMenu();
}

function _onModalOpen(): void {
  modalOpened.value = true;
  setupAppMenu();
}

function _onPopoverClose(): void {
  popoverOpened.value = false;
  setupAppMenu();
}

function _onPopoverOpen(): void {
  popoverOpened.value = true;
  setupAppMenu();
}

document.addEventListener('ionModalDidPresent', _onModalOpen);
document.addEventListener('ionModalWillDismiss', _onModalClose);
document.addEventListener('ionPopoverDidPresent', _onPopoverOpen);
document.addEventListener('ionPopoverWillDismiss', _onPopoverClose);

watchRoute(async (newRoute) => {
  const old = loggedIn.value;
  loggedIn.value = newRoute.params.handle !== undefined;
  if (old !== loggedIn.value) {
    setupAppMenu();
  }
});

export enum MenuBarActions {
  OpenSettings = 'parsec-open-settings',
  OpenAbout = 'parsec-open-about',
  LogOut = 'parsec-log-out',
  GoToWorkspaces = 'parsec-go-to-workspaces',
  GoToUsers = 'parsec-go-to-users',
  GoToProfile = 'parsec-go-to-profile',
  OpenDocumentation = 'parsec-open-documentation',
  ContactUs = 'parsec-contact-us',
  ReportBug = 'parsec-report-bug',
  CloseApp = 'parsec-close-app',
}

export async function dispatchMenuAction(injectionProvider: InjectionProvider, action: MenuBarActions): Promise<void> {
  switch (action) {
    case MenuBarActions.OpenAbout: {
      if (loggedIn.value) {
        await navigateTo(Routes.MyProfile, { query: { profilePage: ProfilePages.About } });
      } else {
        await openAboutModal();
      }
      break;
    }
    case MenuBarActions.OpenSettings: {
      if (loggedIn.value) {
        await navigateTo(Routes.MyProfile, { query: { profilePage: ProfilePages.Settings } });
      } else {
        await openSettingsModal();
      }
      break;
    }
    case MenuBarActions.LogOut: {
      if (!loggedIn.value) {
        window.nativeAPI.log('warn', 'Log out action while not being logged in');
        break;
      }
      const handle = getConnectionHandle();
      if (!handle) {
        break;
      }
      injectionProvider.getInjections(handle).eventDistributor.dispatchEvent(Events.LogoutRequested);
      break;
    }
    case MenuBarActions.GoToProfile: {
      if (!loggedIn.value) {
        window.nativeAPI.log('warn', 'Log out action while not being logged in');
        break;
      }
      await navigateTo(Routes.MyProfile);
      break;
    }
    case MenuBarActions.GoToUsers: {
      if (!loggedIn.value) {
        window.nativeAPI.log('warn', 'Log out action while not being logged in');
        break;
      }
      await navigateTo(Routes.Users);
      break;
    }
    case MenuBarActions.GoToWorkspaces: {
      if (!loggedIn.value) {
        window.nativeAPI.log('warn', 'Log out action while not being logged in');
        break;
      }
      await navigateTo(Routes.Workspaces);
      break;
    }
    case MenuBarActions.OpenDocumentation: {
      await Env.Links.openDocumentationUserGuideLink('introduction');
      break;
    }
    case MenuBarActions.ContactUs: {
      await Env.Links.openContactLink();
      break;
    }
    case MenuBarActions.ReportBug: {
      await openBugReportModal();
      break;
    }
    default: {
      break;
    }
  }
}

export async function setupAppMenu(): Promise<void> {
  if (window.getPlatform() !== Platform.MacOS) {
    return;
  }
  const handle = getConnectionHandle();
  let profile: UserProfile | undefined = undefined;

  if (handle) {
    const result = await libparsec.clientInfo(handle);
    if (result.ok) {
      profile = result.value.currentProfile;
    }
  }
  window.nativeAPI.setupActionMenu([
    {
      type: 'action',
      label: I18n.translate('app.name'),
      children: [
        {
          type: 'action',
          label: I18n.translate('appMenu.main.settings'),
          action: MenuBarActions.OpenSettings,
          disabled: modalOpened.value || popoverOpened.value,
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.main.about'),
          action: MenuBarActions.OpenAbout,
          disabled: modalOpened.value || popoverOpened.value,
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.main.logOut'),
          action: MenuBarActions.LogOut,
          disabled: modalOpened.value || popoverOpened.value || !loggedIn.value,
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.main.quit'),
          accelerator: 'Cmd+Q',
          action: MenuBarActions.CloseApp,
        },
      ],
    },
    {
      type: 'action',
      label: I18n.translate('appMenu.edit.name'),
      children: [
        {
          type: 'action',
          label: I18n.translate('appMenu.edit.cut'),
          accelerator: 'Cmd+X',
          action: 'cut',
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.edit.copy'),
          accelerator: 'Cmd+C',
          action: 'copy',
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.edit.paste'),
          accelerator: 'Cmd+V',
          action: 'paste',
        },
      ],
    },
    {
      type: 'action',
      label: I18n.translate('appMenu.window.name'),
      children: [
        {
          type: 'action',
          label: I18n.translate('appMenu.window.minimize'),
          action: 'minimize',
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.window.zoomIn'),
          action: 'zoomIn',
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.window.zoomOut'),
          action: 'zoomOut',
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.window.resetZoom'),
          action: 'resetZoom',
        },
      ],
    },
    {
      type: 'action',
      label: I18n.translate('appMenu.go.name'),
      children: [
        {
          type: 'action',
          label: I18n.translate('appMenu.go.workspaces'),
          action: MenuBarActions.GoToWorkspaces,
          disabled: modalOpened.value || popoverOpened.value || !loggedIn.value,
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.go.users'),
          action: MenuBarActions.GoToUsers,
          disabled: modalOpened.value || popoverOpened.value || !loggedIn.value,
          hidden: profile === UserProfile.Outsider,
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.go.profile'),
          action: MenuBarActions.GoToProfile,
          disabled: modalOpened.value || popoverOpened.value || !loggedIn.value,
        },
      ],
    },
    {
      type: 'action',
      label: I18n.translate('appMenu.help.name'),
      children: [
        {
          type: 'action',
          label: I18n.translate('appMenu.help.documentation'),
          action: MenuBarActions.OpenDocumentation,
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.help.contact'),
          action: MenuBarActions.ContactUs,
        },
        {
          type: 'action',
          label: I18n.translate('appMenu.help.bugReport'),
          action: MenuBarActions.ReportBug,
          disabled: modalOpened.value || popoverOpened.value,
        },
      ],
    },
  ]);
}
