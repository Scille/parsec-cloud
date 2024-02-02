<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <!-- contextual menu -->
      <ms-action-bar id="revoked-users-ms-action-bar">
        <!-- view common workspace -->
        <div v-if="selectedUsersCount >= 1">
          <ms-action-bar-button
            :icon="informationCircle"
            v-show="selectedUsersCount === 1"
            id="button-common-workspaces"
            :button-label="$t('UsersPage.userContextMenu.actionDetails')"
            @click="openSelectedUserDetails"
          />
        </div>
        <div class="right-side">
          <div class="counter">
            <ion-text
              class="body"
              v-if="selectedUsersCount === 0"
            >
              {{ $t('UsersPage.itemCount', { count: userList.length }, userList.length) }}
            </ion-text>
            <ion-text
              class="body item-selected"
              v-if="selectedUsersCount !== 0"
            >
              {{ $t('UsersPage.userSelectedCount', { count: selectedUsersCount }, selectedUsersCount) }}
            </ion-text>
          </div>
          <ms-grid-list-toggle
            v-model="displayView"
            @update:model-value="resetSelection()"
          />
        </div>
      </ms-action-bar>
      <!-- users -->
      <div class="revoked-users-container scroll">
        <div v-if="userList.length === 0">
          {{ $t('UsersPage.revokedEmptyList') }}
        </div>
        <div v-else>
          <div v-if="displayView === DisplayState.List">
            <ion-list class="list">
              <ion-list-header
                class="user-list-header"
                lines="full"
              >
                <ion-label class="user-list-header__label label-selected">
                  <ion-checkbox
                    aria-label=""
                    class="checkbox"
                    @ion-change="selectAllUsers($event.detail.checked)"
                    v-model="allUsersSelected"
                    :indeterminate="indeterminateState"
                  />
                </ion-label>
                <ion-label class="user-list-header__label cell-title label-name">
                  {{ $t('UsersPage.listDisplayTitles.name') }}
                </ion-label>
                <ion-label class="user-list-header__label cell-title label-email">
                  {{ $t('UsersPage.listDisplayTitles.email') }}
                </ion-label>
                <ion-label class="user-list-header__label cell-title label-profile">
                  {{ $t('UsersPage.listDisplayTitles.profile') }}
                </ion-label>
                <ion-label class="user-list-header__label cell-title label-joined-on">
                  {{ $t('UsersPage.listDisplayTitles.joinedOn') }}
                </ion-label>
                <ion-label class="user-list-header__label cell-title label-space" />
              </ion-list-header>
              <user-list-item
                v-for="user in userList"
                :key="user.id"
                :user="user"
                :show-checkbox="selectedUsersCount > 0 || allUsersSelected"
                @menu-click="openUserContextMenu"
                @select="onUserSelect"
                ref="userListItemRefs"
              />
            </ion-list>
          </div>
          <div
            v-else
            class="users-container-grid"
          >
            <ion-item
              class="users-grid-item"
              v-for="user in userList"
              :key="user.id"
            >
              <user-card
                ref="userGridItemRefs"
                :user="user"
                :show-checkbox="selectedUsersCount > 0 || allUsersSelected"
                @menu-click="openUserContextMenu"
                @select="onUserSelect"
                :show-options="selectedUsersCount === 0"
              />
            </ion-item>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { DisplayState, MsActionBar, MsActionBarButton, MsGridListToggle } from '@/components/core';
import UserCard from '@/components/users/UserCard.vue';
import UserListItem from '@/components/users/UserListItem.vue';
import { UserInfo, listRevokedUsers as parsecListRevokedUsers } from '@/parsec';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import UserContextMenu, { UserAction } from '@/views/users/UserContextMenu.vue';
import UserDetailsModal from '@/views/users/UserDetailsModal.vue';
import {
  IonCheckbox,
  IonContent,
  IonItem,
  IonLabel,
  IonList,
  IonListHeader,
  IonPage,
  IonText,
  modalController,
  popoverController,
} from '@ionic/vue';
import { informationCircle } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, ref } from 'vue';

const displayView = ref(DisplayState.List);
const userList: Ref<UserInfo[]> = ref([]);
const userListItemRefs: Ref<(typeof UserListItem)[]> = ref([]);
const userGridItemRefs: Ref<(typeof UserCard)[]> = ref([]);
const informationManager: InformationManager = inject(InformationKey)!;

const allUsersSelected = computed({
  get: (): boolean => selectedUsersCount.value === userList.value.length,
  set: (_val) => _val,
});

const indeterminateState = computed({
  get: (): boolean => selectedUsersCount.value > 0 && selectedUsersCount.value < userList.value.length,
  set: (_val) => _val,
});

const selectedUsersCount = computed(() => {
  if (displayView.value === DisplayState.List) {
    return userListItemRefs.value.filter((item) => item.isSelected).length;
  } else {
    return userGridItemRefs.value.filter((item) => item.isSelected).length;
  }
});

function getSelectedUsers(): UserInfo[] {
  const selectedUsers: UserInfo[] = [];

  if (displayView.value === DisplayState.List) {
    for (const item of userListItemRefs.value) {
      if (item.isSelected) {
        selectedUsers.push(item.getUser());
      }
    }
  } else {
    for (const item of userGridItemRefs.value) {
      if (item.isSelected) {
        selectedUsers.push(item.getUser());
      }
    }
  }
  return selectedUsers;
}

function onUserSelect(_user: UserInfo, _selected: boolean): void {
  if (selectedUsersCount.value === 0) {
    selectAllUsers(false);
  }
}

function selectAllUsers(checked: boolean): void {
  if (displayView.value === DisplayState.List) {
    for (const item of userListItemRefs.value || []) {
      item.isSelected = checked;
      if (checked) {
        item.showCheckbox = true;
      } else {
        item.showCheckbox = false;
      }
    }
  } else {
    for (const item of userGridItemRefs.value || []) {
      item.isSelected = checked;
      if (checked) {
        item.showCheckbox = true;
      } else {
        item.showCheckbox = false;
      }
    }
  }
}

async function openUserDetails(user: UserInfo): Promise<void> {
  const modal = await modalController.create({
    component: UserDetailsModal,
    cssClass: 'user-details-modal',
    componentProps: {
      user: user,
    },
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
}

async function openSelectedUserDetails(): Promise<void> {
  const selectedUsers = getSelectedUsers();

  if (selectedUsers.length === 1) {
    return await openUserDetails(selectedUsers[0]);
  }
}

async function openUserContextMenu(event: Event, user: UserInfo, onFinished?: () => void): Promise<void> {
  const popover = await popoverController.create({
    component: UserContextMenu,
    cssClass: 'user-context-menu',
    event: event,
    translucent: true,
    showBackdrop: false,
    dismissOnSelect: true,
    alignment: 'end',
    componentProps: {
      isRevoked: user.isRevoked(),
    },
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  const actions = new Map<UserAction, (user: UserInfo) => Promise<void>>([[UserAction.Details, openUserDetails]]);

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

function resetSelection(): void {
  userListItemRefs.value = [];
  userGridItemRefs.value = [];
}

async function refreshUserList(): Promise<void> {
  const result = await parsecListRevokedUsers();
  if (result.ok) {
    userList.value = result.value;
  } else {
    informationManager.present(
      new Information({
        message: translate('UsersPage.listRevokedUsersFailed.message'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

onMounted(async (): Promise<void> => {
  await refreshUserList();
});
</script>

<style scoped lang="scss">
.user-list-header {
  &__label {
    padding: 0 1rem;
    height: 100%;
    display: flex;
    align-items: center;
  }

  .label-selected {
    min-width: 4rem;
    flex-grow: 0;
    display: flex;
    align-items: center;
    justify-content: end;
  }

  .label-name {
    width: 100%;
    max-width: 20vw;
    min-width: 11.25rem;
    white-space: nowrap;
    overflow: hidden;
  }

  .label-email {
    min-width: 17.5rem;
    flex-grow: 0;
  }

  .label-profile {
    min-width: 11.5rem;
    max-width: 10vw;
    flex-grow: 2;
  }

  .label-joined-on {
    min-width: 11.25rem;
    flex-grow: 0;
  }

  .label-space {
    min-width: 4rem;
    flex-grow: 0;
    margin-left: auto;
    margin-right: 1rem;
  }
}

.users-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}

.users-toolbar {
  padding: 1em 2em;
  height: 6em;
  background-color: var(--parsec-color-light-secondary-background);
  border-top: 1px solid var(--parsec-color-light-secondary-light);
}

.users-grid-item {
  --inner-padding-end: 0px;
  --inner-padding-start: 0px;
}
</style>
