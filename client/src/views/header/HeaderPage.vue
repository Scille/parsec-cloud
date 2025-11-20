<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-header id="connected-header">
      <ion-toolbar
        v-if="showHeader"
        class="topbar"
        :class="currentRouteIs(Routes.History) ? 'topbar-history' : ''"
      >
        <!-- icon visible when menu is hidden -->
        <ms-image
          v-if="!isMobile() && isLargeDisplay"
          slot="start"
          id="trigger-toggle-menu-button"
          class="topbar-button__item"
          @click="isSidebarMenuVisible ? hideSidebarMenu() : resetSidebarMenu()"
          :image="SidebarToggle"
        />
        <div
          class="topbar-left"
          ref="topbarLeft"
        >
          <div
            class="topbar-left-content"
            ref="backBlock"
            v-if="hasHistory() && !currentRouteIs(Routes.Workspaces)"
          >
            <header-back-button
              :short="currentRouteIsFileRoute()"
              class="topbar-left__back-button"
            />
          </div>

          <div
            v-if="!currentRouteIsFileRoute()"
            class="topbar-left-text"
          >
            <ion-label
              class="topbar-left-text__title title-h2"
              :class="hasHistory() ? 'align-center' : 'align-left'"
              v-if="(!currentRouteIs(Routes.Users) && isSmallDisplay) || isLargeDisplay"
            >
              {{ $msTranslate(getTitleForRoute()) }}
            </ion-label>
            <ion-text
              v-if="currentRouteIs(Routes.History) && workspaceName && isSmallDisplay"
              class="topbar-left-text__workspace subtitles-sm"
            >
              {{ workspaceName }}
            </ion-text>
          </div>

          <div
            class="topbar-left__breadcrumb"
            v-if="currentRouteIsFileRoute() && (!currentRouteIs(Routes.Workspaces) || isLargeDisplay)"
          >
            <header-breadcrumbs
              :path-nodes="fullPath"
              @change="onNodeSelected"
              :from-header-page="true"
              :available-width="breadcrumbsWidth"
            />
          </div>

          <div
            v-if="isSmallDisplay && userInfo && currentRouteIs(Routes.Workspaces)"
            class="topbar-left-workspaces-mobile"
          >
            <ion-text class="topbar-left-workspaces-mobile__orga body">{{ userInfo.organizationId }}</ion-text>
            <ion-text class="topbar-left-workspaces-mobile__title title-h2">{{ $msTranslate('HeaderPage.titles.workspaces') }}</ion-text>
          </div>
        </div>

        <!-- top right icon + profile -->
        <ion-buttons
          slot="primary"
          class="topbar-right"
        >
          <div
            class="topbar-right-button"
            v-if="!currentRouteIs(Routes.History) && !currentRouteIs(Routes.MyProfile) && !currentRouteIs(Routes.Invitations)"
          >
            <invitations-button v-if="!currentRouteIs(Routes.Invitations)" />

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
            <div
              v-if="!isMobile() && securityWarningsCount > 0 && securityWarnings && isSmallDisplay && !currentRouteIs(Routes.Invitations)"
              id="trigger-checklist-button"
              class="topbar-right-button__item unread"
              @click="openSecurityWarningsModal()"
              ref="checklistSecurityButton"
            >
              <ion-icon :icon="checkmarkCircle" />
              <div class="checklist-security-levels">
                <span
                  v-if="securityWarnings?.isWorkspaceOwner"
                  class="security-level"
                  :class="{ 'security-level--done': securityWarningsCount < 3 }"
                />
                <span
                  class="security-level"
                  :class="{ 'security-level--done': securityWarningsCount < 2 }"
                />
                <span class="security-level" />
              </div>
            </div>
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

          <profile-header-organization
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
import { pxToRem } from '@/common/utils';
import HeaderBackButton from '@/components/header/HeaderBackButton.vue';
import HeaderBreadcrumbs, { RouterPathNode } from '@/components/header/HeaderBreadcrumbs.vue';
import InvitationsButton from '@/components/header/InvitationsButton.vue';
import { RecommendationAction, SecurityWarnings, getSecurityWarnings } from '@/components/misc';
import RecommendationChecklistPopoverModal from '@/components/misc/RecommendationChecklistPopoverModal.vue';
import { ClientInfo, Path, UserProfile, WorkspaceRole, getClientInfo, getWorkspaceName, isMobile } from '@/parsec';
import {
  ProfilePages,
  Routes,
  currentRouteIs,
  currentRouteIsFileRoute,
  currentRouteIsLoggedRoute,
  getCurrentRouteName,
  getCurrentRouteParams,
  getDocumentPath,
  getWorkspaceHandle,
  hasHistory,
  navigateTo,
  routerGoBack,
  watchRoute,
} from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events, WorkspaceRoleUpdateData } from '@/services/eventDistributor';
import useHeaderControl from '@/services/headerControl';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { InformationManager, InformationManagerKey } from '@/services/informationManager';
import useSidebarMenu from '@/services/sidebarMenu';
import NotificationCenterModal from '@/views/header/NotificationCenterModal.vue';
import NotificationCenterPopover from '@/views/header/NotificationCenterPopover.vue';
import ProfileHeaderOrganization from '@/views/header/ProfileHeaderOrganization.vue';
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
  IonText,
  IonToolbar,
  modalController,
  popoverController,
} from '@ionic/vue';
import { checkmarkCircle, home, notifications, search } from 'ionicons/icons';
import { MsImage, MsModalResult, SidebarToggle, Translatable, useWindowSize } from 'megashark-lib';
import { Ref, computed, inject, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const { windowWidth, isLargeDisplay, isSmallDisplay } = useWindowSize();
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
let hotkeys: HotkeyGroup | null = null;
let eventDistributorCbId: string | null = null;
const workspaceName = ref('');
const { isVisible: isSidebarMenuVisible, reset: resetSidebarMenu, hide: hideSidebarMenu } = useSidebarMenu();
const { isHeaderVisible } = useHeaderControl();
const userInfo: Ref<ClientInfo | null> = ref(null);
const fullPath: Ref<RouterPathNode[]> = ref([]);
const notificationPopoverIsVisible: Ref<boolean> = ref(false);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const notificationCenterButtonRef = useTemplateRef('notificationCenterButton');
const securityWarnings = ref<SecurityWarnings | undefined>();
const securityWarningsCount = computed(() => {
  if (!securityWarnings.value) {
    return 0;
  }
  return (
    (securityWarnings.value.hasRecoveryDevice ? 0 : 1) +
    (securityWarnings.value.hasMultipleDevices ? 0 : 1) +
    (securityWarnings.value.soloOwnerWorkspaces.length === 0 ? 0 : 1)
  );
});

const showHeader = computed(() => {
  // Override visibility for specific routes
  if (currentRouteIs(Routes.FileHandler)) {
    return isHeaderVisible();
  }
  // Hide header on small displays for specific routes
  if (isSmallDisplay.value && (currentRouteIs(Routes.Organization) || currentRouteIs(Routes.MyProfile))) {
    return false;
  }
  return true;
});
const breadcrumbsWidth = ref(0);
const backBlockRef = useTemplateRef('backBlock');
const topbarLeftRef = useTemplateRef('topbarLeft');

const topbarWidthWatchCancel = watch([windowWidth, fullPath], () => {
  if (topbarLeftRef.value?.offsetWidth && backBlockRef.value?.offsetWidth) {
    breadcrumbsWidth.value = pxToRem(topbarLeftRef.value.offsetWidth - backBlockRef.value.offsetWidth);
  }
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
  await navigateTo(node.route, { params: node.params, query: node.query });
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
        route: Routes.Workspaces,
        params: {},
      },
    ];
  } else if (currentRouteIs(Routes.Documents)) {
    const workspaceHandle = getWorkspaceHandle();
    if (workspaceHandle) {
      workspaceName.value = await getWorkspaceName(workspaceHandle, true);
    }

    const finalPath: RouterPathNode[] = [];
    finalPath.push({
      id: 0,
      icon: home,
      route: Routes.Workspaces,
      params: {},
    });

    const rebuildPath: string[] = [];
    const workspacePath = await Path.parse(getDocumentPath());
    finalPath.push({
      id: 1,
      display: workspaceName.value,
      route: Routes.Documents,
      popoverIcon: home,
      query: { documentPath: '/' },
      params: getCurrentRouteParams(),
    });
    for (let i = 0; i < workspacePath.length; i++) {
      rebuildPath.push(workspacePath[i]);
      finalPath.push({
        id: i + 2,
        display: workspacePath[i],
        route: Routes.Documents,
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
    async () => await notificationCenterButtonRef.value?.$el.click(),
  );

  const result = await getClientInfo();
  if (result.ok) {
    userInfo.value = result.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(result.error)}`);
  }
  await updateRoute();

  eventDistributorCbId = await eventDistributor.registerCallback(
    Events.WorkspaceCreated | Events.MenuAction | Events.WorkspaceRoleUpdate | Events.DeviceCreated | Events.WorkspaceUpdated,
    async (event: Events, data?: EventData) => {
      if (event === Events.WorkspaceCreated) {
        securityWarnings.value = await getSecurityWarnings();
      } else if (event === Events.DeviceCreated) {
        if (!securityWarnings.value || !securityWarnings.value.hasMultipleDevices || !securityWarnings.value.hasRecoveryDevice) {
          securityWarnings.value = await getSecurityWarnings();
        }
      } else if (event === Events.WorkspaceRoleUpdate) {
        const updateData = data as WorkspaceRoleUpdateData;

        if (updateData.newRole === null || updateData.newRole === WorkspaceRole.Owner) {
          securityWarnings.value = await getSecurityWarnings();
        }
      } else if (event === Events.Online) {
        securityWarnings.value = await getSecurityWarnings();
      } else if (event === Events.WorkspaceUpdated) {
        await updateRoute();
      }
    },
  );
  securityWarnings.value = await getSecurityWarnings();
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  if (eventDistributorCbId) {
    eventDistributor.removeCallback(eventDistributorCbId);
  }
  routeWatchCancel();
  topbarWidthWatchCancel();
});

function getTitleForRoute(): Translatable {
  switch (getCurrentRouteName()) {
    case Routes.Users:
      return 'HeaderPage.titles.users';
    case Routes.Invitations:
      return 'HeaderPage.titles.invitations';
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
    case null:
      return '';
  }
  return '';
}

async function openNotificationCenter(event: Event): Promise<void> {
  event.stopPropagation();
  notificationPopoverIsVisible.value = true;

  if (isLargeDisplay.value) {
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
    await popover.onDidDismiss();
    await popover.dismiss();
  } else {
    const modal = await modalController.create({
      component: NotificationCenterModal,
      cssClass: 'notification-center-modal',
      showBackdrop: true,
      handle: true,
      backdropDismiss: true,
      breakpoints: isLargeDisplay.value ? undefined : [0, 0.5, 1],
      expandToScroll: false,
      initialBreakpoint: isLargeDisplay.value ? undefined : 0.5,
      componentProps: {
        notificationManager: informationManager.notificationManager,
        eventDistributor: eventDistributor,
      },
    });
    await modal.present();
    await modal.onDidDismiss();
    await modal.dismiss();
  }
  notificationPopoverIsVisible.value = false;
}

async function openSecurityWarningsModal(): Promise<void> {
  if (!securityWarnings.value) {
    return;
  }
  const modal = await modalController.create({
    component: RecommendationChecklistPopoverModal,
    cssClass: 'small-display-recommendation-checklist',
    showBackdrop: true,
    handle: true,
    backdropDismiss: true,
    expandToScroll: false,
    initialBreakpoint: isLargeDisplay.value ? undefined : 1,
    componentProps: {
      securityWarnings: securityWarnings.value,
      isModal: true,
    },
  });
  await modal.present();
  const { data, role } = await modal.onWillDismiss();
  await modal.dismiss();
  if (role === MsModalResult.Confirm && data.action) {
    if (data.action === RecommendationAction.AddDevice) {
      await navigateTo(Routes.MyProfile, { query: { profilePage: ProfilePages.Devices } });
    } else if (data.action === RecommendationAction.CreateRecoveryFiles) {
      await navigateTo(Routes.MyProfile, { query: { profilePage: ProfilePages.Recovery } });
    } else if (data.action === RecommendationAction.AddWorkspaceOwner) {
      await navigateTo(Routes.Workspaces);
    }
  }
}
</script>

<style scoped lang="scss">
// remove toolbar border added by ionic on iOS
#connected-header {
  ion-toolbar:last-of-type {
    --border-width: 0;
  }
}

.topbar {
  --background: var(--parsec-color-light-secondary-white);
  display: flex;
  padding: 1.5rem 2rem 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem 1.5rem 1rem;
    --background: var(--parsec-color-light-secondary-background);
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

    &::part(native) {
      --background: transparent;
      --background-activated: transparent;
      --background-focused: transparent;
    }

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
          background: var(--parsec-color-light-danger-500);
          right: 0.45rem;
          top: 0.35rem;
          width: 0.625rem;
          height: 0.625rem;
          border: 2px solid var(--parsec-color-light-secondary-inversed-contrast);
          border-radius: var(--parsec-radius-12);
        }
      }
    }
  }

  #trigger-checklist-button {
    padding: 0.625rem;
    border-radius: var(--parsec-radius-12);
    cursor: pointer;
    position: relative;
    display: flex;

    &:hover {
      background: var(--parsec-color-light-secondary-premiere);
    }

    .checklist-security-levels {
      position: absolute;
      display: flex;
      flex-direction: column;
      top: 0;
      bottom: 0;
      right: 0;
      transform: translateY(15%);
      justify-content: center;
      align-items: center;
      gap: 0.15rem;
      height: fit-content;

      .security-level {
        width: 0.5rem;
        height: 0.5rem;
        background: var(--parsec-color-light-primary-500);
        opacity: 0.3;
        border-radius: var(--parsec-radius-2);

        &--done {
          opacity: 1;
          background: var(--parsec-color-light-primary-400);
        }
      }
    }
  }
}

.topbar-left {
  display: flex;
  align-items: center;
  margin-right: 0.5rem;
  overflow: hidden;

  @include ms.responsive-breakpoint('sm') {
    gap: 0.5rem;
  }

  &-workspaces-mobile {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    width: 100%;
    overflow: hidden;

    &__orga {
      color: var(--parsec-color-light-secondary-grey);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &__title {
      color: var(--parsec-color-light-primary-800);
    }
  }

  &-text {
    width: 100%;
    color: var(--parsec-color-light-primary-800);
    text-align: center;

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

.topbar-history {
  --background: var(--parsec-color-light-secondary-background);

  @include ms.responsive-breakpoint('sm') {
    .topbar-left {
      justify-content: center;
      width: 100%;
      margin: 0;

      &-content {
        position: absolute;
        top: 0;
        left: 0;
      }

      &-text {
        display: flex;
        flex-direction: column;
        align-items: center;

        &__title {
          color: var(--parsec-color-light-primary-800);
        }

        &__workspace {
          color: var(--parsec-color-light-secondary-grey);
        }
      }
    }
  }
}
</style>
