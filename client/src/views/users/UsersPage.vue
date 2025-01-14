<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <!-- contextual menu -->
      <ms-action-bar id="activate-users-ms-action-bar">
        <div v-show="users.selectedCount() === 0 && isAdmin">
          <ms-action-bar-button
            :icon="personAdd"
            id="button-invite-user"
            :button-label="'UsersPage.inviteUser'"
            @click="inviteUser()"
          />
        </div>
        <!-- revoke or view common workspace -->
        <div v-show="users.selectedCount() >= 1 && isAdmin">
          <ms-action-bar-button
            :icon="personRemove"
            class="danger"
            id="button-revoke-user"
            :button-label="{ key: 'UsersPage.userContextMenu.actionRevoke', count: users.selectedCount() }"
            @click="revokeSelectedUsers"
          />
        </div>
        <div v-show="users.selectedCount() === 1">
          <ms-action-bar-button
            :icon="informationCircle"
            id="button-common-workspaces"
            :button-label="'UsersPage.userContextMenu.actionDetails'"
            @click="openSelectedUserDetails"
          />
        </div>
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
          <div v-if="displayView === DisplayState.List">
            <!-- prettier-ignore -->
            <user-list-display
              :users="(users as UserCollection)"
              @menu-click="openUserContextMenu"
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
            />
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { emailValidator } from '@/common/validators';
import {
  Answer,
  MsOptions,
  askQuestion,
  getTextFromUser,
  MsImage,
  NoActiveUser,
  DisplayState,
  MsActionBar,
  MsActionBarButton,
  MsGridListToggle,
  MsSearchInput,
  MsSorter,
  MsSorterChangeEvent,
  Translatable,
} from 'megashark-lib';
import { SortProperty, UserCollection, UserFilter, UserFilterLabels, UserModel } from '@/components/users';
import {
  ClientInfo,
  ClientNewUserInvitationErrorTag,
  InvitationEmailSentStatus,
  UserID,
  UserInfo,
  UserProfile,
  getClientInfo as parsecGetClientInfo,
  inviteUser as parsecInviteUser,
  listUsers as parsecListUsers,
  revokeUser as parsecRevokeUser,
  InvitationStatus,
} from '@/parsec';
import { Routes, getCurrentRouteQuery, watchRoute, currentRouteIsUserRoute } from '@/router';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import UserContextMenu, { UserAction } from '@/views/users/UserContextMenu.vue';
import UserDetailsModal from '@/views/users/UserDetailsModal.vue';
import UserGridDisplay from '@/views/users/UserGridDisplay.vue';
import UserListDisplay from '@/views/users/UserListDisplay.vue';
import { IonContent, IonPage, IonText, modalController, popoverController } from '@ionic/vue';
import { informationCircle, personAdd, personRemove } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, ref, toRaw } from 'vue';
import BulkRoleAssignmentModal from '@/views/users/BulkRoleAssignmentModal.vue';
import { EventData, EventDistributor, EventDistributorKey, Events, InvitationUpdatedData } from '@/services/eventDistributor';

const displayView = ref(DisplayState.List);
const isAdmin = ref(false);
const clientInfo: Ref<ClientInfo | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;

let hotkeys: HotkeyGroup | null = null;
const users = ref(new UserCollection());
const currentSortProperty: Ref<SortProperty> = ref(SortProperty.Profile);
const currentSortOrder = ref(true);
let eventCbId: string | null = null;

const USERS_PAGE_DATA_KEY = 'UsersPage';

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

const msSorterOptions: MsOptions = new MsOptions([
  { label: 'UsersPage.sort.byName', key: SortProperty.Name },
  { label: 'UsersPage.sort.byJoined', key: SortProperty.JoinedDate },
  { label: 'UsersPage.sort.byProfile', key: SortProperty.Profile },
  { label: 'UsersPage.sort.byStatus', key: SortProperty.Status },
]);

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

async function openUserContextMenu(event: Event, user: UserInfo, onFinished?: () => void): Promise<void> {
  const popover = await popoverController.create({
    component: UserContextMenu,
    cssClass: 'user-context-menu',
    event: event,
    translucent: true,
    reference: event.type === 'contextmenu' ? 'event' : 'trigger',
    showBackdrop: false,
    dismissOnSelect: true,
    alignment: 'start',
    componentProps: {
      isRevoked: user.isRevoked(),
      clientIsAdmin: isAdmin.value,
    },
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  const actions = new Map<UserAction, (user: UserInfo) => Promise<void>>([
    [UserAction.Revoke, revokeUser],
    [UserAction.Details, openUserDetails],
    [UserAction.AssignRoles, assignWorkspaceRoles],
  ]);

  if (!data) {
    if (onFinished) {
      onFinished();
    }
    return;
  }

  const fn = actions.get(data.action);
  if (fn) {
    await fn(user);
  }
  if (onFinished) {
    onFinished();
  }
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
  await modal.onWillDismiss();
  await modal.dismiss();
}

async function inviteUser(): Promise<void> {
  const email = await getTextFromUser({
    title: 'UsersPage.CreateUserInvitationModal.pageTitle',
    trim: true,
    validator: emailValidator,
    inputLabel: 'UsersPage.CreateUserInvitationModal.label',
    placeholder: 'UsersPage.CreateUserInvitationModal.placeholder',
    okButtonText: 'UsersPage.CreateUserInvitationModal.create',
  });
  if (!email) {
    return;
  }
  const result = await parsecInviteUser(email);
  if (result.ok) {
    if (result.value.emailSentStatus === InvitationEmailSentStatus.Success) {
      informationManager.present(
        new Information({
          message: {
            key: 'UsersPage.invitation.inviteSuccessMailSent',
            data: {
              email: email,
            },
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: {
            key: 'UsersPage.invitation.inviteSuccessNoMail',
            data: {
              email: email,
            },
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    let message: Translatable = '';
    switch (result.error.tag) {
      case ClientNewUserInvitationErrorTag.AlreadyMember:
        message = { key: 'UsersPage.invitation.inviteFailedAlreadyMember', data: { email: email } };
        break;
      case ClientNewUserInvitationErrorTag.Offline:
        message = 'UsersPage.invitation.inviteFailedOffline';
        break;
      case ClientNewUserInvitationErrorTag.NotAllowed:
        message = 'UsersPage.invitation.inviteFailedNotAllowed';
        break;
      default:
        message = {
          key: 'UsersPage.invitation.inviteFailedUnknown',
          data: {
            reason: result.error.tag,
          },
        };
        break;
    }
    informationManager.present(
      new Information({
        message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
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
  users.value.sort(currentSortProperty.value, currentSortOrder.value);
}

const routeWatchCancel = watchRoute(async () => {
  if (!currentRouteIsUserRoute()) {
    return;
  }
  const query = getCurrentRouteQuery();
  if (query.openInvite) {
    await inviteUser();
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
    users.value.selectAll(true),
  );

  const result = await parsecGetClientInfo();

  if (result.ok) {
    clientInfo.value = result.value;
    isAdmin.value = clientInfo.value.currentProfile === UserProfile.Admin;
  }
  await refreshUserList();
  const query = getCurrentRouteQuery();
  if (query.openInvite) {
    await inviteUser();
  }
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  if (eventCbId) {
    await eventDistributor.removeCallback(eventCbId);
  }
  routeWatchCancel();
});
</script>

<style scoped lang="scss">
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
