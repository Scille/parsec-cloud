<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-header>
      <ion-toolbar class="topbar">
        <!-- icon visible when menu is hidden -->
        <ion-buttons slot="start">
          <ion-button
            v-if="!isMobile() && !isSidebarMenuVisible()"
            slot="icon-only"
            id="trigger-toggle-menu-button"
            class="ion-hide-lg-down topbar-button__item"
            @click="resetSidebarMenu()"
          >
            <ion-icon
              slot="icon-only"
              :icon="menu"
            />
          </ion-button>
        </ion-buttons>
        <!-- end of icon visible when menu is hidden -->
        <div class="topbar-left">
          <div
            id="back-block"
            v-if="hasHistory()"
          >
            <header-back-button :short="currentRouteIsFileRoute() ? true : false" />
          </div>

          <div
            v-if="!currentRouteIsFileRoute()"
            class="topbar-left__title"
          >
            <ion-label
              class="title-h2"
              :class="hasHistory() ? 'align-center' : 'align-left'"
            >
              {{ getTitleForRoute() }}
            </ion-label>
          </div>

          <div class="topbar-left__breadcrumb">
            <header-breadcrumbs
              :path-nodes="fullPath"
              @change="onNodeSelected"
            />
          </div>
        </div>

        <!-- icon menu on mobile -->
        <ion-buttons slot="start">
          <ion-menu-button />
        </ion-buttons>
        <!-- end of icon menu on mobile -->

        <!-- top right icon + profile -->
        <ion-buttons
          slot="primary"
          class="topbar-right"
        >
          <div class="topbar-right-button">
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
            >
              <ion-icon :icon="notifications" />
            </ion-button>
          </div>

          <profile-header
            id="profile-button"
            :name="userInfo ? userInfo.humanHandle.label : ''"
            class="profile-header"
          />
        </ion-buttons>
        <!-- top right icon + profil -->
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
import { ClientInfo, Path, WorkspaceName, getClientInfo, getWorkspaceName, isMobile } from '@/parsec';
import {
  Routes,
  currentRouteIs,
  currentRouteIsFileRoute,
  getCurrentRouteName,
  getCurrentRouteParams,
  getDocumentPath,
  getWorkspaceId,
  hasHistory,
  navigateTo,
  watchRoute,
} from '@/router';
import { InformationKey, InformationManager } from '@/services/informationManager';
import useSidebarMenu from '@/services/sidebarMenu';
import { translate } from '@/services/translation';
import NotificationCenterPopover from '@/views/header/NotificationCenterPopover.vue';
import ProfileHeader from '@/views/header/ProfileHeader.vue';
import {
  IonButton,
  IonButtons,
  IonContent,
  IonHeader,
  IonIcon,
  IonLabel,
  IonMenuButton,
  IonPage,
  IonRouterOutlet,
  IonToolbar,
  popoverController,
} from '@ionic/vue';
import { home, menu, notifications, search } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';

const workspaceName: Ref<WorkspaceName> = ref('');
const { isVisible: isSidebarMenuVisible, reset: resetSidebarMenu } = useSidebarMenu();
const userInfo: Ref<ClientInfo | null> = ref(null);
const fullPath: Ref<RouterPathNode[]> = ref([]);
const notificationPopoverIsVisible: Ref<boolean> = ref(false);
const informationManager: InformationManager = inject(InformationKey)!;

const routeWatchCancel = watchRoute(async () => {
  const result = await getClientInfo();
  if (result.ok) {
    userInfo.value = result.value;
  } else {
    console.log('Could not get user info', result.error);
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
        display: translate('HeaderPage.titles.workspaces'),
        icon: home,
        name: Routes.Workspaces,
        params: {},
      },
    ];
  } else if (currentRouteIs(Routes.Documents)) {
    const workspaceId = getWorkspaceId();
    if (workspaceId !== '') {
      const result = await getWorkspaceName(workspaceId);
      if (result.ok) {
        workspaceName.value = result.value;
      } else {
        console.warn('Could not get workspace name', result.error);
      }
    }

    const finalPath: RouterPathNode[] = [];
    finalPath.push({
      id: 0,
      display: '',
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
  const result = await getClientInfo();
  if (result.ok) {
    userInfo.value = result.value;
  } else {
    console.log('Could not get user info', result.error);
  }
  await updateRoute();
});

onUnmounted(async () => {
  routeWatchCancel();
});

function getTitleForRoute(): string {
  switch (getCurrentRouteName()) {
    case Routes.Settings:
      return translate('HeaderPage.titles.settings');
    case Routes.Devices:
      return translate('HeaderPage.titles.devices');
    case Routes.ActiveUsers:
      return translate('HeaderPage.titles.users.activeUsers');
    case Routes.RevokedUsers:
      return translate('HeaderPage.titles.users.revokedUsers');
    case Routes.Invitations:
      return translate('HeaderPage.titles.users.invitations');
    case Routes.Storage:
      return translate('HeaderPage.titles.organization.storage');
    case Routes.Organization:
      return translate('HeaderPage.titles.organization.information');
    case Routes.About:
      return translate('HeaderPage.titles.about');
    case Routes.ContactDetails:
      return translate('HeaderPage.titles.myContactDetails');
    case Routes.RecoveryExport:
      return translate('HeaderPage.titles.recoveryExport');
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
  padding: 1.5rem 2em 1rem;
}

.topbar-right {
  display: flex;
  gap: 1.5em;

  &-button {
    display: flex;
    gap: 1em;
    align-items: center;

    &::after {
      content: '';
      display: block;
      width: 1px;
      height: 1.5em;
      margin: 0 0.5em 0 1em;
      background: var(--parsec-color-light-secondary-light);
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

  &__title {
    width: 100%;
    color: var(--parsec-color-light-primary-600);

    .align-left {
      display: flex;
      justify-content: start;
      align-items: center;
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
