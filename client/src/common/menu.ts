// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { openBugReportModal } from '@/components/misc';
import { isElectron, UserProfile } from '@/parsec';
import { libparsec, Platform } from '@/plugins/libparsec';
import { getConnectionHandle, navigateTo, ProfilePages, Routes, watchRoute } from '@/router';
import { Env } from '@/services/environment';
import { Events } from '@/services/eventDistributor';
import { InjectionProvider } from '@/services/injectionProvider';
import { openAboutModal } from '@/views/about';
import { openSettingsModal } from '@/views/settings';
import { I18n, useTheme } from 'megashark-lib';
import { ref, watch } from 'vue';

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
const overlaysWithBackdrop = new Set<EventTarget>();

function _sendWindowTheme(): void {
  if (!isElectron()) {
    return;
  }
  window.nativeAPI.setWindowTheme(useTheme().isDarkTheme.value ? 'dark' : 'light', overlaysWithBackdrop.size > 0);
}

function _onOverlayOpen(event: Event): void {
  if ((event.target as HTMLIonModalElement | HTMLIonPopoverElement).showBackdrop) {
    overlaysWithBackdrop.add(event.target as EventTarget);
    _sendWindowTheme();
  }
}

function _onOverlayClose(event: Event): void {
  if (overlaysWithBackdrop.delete(event.target as EventTarget)) {
    _sendWindowTheme();
  }
}

function _onModalClose(event: Event): void {
  modalOpened.value = false;
  setupAppMenu();
  _onOverlayClose(event);
}

function _onModalOpen(event: Event): void {
  modalOpened.value = true;
  setupAppMenu();
  _onOverlayOpen(event);
}

function _onPopoverClose(event: Event): void {
  popoverOpened.value = false;
  setupAppMenu();
  _onOverlayClose(event);
}

function _onPopoverOpen(event: Event): void {
  popoverOpened.value = true;
  setupAppMenu();
  _onOverlayOpen(event);
}

document.addEventListener('ionModalWillPresent', _onModalOpen);
document.addEventListener('ionModalWillDismiss', _onModalClose);
document.addEventListener('ionPopoverWillPresent', _onPopoverOpen);
document.addEventListener('ionPopoverWillDismiss', _onPopoverClose);

// Must be called after the ThemeManager is created, `themeSource` being what "system" is resolved from
export function setupWindowTheme(): void {
  watch(useTheme().isDarkTheme, _sendWindowTheme, { immediate: true });
}

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
