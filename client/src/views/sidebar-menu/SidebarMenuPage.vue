<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <div
      class="divider"
      ref="divider"
    />
    <ion-split-pane content-id="main">
      <ion-menu
        content-id="main"
        class="sidebar"
      >
        <ion-header class="sidebar-header">
          <div v-show="!currentRouteIsOrganizationManagementRoute()">
            <!-- active organization -->
            <ion-card class="organization-card">
              <ion-card-header
                class="organization-card-header"
                :class="{ 'organization-card-header-hover': isDesktop() }"
                @click="openOrganizationChoice($event)"
              >
                <div class="header-container">
                  <ion-avatar class="orga-avatar">
                    <span>{{ userInfo ? userInfo.organizationId.substring(0, 2) : '' }}</span>
                  </ion-avatar>
                  <div class="orga-text">
                    <ion-card-subtitle class="caption-info">
                      {{ $t('HomePage.organizationActionSheet') }}
                    </ion-card-subtitle>
                    <ion-card-title class="title-h4">
                      {{ userInfo?.organizationId }}
                    </ion-card-title>
                  </div>
                </div>
                <div
                  class="header-icon"
                  v-show="isDesktop()"
                >
                  <ms-image :image="CaretExpand" />
                </div>
              </ion-card-header>

              <div
                class="organization-card-manageBtn"
                v-show="userInfo && userInfo.currentProfile != UserProfile.Outsider"
                @click="navigateTo(Routes.Users)"
              >
                <ion-text
                  class="subtitles-sm"
                  button
                >
                  {{
                    userInfo && userInfo.currentProfile === UserProfile.Admin
                      ? $t('SideMenu.manageOrganization')
                      : $t('SideMenu.organizationInfo')
                  }}
                </ion-text>
              </div>
            </ion-card>
            <!-- end of active organization -->
          </div>
          <div v-show="currentRouteIsOrganizationManagementRoute()">
            <div
              class="back-organization"
              @click="navigateTo(Routes.Workspaces)"
            >
              <ion-button
                fill="clear"
                class="back-button"
              >
                <ion-icon :icon="chevronBack" />
                {{ $t('SideMenu.back') }}
              </ion-button>
            </div>
          </div>
        </ion-header>

        <ion-content class="ion-padding">
          <div
            v-show="!currentRouteIsOrganizationManagementRoute()"
            class="workspaces-organization"
          >
            <!-- list of workspaces -->
            <ion-list class="list-workspaces">
              <ion-header
                lines="none"
                button
                class="list-workspaces-header menu-default"
              >
                <span
                  @click="goToWorkspaceList()"
                  class="list-workspaces-header__title"
                >
                  {{ $t('SideMenu.workspaces') }}
                </span>
                <ion-icon
                  class="list-workspaces-header__button"
                  id="new-workspace"
                  :icon="add"
                  v-show="userInfo && userInfo.currentProfile !== UserProfile.Outsider"
                  @click="createWorkspace"
                />
              </ion-header>
              <ion-text
                class="body list-workspaces__no-workspace"
                v-if="workspaces.length === 0"
              >
                {{ $t('SideMenu.noWorkspace') }}
              </ion-text>
              <ion-item
                lines="none"
                button
                v-for="workspace in workspaces"
                :key="workspace.handle"
                @click="goToWorkspace(workspace.handle)"
                :class="currentRouteIsWorkspaceRoute(workspace.handle) ? 'item-selected' : 'item-not-selected'"
                class="sidebar-item menu-default"
              >
                <ion-icon
                  :icon="business"
                  slot="start"
                />
                <ion-label>{{ workspace.currentName }}</ion-label>
              </ion-item>
            </ion-list>
            <!-- list of workspaces -->
          </div>
          <div
            v-show="currentRouteIsOrganizationManagementRoute()"
            class="manage-organization"
          >
            <ion-label class="title-h4">
              {{
                userInfo && userInfo.currentProfile === UserProfile.Admin
                  ? $t('SideMenu.manageOrganization')
                  : $t('SideMenu.organizationInfo')
              }}
            </ion-label>
            <!-- user actions -->
            <ion-list class="manage-organization-list users">
              <ion-item
                lines="none"
                class="sidebar-item users-title menu-default"
                :class="currentRouteIsUserRoute() ? 'item-selected' : 'item-not-selected'"
                @click="navigateTo(Routes.Users)"
              >
                <ion-icon
                  :icon="people"
                  slot="start"
                />
                <ion-label>{{ $t('SideMenu.users') }}</ion-label>
              </ion-item>
            </ion-list>
            <!-- storage -->
            <ion-list
              class="manage-organization-list storage"
              v-show="userInfo && userInfo.currentProfile === UserProfile.Admin && false"
            >
              <ion-item
                lines="none"
                class="sidebar-item storage-title menu-default"
                :class="currentRouteIs(Routes.Storage) ? 'item-selected' : 'item-not-selected'"
                @click="navigateTo(Routes.Storage)"
              >
                <ion-icon
                  :icon="pieChart"
                  slot="start"
                />
                <ion-label> {{ $t('SideMenu.storage') }}</ion-label>
              </ion-item>
            </ion-list>
            <!-- org info -->
            <ion-list class="manage-organization-list organization">
              <ion-item
                lines="none"
                class="sidebar-item organization-title menu-default"
                :class="currentRouteIs(Routes.Organization) ? 'item-selected' : 'item-not-selected'"
                @click="navigateTo(Routes.Organization)"
              >
                <ion-icon
                  :icon="informationCircle"
                  slot="start"
                />
                <ion-label>{{ $t('SideMenu.organizationInfo') }}</ion-label>
              </ion-item>
            </ion-list>
          </div>
        </ion-content>
      </ion-menu>

      <ion-router-outlet id="main" />
    </ion-split-pane>
  </ion-page>
</template>

<script setup lang="ts">
import { workspaceNameValidator } from '@/common/validators';
import { CaretExpand, MsImage, getTextInputFromUser } from '@/components/core';
import OrganizationSwitchPopover from '@/components/organizations/OrganizationSwitchPopover.vue';
import {
  ClientInfo,
  UserProfile,
  WorkspaceHandle,
  WorkspaceInfo,
  isDesktop,
  getClientInfo as parsecGetClientInfo,
  listWorkspaces as parsecListWorkspaces,
} from '@/parsec';
import {
  Routes,
  currentRouteIs,
  currentRouteIsOrganizationManagementRoute,
  currentRouteIsUserRoute,
  currentRouteIsWorkspaceRoute,
  navigateTo,
  navigateToWorkspace,
  switchOrganization,
  watchOrganizationSwitch,
} from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import useSidebarMenu from '@/services/sidebarMenu';
import { translate } from '@/services/translation';
import {
  GestureDetail,
  IonAvatar,
  IonButton,
  IonCard,
  IonCardHeader,
  IonCardSubtitle,
  IonCardTitle,
  IonContent,
  IonHeader,
  IonIcon,
  IonItem,
  IonLabel,
  IonList,
  IonMenu,
  IonPage,
  IonRouterOutlet,
  IonSplitPane,
  IonText,
  createGesture,
  menuController,
  popoverController,
} from '@ionic/vue';
import { add, business, chevronBack, informationCircle, people, pieChart } from 'ionicons/icons';
import { Ref, WatchStopHandle, inject, onMounted, onUnmounted, ref, watch } from 'vue';

const workspaces: Ref<Array<WorkspaceInfo>> = ref([]);
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
let eventDistributorCbId: string | null = null;
const divider = ref();
const { defaultWidth, initialWidth, computedWidth, wasReset } = useSidebarMenu();
const userInfo: Ref<ClientInfo | null> = ref(null);

// watching wasReset value
const resetWatchCancel: WatchStopHandle = watch(wasReset, (value) => {
  if (value) {
    resizeMenu(defaultWidth);
    wasReset.value = false;
  }
});

async function goToWorkspace(workspaceHandle: WorkspaceHandle): Promise<void> {
  await navigateToWorkspace(workspaceHandle);
  await menuController.close();
}

async function goToWorkspaceList(): Promise<void> {
  await navigateTo(Routes.Workspaces);
  await menuController.close();
}

async function createWorkspace(): Promise<void> {
  const workspaceName = await getTextInputFromUser({
    title: translate('WorkspacesPage.CreateWorkspaceModal.pageTitle'),
    trim: true,
    validator: workspaceNameValidator,
    inputLabel: translate('WorkspacesPage.CreateWorkspaceModal.label'),
    placeholder: translate('WorkspacesPage.CreateWorkspaceModal.placeholder'),
    okButtonText: translate('WorkspacesPage.CreateWorkspaceModal.create'),
  });

  if (workspaceName) {
    await navigateTo(Routes.Workspaces, { query: { workspaceName: workspaceName } });
  }
}

const organizationWatchCancel = watchOrganizationSwitch(loadAll);

async function loadAll(): Promise<void> {
  const infoResult = await parsecGetClientInfo();

  if (infoResult.ok) {
    userInfo.value = infoResult.value;
  } else {
    console.log('Failed to get user info', infoResult.error);
  }

  const result = await parsecListWorkspaces();
  if (result.ok) {
    workspaces.value = result.value;
  } else {
    console.log('Failed to list workspaces', result.error);
  }
}

onMounted(async () => {
  eventDistributorCbId = await eventDistributor.registerCallback(Events.WorkspaceCreated, async (event: Events, _data: EventData) => {
    if (event === Events.WorkspaceCreated) {
      await loadAll();
    }
  });
  await loadAll();
  if (divider.value) {
    const gesture = createGesture({
      gestureName: 'resize-menu',
      el: divider.value,
      onEnd,
      onMove,
    });
    gesture.enable();
  }
});

onUnmounted(() => {
  if (eventDistributorCbId) {
    eventDistributor.removeCallback(eventDistributorCbId);
  }
  resetWatchCancel();
  organizationWatchCancel();
});

function onMove(detail: GestureDetail): void {
  requestAnimationFrame(() => {
    let currentWidth = initialWidth.value + detail.deltaX;
    if (currentWidth >= 4 && currentWidth <= 500) {
      if (currentWidth <= 150) {
        currentWidth = 4;
      }
      resizeMenu(currentWidth);
      computedWidth.value = currentWidth;
    }
  });
}

function onEnd(): void {
  initialWidth.value = computedWidth.value;
}

function resizeMenu(newWidth: number): void {
  document.documentElement.style.setProperty('--parsec-sidebar-menu-width', `${newWidth}px`);
}

async function openOrganizationChoice(event: Event): Promise<void> {
  if (!isDesktop()) {
    return;
  }
  const popover = await popoverController.create({
    component: OrganizationSwitchPopover,
    cssClass: 'dropdown-popover',
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  const { data } = await popover.onDidDismiss();
  await popover.dismiss();
  if (data) {
    switchOrganization(data.handle);
  }
}
</script>

<style lang="scss" scoped>
.divider {
  width: 8px;
  height: 100%;
  position: absolute;
  left: calc(var(--parsec-sidebar-menu-width) - 4px);
  top: 0;
  z-index: 10000;
  cursor: ew-resize;
  display: flex;
  justify-content: center;

  &::after {
    content: '';
    width: 4px;
    height: 100%;
    padding: 20rem 0;
  }

  &:hover::after,
  &:active::after {
    background: var(--parsec-color-light-primary-200);
  }
}

.sidebar,
.sidebar ion-content {
  --background: var(--parsec-color-light-primary-800);
  --padding-end: 0;
  --padding-start: 0;
  --padding-top: 0;
}

.sidebar {
  border: none;
  user-select: none;
  border-radius: 0 0.5rem 0.5rem 0;

  &-header {
    padding: 0.5rem;
  }

  // logo parsec
  &::after {
    content: url('@/assets/images/background/logo-icon-white.svg');
    opacity: 0.03;
    width: 100%;
    max-width: 270px;
    max-height: 170px;
    display: block;
    position: fixed;
    bottom: 16px;
    right: -60px;
  }

  .workspaces-organization {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 0 1.25rem;
  }
}

.organization-card-header-hover:hover {
  cursor: pointer;
  background: var(--parsec-color-light-primary-30-opacity15);
}

.organization-card {
  --background: var(--parsec-color-light-primary-30-opacity15);
  box-shadow: none;
  margin: 0;

  &-header {
    display: flex;
    justify-content: space-between;
    flex-direction: row;
  }

  .header-container {
    box-shadow: none;
    display: flex;
    align-items: center;
    justify-content: left;
    gap: 0.75em;
    position: relative;
    z-index: 2;
    min-width: 0;

    .orga-avatar {
      background-color: var(--parsec-color-light-primary-800);
      color: var(--parsec-color-light-primary-50);
      width: 42px;
      height: 42px;
      border-radius: 50%;
      text-align: center;
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      position: relative;
      z-index: 1;
    }

    .orga-text {
      display: flex;
      flex-direction: column;
      white-space: nowrap;
      overflow: hidden;
      ion-card-subtitle {
        --color: var(--parsec-color-light-secondary-light);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }

      ion-card-title {
        padding: 0.1875em 0;
        margin: 0;
        --color: var(--parsec-color-light-primary-50);
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }
    }
  }

  .header-icon {
    white-space: nowrap;
    display: flex;
    align-items: center;
    background-color: transparent;
  }

  &-manageBtn {
    padding: 0.625em 1em;
    cursor: pointer;
    align-items: center;
    color: var(--parsec-color-light-secondary-light);
    border-top: 1px solid var(--parsec-color-light-primary-30-opacity15);

    &:hover {
      background: var(--parsec-color-light-primary-30-opacity15);
    }

    ion-text {
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }
  }
}

.back-organization {
  display: flex;
  align-items: center;
  align-self: stretch;
  padding: 1rem;
  color: var(--parsec-color-light-secondary-inversed-contrast);
  gap: 1rem;

  .back-button {
    --background: none;
    --background-hover: var(--parsec-color-light-primary-30-opacity15);
    --color: var(--parsec-color-light-secondary-light);
    --color-hover: var(--parsec-color-light-secondary-light);
    &::part(native) {
      padding: 0.5rem;
    }

    & > ion-icon {
      font-size: 1.25em;
      margin-inline-end: 12px;
    }
  }

  ion-icon {
    color: var(--parsec-color-light-secondary-light);
    font-size: 1.875rem;
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.list-md {
  background: none;
}

ion-split-pane {
  --side-min-width: var(--parsec-sidebar-menu-min-width);
  --side-max-width: var(--parsec-sidebar-menu-max-width);
  --side-width: var(--parsec-sidebar-menu-width);
}

ion-menu {
  user-select: none;
}

.list-workspaces {
  display: flex;
  flex-direction: column;
  flex: 1;
  gap: 0.5rem;

  &-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: var(--parsec-color-light-primary-100);
    margin-bottom: 0.5rem;
    transition: border 0.2s ease-in-out;
    cursor: pointer;

    &__title {
      position: relative;
      opacity: 0.6;

      &::after {
        content: '';
        position: absolute;
        width: 0;
        left: 0;
        background: var(--parsec-color-light-primary-100);
        border-radius: var(--parsec-radius-6);
        transition: width 0.2s ease-in-out;
        transition-delay: 100ms;
      }

      &:hover::after {
        content: '';
        position: absolute;
        bottom: -0.25rem;
        left: 0;
        height: 2px;
        width: 100%;
        background: var(--parsec-color-light-primary-100);
        border-radius: var(--parsec-radius-6);
      }
    }

    &__button {
      padding: 0.25rem;
      margin-right: 0.25rem;
      font-size: 1.25rem;
      border-radius: var(--parsec-radius-6);
      color: var(--parsec-color-light-primary-100);
      background: var(--parsec-color-light-primary-30-opacity15);
      cursor: pointer;
      scale: 1;
      transition: scale 0.2s ease-in-out;
      opacity: 0.6;

      &:hover {
        opacity: 1;
        scale: 1.1;
      }
    }
  }

  &__no-workspace {
    color: var(--parsec-color-light-primary-30);
    opacity: 0.3;
  }
}

.sidebar-item {
  --background: none;
  border-radius: var(--parsec-radius-6);
  border: solid 1px var(--parsec-color-light-primary-800);
  --min-height: 0;

  &:hover {
    border: solid 1px var(--parsec-color-light-primary-30-opacity15);
    cursor: pointer;
  }

  &:active,
  &.item-selected {
    --background: var(--parsec-color-light-primary-30-opacity15);
  }

  & > ion-label {
    --color: var(--parsec-color-light-primary-100);
  }

  & > ion-icon {
    color: var(--parsec-color-light-primary-100);
    font-size: 1.25em;
    margin: 0;
    margin-inline-end: 12px;
  }
}

.manage-organization {
  display: flex;
  flex-direction: column;
  color: var(--parsec-color-light-secondary-inversed-contrast);
  padding: 0 1.25rem;
  gap: 0.5rem;

  .title-h4 {
    padding: 1.5em 0 1em;
    border-top: 1px solid var(--parsec-color-light-primary-30-opacity15);
  }

  &-list {
    padding: 0;
  }
}
</style>
