<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <div
      class="resize-divider"
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
                :class="{ 'organization-card-header-desktop': isDesktop() }"
                @click="openOrganizationChoice($event)"
              >
                <div class="header-container">
                  <ion-avatar class="orga-avatar body-lg">
                    <span v-if="isTrialOrg">{{ userInfo ? userInfo.organizationId.substring(0, 2) : '' }}</span>
                    <ms-image
                      v-else
                      :image="LogoIconGradient"
                      class="orga-avatar-logo"
                    />
                  </ion-avatar>
                  <div class="orga-text">
                    <ion-card-title class="title-h3">
                      {{ userInfo?.organizationId }}
                    </ion-card-title>
                  </div>
                </div>
                <div
                  class="header-icon"
                  v-show="isDesktop()"
                >
                  <ms-image :image="ChevronExpand" />
                </div>
              </ion-card-header>

              <div
                class="organization-card-manageBtn"
                v-show="userInfo && userInfo.currentProfile != UserProfile.Outsider"
                @click="navigateTo(Routes.Users)"
              >
                <ion-text
                  class="button-medium"
                  button
                >
                  {{
                    userInfo && userInfo.currentProfile === UserProfile.Admin
                      ? $msTranslate('SideMenu.manageOrganization')
                      : $msTranslate('SideMenu.organizationInfo')
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
                {{ $msTranslate('SideMenu.back') }}
              </ion-button>
            </div>
          </div>
        </ion-header>

        <ion-content class="ion-padding sidebar-content">
          <!-- trial section -->
          <div
            v-show="isTrialOrg"
            v-if="expirationTime"
            class="trial-card"
          >
            <ion-text class="trial-card__tag title-h5">{{ $msTranslate('SideMenu.trial.tag') }}</ion-text>
            <div class="trial-card-text">
              <ion-text class="trial-card-text__time body">{{ $msTranslate(formatExpirationTime()) }}</ion-text>
              <ion-text class="trial-card-text__info body">{{ $msTranslate('SideMenu.trial.description') }}</ion-text>
            </div>
            <ion-button class="trial-card__button">
              <a href="https://parsec.cloud/tarification">
                {{ $msTranslate('SideMenu.trial.subscribe') }}
              </a>
            </ion-button>
          </div>

          <!-- workspaces -->
          <div
            v-show="!currentRouteIsOrganizationManagementRoute()"
            class="organization-workspaces"
          >
            <!-- list of favorite workspaces -->
            <!-- show only favorites -->
            <ion-list
              class="list-workspaces favorites"
              v-if="favoritesWorkspaces.length > 0"
            >
              <ion-header
                lines="none"
                class="list-workspaces-header menu-default"
              >
                <div class="list-workspaces-header-title">
                  <ion-icon
                    class="list-workspaces-header-title__icon"
                    :icon="star"
                  />
                  <ion-text class="list-workspaces-header-title__text">
                    {{ $msTranslate('SideMenu.favorites') }}
                  </ion-text>
                </div>
              </ion-header>
              <ion-item
                lines="none"
                button
                v-for="workspace in favoritesWorkspaces"
                :key="workspace.id"
                @click="goToWorkspace(workspace.handle)"
                :class="currentRouteIsWorkspaceRoute(workspace.handle) ? 'item-selected' : 'item-not-selected'"
                class="sidebar-item menu-default"
              >
                <ion-label class="sidebar-item-workspace-label">{{ workspace.currentName }}</ion-label>
                <div
                  class="workspace-option"
                  @click.stop="
                    openWorkspaceContextMenu($event, workspace, favorites, eventDistributor, informationManager, storageManager, true)
                  "
                >
                  <ion-icon :icon="ellipsisHorizontal" />
                </div>
              </ion-item>
            </ion-list>

            <div
              class="list-workspaces-divider"
              v-if="favoritesWorkspaces.length > 0"
            />

            <!-- list of workspaces -->
            <ion-list class="list-workspaces">
              <ion-header
                lines="none"
                class="list-workspaces-header menu-default"
              >
                <div class="list-workspaces-header-title">
                  <ion-icon
                    class="list-workspaces-header-title__icon"
                    :icon="business"
                  />
                  <ion-text class="list-workspaces-header-title__text">
                    {{ $msTranslate('SideMenu.workspaces') }}
                  </ion-text>
                </div>
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
                {{ $msTranslate('SideMenu.noWorkspace') }}
              </ion-text>
              <ion-item
                lines="none"
                button
                v-for="workspace in nonFavoriteWorkspaces"
                :key="workspace.id"
                @click="goToWorkspace(workspace.handle)"
                :class="currentRouteIsWorkspaceRoute(workspace.handle) ? 'item-selected' : 'item-not-selected'"
                class="sidebar-item menu-default"
              >
                <ion-label class="sidebar-item-workspace-label">{{ workspace.currentName }}</ion-label>
                <div
                  class="workspace-option"
                  @click.stop="
                    openWorkspaceContextMenu($event, workspace, favorites, eventDistributor, informationManager, storageManager, true)
                  "
                >
                  <ion-icon :icon="ellipsisHorizontal" />
                </div>
              </ion-item>
            </ion-list>
          </div>
          <!-- manage organization -->
          <div
            v-show="currentRouteIsOrganizationManagementRoute()"
            class="manage-organization"
          >
            <ion-label class="title-h4">
              {{
                userInfo && userInfo.currentProfile === UserProfile.Admin
                  ? $msTranslate('SideMenu.manageOrganization')
                  : $msTranslate('SideMenu.organizationInfo')
              }}
            </ion-label>
            <!-- user actions -->
            <ion-list class="manage-organization-list users">
              <ion-item
                button
                lines="none"
                class="sidebar-item menu-default users-title"
                :class="currentRouteIsUserRoute() ? 'item-selected' : 'item-not-selected'"
                @click="navigateTo(Routes.Users)"
              >
                <ion-icon
                  :icon="people"
                  class="sidebar-item-icon"
                  slot="start"
                />
                <ion-label>{{ $msTranslate('SideMenu.users') }}</ion-label>
              </ion-item>
            </ion-list>
            <!-- storage -->
            <ion-list
              class="manage-organization-list storage"
              v-show="userInfo && userInfo.currentProfile === UserProfile.Admin && false"
            >
              <ion-item
                button
                lines="none"
                class="sidebar-item storage-title menu-default"
                :class="currentRouteIs(Routes.Storage) ? 'item-selected' : 'item-not-selected'"
                @click="navigateTo(Routes.Storage)"
              >
                <ion-icon
                  :icon="pieChart"
                  class="sidebar-item-icon"
                  slot="start"
                />
                <ion-label> {{ $msTranslate('SideMenu.storage') }}</ion-label>
              </ion-item>
            </ion-list>
            <!-- org info -->
            <ion-list class="manage-organization-list organization">
              <ion-item
                button
                lines="none"
                class="sidebar-item organization-title menu-default"
                :class="currentRouteIs(Routes.Organization) ? 'item-selected' : 'item-not-selected'"
                @click="navigateTo(Routes.Organization)"
              >
                <ion-icon
                  :icon="informationCircle"
                  class="sidebar-item-icon"
                  slot="start"
                />
                <ion-label>{{ $msTranslate('SideMenu.organizationInfo') }}</ion-label>
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
import { ChevronExpand, MsImage, getTextFromUser, Translatable, LogoIconGradient } from 'megashark-lib';
import OrganizationSwitchPopover from '@/components/organizations/OrganizationSwitchPopover.vue';
import { WORKSPACES_PAGE_DATA_KEY, WorkspaceDefaultData, WorkspacesPageSavedData, openWorkspaceContextMenu } from '@/components/workspaces';
import {
  AvailableDevice,
  ClientInfo,
  UserProfile,
  WorkspaceHandle,
  WorkspaceID,
  WorkspaceInfo,
  getCurrentAvailableDevice,
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
} from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { InformationManager, InformationManagerKey } from '@/services/informationManager';
import useSidebarMenu from '@/services/sidebarMenu';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import {
  GestureDetail,
  IonAvatar,
  IonButton,
  IonCard,
  IonCardHeader,
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
import { add, business, chevronBack, ellipsisHorizontal, informationCircle, people, pieChart, star } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref, watch } from 'vue';
import { Duration } from 'luxon';

const workspaces: Ref<Array<WorkspaceInfo>> = ref([]);
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const informationManager: InformationManager = inject(InformationManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
let eventDistributorCbId: string | null = null;
const divider = ref();
const { defaultWidth, initialWidth, computedWidth } = useSidebarMenu();
const userInfo: Ref<ClientInfo | null> = ref(null);
const currentDevice = ref<AvailableDevice | null>(null);
const favorites: Ref<WorkspaceID[]> = ref([]);
const sidebarWidthProperty = ref(`${defaultWidth}px`);
const isTrialOrg = ref(true);
const expirationTime = ref<Duration | undefined>(undefined);

const watchSidebarWidthCancel = watch(computedWidth, (value: number) => {
  sidebarWidthProperty.value = `${value}px`;
  // set toast offset
  setToastOffset(value);
});

function formatExpirationTime(): Translatable {
  if (!expirationTime.value) {
    return '';
  }
  if (expirationTime.value.days > 0) {
    return {
      key: 'SideMenu.trial.expiration.days',
      count: expirationTime.value.days,
      data: { days: expirationTime.value.days },
    };
  } else if (expirationTime.value.hours > 0) {
    return {
      key: 'SideMenu.trial.expiration.hours',
      count: Math.floor(expirationTime.value.hours),
      data: { hours: Math.floor(expirationTime.value.hours) },
    };
  }
  return { key: 'SideMenu.trial.expiration.expired' };
}

async function goToWorkspace(workspaceHandle: WorkspaceHandle): Promise<void> {
  await navigateToWorkspace(workspaceHandle);
  await menuController.close();
}

async function createWorkspace(): Promise<void> {
  const workspaceName = await getTextFromUser({
    title: 'WorkspacesPage.CreateWorkspaceModal.pageTitle',
    trim: true,
    validator: workspaceNameValidator,
    inputLabel: 'WorkspacesPage.CreateWorkspaceModal.label',
    placeholder: 'WorkspacesPage.CreateWorkspaceModal.placeholder',
    okButtonText: 'WorkspacesPage.CreateWorkspaceModal.create',
  });

  if (workspaceName) {
    await navigateTo(Routes.Workspaces, { query: { workspaceName: workspaceName } });
  }
}

async function loadAll(): Promise<void> {
  const infoResult = await parsecGetClientInfo();

  favorites.value = (
    await storageManager.retrieveComponentData<WorkspacesPageSavedData>(WORKSPACES_PAGE_DATA_KEY, WorkspaceDefaultData)
  ).favoriteList;

  if (infoResult.ok) {
    userInfo.value = infoResult.value;
  } else {
    console.log('Failed to get user info', infoResult.error);
  }

  const deviceResult = await getCurrentAvailableDevice();
  if (deviceResult.ok) {
    currentDevice.value = deviceResult.value;
  }

  const result = await parsecListWorkspaces();
  if (result.ok) {
    workspaces.value = result.value.sort((w1, w2) => w1.currentName.toLocaleLowerCase().localeCompare(w2.currentName.toLocaleLowerCase()));
  } else {
    console.log('Failed to list workspaces', result.error);
  }
}

onMounted(async () => {
  eventDistributorCbId = await eventDistributor.registerCallback(
    Events.WorkspaceCreated | Events.WorkspaceFavorite | Events.WorkspaceUpdated,
    async (event: Events, _data?: EventData) => {
      if (event === Events.WorkspaceCreated || event === Events.WorkspaceFavorite || event === Events.WorkspaceUpdated) {
        await loadAll();
      }
    },
  );

  setToastOffset(computedWidth.value);
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
  watchSidebarWidthCancel();
  setToastOffset(0);
});

function setToastOffset(width: number): void {
  document.documentElement.style.setProperty('--ms-toast-offset', `${width}px`);
}

const favoritesWorkspaces = computed(() => {
  return Array.from(workspaces.value)
    .filter((workspace: WorkspaceInfo) => favorites.value.includes(workspace.id))
    .sort((a: WorkspaceInfo, b: WorkspaceInfo) => {
      if (favorites.value.includes(b.id) !== favorites.value.includes(a.id)) {
        return favorites.value.includes(b.id) ? 1 : -1;
      }
      return 0;
    });
});

const nonFavoriteWorkspaces = computed(() => {
  return workspaces.value.filter((workspace: WorkspaceInfo) => !favorites.value.includes(workspace.id));
});

function onMove(detail: GestureDetail): void {
  requestAnimationFrame(() => {
    let currentWidth = initialWidth.value + detail.deltaX;
    if (currentWidth >= 2 && currentWidth <= 500) {
      if (currentWidth <= 150) {
        currentWidth = 2;
      }
      computedWidth.value = currentWidth;
    }
  });
}

function onEnd(): void {
  initialWidth.value = computedWidth.value;
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
.resize-divider {
  width: 0.25rem;
  height: 100%;
  position: absolute;
  left: calc(v-bind(sidebarWidthProperty) - 2px);
  top: 0;
  z-index: 10000;
  cursor: ew-resize;
  display: flex;
  justify-content: center;

  &::after {
    content: '';
    width: 0.125rem;
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

  &::part(container) {
    padding: 0.5rem;
    display: flex;
    gap: 1.5rem;
  }

  // logo parsec
  &::after {
    content: url('@/assets/images/background/logo-icon-white.svg');
    opacity: 0.03;
    width: 100%;
    max-width: 270px;
    max-height: 170px;
    position: absolute;
    bottom: 16px;
    right: -60px;
    z-index: 0;
  }

  .sidebar-content {
    position: relative;
    z-index: 12;
  }

  .organization-workspaces {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    padding: 0 0.75rem;
  }
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

  &-header-desktop:hover {
    cursor: pointer;
    background: var(--parsec-color-light-primary-30-opacity15);
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
      background-color: var(--parsec-color-light-secondary-premiere);
      color: var(--parsec-color-light-primary-600);
      width: 2rem;
      height: 2rem;
      border-radius: var(--parsec-radius-8);
      text-align: center;
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      position: relative;
      z-index: 1;

      &-logo {
        width: 1.5rem;
      }
    }

    .orga-text {
      display: flex;
      flex-direction: column;
      white-space: nowrap;
      overflow: hidden;

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
      margin-inline-end: 0.75rem;
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
  padding: 0;
}

ion-split-pane {
  --side-min-width: var(--parsec-sidebar-menu-min-width);
  --side-max-width: var(--parsec-sidebar-menu-max-width);
  --side-width: v-bind(sidebarWidthProperty);
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
    margin-bottom: 0.5rem;
    transition: border 0.2s ease-in-out;
    position: relative;
    overflow: visible;

    &-title {
      color: var(--parsec-color-light-secondary-inversed-contrast);
      opacity: 0.6;
      display: flex;
      align-items: center;
      padding: 0.125rem 0;

      &__icon {
        font-size: 1rem;
        margin-right: 0.5rem;
      }

      &__text {
        margin-right: 0.5rem;
        position: relative;
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
      position: relative;
      z-index: 4;

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

.list-workspaces-divider {
  background: var(--parsec-color-light-primary-30-opacity15);
  display: flex;
  justify-content: center;
  height: 1px;
  width: 100%;
}

.sidebar-item {
  --background: none;
  border-radius: var(--parsec-radius-8);
  border: solid 1px var(--parsec-color-light-primary-800);
  --min-height: 0;

  .workspace-option {
    color: var(--parsec-color-light-secondary-grey);
    text-align: right;
    position: absolute;
    display: flex;
    align-items: center;
    top: 0;
    right: 1rem;
    font-size: 1.2rem;
    padding-top: 0.5rem;
    opacity: 0;

    &:hover {
      color: var(--parsec-color-light-primary-30);
    }
  }

  &:hover {
    border: solid 1px var(--parsec-color-light-primary-30-opacity15);
    cursor: pointer;

    .workspace-option {
      opacity: 1;
    }
  }

  &:active,
  &.item-selected {
    --background: var(--parsec-color-light-primary-30-opacity15);
  }

  &-icon {
    color: var(--parsec-color-light-primary-100);
    font-size: 1.25em;
    margin: 0;
    margin-inline-end: 12px;
  }

  & > ion-label {
    --color: var(--parsec-color-light-primary-100);
  }

  &-workspace-label {
    position: relative;
    margin-right: 1.1rem;
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
    --color: var(--parsec-color-light-primary-100);
  }
}

.trial-card {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 0 1rem 1.5rem 1rem;
  margin-bottom: 1.5rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-white);

  &__tag {
    color: var(--parsec-color-light-secondary-white);
    background: var(--parsec-color-light-gradient-background);;
    padding: 0.25rem 0.5rem;
    border-radius: var(--parsec-radius-6);
    width: fit-content;
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &__time {
      color: var(--parsec-color-light-secondary-white);
    }

    &__info {
      color: var(--parsec-color-light-primary-30);
      opacity: 0.8;
    }
  }

  // !important is used when waiting for a custom button
  &__button {
    --background: var(--parsec-color-light-secondary-white) !important;
    --color: var(--parsec-color-light-primary-600) !important;
    --background-hover: var(--parsec-color-light-secondary-premiere) !important;
    --color-hover: var(--parsec-color-light-primary-700) !important;
    --border-radius: var(--parsec-radius-8);
    --padding-start: 1rem;
    --padding-end: 1rem;
    --padding-top: 0.625rem;
    --padding-bottom: 0.625rem;

    a {
      color: var(--parsec-color-light-primary-600);
    }
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
