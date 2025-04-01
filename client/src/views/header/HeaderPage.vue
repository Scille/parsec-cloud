<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-header
      id="connected-header"
      v-if="showHeader"
    >
      <ion-toolbar
        class="topbar"
        :class="currentRouteIs(Routes.History) ? 'topbar-history' : ''"
      >
        <!-- icon visible when menu is hidden -->
        <ms-image
          v-if="!isMobile() && isLargeDisplay"
          slot="start"
          id="trigger-toggle-menu-button"
          class="topbar-button__item"
          @click="isSidebarMenuVisible() ? hideSidebarMenu() : resetSidebarMenu()"
          :image="SidebarToggle"
        />
        <div class="topbar-left">
          <div
            id="back-block"
            v-if="hasHistory() && !currentRouteIs(Routes.Workspaces)"
          >
            <header-back-button :short="currentRouteIsFileRoute() ? true : false" />
          </div>

          <div
            v-if="!currentRouteIsFileRoute() && isLargeDisplay"
            class="topbar-left__title"
          >
            <ion-label
              class="title-h2"
              :class="hasHistory() ? 'align-center' : 'align-left'"
            >
              {{ $msTranslate(getTitleForRoute()) }}
            </ion-label>
          </div>

          <div
            class="topbar-left__breadcrumb"
            v-if="currentRouteIsFileRoute() && (!currentRouteIs(Routes.Workspaces) || isLargeDisplay)"
          >
            <header-breadcrumbs
              :path-nodes="fullPath"
              @change="onNodeSelected"
            />
          </div>

          <div
            v-if="isSmallDisplay && userInfo && currentRouteIs(Routes.Workspaces)"
            class="topbar-left-workspaces-mobile"
          >
            <ion-text class="topbar-left-workspaces-mobile__orga body">{{ userInfo.organizationId }}</ion-text>
            <ion-title class="topbar-left-workspaces-mobile__title title-h2">{{ $msTranslate('HeaderPage.titles.workspaces') }}</ion-title>
          </div>
        </div>

        <!-- top right icon + profile -->
        <ion-buttons
          slot="primary"
          class="topbar-right"
        >
          <div
            class="topbar-right-button"
            v-if="!currentRouteIs(Routes.History)"
          >
            <invitations-button />

            <ion-button
              v-show="false"
              v-if="!isMobile()"
              slot="icon-only"
              id="trigger-search-button"
              class="topbar-right-button__item"
            >
              <ion-icon
                slot="icon-only"
                :icon="search"
              />
            </ion-button>
            <ion-button
              v-if="!isMobile()"
              slot="icon-only"
              id="trigger-notifications-button"
              class="topbar-right-button__item"
              :class="{
                active: notificationPopoverIsVisible,
                unread: informationManager.notificationManager.hasUnreadNotifications(),
              }"
              @click="openNotificationCenter($event)"
              ref="notificationCenterButton"
            >
              <ion-icon :icon="notifications" />
            </ion-button>
          </div>

          <profile-header
            v-if="isLargeDisplay"
            id="profile-button"
            :name="userInfo ? userInfo.humanHandle.label : ''"
            :email="userInfo ? userInfo.humanHandle.email : ''"
            :profile="userInfo ? userInfo.currentProfile : UserProfile.Outsider"
            class="profile-header"
          />
        </ion-buttons>
      </ion-toolbar>
    </ion-header>

    <ion-content>
      <ion-router-outlet />
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import HeaderBackButton from '@/components/header/HeaderBackButton.vue';
import HeaderBreadcrumbs, { RouterPathNode } from '@/components/header/HeaderBreadcrumbs.vue';
import InvitationsButton from '@/components/header/InvitationsButton.vue';
import { ClientInfo, Path, UserProfile, getClientInfo, isMobile, getWorkspaceName } from '@/parsec';
import {
  Routes,
  currentRouteIs,
  currentRouteIsFileRoute,
  getCurrentRouteName,
  getCurrentRouteParams,
  getDocumentPath,
  getWorkspaceHandle,
  hasHistory,
  navigateTo,
  routerGoBack,
  watchRoute,
  currentRouteIsLoggedRoute,
} from '@/router';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { InformationManager, InformationManagerKey } from '@/services/informationManager';
import useSidebarMenu from '@/services/sidebarMenu';
import { Translatable, MsImage, SidebarToggle, useWindowSize } from 'megashark-lib';
import NotificationCenterPopover from '@/views/header/NotificationCenterPopover.vue';
import ProfileHeader from '@/views/header/ProfileHeader.vue';
import { openSettingsModal } from '@/views/settings';
import {
  IonButton,
  IonButtons,
  IonContent,
  IonHeader,
  IonIcon,
  IonLabel,
  IonPage,
  IonRouterOutlet,
  IonToolbar,
  IonTitle,
  IonText,
  popoverController,
} from '@ionic/vue';
import { home, notifications, search } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, ref, computed } from 'vue';
import { EventDistributor, EventDistributorKey } from '@/services/eventDistributor';

const { isLargeDisplay, isSmallDisplay } = useWindowSize();
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
let hotkeys: HotkeyGroup | null = null;
const workspaceName = ref('');
const { isVisible: isSidebarMenuVisible, reset: resetSidebarMenu, hide: hideSidebarMenu } = useSidebarMenu();
const userInfo: Ref<ClientInfo | null> = ref(null);
const fullPath: Ref<RouterPathNode[]> = ref([]);
const notificationPopoverIsVisible: Ref<boolean> = ref(false);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const notificationCenterButton = ref();
const showHeader = computed(() => {
  return !(currentRouteIs(Routes.Organization) && isSmallDisplay.value);
});

const routeWatchCancel = watchRoute(async () => {
  if (!currentRouteIsLoggedRoute()) {
    return;
  }
  const result = await getClientInfo();
  if (result.ok) {
    userInfo.value = result.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(result.error)}`);
  }
  await updateRoute();
});

async function onNodeSelected(node: RouterPathNode): Promise<void> {
  await navigateTo(node.name as Routes, { params: node.params, query: node.query });
}

async function updateRoute(): Promise<void> {
  if (!currentRouteIsFileRoute()) {
    fullPath.value = [];
    return;
  }

  if (currentRouteIs(Routes.Workspaces)) {
    fullPath.value = [
      {
        id: 0,
        title: 'HeaderPage.titles.workspaces',
        icon: home,
        name: Routes.Workspaces,
        params: {},
      },
    ];
  } else if (currentRouteIs(Routes.Documents)) {
    const workspaceHandle = getWorkspaceHandle();
    if (workspaceHandle) {
      workspaceName.value = await getWorkspaceName(workspaceHandle);
    }

    const finalPath: RouterPathNode[] = [];
    finalPath.push({
      id: 0,
      icon: home,
      name: Routes.Workspaces,
      params: {},
    });

    const rebuildPath: string[] = [];
    const workspacePath = await Path.parse(getDocumentPath());
    finalPath.push({
      id: 1,
      display: workspaceName.value,
      name: Routes.Documents,
      query: { documentPath: '/' },
      params: getCurrentRouteParams(),
    });
    for (let i = 0; i < workspacePath.length; i++) {
      rebuildPath.push(workspacePath[i]);
      finalPath.push({
        id: i + 2,
        display: workspacePath[i],
        name: Routes.Documents,
        query: { documentPath: `/${rebuildPath.join('/')}` },
        params: getCurrentRouteParams(),
      });
    }

    fullPath.value = finalPath;
  }
}

onMounted(async () => {
  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add({ key: ',', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true }, openSettingsModal);
  hotkeys.add(
    { key: 'arrowup', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true },
    async () => await routerGoBack(),
  );
  hotkeys.add(
    { key: 'arrowleft', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true },
    async () => await routerGoBack(),
  );
  hotkeys.add(
    { key: 'n', modifiers: Modifiers.Ctrl | Modifiers.Shift, platforms: Platforms.Desktop, disableIfModal: true },
    async () => await notificationCenterButton.value.$el.click(),
  );

  const result = await getClientInfo();
  if (result.ok) {
    userInfo.value = result.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(result.error)}`);
  }
  await updateRoute();
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  routeWatchCancel();
});

function getTitleForRoute(): Translatable {
  switch (getCurrentRouteName()) {
    case Routes.Users:
      return 'HeaderPage.titles.users';
    case Routes.Storage:
      return 'HeaderPage.titles.organization.storage';
    case Routes.Organization:
      return 'HeaderPage.titles.organization.information';
    case Routes.About:
      return 'HeaderPage.titles.about';
    case Routes.MyProfile:
      return 'HeaderPage.titles.myProfile';
    case Routes.History:
      return 'HeaderPage.titles.history';
    case Routes.Viewer:
      return 'HeaderPage.titles.viewer';
    case null:
      return '';
  }
  return '';
}

async function openNotificationCenter(event: Event): Promise<void> {
  event.stopPropagation();
  notificationPopoverIsVisible.value = true;
  const popover = await popoverController.create({
    component: NotificationCenterPopover,
    alignment: 'center',
    event: event,
    cssClass: 'notification-center-popover',
    showBackdrop: false,
    componentProps: {
      notificationManager: informationManager.notificationManager,
      eventDistributor: eventDistributor,
    },
  });
  await popover.present();
  await popover.onWillDismiss();
  notificationPopoverIsVisible.value = false;
  await popover.dismiss();
}
</script>

<style scoped lang="scss">
.topbar {
  --background: var(--parsec-color-light-secondary-white);
  display: flex;
  padding: 1.5rem 2rem 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem 1.5rem 1rem;
  }

  &-history {
    --background: var(--parsec-color-light-secondary-inversed-contrast);

    .topbar-left {
      min-height: 2.25rem;
    }
  }

  #trigger-toggle-menu-button {
    --fill-color: var(--parsec-color-light-secondary-grey);
    padding: 0.625rem;
    border-radius: var(--parsec-radius-12);
    cursor: pointer;

    &:hover {
      background: var(--parsec-color-light-secondary-premiere);
      --fill-color: var(--parsec-color-light-secondary-hard-grey);
    }
  }
}

.topbar-right {
  display: flex;
  gap: 1.5em;
  margin-inline-end: 0;

  &-button {
    display: flex;
    gap: 1em;
    align-items: center;

    &::after {
      content: '';
      display: block;
      width: 1px;
      height: 1.5em;
      background: var(--parsec-color-light-secondary-light);

      @include ms.responsive-breakpoint('sm') {
        display: none;
      }
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &__item {
      padding: 0.625rem;
      border-radius: var(--parsec-radius-12);
      cursor: pointer;

      &::part(native) {
        --padding-top: 0;
        --padding-start: 0;
        --padding-end: 0;
        --padding-bottom: 0;
        min-height: 0;
      }

      ion-icon {
        color: var(--parsec-color-light-secondary-grey);
        font-size: 1.375rem;
      }

      &:hover {
        --background-hover: var(--parsec-color-light-secondary-premiere);
        background: var(--parsec-color-light-secondary-premiere);
      }

      &.active {
        --background-hover: var(--parsec-color-light-primary-50);
        background: var(--parsec-color-light-primary-50);

        ion-icon {
          color: var(--parsec-color-light-primary-700);
        }
      }

      &#trigger-notifications-button.unread {
        position: relative;

        &::after {
          content: '';
          position: absolute;
          right: 0.45rem;
          top: 0.35rem;
          width: 0.625rem;
          height: 0.625rem;
          background: var(--parsec-color-light-danger-500);
          border: 2px solid var(--parsec-color-light-secondary-inversed-contrast);
          border-radius: var(--parsec-radius-12);
        }
      }
    }
  }
}

.topbar-left {
  display: flex;
  align-items: center;

  .topbar-left-workspaces-mobile {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;

    &__orga {
      color: var(--parsec-color-light-secondary-grey);
    }

    &__title {
      color: var(--parsec-color-light-primary-800);
    }
  }

  &__title {
    width: 100%;
    color: var(--parsec-color-light-primary-600);

    .align-left {
      display: flex;
      justify-content: start;
      align-items: center;
      margin-inline: 1.5rem;
    }

    .align-center {
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100%;
    }
  }

  &__breadcrumb {
    display: flex;
  }
}
</style>
