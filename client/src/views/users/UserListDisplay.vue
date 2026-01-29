<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list
    class="list users-container-list"
    id="users-page-user-list"
    :class="{ 'user-list-mobile': isSmallDisplay }"
  >
    <ion-list-header
      class="user-list-header"
      lines="full"
      v-if="isLargeDisplay"
    >
      <ion-label class="user-list-header__label label-selected">
        <ms-checkbox
          @change="users.selectAll($event)"
          :checked="allSelected"
          :indeterminate="someSelected && !allSelected"
        />
      </ion-label>
      <ion-label class="user-list-header__label cell-title label-name">
        {{ $msTranslate('UsersPage.listDisplayTitles.name') }}
      </ion-label>
      <ion-label class="user-list-header__label cell-title label-profile">
        {{ $msTranslate('UsersPage.listDisplayTitles.profile') }}
      </ion-label>
      <ion-label class="user-list-header__label cell-title label-email">
        {{ $msTranslate('UsersPage.listDisplayTitles.email') }}
      </ion-label>
      <ion-label class="user-list-header__label cell-title label-joined-on">
        {{ $msTranslate('UsersPage.listDisplayTitles.joinedOn') }}
      </ion-label>
      <ion-label class="user-list-header__label cell-title label-status">
        {{ $msTranslate('UsersPage.listDisplayTitles.status') }}
      </ion-label>
      <ion-label class="user-list-header__label cell-title label-space" />
    </ion-list-header>
    <ion-text
      class="no-match-result body"
      v-show="users.getUsers().length === 0"
    >
      {{ $msTranslate('UsersPage.noMatch') }}
    </ion-text>
    <user-list-item
      v-for="user in users.getUsers()"
      :key="user.id"
      :user="user"
      :disabled="user.isCurrent"
      :class="{
        'current-user': user.isCurrent,
      }"
      :show-checkbox="!user.isCurrent && !user.isRevoked() && (someSelected || selectionEnabled === true)"
      @menu-click="onMenuClick"
      @select="onUserSelected"
    />
  </ion-list>
</template>

<script setup lang="ts">
import { UserCollection, UserListItem, UserModel } from '@/components/users';
import { IonLabel, IonList, IonListHeader, IonText } from '@ionic/vue';
import { MsCheckbox, useWindowSize } from 'megashark-lib';
import { computed } from 'vue';

const { isLargeDisplay, isSmallDisplay } = useWindowSize();

const props = defineProps<{
  users: UserCollection;
  selectionEnabled?: boolean;
}>();

const emits = defineEmits<{
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
  (e: 'checkboxClick'): void;
}>();

const allSelected = computed(() => {
  return props.users.selectedCount() > 0 && props.users.selectedCount() === props.users.selectableUsersCount();
});

const someSelected = computed(() => {
  return props.users.selectedCount() > 0;
});

async function onMenuClick(event: Event, user: UserModel, onFinished: () => void): Promise<void> {
  emits('menuClick', event, user, onFinished);
}

function onUserSelected(user: UserModel, selected: boolean): void {
  user.isSelected = selected;
  emits('checkboxClick');
}
</script>

<style scoped lang="scss">
.user-list-mobile {
  padding-top: 1rem;
  padding: 1rem 1rem 0;
}
</style>
