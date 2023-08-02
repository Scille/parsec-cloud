<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <!-- contextual menu -->
      <action-bar
        id="activate-users-action-bar"
      >
        <div v-if="selectedUsersCount === 0">
          <button-option
            :icon="personAdd"
            id="button-invite-user"
            :button-label="$t('UsersPage.inviteUser')"
            @click="inviteUser()"
            v-show="isAdmin()"
          />
        </div>
        <!-- revoke or view common workspace -->
        <div v-else-if="selectedUsersCount >= 1">
          <button-option
            :icon="personRemove"
            class="danger"
            id="button-revoke-user"
            :button-label="$t('UsersPage.userContextMenu.actionRevoke', selectedUsersCount)"
            @click="revokeSelectedUsers()"
            v-show="isAdmin()"
          />
          <button-option
            :icon="eye"
            v-show="selectedUsersCount === 1"
            id="button-common-workspaces"
            :button-label="$t('UsersPage.userContextMenu.actionSeeCommonWorkspaces')"
            @click="viewCommonWorkspace()"
          />
        </div>
        <div class="right-side">
          <list-grid-toggle
            v-model="displayView"
            @update:model-value="resetSelection()"
          />
        </div>
      </action-bar>
      <!-- users -->
      <div class="users-container">
        <div v-if="displayView === DisplayState.List">
          <ion-list>
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
              <ion-label class="user-list-header__label cell-title label-role">
                {{ $t('UsersPage.listDisplayTitles.role') }}
              </ion-label>
              <ion-label class="user-list-header__label cell-title label-joined-on">
                {{ $t('UsersPage.listDisplayTitles.joinedOn') }}
              </ion-label>
              <ion-label class="user-list-header__label cell-title label-space" />
            </ion-list-header>
            <user-list-item
              v-for="user in filteredUsers"
              :key="user.id"
              :user="user"
              :show-checkbox="selectedUsersCount > 0 || allUsersSelected"
              @menu-click="openUserContextMenu($event, user)"
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
            v-for="user in filteredUsers"
            :key="user.id"
          >
            <user-card
              ref="userGridItemRefs"
              :user="user"
              :show-checkbox="selectedUsersCount > 0 || allUsersSelected"
              @menu-click="openUserContextMenu($event, user)"
              @select="onUserSelect"
            />
          </ion-item>
        </div>
      </div>
      <div class="user-footer">
        <div class="user-footer__container">
          <ion-text
            class="text title-h5"
            v-if="selectedUsersCount === 0"
          >
            {{ $t('UsersPage.itemCount', { count: userList.length }, userList.length) }}
          </ion-text>
          <ion-text
            class="text title-h5"
            v-if="selectedUsersCount !== 0"
          >
            {{ $t('UsersPage.userSelectedCount', { count: selectedUsersCount }, selectedUsersCount) }}
          </ion-text>
          <div
            class="content"
            v-if="selectedUsersCount >= 1"
          >
            <button-option
              class="shortcuts-btn danger"
              :icon="personRemove"
              id="button-revoke-user"
              @click="revokeSelectedUsers()"
              v-show="isAdmin()"
            />
            <button-option
              v-show="selectedUsersCount === 1"
              class="shortcuts-btn"
              :icon="eye"
              id="button-common-workspaces"
              @click="viewCommonWorkspace()"
            />
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { ref, Ref, computed } from 'vue';
import {
  IonContent,
  IonItem,
  IonList,
  IonPage,
  IonLabel,
  IonListHeader,
  IonCheckbox,
  IonText,
  popoverController,
} from '@ionic/vue';
import {
  personRemove,
  personAdd,
  eye,
} from 'ionicons/icons';
import UserListItem from '@/components/Users/UserListItem.vue';
import UserCard from '@/components/Users/UserCard.vue';
import ButtonOption from '@/components/ButtonOption.vue';
import { isAdmin } from '@/common/permissions';
import ListGridToggle from '@/components/ListGridToggle.vue';
import { DisplayState } from '@/components/ListGridToggle.vue';
import UserContextMenu from '@/components/UserContextMenu.vue';
import { UserAction } from '@/components/UserContextMenu.vue';
import ActionBar from '@/components/ActionBar.vue';
import { MockUser, getMockUsers } from '@/common/mocks';
import { onMounted } from '@vue/runtime-core';

const displayView = ref(DisplayState.List);
const userList: Ref<MockUser[]> = ref([]);
const userListItemRefs: Ref<typeof UserListItem[]> = ref([]);
const userGridItemRefs: Ref<typeof UserCard[]> = ref([]);

const allUsersSelected = computed({
  get: (): boolean => selectedUsersCount.value === userList.value.length,
  set: (_val) => _val,
});

const indeterminateState = computed({
  get: (): boolean => selectedUsersCount.value > 0 && selectedUsersCount.value < userList.value.length,
  set: (_val) => _val,
});

const filteredUsers = computed(() => {
  const revokedUsers = userList.value.filter((user) => {
    return user.revoked === false;
  });
  return revokedUsers;
});

const selectedUsersCount = computed(() => {
  if (displayView.value === DisplayState.List) {
    return userListItemRefs.value.filter((item) => item.isSelected).length;
  } else {
    return userGridItemRefs.value.filter((item) => item.isSelected).length;
  }
});

function getSelectedUsers(): MockUser[] {
  const selectedUsers: MockUser[] = [];

  if (displayView.value === DisplayState.List) {
    for (const item of userListItemRefs.value) {
      selectedUsers.push(item.getUser());
    }
  } else {
    for (const item of userGridItemRefs.value) {
      selectedUsers.push(item.getUser());
    }
  }
  return selectedUsers;
}

function inviteUser(): void {
  console.log('Invite user clicked');
}

function viewCommonWorkspace(): void {
  console.log('View common workspace clicked');
}

function onUserSelect(_user: MockUser, _selected: boolean): void {
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

function revokeUser(user: MockUser): void {
  console.log(`Revoke user ${user.name}`);
}

function details(user: MockUser): void {
  console.log(`Show details on user ${user.name}`);
}

function revokeSelectedUsers(): void {
  for (const user of getSelectedUsers()) {
    revokeUser(user);
  }
}

async function openUserContextMenu(event: Event, user: MockUser): Promise<void> {
  const popover = await popoverController
    .create({
      component: UserContextMenu,
      cssClass: 'user-context-menu',
      event: event,
      translucent: true,
      showBackdrop: false,
      dismissOnSelect: true,
      reference: 'event',
    });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  const actions = new Map<UserAction, (user: MockUser) => void>([
    [UserAction.Revoke, revokeUser],
    [UserAction.Details, details],
  ]);

  if (!data) {
    return;
  }

  const fn = actions.get(data.action);
  if (fn) {
    fn(user);
  }
}

function resetSelection(): void {
  userListItemRefs.value = [];
  userGridItemRefs.value = [];
}

onMounted(async (): Promise<void> => {
  userList.value = await getMockUsers();
});
</script>

<style scoped lang="scss">
.users-container {
  margin: 2em;
}

.user-list-header {
  color: var(--parsec-color-light-secondary-grey);
  padding-inline-start:0;

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

  .label-role {
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

.right-side {
  margin-left: auto;
  display: flex;
}

.users-grid-item {
  --inner-padding-end: 0px;
  --inner-padding-start: 0px;
}
</style>
