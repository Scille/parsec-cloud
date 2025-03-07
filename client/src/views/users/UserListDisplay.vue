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
      :show-checkbox="someSelected"
      @menu-click="(event: Event, user: UserModel, onFinished: () => void) => $emit('menuClick', event, user, onFinished)"
    />
  </ion-list>
</template>

<script setup lang="ts">
import { UserCollection, UserListItem, UserModel } from '@/components/users';
import { IonLabel, IonList, IonListHeader, IonText } from '@ionic/vue';
import { computed } from 'vue';
import { MsCheckbox, useWindowSize } from 'megashark-lib';

const { isLargeDisplay, isSmallDisplay } = useWindowSize();

const props = defineProps<{
  users: UserCollection;
}>();

defineEmits<{
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
}>();

const allSelected = computed(() => {
  return props.users.selectedCount() > 0 && props.users.selectedCount() === props.users.selectableUsersCount();
});

const someSelected = computed(() => {
  return props.users.selectedCount() > 0;
});
</script>

<style scoped lang="scss">
.user-list-mobile {
  padding-top: 1rem;
}
</style>
