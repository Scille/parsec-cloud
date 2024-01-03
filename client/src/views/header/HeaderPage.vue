<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-header>
      <ion-toolbar class="topbar">
        <!-- icon visible when menu is hidden -->
        <ion-buttons slot="start">
          <ion-button
            v-if="!isPlatform('mobile') && !isSidebarMenuVisible()"
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
            <header-back-button :short="isDocumentRoute() ? true : false" />
          </div>

          <div
            v-if="!isDocumentRoute()"
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
          <div
            v-show="false"
            class="topbar-button__list"
          >
            <ion-button
              v-if="!isPlatform('mobile')"
              slot="icon-only"
              id="trigger-search-button"
              class="topbar-button__item"
            >
              <ion-icon
                slot="icon-only"
                :icon="search"
              />
            </ion-button>
            <ion-button
              v-if="!isPlatform('mobile')"
              slot="icon-only"
              id="trigger-notifications-button"
              class="topbar-button__item"
            >
              <ion-icon
                slot="icon-only"
                :icon="notifications"
              />
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
import { ClientInfo, Path, WorkspaceID, WorkspaceName, getClientInfo, getWorkspaceName, hasHistory, isDocumentRoute } from '@/parsec';
import { routerNavigateTo } from '@/router';
import useSidebarMenu from '@/services/sidebarMenu';
import { translate } from '@/services/translation';
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
  isPlatform,
} from '@ionic/vue';
import { home, menu, notifications, search } from 'ionicons/icons';
import { Ref, onMounted, onUnmounted, ref, watch } from 'vue';
import { RouteLocationNormalizedLoaded, useRoute } from 'vue-router';

const currentRoute = useRoute();
const workspaceName: Ref<WorkspaceName> = ref('');
const { isVisible: isSidebarMenuVisible, reset: resetSidebarMenu } = useSidebarMenu();
const userInfo: Ref<ClientInfo | null> = ref(null);
const fullPath: Ref<RouterPathNode[]> = ref([]);

const unwatchRoute = watch(currentRoute, async (newRoute) => {
  await parseRoute(newRoute);
});

async function onNodeSelected(node: RouterPathNode): Promise<void> {
  routerNavigateTo(node.name, node.params, node.query);
}

async function parseRoute(route: RouteLocationNormalizedLoaded): Promise<void> {
  if (route.query && route.query.workspaceId) {
    const result = await getWorkspaceName(route.query.workspaceId as WorkspaceID);
    if (result.ok) {
      workspaceName.value = result.value;
    } else {
      console.log('Could not get workspace name', result.error);
    }
  }

  // Parse path and remove /deviceId
  const routePath = (await Path.parse(currentRoute.path)).slice(1);
  const finalPath: RouterPathNode[] = [];

  // If route is 'workspaces' and we have an id, we're visiting a workspace
  if (routePath[0] === 'workspaces') {
    // Always put the document route first
    finalPath.push({
      id: 0,
      display: routePath.length >= 2 ? '' : translate('HeaderPage.titles.workspaces'),
      icon: home,
      name: 'workspaces',
      params: { deviceId: currentRoute.params.deviceId },
    });

    if (routePath.length >= 2 && currentRoute.query.path) {
      const rebuildPath: string[] = [];
      const workspacePath = await Path.parse(currentRoute.query.path as string);
      finalPath.push({
        id: 1,
        display: workspaceName.value,
        name: 'folder',
        query: { path: '/' },
        params: currentRoute.params,
      });
      for (let i = 0; i < workspacePath.length; i++) {
        rebuildPath.push(workspacePath[i]);
        finalPath.push({
          id: i + 2,
          display: workspacePath[i],
          name: 'folder',
          query: { path: `/${rebuildPath.join('/')}` },
          params: currentRoute.params,
        });
      }
    }
  }
  fullPath.value = finalPath;
}

onMounted(async () => {
  const result = await getClientInfo();
  if (result.ok) {
    userInfo.value = result.value;
  } else {
    console.log('Could not get user info', result.error);
  }
  await parseRoute(currentRoute);
});

onUnmounted(async () => {
  unwatchRoute();
});

function getTitleForRoute(): string {
  const route = currentRoute.name;

  if (route === 'settings') {
    return translate('HeaderPage.titles.settings');
  } else if (route === 'devices') {
    return translate('HeaderPage.titles.devices');
  } else if (route === 'activeUsers') {
    return translate('HeaderPage.titles.users.activeUsers');
  } else if (route === 'revokedUsers') {
    return translate('HeaderPage.titles.users.revokedUsers');
  } else if (route === 'invitations') {
    return translate('HeaderPage.titles.users.invitations');
  } else if (route === 'storage') {
    return translate('HeaderPage.titles.organization.storage');
  } else if (route === 'organization') {
    return translate('HeaderPage.titles.organization.information');
  } else if (route === 'about') {
    return translate('HeaderPage.titles.about');
  } else if (route === 'myContactDetails') {
    return translate('HeaderPage.titles.myContactDetails');
  } else if (route === 'recoveryExport') {
    return translate('HeaderPage.titles.recoveryExport');
  }

  return '';
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
}

.topbar-button__list {
  display: flex;
  gap: 1.5em;
  align-items: center;
  &::after {
    content: '';
    display: block;
    width: 1px;
    height: 1.5em;
    margin: 0 0.5em 0 1em;
    background: var(--parsec-color-light-secondary-light);
  }
}

.topbar-button__item,
.sc-ion-buttons-md-s .button {
  border: 1px solid var(--parsec-color-light-secondary-light);
  color: var(--parsec-color-light-primary-700);
  border-radius: 50%;
  --padding-top: 0;
  --padding-end: 0;
  --padding-bottom: 0;
  --padding-start: 0;
  width: 3em;
  height: 3em;

  &:hover {
    --background-hover: var(--parsec-color-light-primary-50);
    background: var(--parsec-color-light-primary-50);
    border: var(--parsec-color-light-primary-50);
  }

  .button-native {
    --padding-top: 0;
    --padding-end: 0;
    --padding-bottom: 0;
    --padding-start: 0;
  }

  ion-icon {
    font-size: 1.375rem;
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
