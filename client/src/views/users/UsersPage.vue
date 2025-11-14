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
            :label="'UsersPage.sort.byName'"
            :options="msSorterOptions"
            :default-option="currentSortProperty"
            :sorter-labels="msSorterLabels"
            :sort-by-asc="currentSortOrder"
            @change="onSortChange"
          />
          <ms-grid-list-toggle
            v-model="displayView"
            @update:model-value="onDisplayStateChange"
          />
        </div>
      </ms-action-bar>
      <small-display-header-title
        v-if="isSmallDisplay"
        :title="
          users.selectedCount() > 0
            ? { key: 'UsersPage.userSelectedCount', data: { count: users.selectedCount() }, count: users.selectedCount() }
            : 'HeaderPage.titles.users'
        "
        @open-contextual-modal="openGlobalUserContextMenu"
        @select="selectAllUsers"
        @unselect="unselectAllUsers"
        @cancel-selection="onSelectionCancel"
        :selection-enabled="selectionEnabled"
        :some-selected="users.hasSelected()"
        :options-disabled="users.selectableUsersCount() === 0"
      />
      <!-- users -->
      <div class="users-container scroll">
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
                :label="'UsersPage.sort.byName'"
                :options="msSorterOptions"
                :default-option="currentSortProperty"
                :sorter-labels="msSorterLabels"
                :sort-by-asc="currentSortOrder"
                @change="onSortChange"
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
            <!-- prettier-ignore -->
            <user-list-display
              :users="(users as UserCollection)"
              @menu-click="openUserContextMenu"
              @checkbox-click="selectionEnabled = true"
              :selection-enabled="selectionEnabled && isSmallDisplay"
            />
          </div>
          <div
            v-else
            class="users-container-grid"
          >
            <!-- prettier-ignore -->
            <user-grid-display
              :users="(users as UserCollection)"
              @menu-click="openUserContextMenu"
              @checkbox-click="selectionEnabled = true"
              :selection-enabled="selectionEnabled && isSmallDisplay"
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
import {
  Answer,
  MsOptions,
  askQuestion,
  MsImage,
  NoActiveUser,
  DisplayState,
  MsActionBar,
  MsGridListToggle,
  MsSearchInput,
  MsSorter,
  MsSorterChangeEvent,
  MsModalResult,
  useWindowSize,
} from 'megashark-lib';
import SmallDisplayHeaderTitle from '@/components/header/SmallDisplayHeaderTitle.vue';
import { SortProperty, UserCollection, UserFilter, UserFilterLabels, UserModel } from '@/components/users';
import {
  ClientInfo,
  UserID,
  UserInfo,
  UserProfile,
  getClientInfo as parsecGetClientInfo,
  listUsers as parsecListUsers,
  revokeUser as parsecRevokeUser,
  InvitationStatus,
  updateProfile as parsecUpdateProfile,
  ClientUserUpdateProfileError,
  ClientUserUpdateProfileErrorTag,
  getPkiJoinOrganizationLink,
} from '@/parsec';
import { Routes, watchRoute, currentRouteIsUserRoute, navigateTo } from '@/router';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { UserAction } from '@/views/users/types';
import UserDetailsModal from '@/views/users/UserDetailsModal.vue';
import UserGridDisplay from '@/views/users/UserGridDisplay.vue';
import UserListDisplay from '@/views/users/UserListDisplay.vue';
import { IonContent, IonPage, IonText, modalController } from '@ionic/vue';
import { informationCircle, link, personAdd, personRemove, repeat, returnUpForward } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, ref, toRaw, computed, watch } from 'vue';
import BulkRoleAssignmentModal from '@/views/users/BulkRoleAssignmentModal.vue';
import { EventData, EventDistributor, EventDistributorKey, Events, InvitationUpdatedData } from '@/services/eventDistributor';
import UpdateProfileModal from '@/views/users/UpdateProfileModal.vue';
import { openUserContextMenu as _openUserContextMenu, openGlobalUserContextMenu as _openGlobalUserContextMenu } from '@/views/users/utils';
import { MenuAction, TabBarOptions, useCustomTabBar } from '@/views/menu';
import { copyToClipboard } from '@/common/clipboard';

const displayView = ref(DisplayState.List);
const isAdmin = ref(false);
const clientInfo: Ref<ClientInfo | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const selectionEnabled = ref<boolean>(false);

let hotkeys: HotkeyGroup | null = null;
const users = ref(new UserCollection());
const currentSortProperty: Ref<SortProperty> = ref(SortProperty.Profile);
const currentSortOrder = ref(true);
let eventCbId: string | null = null;

const USERS_PAGE_DATA_KEY = 'UsersPage';
const { isLargeDisplay, isSmallDisplay } = useWindowSize();

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
];

const msSorterOptions = ref<MsOptions>(new MsOptions(ALL_SORT_OPTIONS));

const msSorterLabels = {
  asc: 'UsersPage.sort.asc',
  desc: 'UsersPage.sort.desc',
};

async function onSortChange(event: MsSorterChangeEvent): Promise<void> {
  currentSortProperty.value = event.option.key;
  currentSortOrder.value = event.sortByAsc;
  users.value.sort(currentSortProperty.value, currentSortOrder.value);
  await storeComponentData();
}

async function onFilterUpdated(): Promise<void> {
  await storeComponentData();
}

async function revokeUser(user: UserInfo): Promise<void> {
  const answer = await askQuestion(
    { key: 'UsersPage.revocation.revokeTitle', count: 1 },
    { key: 'UsersPage.revocation.revokeQuestion', data: { user: user.humanHandle.label }, count: 1 },
    {
      yesIsDangerous: true,
      yesText: 'UsersPage.revocation.revokeYes',
      noText: 'UsersPage.revocation.revokeNo',
    },
  );
  if (answer === Answer.No) {
    return;
  }
  const result = await parsecRevokeUser(user.id);

  if (!result.ok) {
    informationManager.present(
      new Information({
        message: { key: 'UsersPage.revocation.revokeFailed', count: 1 },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: { key: 'UsersPage.revocation.revokeSuccess', data: { user: user.humanHandle.label }, count: 1 },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
  await onSelectionCancel();
  await refreshUserList();
}

function getUsersCount(): number {
  return users.value.usersCount();
}

async function revokeSelectedUsers(): Promise<void> {
  const selectedUsers = users.value.getSelectedUsers();

  if (selectedUsers.length === 1) {
    return await revokeUser(selectedUsers[0]);
  }

  const answer = await askQuestion(
    { key: 'UsersPage.revocation.revokeTitle', count: selectedUsers.length },
    { key: 'UsersPage.revocation.revokeQuestion', data: { count: selectedUsers.length }, count: selectedUsers.length },
    {
      yesIsDangerous: true,
      yesText: 'UsersPage.revocation.revokeYes',
      noText: 'UsersPage.revocation.revokeNo',
    },
  );
  if (answer === Answer.No) {
    return;
  }
  let errorCount = 0;

  for (const user of selectedUsers) {
    const result = await parsecRevokeUser(user.id);
    if (!result.ok) {
      errorCount += 1;
    }
  }
  if (errorCount === 0) {
    informationManager.present(
      new Information({
        message: {
          key: 'UsersPage.revocation.revokeSuccess',
          data: { count: selectedUsers.length },
          count: selectedUsers.length,
        },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  } else if (errorCount < selectedUsers.length) {
    informationManager.present(
      new Information({
        message: 'UsersPage.revocation.revokeSomeFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: { key: 'UsersPage.revocation.revokeFailed', count: selectedUsers.length },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  await onSelectionCancel();
  await refreshUserList();
}

async function openUserDetails(user: UserInfo): Promise<void> {
  const modal = await modalController.create({
    component: UserDetailsModal,
    cssClass: 'user-details-modal',
    componentProps: {
      user: user,
      informationManager: informationManager,
    },
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
}

async function openSelectedUserDetails(): Promise<void> {
  const selectedUsers = users.value.getSelectedUsers();

  if (selectedUsers.length === 1) {
    return await openUserDetails(selectedUsers[0]);
  }
}

function isCurrentUser(userId: UserID): boolean {
  return clientInfo.value !== null && clientInfo.value.userId === userId;
}

async function openUserContextMenu(event: Event, user: UserModel, onFinished?: () => void): Promise<void> {
  let selectedUsers = users.value.getSelectedUsers();
  if (selectedUsers.length === 0 || !selectedUsers.includes(user)) {
    selectedUsers = [user];
  }

  // a Standard or Outsider user can't do anything with multiple users
  if (!isAdmin.value && selectedUsers.length > 1) {
    return;
  }

  const data = await _openUserContextMenu(event, selectedUsers, isAdmin.value, isLargeDisplay.value);

  const actions = new Map<UserAction, (user: UserModel) => Promise<void>>([
    [UserAction.Revoke, revokeUser],
    [UserAction.Details, openUserDetails],
    [UserAction.AssignRoles, assignWorkspaceRoles],
    [UserAction.UpdateProfile, updateUserProfile],
  ]);
  const actionsMultiple = new Map<UserAction, () => Promise<void>>([
    [UserAction.Revoke, revokeSelectedUsers],
    [UserAction.UpdateProfile, updateSelectedUserProfiles],
  ]);

  if (!data) {
    if (onFinished) {
      onFinished();
    }
    return;
  }

  if (selectedUsers.length === 1) {
    const fn = actions.get(data.action);
    if (fn) {
      await fn(selectedUsers[0]);
    }
  } else {
    const fn = actionsMultiple.get(data.action);
    if (fn) {
      await fn();
    }
  }

  if (onFinished) {
    onFinished();
  }
}

async function openGlobalUserContextMenu(): Promise<void> {
  const data = await _openGlobalUserContextMenu();

  const actions = new Map<UserAction, () => Promise<void>>([
    [UserAction.ToggleSelect, toggleSelection],
    [UserAction.SelectAll, selectAllUsers],
  ]);

  if (!data) {
    return;
  }

  const fn = actions.get(data.action);
  if (fn) {
    await fn();
  }
}

async function toggleSelection(): Promise<void> {
  selectionEnabled.value = !selectionEnabled.value;
}

async function selectAllUsers(): Promise<void> {
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

async function updateUserProfile(user: UserInfo): Promise<void> {
  await updateUserProfiles([user]);
}

async function updateUserProfiles(selectedUsers: Array<UserInfo>): Promise<void> {
  const modal = await modalController.create({
    component: UpdateProfileModal,
    cssClass: 'update-profile-modal',
    componentProps: {
      users: selectedUsers,
    },
  });
  await modal.present();
  const { data, role } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm) {
    return;
  }
  const newProfile = data.profile;
  let firstError: ClientUserUpdateProfileError | undefined = undefined;
  const affectedUsers = selectedUsers.filter((u) => u.currentProfile !== UserProfile.Outsider);

  for (const user of affectedUsers) {
    if (user.currentProfile === newProfile) {
      continue;
    }
    const result = await parsecUpdateProfile(user.id, newProfile);
    if (!result.ok) {
      if (!firstError) {
        firstError = result.error;
      }
    }
  }
  let message = '';
  if (!firstError) {
    message = 'UsersPage.updateProfile.success';
  } else {
    switch (firstError.tag) {
      case ClientUserUpdateProfileErrorTag.Offline:
        message = 'UsersPage.updateProfile.failedOffline';
        break;
      default:
        message = 'UsersPage.updateProfile.failedGeneric';
        break;
    }
  }
  informationManager.present(
    new Information({
      message: { key: message, count: affectedUsers.length },
      level: firstError === undefined ? InformationLevel.Success : InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );

  await onSelectionCancel();
  await refreshUserList();
}

async function assignWorkspaceRoles(user: UserInfo): Promise<void> {
  const modal = await modalController.create({
    component: BulkRoleAssignmentModal,
    cssClass: 'role-assignment-modal',
    componentProps: {
      sourceUser: user,
      currentUser: users.value.getCurrentUser(),
      informationManager: informationManager,
    },
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();
  if (result.role === MsModalResult.Confirm) {
    onSelectionCancel();
  }
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
    informationManager.present(
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

onMounted(async (): Promise<void> => {
  eventCbId = await eventDistributor.registerCallback(Events.InvitationUpdated, async (event: Events, data?: EventData) => {
    if (event === Events.InvitationUpdated && data) {
      if ((data as InvitationUpdatedData).status === InvitationStatus.Finished) {
        await refreshUserList();
      }
    }
  });
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
    await eventDistributor.removeCallback(eventCbId);
  }
  selectionEnabled.value = false;
  customTabBar.hide();
  routeWatchCancel();
  tabBarWatchCancel();
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
    // eslint-disable-next-line no-constant-condition
    if (false) {
      // TODO enable with PKI support
      actionArray.push({
        label: 'InvitationsPage.pkiRequests.copyLink',
        icon: link,
        onClick: async (): Promise<void> => {
          const result = await getPkiJoinOrganizationLink();
          if (result.ok) {
            await copyToClipboard(
              result.value,
              informationManager,
              'InvitationsPage.pkiRequests.linkCopiedToClipboard.success',
              'InvitationsPage.pkiRequests.linkCopiedToClipboard.failed',
            );
          }
        },
      });
    }
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

.users-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}
</style>
