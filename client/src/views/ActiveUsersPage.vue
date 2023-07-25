<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <!-- contextual menu -->
      <ion-item-divider class="users-toolbar ion-margin-bottom secondary">
        <button-option
          id="button-invite-user"
          :button-label="$t('UsersPage.inviteUser')"
          @click="inviteUser()"
          v-show="isAdmin()"
        />
        <div class="right-side">
          <list-grid-toggle
            v-model="displayView"
          />
        </div>
      </ion-item-divider>
      <!-- users -->
      <div class="users-container">
        <div v-if="displayView === DisplayState.List">
          <ion-list>
            <ion-list-header
              class="user-list-header"
              lines="full"
            >
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
              :key="user"
              :user="user"
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
            />
          </ion-item>
        </div>
      </div>
      <div class="users-footer title-h5">
        {{ $t('UsersPage.itemCount', { count: userList.length }, userList.length) }}
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
} from '@ionic/vue';
import UserListItem from '@/components/Users/UserListItem.vue';
import UserCard from '@/components/Users/UserCard.vue';
import ButtonOption from '@/components/ButtonOption.vue';
import { isAdmin } from '@/common/permissions';
import ListGridToggle from '@/components/ListGridToggle.vue';
import { DisplayState } from '@/components/ListGridToggle.vue';

const displayView = ref(DisplayState.List);
const userList = ['User1', 'User2', 'User3'];

const filteredUsers = computed(() => {
  return userList;
});

function inviteUser(): void {
  console.log('Invite user clicked');
}
</script>

<style scoped lang="scss">
.users-container {
  margin: 2em;
  // background-color: white;
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

  .label-name {
    width: 100%;
    max-width: 20vw;
    min-width: 11.25rem;
    white-space: nowrap;
    overflow: hidden;
  }

  .label-role {
    min-width: 11.25rem;
    max-width: 10vw;
    flex-grow: 2;
  }

  .label-joined-on {
    min-width: 14.5rem;
    flex-grow: 0;
  }

  .label-email {
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
</style>
