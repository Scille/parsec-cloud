<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="users-page">
    <ion-content class="content-scroll">
      <!-- contextual menu -->
      <ms-action-bar
        id="activate-users-ms-action-bar"
        v-if="isLargeDisplay"
        :buttons="actionBarOptionsUsersPage"
      >
        <div class="right-side">
          <div class="counter">
            <ion-text
              class="body"
              v-show="users.selectedCount() === 0"
            >
              {{ $msTranslate({ key: 'UsersPage.itemCount', data: { count: getUsersCount() }, count: getUsersCount() }) }}
            </ion-text>
            <ion-text
              class="body item-selected"
              v-show="users.selectedCount() > 0"
            >
              {{
                $msTranslate({ key: 'UsersPage.userSelectedCount', data: { count: users.selectedCount() }, count: users.selectedCount() })
              }}
            </ion-text>
          </div>
          <ms-search-input
            :placeholder="'HomePage.organizationList.search'"
            v-model="users.searchFilter"
            @change="users.unselectHiddenUsers()"
            id="search-input-users"
          />
          <!-- prettier-ignore -->
          <user-filter
            :users="(users as UserCollection)"
            @change="onFilterUpdated"
          />
          <ms-sorter
            :key="`${currentSortProperty}-${currentSortOrder}`"
            :options="msSorterOptions"
            :default-option="currentSortProperty"
            :sorter-labels="msSorterLabels"
            :sort-by-asc="currentSortOrder"
            @change="onSortChange($event.option.key, $event.sortByAsc)"
          />
          <ms-grid-list-toggle
            v-model="displayView"
            @update:model-value="onDisplayStateChange"
          />
        </div>
      </ms-action-bar>
      <small-display-selection-header
        v-if="isSmallDisplay && selectionEnabled"
        :title="{ key: 'UsersPage.userSelectedCount', data: { count: users.selectedCount() }, count: users.selectedCount() }"
        @open-contextual-modal="openGlobalUserContextMenu"
        @select="selectAllUsers"
        @unselect="unselectAllUsers"
        @cancel-selection="onSelectionCancel"
        :some-selected="users.hasSelected()"
        :options-disabled="users.selectableUsersCount() === 0"
      />
      <!-- users -->
      <div class="main-container users-container scroll">
        <div
          v-show="users.totalUsersCount() === 0"
          class="no-active body-lg"
        >
          <div class="no-active-content">
            <ms-image :image="NoActiveUser" />
            <ion-text>
              {{ $msTranslate('UsersPage.emptyList') }}
            </ion-text>
          </div>
        </div>
        <div v-show="users.totalUsersCount() > 0">
          <div
            class="mobile-filters"
            v-if="isSmallDisplay"
          >
            <div class="mobile-filters-buttons">
              <user-filter
                :users="users as UserCollection"
                @change="onFilterUpdated"
                class="mobile-filters-buttons__filter"
              />
              <ms-sorter
                :key="`${currentSortProperty}-${currentSortOrder}`"
                :options="msSorterOptions"
                :default-option="currentSortProperty"
                :sorter-labels="msSorterLabels"
                :sort-by-asc="currentSortOrder"
                @change="onSortChange($event.option.key, $event.sortByAsc)"
                class="mobile-filterss-buttons__sorter"
              />
            </div>
            <ms-search-input
              :placeholder="'HomePage.organizationList.search'"
              v-model="users.searchFilter"
              @change="users.unselectHiddenUsers()"
              id="search-input-users"
              class="mobile-filters__search"
            />
          </div>
          <div v-if="displayView === DisplayState.List">
            <user-list-display
              :users="users"
              @menu-click="openUserContextMenu"
              @checkbox-click="selectionEnabled = users.hasSelected()"
              :selection-enabled="selectionEnabled && isSmallDisplay"
              :sort-by="currentSortProperty"
              :sort-ascending="currentSortOrder"
              @sort-update="onSortChange"
              :allow-selection="isAdmin"
            />
          </div>
          <div v-else>
            <user-grid-display
              :users="users"
              @menu-click="openUserContextMenu"
              @checkbox-click="selectionEnabled = users.hasSelected()"
              :selection-enabled="selectionEnabled && isSmallDisplay"
              :allow-selection="isAdmin"
            />
          </div>
        </div>
      </div>
      <tab-bar-options
        v-if="customTabBar.isVisible.value"
        :actions="tabBarActions"
      />
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { copyToClipboard } from '@/common/clipboard';
import SmallDisplaySelectionHeader from '@/components/header/SmallDisplaySelectionHeader.vue';
import { SortProperty, UserCollection, UserFilter, UserFilterLabels, UserModel } from '@/components/users';
import {
  ClientInfo,
  InvitationStatus,
  UserID,
  UserInfo,
  UserProfile,
  getAsyncEnrollmentAddr,
  getClientInfo as parsecGetClientInfo,
  listUsers as parsecListUsers,
} from '@/parsec';
import { Routes, currentRouteIsUserRoute, navigateTo, watchRoute } from '@/router';
import { UserAction, useUserActions, useUserContextMenu } from '@/services/contextMenu';
import { EventData, EventDistributor, EventDistributorKey, Events, InvitationUpdatedData } from '@/services/eventDistributor';
import useHeaderControl from '@/services/headerControl';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { MenuAction, TabBarOptions, useCustomTabBar } from '@/views/menu';
import UserGridDisplay from '@/views/users/UserGridDisplay.vue';
import UserListDisplay from '@/views/users/UserListDisplay.vue';
import { IonContent, IonPage, IonText } from '@ionic/vue';
import { informationCircle, link, personAdd, personRemove, repeat, returnUpForward } from 'ionicons/icons';
import {
  DisplayState,
  MsActionBar,
  MsGridListToggle,
  MsImage,
  MsOptions,
  MsSearchInput,
  MsSorter,
  NoActiveUser,
  useWindowSize,
} from 'megashark-lib';
import { Ref, computed, inject, onMounted, onUnmounted, ref, toRaw, watch } from 'vue';

const contextMenu = useUserContextMenu();
const actions = useUserActions();

const displayView = ref(DisplayState.List);
const isAdmin = ref(false);
const clientInfo: Ref<ClientInfo | null> = ref(null);
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
const selectionEnabled = ref<boolean>(false);

let hotkeys: HotkeyGroup | null = null;
const users = ref(new UserCollection());
const currentSortProperty: Ref<SortProperty> = ref(SortProperty.Profile);
const currentSortOrder = ref(true);
let eventCbId: string | null = null;

const USERS_PAGE_DATA_KEY = 'UsersPage';
const { isLargeDisplay, isSmallDisplay } = useWindowSize();
const { hideHeader, showHeader } = useHeaderControl();

const customTabBar = useCustomTabBar();

const tabBarActions = computed(() => {
  const selectedUsers = users.value.getSelectedUsers();
  const actions: MenuAction[] = [];
  if (selectedUsers.length === 1) {
    actions.push({ label: 'UsersPage.tabbar.details', action: async () => await openSelectedUserDetails(), icon: informationCircle });
    actions.push({
      label: 'UsersPage.tabbar.roles',
      action: async () => await assignWorkspaceRoles(selectedUsers[0]),
      icon: returnUpForward,
    });
  }
  if (isAdmin.value) {
    if (selectedUsers.some((u: UserModel) => u.currentProfile !== UserProfile.Outsider)) {
      actions.push({ label: 'UsersPage.tabbar.update', action: async () => await updateSelectedUserProfiles(), icon: repeat });
    }
    actions.push({ label: 'UsersPage.tabbar.revoke', action: async () => await revokeSelectedUsers(), icon: personRemove, danger: true });
  }
  return actions;
});

const tabBarWatchCancel = watch([(): boolean => isSmallDisplay.value, (): number => users.value.getSelectedUsers().length], () => {
  if (isSmallDisplay.value && users.value.hasSelected() && tabBarActions.value.length > 0) {
    customTabBar.show();
  } else {
    customTabBar.hide();
  }
});

interface UsersPageSavedData {
  displayState: DisplayState;
  filters: UserFilterLabels;
  sortProperty: SortProperty;
  sortAscending: boolean;
}

async function storeComponentData(): Promise<void> {
  await storageManager.storeComponentData<UsersPageSavedData>(USERS_PAGE_DATA_KEY, {
    displayState: displayView.value,
    filters: toRaw(users.value.getFilters()),
    sortProperty: currentSortProperty.value,
    sortAscending: currentSortOrder.value,
  });
}

async function restoreComponentData(): Promise<void> {
  const data: UsersPageSavedData = await storageManager.retrieveComponentData<UsersPageSavedData>(USERS_PAGE_DATA_KEY, {
    displayState: DisplayState.List,
    filters: {
      statusActive: true,
      statusRevoked: true,
      statusFrozen: true,
      profileAdmin: true,
      profileStandard: true,
      profileOutsider: true,
    },
    sortProperty: SortProperty.Profile,
    sortAscending: true,
  });

  displayView.value = data.displayState;
  users.value.setFilters(data.filters);
  currentSortProperty.value = data.sortProperty;
  currentSortOrder.value = data.sortAscending;
}

async function onDisplayStateChange(): Promise<void> {
  await storeComponentData();
}

const ALL_SORT_OPTIONS = [
  { label: 'UsersPage.sort.byName', key: SortProperty.Name },
  { label: 'UsersPage.sort.byJoined', key: SortProperty.JoinedDate },
  { label: 'UsersPage.sort.byProfile', key: SortProperty.Profile },
  { label: 'UsersPage.sort.byStatus', key: SortProperty.Status },
  { label: 'UsersPage.sort.byEmail', key: SortProperty.Email },
];

const msSorterOptions = ref<MsOptions>(new MsOptions(ALL_SORT_OPTIONS));

const msSorterLabels = {
  asc: 'UsersPage.sort.asc',
  desc: 'UsersPage.sort.desc',
};

async function onSortChange(property: SortProperty, ascending: boolean): Promise<void> {
  currentSortProperty.value = property;
  currentSortOrder.value = ascending;
  users.value.sort(currentSortProperty.value, currentSortOrder.value);
  await storeComponentData();
}

async function onFilterUpdated(): Promise<void> {
  await storeComponentData();
}

function getUsersCount(): number {
  return users.value.usersCount();
}

async function revokeSelectedUsers(): Promise<void> {
  const selectedUsers = users.value.getSelectedUsers();
  await actions.revokeUsers(selectedUsers);

  await onSelectionCancel();
  await refreshUserList();
}

async function openSelectedUserDetails(): Promise<void> {
  const selectedUsers = users.value.getSelectedUsers();

  if (selectedUsers.length === 1) {
    await actions.openDetails(selectedUsers[0]);
  }
}

function isCurrentUser(userId: UserID): boolean {
  return clientInfo.value !== null && clientInfo.value.userId === userId;
}

async function openUserContextMenu(event: Event, user: UserModel, onFinished?: () => void): Promise<void> {
  const currentUser = users.value.getCurrentUser();
  if (!currentUser) {
    return;
  }
  let selectedUsers = users.value.getSelectedUsers();
  if (selectedUsers.length === 0 || !selectedUsers.includes(user)) {
    users.value.selectAll(false);
    selectedUsers = [user];
  }

  await contextMenu.openUserContextMenu(selectedUsers, currentUser, event);

  if (onFinished) {
    onFinished();
  }
  await onSelectionCancel();
  await refreshUserList();
}

async function openGlobalUserContextMenu(): Promise<void> {
  const action = await contextMenu.openGlobalUserContextMenu();

  switch (action) {
    case UserAction.ToggleSelect: {
      await toggleSelection();
      break;
    }
    case UserAction.SelectAll: {
      await selectAllUsers();
      break;
    }
    default: {
      break;
    }
  }
}

async function toggleSelection(): Promise<void> {
  selectionEnabled.value = !selectionEnabled.value;
}

async function selectAllUsers(): Promise<void> {
  if (!isAdmin.value) {
    return;
  }
  selectionEnabled.value = true;
  users.value.selectAll(true);
}

async function unselectAllUsers(): Promise<void> {
  users.value.selectAll(false);
}

async function onSelectionCancel(): Promise<void> {
  await unselectAllUsers();
  selectionEnabled.value = false;
}

async function updateSelectedUserProfiles(): Promise<void> {
  await updateUserProfiles(users.value.getSelectedUsers());
}

async function updateUserProfiles(selectedUsers: Array<UserInfo>): Promise<void> {
  await actions.updateProfiles(selectedUsers);
  await onSelectionCancel();
  await refreshUserList();
}

async function assignWorkspaceRoles(user: UserInfo): Promise<void> {
  const currentUser = users.value.getCurrentUser();
  if (!currentUser) {
    return;
  }
  await actions.copyWorkspaceRoles(user, currentUser);
  onSelectionCancel();
}

async function refreshUserList(): Promise<void> {
  const result = await parsecListUsers(false);
  const newUsers: UserModel[] = [];
  if (result.ok) {
    for (const user of result.value) {
      (user as UserModel).isSelected = false;
      (user as UserModel).isCurrent = isCurrentUser(user.id);
      newUsers.push(user as UserModel);
    }
    users.value.replace(newUsers);
  } else {
    informationManager.value.present(
      new Information({
        message: 'UsersPage.listUsersFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  const hasInactive = users.value.hasInactive();
  if (!hasInactive && currentSortProperty.value === SortProperty.Status) {
    currentSortProperty.value = SortProperty.Name;
  }

  if (!hasInactive) {
    msSorterOptions.value = new MsOptions(ALL_SORT_OPTIONS.filter((opt) => opt.key !== SortProperty.Status));
  } else {
    msSorterOptions.value = new MsOptions(ALL_SORT_OPTIONS);
  }
  users.value.sort(currentSortProperty.value, currentSortOrder.value);
}

const routeWatchCancel = watchRoute(async () => {
  if (!currentRouteIsUserRoute()) {
    return;
  }
  await refreshUserList();
});

async function handleEvents(event: Events, data?: EventData): Promise<void> {
  if (event === Events.InvitationUpdated && data) {
    if ((data as InvitationUpdatedData).status === InvitationStatus.Finished) {
      await refreshUserList();
    }
  } else if (event === Events.OpenContextMenu) {
    await openGlobalUserContextMenu();
  }
}

onMounted(async (): Promise<void> => {
  eventCbId = await eventDistributor.value.registerCallback([Events.InvitationUpdated, Events.OpenContextMenu], handleEvents);
  await restoreComponentData();

  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add(
    { key: 'g', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Users },
    async () => {
      displayView.value = displayView.value === DisplayState.List ? DisplayState.Grid : DisplayState.List;
    },
  );
  hotkeys.add({ key: 'a', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Users }, async () =>
    selectAllUsers(),
  );

  const result = await parsecGetClientInfo();

  if (result.ok) {
    clientInfo.value = result.value;
    isAdmin.value = clientInfo.value.currentProfile === UserProfile.Admin;
  }
  await refreshUserList();
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  if (eventCbId) {
    await eventDistributor.value.removeCallback(eventCbId);
  }
  selectionEnabled.value = false;
  customTabBar.hide();
  routeWatchCancel();
  tabBarWatchCancel();
  headerWatchCancel();
});

const actionBarOptionsUsersPage = computed(() => {
  const actionArray = [];

  if (users.value.selectedCount() === 0 && isAdmin.value) {
    actionArray.push({
      label: 'UsersPage.inviteUser',
      icon: personAdd,
      onClick: async (): Promise<void> => {
        await navigateTo(Routes.Invitations, { query: { openInvite: true } });
      },
    });

    actionArray.push({
      label: 'InvitationsPage.asyncEnrollmentRequest.copyLink',
      icon: link,
      onClick: async (): Promise<void> => {
        const result = await getAsyncEnrollmentAddr();
        if (result.ok) {
          const [_, invitationAddrAsHttpRedirection] = result.value;
          await copyToClipboard(
            invitationAddrAsHttpRedirection,
            informationManager.value,
            'InvitationsPage.asyncEnrollmentRequest.linkCopiedToClipboard.success',
            'InvitationsPage.asyncEnrollmentRequest.linkCopiedToClipboard.failed',
          );
        }
      },
    });
  }

  if (users.value.selectedCount() >= 1 && isAdmin.value) {
    actionArray.push({
      label: { key: 'UsersPage.userContextMenu.actionRevoke', count: users.value.selectedCount() },
      icon: personRemove,
      onClick: async (): Promise<void> => {
        await revokeSelectedUsers();
      },
    });
  }

  if (
    users.value.selectedCount() >= 1 &&
    users.value.getSelectedUsers().some((u: UserModel) => u.currentProfile !== UserProfile.Outsider) &&
    isAdmin.value
  ) {
    actionArray.push({
      label: { key: 'UsersPage.userContextMenu.actionUpdateProfile', count: users.value.selectedCount() },
      icon: repeat,
      onClick: async (): Promise<void> => {
        await updateSelectedUserProfiles();
      },
    });
  }

  if (users.value.selectedCount() === 1) {
    actionArray.push({
      label: 'UsersPage.userContextMenu.actionDetails',
      icon: informationCircle,
      onClick: async (): Promise<void> => {
        await openSelectedUserDetails();
      },
    });
  }

  return actionArray;
});

const headerWatchCancel = watch([isSmallDisplay, selectionEnabled], () => {
  isSmallDisplay.value && selectionEnabled.value ? hideHeader() : showHeader();
});
</script>

<style scoped lang="scss">
.users-page {
  @include ms.responsive-breakpoint('sm') {
    --background: var(--parsec-color-light-secondary-background);
  }

  .content-scroll::part(background) {
    @include ms.responsive-breakpoint('sm') {
      background: var(--parsec-color-light-secondary-background);
    }
  }
}

.no-active {
  width: 100%;
  height: 100%;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  margin: auto;
  align-items: center;

  &-content {
    border-radius: var(--parsec-radius-8);
    display: flex;
    height: fit-content;
    width: 100%;
    text-align: center;
    flex-direction: column;
    gap: 1rem;
    align-items: center;
    padding: 2rem 1rem;
  }
}

.users-container {
  @include ms.responsive-breakpoint('sm') {
    position: sticky;
    z-index: 10;
    background: var(--parsec-color-light-secondary-white);
    box-shadow: var(--parsec-shadow-strong);
    border-radius: var(--parsec-radius-18) var(--parsec-radius-18) 0 0;
  }
}

.users-container > div {
  height: 100%;
}
</style>
