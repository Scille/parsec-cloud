<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <div
      class="divider"
      ref="divider"
    />
    <ion-split-pane
      content-id="main"
      ref="splitPane"
    >
      <ion-menu
        content-id="main"
        class="sidebar"
      >
        <ion-header class="sidebar__header">
          <div
            v-show="!isOrganizationManagementRoute()"
          >
            <!-- active organization -->
            <ion-card class="organization-card">
              <ion-card-header class="organization-card__header">
                <div class="organization-card__container">
                  <ion-avatar class="orga-avatar">
                    <span>{{ userInfo ? userInfo.organizationId.substring(0, 2) : '' }}</span>
                  </ion-avatar>
                  <div class="orga-text">
                    <ion-card-subtitle class="caption-info">
                      {{ $t('HomePage.organizationActionSheet.header') }}
                    </ion-card-subtitle>
                    <ion-card-title class="title-h4">
                      {{ userInfo?.organizationId }}
                    </ion-card-title>
                  </div>
                </div>
                <!-- new icon to provide -->
                <div class="organization-card__icon">
                  <svg
                    width="32"
                    height="32"
                    viewBox="0 0 32 32"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                  >
                    <path
                      d="M10.3495 20.4231L15.6049 26.795C15.6896 26.8977 15.7947 26.9801
                      15.9129 27.0366C16.0311 27.0931 16.1597 27.1223 16.2899 27.1223C16.42 27.1223 16.5486 27.0931
                      16.6669 27.0366C16.7851 26.9801 16.8902 26.8977 16.9749 26.795L22.2303 20.4231C22.7319 19.8149
                      22.316 18.8755 21.5453 18.8755L11.033 18.8755C10.2622 18.8755 9.84641 19.8149 10.3495 20.4231Z"
                      fill="#F9F9FB"
                    />
                    <path
                      d="M22.2326 13.4558L16.9772 7.08389C16.8925 6.98124 16.7874 6.89884 16.6691 6.84234C16.5509 6.78585
                      16.4223 6.7566 16.2921 6.7566C16.162 6.7566 16.0334 6.78585 15.9151 6.84234C15.7969 6.89884 15.6918
                      6.98124 15.6071 7.08389L10.3517 13.4558C9.85015 14.064 10.266 15.0034 11.0367 15.0034L21.549 15.0034C22.3198
                      15.0034 22.7356 14.064 22.2326 13.4558Z"
                      fill="#F9F9FB"
                    />
                  </svg>
                </div>
              </ion-card-header>

              <div
                class="organization-card__manageBtn"
                v-show="userInfo && userInfo.currentProfile != UserProfile.Outsider"
                @click="routerNavigateTo('activeUsers')"
              >
                <ion-text
                  class="subtitles-sm"
                  button
                >
                  {{ userInfo && userInfo.currentProfile === UserProfile.Admin ?
                    $t('SideMenu.manageOrganization') :
                    $t('SideMenu.organizationInfo') }}
                </ion-text>
              </div>
            </ion-card>
            <!-- end of active organization -->
          </div>
          <div
            v-show="isOrganizationManagementRoute()"
          >
            <div
              class="back-organization"
              @click="routerNavigateTo('workspaces')"
            >
              <ion-button
                fill="clear"
                class="back-button"
              >
                <ion-icon
                  slot="icon-only"
                  :icon="chevronBack"
                />
              </ion-button>
              <ion-label class="title-h3">
                {{ userInfo && userInfo.currentProfile === UserProfile.Admin ?
                  $t('SideMenu.manageOrganization') :
                  $t('SideMenu.organizationInfo') }}
              </ion-label>
            </div>
          </div>
        </ion-header>

        <ion-content class="ion-padding">
          <div
            v-show="!isOrganizationManagementRoute()"
            class="workspaces-organization"
          >
            <!-- list of workspaces -->
            <ion-list class="list-workspaces">
              <ion-header
                lines="none"
                button
                @click="navigateToWorkspaceList()"
                class="list-workspaces__header menu-default"
              >
                {{ $t('SideMenu.allWorkspaces') }}
              </ion-header>

              <ion-item
                lines="none"
                button
                v-for="workspace in workspaces"
                :key="workspace.id"
                @click="navigateToWorkspace(workspace.id)"
                :class="isSpecificWorkspaceRoute(workspace.id) ? 'item-selected' : 'item-not-selected'"
                class="sidebar-item"
              >
                <ion-icon
                  :icon="business"
                  slot="start"
                />
                <ion-label>{{ workspace.name }}</ion-label>
              </ion-item>
            </ion-list>
            <!-- list of workspaces -->
          </div>
          <div
            v-show="isOrganizationManagementRoute()"
            class="manage-organization"
          >
            <!-- user actions -->
            <ion-list class="users">
              <ion-item
                lines="none"
                class="sidebar-item users-title menu-default"
                :class="isUserRoute() ? 'item-selected' : 'item-not-selected'"
                @click="routerNavigateTo('activeUsers')"
              >
                <ion-icon
                  :icon="people"
                  slot="start"
                />
                <ion-label>{{ $t('SideMenu.users') }}</ion-label>
              </ion-item>

              <ion-list class="user-menu">
                <ion-item
                  lines="none"
                  button
                  class="user-menu__item body"
                  :class="currentRoute.name === 'activeUsers' ? 'user-menu-selected' : 'user-menu-not-selected'"
                  @click="routerNavigateTo('activeUsers')"
                >
                  <ion-label>{{ $t('SideMenu.activeUsers') }}</ion-label>
                </ion-item>
                <ion-item
                  lines="none"
                  button
                  class="user-menu__item body"
                  :class="currentRoute.name === 'revokedUsers' ? 'user-menu-selected' : 'user-menu-not-selected'"
                  @click="routerNavigateTo('revokedUsers')"
                >
                  <ion-label>{{ $t('SideMenu.revokedUsers') }}</ion-label>
                </ion-item>
                <ion-item
                  v-show="userInfo && userInfo.currentProfile === UserProfile.Admin"
                  lines="none"
                  button
                  class="user-menu__item body"
                  :class="currentRoute.name === 'invitations' ? 'user-menu-selected' : 'user-menu-not-selected'"
                  @click="routerNavigateTo('invitations')"
                >
                  <ion-label>{{ $t('SideMenu.invitations') }}</ion-label>
                </ion-item>
              </ion-list>
            </ion-list>
            <!-- storage -->
            <ion-list
              class="storage"
              v-show="userInfo && userInfo.currentProfile === UserProfile.Admin"
            >
              <ion-item
                lines="none"
                class="sidebar-item storage-title menu-default"
                :class="currentRoute.name === 'storage' ? 'item-selected' : 'item-not-selected'"
                @click="routerNavigateTo('storage')"
              >
                <ion-icon
                  :icon="pieChart"
                  slot="start"
                />
                <ion-label> {{ $t('SideMenu.storage') }}</ion-label>
              </ion-item>
            </ion-list>
            <!-- org info -->
            <ion-list
              class="organization"
            >
              <ion-item
                lines="none"
                class="sidebar-item organization-title menu-default"
                :class="currentRoute.name === 'organization' ? 'item-selected' : 'item-not-selected'"
                @click="routerNavigateTo('organization')"
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
import {
  IonContent,
  IonHeader,
  IonMenu,
  IonSplitPane,
  IonIcon,
  IonText,
  IonList,
  IonCard,
  IonCardTitle,
  IonCardSubtitle,
  IonCardHeader,
  IonLabel,
  IonPage,
  IonAvatar,
  IonItem,
  IonRouterOutlet,
  menuController,
  GestureDetail,
  IonButton,
} from '@ionic/vue';
import {
  business,
  chevronBack,
  people,
  pieChart,
  informationCircle,
} from 'ionicons/icons';
import { WatchStopHandle, onMounted, onUnmounted, ref, watch, Ref } from 'vue';
import { createGesture } from '@ionic/vue';
import { useRoute } from 'vue-router';
import useSidebarMenu from '@/services/sidebarMenu';
import { isOrganizationManagementRoute, isSpecificWorkspaceRoute, isUserRoute } from '@/router/conditions';
import { routerNavigateTo } from '@/router';
import {
  listWorkspaces as parsecListWorkspaces,
  getClientInfo as parsecGetClientInfo,
  ClientInfo,
  UserProfile,
  WorkspaceInfo,
} from '@/parsec';

const workspaces: Ref<Array<WorkspaceInfo>> = ref([]);

const currentRoute = useRoute();
const splitPane = ref();
const divider = ref();
const { defaultWidth, initialWidth, computedWidth, wasReset } = useSidebarMenu();
const userInfo: Ref<ClientInfo | null> = ref(null);

// watching wasReset value
const unwatch: WatchStopHandle = watch(wasReset, (value) => {
  if (value) {
    resizeMenu(defaultWidth);
    wasReset.value = false;
  }
});

function navigateToWorkspace(workspaceId: string): void {
  routerNavigateTo('folder', {workspaceId: workspaceId}, {path: '/'});
  menuController.close();
}

function navigateToWorkspaceList(): void {
  routerNavigateTo('workspaces');
  menuController.close();
}

onMounted(async () => {
  const infoResult = await parsecGetClientInfo();

  if (infoResult.ok) {
    userInfo.value = infoResult.value;
  } else {
    console.log('Failed to get user info', infoResult.error);
  }

  if (divider.value) {
    const gesture = createGesture({
      gestureName: 'resize-menu',
      el: divider.value,
      onEnd,
      onMove,
    });
    gesture.enable();
  }
  const result = await parsecListWorkspaces();
  if (result.ok) {
    workspaces.value = result.value;
  } else {
    console.log('Failed to list workspaces', result.error);
  }
});

onUnmounted(() => {
  unwatch();
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
  if (splitPane.value) {
    splitPane.value.$el.style.setProperty('--side-width', `${newWidth}px`);
  }
  if (divider.value) {
    divider.value.style.setProperty('left', `${newWidth - 4}px`);
  }
}
</script>

<style lang="scss" scoped>
.divider {
  width: 8px;
  height: 100%;
  position: absolute;
  left: 296px;
  top: 0;
  z-index: 10000;
  cursor:ew-resize;
  display: flex;
  justify-content: center;

  &::after {
    content: '';
    width: 4px;
    height: 100%;
    padding: 20rem 0;
  }

  &:hover::after, &:active::after {
    background: var(--parsec-color-light-primary-200);
  }
}

.sidebar, .sidebar ion-content {
  --background: var(--parsec-color-light-primary-800);
}

.sidebar {
  border: none;
  user-select: none;
  border-radius: 0 .5rem .5rem 0;

  &__header {
    padding: 0.5rem;
  }

  // logo parsec
  &::after {
    content: url('@/assets/images/Logo/logo_icon_white.svg');
    opacity: .03;
    width: 100%;
    max-width: 270px;
    max-height: 170px;
    display: block;
    position: fixed;
    bottom: 16px;
    right: -60px;
  }
}

.organization-card {
  --background: var(--parsec-color-light-primary-30-opacity15);
  box-shadow: none;
  margin: 0;

  &__header {
    display: flex;
    justify-content: space-between;
    flex-direction: row;
  }

  &__container {
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

  &__icon {
    white-space: nowrap;
    display: flex;
    align-items: center;
    background-color: var(--parsec-color-light-primary-800);
    position: relative;
    z-index: 2;

    &::before {
      content: '';
      height: 100%;
      width: 100%;
      position: absolute;
      z-index: -1;
      background-color: var(--parsec-color-light-primary-30-opacity15);
    }
  }

  &__manageBtn {
    padding:0.625em 1em;
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
  padding: 2rem 1.25rem;
  color: var(--parsec-color-light-secondary-inversed-contrast);
  gap: 1rem;
  cursor: pointer;

  .back-button {
    --color: var(--parsec-color-light-secondary-light);
    margin: 0;

    &::part(native) {
      padding: 0;
    }
  }

  ion-icon {
    color: var(--parsec-color-light-secondary-light);
    font-size: 1.875rem;
  }
}

.list-md {
  background: none;
}

ion-split-pane {
  --side-min-width: 0px;
  --side-max-width: 500px;
  --side-width: 300px;
}

ion-menu {
  user-select: none;
}

.list-workspaces {
  display: flex;
  flex-direction: column;
  flex: 1;
  gap: .5rem;

  &__header {
    opacity: 0.6;
    color: var(--parsec-color-light-primary-100);
    margin-bottom: 1rem;
    width: fit-content;
    transition: border 0.2s ease-in-out;
    cursor: pointer;
    position: relative;

    &:hover::after {
      content: '';
      position: absolute;
      bottom: -.25rem;
      left: 0;
      height: 2px;
      width: 100%;
      background: var(--parsec-color-light-primary-100);
      border-radius: var(--parsec-radius-6);
    }
  }
}

.sidebar-item {
  --background:none;
  border-radius: var(--parsec-radius-6);
  border: solid 1px var(--parsec-color-light-primary-800);
  --min-height: 0;

  &:hover {
    border: solid 1px var(--parsec-color-light-primary-30-opacity15);
    cursor: pointer;
  }

  &:active, &.item-selected {
    --background: var(--parsec-color-light-primary-30-opacity15);
  }

  &>ion-label {
    --color: var(--parsec-color-light-primary-100);
  }

  &>ion-icon {
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
}

.user-menu {
  padding: .5rem 2.5rem;

  &__item {
    --background: none;
    position: relative;
    opacity: .5;
    min-height: 100%;
    --background-hover: none;

    &:hover {
      opacity: 1;
    }

    &.user-menu-selected {
      opacity: 1;
      width: fit-content;

      ion-label {
        position: relative;
        overflow: visible;
        width: auto;

        &::after {
          content: '';
          position: absolute;
          left: 0;
          bottom: -4px;
          height: 1.5px;
          width: 100%;
          background: var(--parsec-color-light-primary-100);
          border-radius: var(--parsec-radius-6);
        }
      }
    }

    &::part(native) {
      padding: 0;
    }

    ion-label {
      --color: var(--parsec-color-light-primary-100);
    }

  }
}

.user-menu-selected {
  text-decoration-color: var(--parsec-color-light-primary-30);
}

.user-menu-not-selected {
  color: var(--parsec-color-light-primary-100);
  text-decoration: none;
}
</style>
