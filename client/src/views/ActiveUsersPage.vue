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
          <div v-else-if="selectedUsersCount === 1">
            <button-option
              :icon="personRemove"
              id="button-revoke-user"
              :button-label="$t('UsersPage.userContextMenu.actionRevoke')"
              @click="revokeUser()"
              v-show="isAdmin()"
            />
            <button-option
              :icon="eye"
              id="button-common-workspaces"
              :button-label="$t('UsersPage.userContextMenu.actionSeeCommonWorkspaces')"
              @click="viewCommonWorkspace()"
            />
          </div>
          <!-- revoke -->
          <div v-else>
            <button-option
              :icon="personRemove"
              class="danger"
              id="button-revoke-users"
              :button-label="$t('UsersPage.userContextMenu.actionRevokeSelectedUser')"
              @click="revokeUser()"
              v-show="isAdmin()"
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
                  class="checkbox"
                  @ion-change="selectAllUsers($event.detail.checked)"
                  v-model="allUsersSelected"
                />
              </ion-label>
              <ion-label class="user-list-header__label label-name">
                {{ $t('UsersPage.listDisplayTitles.name') }}
              </ion-label>
              <ion-label class="user-list-header__label label-email">
                {{ $t('UsersPage.listDisplayTitles.email') }}
              </ion-label>
              <ion-label class="user-list-header__label label-role">
                {{ $t('UsersPage.listDisplayTitles.role') }}
              </ion-label>
              <ion-label class="user-list-header__label label-joined-on">
                {{ $t('UsersPage.listDisplayTitles.joinedOn') }}
              </ion-label>
              <ion-label class="user-list-header__label label-space" />
            </ion-list-header>
            <user-list-item
              v-for="user in filteredUsers"
              :key="user.id"
              :user="user"
              :show-checkbox="selectedUsersCount > 0 || allUsersSelected"
              @menu-click="openUserContextMenu($event, user)"
              @select="onUserSelect"
              ref="userItemRefs"
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
            :key="user"
          >
            <user-card
              :user="user"
              @menu-click="openUserContextMenu($event, user)"
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
            v-if="selectedUsersCount === 1"
          >
            <button-option
              class="shortcuts-btn danger"
              :icon="personRemove"
              id="button-revoke-user"
              @click="revokeUser()"
              v-show="isAdmin()"
            />
            <button-option
              class="shortcuts-btn"
              :icon="eye"
              id="button-common-workspaces"
              @click="viewCommonWorkspace()"
            />
          </div>
          <div
            class="content"
            v-if="selectedUsersCount >= 2"
          >
            <button-option
              class="shortcuts-btn danger"
              id="button-move-to"
              :icon="personRemove"
              @click="revokeUser()"
              v-if="selectedUsersCount >= 1"
            />
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import {
  IonContent,
  IonItem,
  IonList,
  IonPage,
  IonLabel,
  IonListHeader,
  IonItemDivider,
  IonCheckbox
} from '@ionic/vue';
import {
  personRemove,
  personAdd,
  eye
} from 'ionicons/icons';
import UserListItem from '@/components/Users/UserListItem.vue';
import UserCard from '@/components/Users/UserCard.vue';
import ButtonOption from '@/components/ButtonOption.vue';
import { isAdmin } from '@/common/permissions';
import ListGridToggle from '@/components/ListGridToggle.vue';
import DisplayState from '@/components/ListGridToggle.vue';
import { popoverController } from '@ionic/vue';
import UserContextMenu from '@/components/UserContextMenu.vue';
import { UserAction } from '@/components/UserContextMenu.vue';
import { MockUser, getMockUsers } from '@/common/mocks';
import { Ref } from 'vue';
import { onMounted } from '@vue/runtime-core';

const displayView = ref(DisplayState.List);
const userList: Ref<MockUser[]> = ref([]);
const userItemRefs: Ref<typeof UserListItem[]> = ref([]);
const allUsersSelected = ref(false);

const filteredUsers = computed(() => {
  const revokedUsers = userList.value.filter((user) => {
    return user.revoked === false;
  });
  return revokedUsers;
});

const selectedUsersCount = computed(() => {
  const count = userItemRefs.value.filter((item) => item.isSelected).length;
  return count;
});

function inviteUser(): void {
  console.log('Invite user clicked');
}

function revokeUser(): void {
  console.log('Revoke user clicked');
}

function viewCommonWorkspace(): void {
  console.log('View common workspace clicked');
}

function onUserSelect(_user: MockUser, _selected: boolean): void {

  if (selectedUsersCount.value === 0) {
    allUsersSelected.value = false;
    selectAllUsers(false);
  }
  // check global checkbox if all users are selected
  if (selectedUsersCount.value === userList.value.length) {
    allUsersSelected.value = true;
  } else {
    allUsersSelected.value = false;
  }
}

function selectAllUsers(checked: boolean): void {
  for (const item of userItemRefs.value || []) {
    item.isSelected = checked;
    if (checked) {
      item.showCheckbox = true;
    } else {
      item.showCheckbox = false;
    }
  }
}

function actionOnSelectedUser(action: (user: MockUser) => void): void {
  const selected = userItemRefs.value.find((item) => item.isSelected);

  if (!selected) {
    return;
  }
  action(selected.props.file);
}

function actionOnSelectedUsers(action: (user: MockUser) => void): void {
  const selected = userItemRefs.value.filter((item) => item.isSelected);

  for (const item of selected) {
    action(item.props.file);
  }
}

function revokUser(user: MockUser): void {
  console.log('Revok user clicked: ', user);
}

function details(user: MockUser): void {
  console.log('Details user clicked: ', user);
}

async function openUserContextMenu(event: Event, user: MockUser): Promise<void> {
  console.log('menu cliked');
  const popover = await popoverController
    .create({
      component: UserContextMenu,
      cssClass: 'user-context-menu',
      event: event,
      translucent: true,
      showBackdrop: false,
      dismissOnSelect: true,
      reference: 'event'
    });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  const actions = new Map<UserAction, (user: MockUser) => void>([
    [UserAction.Revok, revokUser],
    [UserAction.Details, details]
  ]);

  const fn = actions.get(data.action);
  if (fn) {
    fn(user);
  }
}

function resetSelection(): void {
  userItemRefs.value = [];
  allUsersSelected.value = false;
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
  font-weight: 600;
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

.users-footer {
  width: 100%;
  left: 0;
  position: fixed;
  bottom: 0;
  text-align: center;
  color: var(--parsec-color-light-secondary-text);
  margin-bottom: 2em;
}

.users-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}

ion-item::part(native) {
  --padding-start: 0px;
}

.right-side {
  margin-left: auto;
  display: flex;
}
</style>
