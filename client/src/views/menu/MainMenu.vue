<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-split-pane
    class="large-display-menu-container"
    content-id="main"
    menu-id="main-menu"
    :when="true"
  >
    <ion-menu
      content-id="main"
      class="sidebar"
    >
      <div
        class="resize-divider"
        ref="divider"
        v-show="isVisible()"
      />
      <ion-header class="sidebar-header">
        <div>
          <!-- active organization -->
          <ion-card class="organization-card">
            <ion-card-header
              class="organization-card-header organization-card-header-desktop"
              @click="openOrganizationChoice($event)"
            >
              <ion-avatar class="organization-avatar body-lg">
                <span v-if="!isTrialOrg">{{ userInfo ? userInfo.organizationId.substring(0, 2) : '' }}</span>
                <!-- prettier-ignore -->
                <ms-image
                  v-else
                  :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
                  class="organization-avatar-logo"
                />
              </ion-avatar>
              <div class="organization-text">
                <ion-card-title class="title-h3">
                  {{ userInfo?.organizationId }}
                </ion-card-title>
              </div>
              <ms-image
                :image="ChevronExpand"
                class="header-icon"
              />
            </ion-card-header>
          </ion-card>
          <!-- end of active organization -->
        </div>

        <!-- frozen organization -->
        <div
          class="freeze-card"
          v-if="isExpired"
        >
          <ion-icon
            class="freeze-card__icon"
            :icon="snow"
          />
          <div class="freeze-card-header">
            <ion-icon
              class="freeze-card-header__icon"
              :icon="warning"
            />
            <ion-text class="freeze-card-header__title title-h4">
              {{ $msTranslate('SideMenu.frozen.title') }}
            </ion-text>
          </div>
          <div class="freeze-card-main">
            <ion-text class="body">
              {{ $msTranslate('SideMenu.frozen.description') }}
            </ion-text>
          </div>
          <div class="freeze-card-footer">
            <ion-text class="freeze-card-footer__title title-h5">
              {{ $msTranslate('SideMenu.frozen.contact') }}
            </ion-text>
            <ion-text class="freeze-card-footer__email subtitles-sm">
              {{ $msTranslate('SideMenu.frozen.email') }}
            </ion-text>
          </div>
        </div>
      </ion-header>

      <ion-content class="ion-padding sidebar-content">
        <!-- trial section -->
        <div
          v-show="isTrialOrg"
          v-if="expirationDuration"
          class="trial-card"
        >
          <ion-text class="trial-card__tag button-medium">{{ $msTranslate('SideMenu.trial.tag') }}</ion-text>
          <div class="trial-card-text">
            <ion-text class="trial-card-text__time title-h3">{{ $msTranslate(formatExpirationTime(expirationDuration)) }}</ion-text>
            <ion-text class="trial-card-text__info body">{{ $msTranslate('SideMenu.trial.description') }}</ion-text>
          </div>
          <ion-button
            class="trial-card__button button-medium"
            @click="openPricingLink"
          >
            {{ $msTranslate('SideMenu.trial.subscribe') }}
          </ion-button>
        </div>

        <!-- organisation content -->
        <sidebar-menu-list
          v-if="userInfo && userInfo.currentProfile != UserProfile.Outsider"
          title="SideMenu.organization"
          v-model:is-content-visible="menusVisible.organization"
          @update:is-content-visible="onOrganizationMenuVisibilityChanged"
          id="sidebar-organization"
        >
          <div class="sidebar-content-organization">
            <!-- manage users -->
            <ion-text
              @click="navigateTo(Routes.Users)"
              class="sidebar-content-organization-button button-medium"
              :class="currentRouteIs(Routes.Users) ? 'active' : ''"
              id="sidebar-users"
              button
            >
              <ion-icon
                class="sidebar-content-organization-button__icon"
                :icon="people"
              />
              <span class="sidebar-content-organization-button__text">
                {{ $msTranslate('SideMenu.users') }}
              </span>
            </ion-text>
            <!-- organization information -->
            <ion-text
              @click="navigateTo(Routes.Organization)"
              class="sidebar-content-organization-button button-medium"
              :class="currentRouteIs(Routes.Organization) ? 'active' : ''"
              id="sidebar-organization-information"
              button
            >
              <ion-icon
                class="sidebar-content-organization-button__icon"
                :icon="informationCircle"
              />
              <span class="sidebar-content-organization-button__text">
                {{ $msTranslate('SideMenu.information') }}
              </span>
            </ion-text>
          </div>
        </sidebar-menu-list>

        <!-- workspaces content -->
        <sidebar-menu-list
          title="SideMenu.workspaces"
          v-model:is-content-visible="menusVisible.workspaces"
          @update:is-content-visible="onWorkspacesMenuVisibilityChanged"
          @header-clicked="navigateTo(Routes.Workspaces)"
          :is-header-clickable="currentRouteIs(Routes.Workspaces) ? false : true"
          id="sidebar-workspaces"
        >
          <div
            class="sidebar-content-workspaces current-workspace"
            v-if="!menusVisible.recentWorkspaces && currentWorkspace"
          >
            <sidebar-workspace-item
              :workspace="currentWorkspace"
              @workspace-click="goToWorkspace"
              @context-menu-requested="
                openWorkspaceContextMenu($event, currentWorkspace, favorites, eventDistributor, informationManager, storageManager, true)
              "
            />
          </div>
          <div class="sidebar-content-workspaces-container">
            <div
              class="sidebar-content-workspaces"
              id="sidebar-workspaces-favorites"
            >
              <ion-text
                class="sidebar-content-workspaces__title subtitles-sm"
                @click="onFavoritesMenuVisibilityChanged(!menusVisible.favorites)"
                :class="{ open: menusVisible.favorites }"
              >
                <ion-icon :icon="caretForward" />
                {{ $msTranslate('SideMenu.favorites') }}
              </ion-text>
              <ion-text
                class="sidebar-content-workspaces--no-recent subtitles-sm"
                v-if="menusVisible.favorites && favoritesWorkspaces.length === 0"
              >
                {{ $msTranslate('SideMenu.noFavorites') }}
              </ion-text>
              <sidebar-workspace-item
                v-show="menusVisible.favorites"
                v-for="workspace in favoritesWorkspaces"
                :key="workspace.id"
                :workspace="workspace"
                @workspace-clicked="goToWorkspace"
                @context-menu-requested="
                  openWorkspaceContextMenu($event, workspace, favorites, eventDistributor, informationManager, storageManager, true)
                "
              />
            </div>
            <div class="sidebar-content-workspaces">
              <ion-text
                class="sidebar-content-workspaces__title subtitles-sm"
                @click="onRecentWorkspacesMenuVisibilityChanged(!menusVisible.recentWorkspaces)"
                :class="{ open: menusVisible.recentWorkspaces }"
              >
                <ion-icon :icon="caretForward" />
                {{ $msTranslate('SideMenu.recentWorkspaces') }}
              </ion-text>
              <ion-text
                class="sidebar-content-workspaces--no-recent subtitles-sm"
                v-if="menusVisible.recentWorkspaces && recentDocumentManager.getWorkspaces().length === 0"
              >
                {{ $msTranslate('SideMenu.noRecentWorkspace') }}
              </ion-text>
              <sidebar-workspace-item
                v-show="menusVisible.recentWorkspaces && recentDocumentManager.getWorkspaces().length > 0"
                v-for="workspace in recentDocumentManager.getWorkspaces()"
                :workspace="workspace"
                :key="workspace.id"
                @workspace-clicked="goToWorkspace"
                @context-menu-requested="
                  openWorkspaceContextMenu($event, workspace, favorites, eventDistributor, informationManager, storageManager, true)
                "
              />
            </div>
          </div>
        </sidebar-menu-list>

        <!-- last files content -->
        <sidebar-menu-list
          title="SideMenu.recentDocuments"
          :icon="documentIcon"
          v-model:is-content-visible="menusVisible.recentFiles"
          @update:is-content-visible="onRecentFilesMenuVisibilityChanged"
          id="sidebar-files"
        >
          <div
            class="sidebar-content-files"
            v-if="menusVisible.recentFiles"
          >
            <ion-text
              class="sidebar-content-files--no-recent subtitles-sm"
              v-if="recentDocumentManager.getFiles().length === 0"
            >
              {{ $msTranslate('SideMenu.noRecentDocuments') }}
            </ion-text>
            <sidebar-recent-file-item
              v-for="file in recentDocumentManager.getFiles()"
              :file="file"
              :key="file.entryId"
              @file-clicked="openRecentFile"
              @remove-clicked="removeRecentFile"
            />
          </div>
        </sidebar-menu-list>

        <!-- security checklist -->
        <div
          class="organization-checklist"
          v-show="securityWarningsCount > 0"
        >
          <ion-item
            button
            lines="none"
            class="sidebar-item-manage item-selected checklist ion-no-padding"
            @click="openSecurityWarningsPopover"
          >
            <div class="checklist-text">
              <ion-text class="checklist-text__title title-h5">{{ $msTranslate('SideMenu.checklist.title') }}</ion-text>
              <ion-text class="checklist-text__description button-small">
                {{
                  $msTranslate({
                    key: 'SideMenu.checklist.remaining',
                    data: { count: securityWarningsCount },
                    count: securityWarningsCount,
                  })
                }}
              </ion-text>
            </div>
            <div class="checklist-button">
              <ion-text class="checklist-button__text button-small">{{ $msTranslate({ key: 'SideMenu.checklist.open' }) }}</ion-text>
              <ion-icon
                :icon="chevronForward"
                class="checklist-button__icon"
              />
            </div>
          </ion-item>
        </div>
      </ion-content>
    </ion-menu>
    <tab-bar-menu
      class="tab-bar-menu"
      v-if="isSmallDisplay && !customTabBar.isVisible.value && !currentRouteIs(Routes.History)"
      :user-info="userInfo"
      @action-clicked="onActionClicked"
      :actions="actions"
    />
    <ion-router-outlet id="main" />
  </ion-split-pane>
</template>

<script setup lang="ts">
import {
  GestureDetail,
  createGesture,
  IonAvatar,
  IonButton,
  IonCard,
  IonCardHeader,
  IonCardTitle,
  IonContent,
  IonHeader,
  IonIcon,
  IonItem,
  IonMenu,
  IonRouterOutlet,
  IonSplitPane,
  IonText,
  menuController,
  popoverController,
} from '@ionic/vue';
import TabBarMenu from '@/views/menu/TabBarMenu.vue';
import {
  document as documentIcon,
  informationCircle,
  people,
  snow,
  warning,
  cloudUpload,
  folderOpen,
  addCircle,
  personAdd,
  chevronForward,
  caretForward,
} from 'ionicons/icons';
import { SidebarWorkspaceItem, SidebarRecentFileItem, SidebarMenuList } from '@/components/sidebar';
import {
  ProfilePages,
  Routes,
  currentRouteIs,
  currentRouteIsWorkspaceRoute,
  navigateTo,
  navigateToWorkspace,
  switchOrganization,
  watchRoute,
} from '@/router';
import {
  ClientInfo,
  LoggedInDeviceInfo,
  WorkspaceID,
  WorkspaceInfo,
  UserProfile,
  listWorkspaces,
  getCurrentAvailableDevice,
  getLoggedInDevices,
  getClientInfo,
  WorkspaceRole,
  getOrganizationCreationDate,
} from '@/parsec';
import { ChevronExpand, MsImage, LogoIconGradient, I18n, MsModalResult, useWindowSize } from 'megashark-lib';
import { Ref, computed, inject, onMounted, onUnmounted, ref, watch, useTemplateRef } from 'vue';
import { recentDocumentManager, RecentFile } from '@/services/recentDocuments';
import { openPath } from '@/services/fileOpener';
import { InformationManager, InformationManagerKey } from '@/services/informationManager';
import { StorageManagerKey, StorageManager } from '@/services/storageManager';
import OrganizationSwitchPopover from '@/components/organizations/OrganizationSwitchPopover.vue';
import {
  EventData,
  EventDistributor,
  EventDistributorKey,
  Events,
  MenuActionData,
  WorkspaceRoleUpdateData,
} from '@/services/eventDistributor';
import { WorkspaceDefaultData, WORKSPACES_PAGE_DATA_KEY, WorkspacesPageSavedData } from '@/components/workspaces';
import { getDurationBeforeExpiration, isTrialOrganizationDevice } from '@/common/organization';
import { Duration } from 'luxon';
import { openWorkspaceContextMenu } from '@/components/workspaces';
import { formatExpirationTime } from '@/common/organization';
import useSidebarMenu from '@/services/sidebarMenu';
import useUploadMenu from '@/services/fileUploadMenu';
import { MenuAction, SIDEBAR_MENU_DATA_KEY, SidebarDefaultData, SidebarSavedData } from '@/views/menu';
import { FolderGlobalAction } from '@/views/files';
import { WorkspaceAction } from '@/views/workspaces';
import { isUserAction, UserAction } from '@/views/users';
import { useCustomTabBar } from '@/views/menu/utils';
import { getSecurityWarnings, RecommendationAction, SecurityWarnings } from '@/components/misc';
import RecommendationChecklistPopover from '@/components/misc/RecommendationChecklistPopover.vue';
import { Resources, ResourcesManager } from '@/services/resourcesManager';

defineProps<{
  userInfo: ClientInfo;
}>();

const emits = defineEmits<{
  (event: 'sidebarWidthChanged', value: number): void;
}>();

const customTabBar = useCustomTabBar();
const informationManager: InformationManager = inject(InformationManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const workspaces: Ref<Array<WorkspaceInfo>> = ref([]);
const favorites: Ref<WorkspaceID[]> = ref([]);
const { computedWidth: computedWidth, storedWidth: storedWidth, isVisible: isVisible } = useSidebarMenu();
const sidebarWidthProperty = ref('');
const dividerRef = useTemplateRef('divider');
const dividerWidthProperty = ref('');
let eventDistributorCbId: string | null = null;
const loggedInDevices = ref<LoggedInDeviceInfo[]>([]);
const isExpired = ref(false);
const menusVisible = ref({ organization: true, workspaces: true, recentFiles: true, recentWorkspaces: true, favorites: true });
const expirationDuration = ref<Duration | undefined>(undefined);
const isTrialOrg = ref(false);
let timeoutId: number | undefined = undefined;
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

const { isSmallDisplay, windowWidth } = useWindowSize();
const actions = ref<Array<Array<MenuAction>>>([]);

const MIN_WIDTH = 150;
const MAX_WIDTH = 370;
const MAX_WIDTH_SHOWING_SIDEBAR = 768;

const watchSidebarWidthCancel = watch(computedWidth, async (value: number) => {
  sidebarWidthProperty.value = `${value}px`;

  if (timeoutId !== undefined) {
    clearTimeout(timeoutId);
  }
  timeoutId = window.setTimeout(async () => {
    await storageManager.updateComponentData<SidebarSavedData>(
      SIDEBAR_MENU_DATA_KEY,
      {
        width: value < MIN_WIDTH ? storedWidth.value : computedWidth.value,
        hidden: value < MIN_WIDTH,
      },
      SidebarDefaultData,
    );
    timeoutId = undefined;
  }, 2000);

  updateDividerPosition(value);
  emits('sidebarWidthChanged', value);
});

const currentWorkspace = computed(() => {
  for (const wk of recentDocumentManager.getWorkspaces()) {
    if (currentRouteIsWorkspaceRoute(wk.handle)) {
      return wk;
    }
  }
  return undefined;
});

async function loadAll(): Promise<void> {
  favorites.value = (
    await storageManager.retrieveComponentData<WorkspacesPageSavedData>(WORKSPACES_PAGE_DATA_KEY, WorkspaceDefaultData)
  ).favoriteList;

  const deviceResult = await getCurrentAvailableDevice();
  if (deviceResult.ok) {
    isTrialOrg.value = isTrialOrganizationDevice(deviceResult.value);
    const orgCreationDateResult = await getOrganizationCreationDate();
    if (isTrialOrg.value && orgCreationDateResult.ok) {
      expirationDuration.value = getDurationBeforeExpiration(orgCreationDateResult.value);
    }
  }

  const result = await listWorkspaces();
  if (result.ok) {
    workspaces.value = result.value.sort((w1, w2) => w1.currentName.toLocaleLowerCase().localeCompare(w2.currentName.toLocaleLowerCase()));
  } else {
    window.electronAPI.log('error', `Failed to list workspaces ${JSON.stringify(result.error)}`);
  }

  loggedInDevices.value = await getLoggedInDevices();
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

function onMove(detail: GestureDetail): void {
  if (detail.currentX < MIN_WIDTH) {
    computedWidth.value = MIN_WIDTH;
  } else if (detail.currentX > MAX_WIDTH) {
    computedWidth.value = MAX_WIDTH;
  } else {
    computedWidth.value = detail.currentX;
  }
  emits('sidebarWidthChanged', computedWidth.value);
}

async function updateDividerPosition(value?: number): Promise<void> {
  if (window.innerWidth > MAX_WIDTH_SHOWING_SIDEBAR && computedWidth.value >= window.innerWidth * 0.28) {
    value = computedWidth.value - (computedWidth.value - window.innerWidth * 0.28);
    dividerWidthProperty.value = `${value}px`;
  } else {
    dividerWidthProperty.value = `${computedWidth.value}px`;
  }
}

function setActions(): void {
  if (currentRouteIs(Routes.Documents)) {
    actions.value = [
      [{ action: FolderGlobalAction.CreateFolder, label: 'FoldersPage.createFolder', icon: folderOpen }],
      [
        { action: FolderGlobalAction.ImportFolder, label: 'FoldersPage.ImportFile.importFolderAction', icon: cloudUpload },
        { action: FolderGlobalAction.ImportFiles, label: 'FoldersPage.ImportFile.importFilesAction', icon: cloudUpload },
      ],
      [{ action: UserAction.Invite, label: 'UsersPage.inviteUser', icon: personAdd }],
    ];
  } else if (currentRouteIs(Routes.Workspaces)) {
    actions.value = [
      [{ action: WorkspaceAction.CreateWorkspace, label: 'WorkspacesPage.createWorkspace', icon: addCircle }],
      [{ action: UserAction.Invite, label: 'UsersPage.inviteUser', icon: personAdd }],
    ];
  } else if (currentRouteIs(Routes.Users) || currentRouteIs(Routes.MyProfile) || currentRouteIs(Routes.Organization)) {
    actions.value = [[{ action: UserAction.Invite, label: 'UsersPage.inviteUser', icon: personAdd }]];
  } else {
    actions.value = [];
  }
}

const watchWindowWidthCancel = watch(windowWidth, async () => {
  updateDividerPosition();
});

const watchRouteCancel = watchRoute(async () => {
  setActions();
});

onMounted(async () => {
  eventDistributorCbId = await eventDistributor.registerCallback(
    Events.WorkspaceCreated |
      Events.WorkspaceFavorite |
      Events.WorkspaceUpdated |
      Events.ExpiredOrganization |
      Events.MenuAction |
      Events.WorkspaceRoleUpdate |
      Events.DeviceCreated,
    async (event: Events, data?: EventData) => {
      if (event === Events.WorkspaceCreated) {
        await loadAll();
        securityWarnings.value = await getSecurityWarnings();
      } else if (event === Events.WorkspaceFavorite || event === Events.WorkspaceUpdated) {
        await loadAll();
      } else if (event === Events.ExpiredOrganization) {
        isExpired.value = true;
      } else if (event === Events.MenuAction) {
        if (isUserAction((data as MenuActionData).action.action)) {
          const userAction = (data as MenuActionData).action.action as UserAction;
          if (userAction === UserAction.Invite) {
            await navigateTo(Routes.Users, { query: { openInvite: true } });
          }
        }
      } else if (event === Events.DeviceCreated) {
        if (!securityWarnings.value || !securityWarnings.value.hasMultipleDevices || !securityWarnings.value.hasRecoveryDevice) {
          securityWarnings.value = await getSecurityWarnings();
        }
      } else if (event === Events.WorkspaceRoleUpdate) {
        const updateData = data as WorkspaceRoleUpdateData;

        // Don't need to check everything, we're only interested if a owner was added or removed
        if (updateData.newRole === null || updateData.newRole === WorkspaceRole.Owner) {
          securityWarnings.value = await getSecurityWarnings();
        }
      } else if (event === Events.Online) {
        securityWarnings.value = await getSecurityWarnings();
      }
    },
  );

  setActions();

  const savedSidebarData = await storageManager.retrieveComponentData<SidebarSavedData>(SIDEBAR_MENU_DATA_KEY, SidebarDefaultData);

  if (savedSidebarData.hidden) {
    computedWidth.value = 2;
    storedWidth.value = savedSidebarData.width;
  } else {
    computedWidth.value = savedSidebarData.width;
  }
  sidebarWidthProperty.value = `${computedWidth.value}px`;
  emits('sidebarWidthChanged', computedWidth.value);
  menusVisible.value.organization = savedSidebarData.organizationVisible ?? true;
  menusVisible.value.workspaces = savedSidebarData.workspacesVisible ?? true;
  menusVisible.value.favorites = savedSidebarData.favoritesVisible ?? true;
  menusVisible.value.recentWorkspaces = savedSidebarData.recentWorkspacesVisible ?? true;
  menusVisible.value.recentFiles = savedSidebarData.recentFilesVisible ?? false;

  const clientInfoResult = await getClientInfo();
  if (clientInfoResult.ok) {
    // clientInfoResult.organizationIsExpired;
    isExpired.value = false;
  }

  await loadAll();
  if (dividerRef.value) {
    const gesture = createGesture({
      gestureName: 'resize-menu',
      el: dividerRef.value,
      onMove,
    });
    gesture.enable();
  }

  securityWarnings.value = await getSecurityWarnings();

  updateDividerPosition();
});

onUnmounted(async () => {
  if (eventDistributorCbId) {
    eventDistributor.removeCallback(eventDistributorCbId);
  }
  if (timeoutId !== undefined) {
    clearTimeout(timeoutId);
  }
  watchSidebarWidthCancel();
  watchRouteCancel();
  watchWindowWidthCancel();
});

async function onActionClicked(action: MenuAction): Promise<void> {
  await eventDistributor.dispatchEvent(Events.MenuAction, { action: action });
}

async function goToWorkspace(workspace: WorkspaceInfo): Promise<void> {
  recentDocumentManager.addWorkspace(workspace);
  await recentDocumentManager.saveToStorage(storageManager);

  await navigateToWorkspace(workspace.handle);
  await menuController.close();
}

async function openSecurityWarningsPopover(event: MouseEvent): Promise<void> {
  if (!securityWarnings.value) {
    return;
  }
  const popover = await popoverController.create({
    component: RecommendationChecklistPopover,
    cssClass: 'recommendation-checklist',
    event: event,
    side: 'right',
    alignment: 'end',
    showBackdrop: false,
    backdropDismiss: true,
    componentProps: {
      securityWarnings: securityWarnings.value,
    },
  });
  await popover.present();
  const { data, role } = await popover.onWillDismiss();
  await popover.dismiss();
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

async function openOrganizationChoice(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: OrganizationSwitchPopover,
    cssClass: 'dropdown-popover',
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  const { data, role } = await popover.onDidDismiss();
  await popover.dismiss();
  if (role === MsModalResult.Confirm) {
    const menuCtrls = useUploadMenu();
    menuCtrls.hide();
    switchOrganization(data.handle);
  }
}

async function openPricingLink(): Promise<void> {
  window.open(I18n.translate({ key: 'app.pricingLink' }), '_blank');
}

async function openRecentFile(file: RecentFile): Promise<void> {
  const config = await storageManager.retrieveConfig();

  await openPath(file.workspaceHandle, file.path, informationManager, { skipViewers: config.skipViewers });
}

async function removeRecentFile(file: RecentFile): Promise<void> {
  recentDocumentManager.removeFile(file);
}

async function onOrganizationMenuVisibilityChanged(visible: boolean): Promise<void> {
  await storageManager.updateComponentData<SidebarSavedData>(
    SIDEBAR_MENU_DATA_KEY,
    {
      organizationVisible: visible,
    },
    SidebarDefaultData,
  );
}

async function onWorkspacesMenuVisibilityChanged(visible: boolean): Promise<void> {
  await storageManager.updateComponentData<SidebarSavedData>(
    SIDEBAR_MENU_DATA_KEY,
    {
      workspacesVisible: visible,
    },
    SidebarDefaultData,
  );
}

async function onRecentFilesMenuVisibilityChanged(visible: boolean): Promise<void> {
  await storageManager.updateComponentData<SidebarSavedData>(
    SIDEBAR_MENU_DATA_KEY,
    {
      recentFilesVisible: visible,
    },
    SidebarDefaultData,
  );
}

async function onFavoritesMenuVisibilityChanged(visible: boolean): Promise<void> {
  menusVisible.value.favorites = visible;
  await storageManager.updateComponentData<SidebarSavedData>(
    SIDEBAR_MENU_DATA_KEY,
    {
      favoritesVisible: visible,
    },
    SidebarDefaultData,
  );
}

async function onRecentWorkspacesMenuVisibilityChanged(visible: boolean): Promise<void> {
  menusVisible.value.recentWorkspaces = visible;
  await storageManager.updateComponentData<SidebarSavedData>(
    SIDEBAR_MENU_DATA_KEY,
    {
      recentWorkspacesVisible: visible,
    },
    SidebarDefaultData,
  );
}
</script>

<style lang="scss" scoped>
.large-display-menu-container {
  --side-min-width: var(--parsec-sidebar-menu-min-width);

  @include ms.responsive-breakpoint('sm') {
    flex-flow: column-reverse;
    --side-min-width: 100%;
    width: 100%;
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
  border-radius: 0 var(--parsec-radius-12) var(--parsec-radius-12) 0;
  position: relative;
  width: v-bind(sidebarWidthProperty);
  user-select: none;

  &::part(container) {
    display: flex;
    gap: 1.5rem;
  }

  @include ms.responsive-breakpoint('sm') {
    display: none;
  }

  // logo parsec
  &::before {
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

  .resize-divider {
    width: 0.25rem;
    height: 100%;
    position: absolute;
    left: calc(v-bind(dividerWidthProperty) - 3px);
    top: 0;
    z-index: 10000;
    cursor: ew-resize;
    display: flex;
    justify-content: center;

    &::after {
      content: '';
      position: absolute;
      width: 0.125rem;
      height: 100%;
      padding: 20rem 0;
    }

    &:hover::after,
    &:active::after {
      background: var(--parsec-color-light-primary-200);
    }
  }

  &-content {
    --background: transparent;
    position: relative;
    z-index: 12;

    &::part(scroll) {
      display: flex;
      flex-direction: column;
    }
  }
}

.organization-card {
  background: transparent;
  box-shadow: none;
  display: flex;
  flex-direction: column;
  margin: 0;
  border-radius: 0;
  gap: 1rem;

  &-header {
    display: flex;
    flex-direction: row;
    border-radius: var(--parsec-radius-12);
    box-shadow: none;
    align-items: center;
    margin: 1.5rem 0.5rem 0;
    padding: 0.75rem 1rem;
    gap: 0.5em;
    position: relative;
    z-index: 2;
    border: 1px solid transparent;

    &-desktop:hover {
      cursor: pointer;
      border-color: var(--parsec-color-light-primary-30-opacity15);
    }

    &-desktop .organization-avatar {
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

    .organization-text {
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

    .header-icon {
      white-space: nowrap;
      display: flex;
      align-items: center;
      --fill-color: var(--parsec-color-light-secondary-inversed-contrast);
      margin-left: auto;
      flex-shrink: 0;
    }
  }
}

.sidebar-content-organization {
  display: flex;
  flex-direction: column;
  user-select: none;
  gap: 0.5rem;
  padding-inline: 0.25rem;

  .sidebar-content-organization-button {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    cursor: pointer;
    align-items: center;
    color: var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-8);
    border: 1px solid transparent;

    &.active {
      background: var(--parsec-color-light-primary-30-opacity15);
      cursor: default;
    }

    &:hover:not(.active) {
      border: 1px solid var(--parsec-color-light-primary-30-opacity15);
    }

    &__icon {
      color: var(--parsec-color-light-secondary-inversed-contrast);
      flex-shrink: 0;
    }

    &__text {
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }
}

.sidebar-content-workspaces {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &-container {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  &__title {
    opacity: 0.6;
    padding: 0.25rem 0.25rem;
    color: var(--parsec-color-light-secondary-inversed-contrast);
    font-weight: 600;
    font-size: 0.75rem;
    text-transform: uppercase;
    display: flex;
    align-items: center;
    gap: 0.25rem;
    position: relative;

    ion-icon {
      transition: transform 0.15s ease-in-out;
    }

    &.open {
      ion-icon {
        transform: rotate(90deg);
      }
    }

    &:hover {
      cursor: pointer;
      opacity: 0.8;
    }
  }
}

.sidebar-content-files {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding-inline: 0.25rem;
}

.sidebar-content-files,
.sidebar-content-workspaces {
  &--no-recent {
    color: var(--parsec-color-light-secondary-premiere);
    opacity: 0.8;
    padding: 0.375rem 0.5rem;
  }
}

.current-workspace {
  margin-bottom: 1rem;
}

.freeze-card {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 1.5rem 1rem;
  margin-bottom: 1.5rem;
  border-radius: var(--parsec-radius-8);
  background: var(--parsec-color-light-secondary-inversed-contrast);
  position: relative;

  &-header {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &__icon {
      color: var(--parsec-color-light-secondary-grey);
      font-size: 1.5rem;
    }

    &__title {
      color: var(--parsec-color-light-primary-600);
    }
  }

  &-main {
    color: var(--parsec-color-light-secondary-soft-text);
    margin-bottom: 1rem;
    position: relative;
  }

  &-footer {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-text);

    &__email {
      color: var(--parsec-color-light-secondary-soft-text);
      background: var(--parsec-color-light-secondary-premiere);
      width: fit-content;
      border-radius: var(--parsec-radius-6);
      padding: 0.25rem 0.5rem;
    }
  }

  &__icon {
    color: var(--parsec-color-light-secondary-soft-grey);
    font-size: 11.5rem;
    position: absolute;
    bottom: 0;
    right: -5rem;
    z-index: 0;
    opacity: 0.1;
  }
}

.trial-card {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1.5rem 1rem;

  &__tag {
    color: var(--parsec-color-light-secondary-white);
    background: var(--parsec-color-light-gradient-background);
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
    --padding-start: 0;
    --padding-end: 0;
    --padding-top: 0;
    --padding-bottom: 0;
    color: var(--parsec-color-light-primary-600);

    &::part(native) {
      padding: 0.625rem 1rem;
    }
  }
}

.organization-checklist {
  display: flex;
  flex-direction: column;
  padding: 0.75rem 0.5rem;
  margin-top: auto;
  --background: none;

  .checklist {
    margin-top: auto;
    display: flex;
    width: 100%;
    border-radius: var(--parsec-radius-8);
    --min-height: 0;
    border: 1px solid var(--parsec-color-light-primary-30-opacity15);
    --background: none;
    box-shadow: none;
    position: relative;
    top: 0;
    transition: all 0.15s ease-in-out;

    &::part(native) {
      padding: 0.75rem 0.5rem 0.75rem 0.75rem;
    }

    &:hover {
      --background: var(--parsec-color-light-primary-30-opacity15);
      box-shadow: var(--parsec-shadow-light);
      top: -0.25rem;
    }

    & * {
      pointer-events: none;
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
      flex-grow: 1;

      &__title {
        color: var(--parsec-color-light-secondary-inversed-contrast);
      }

      &__description {
        color: var(--parsec-color-light-secondary-premiere);
        opacity: 0.8;
      }
    }

    &-button {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      color: var(--parsec-color-light-secondary-inversed-contrast);

      &__text {
        color: var(--parsec-color-light-secondary-inversed-contrast);
      }

      &__icon {
        color: var(--parsec-color-light-primary-30);
        font-size: 0.875rem;
      }
    }
  }
}
</style>
