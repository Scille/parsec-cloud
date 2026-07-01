<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list
    class="list-container users-list-container"
    id="users-page-user-list"
    :class="{ 'user-list-mobile': isSmallDisplay }"
  >
    <ion-list-header
      class="user-list-header list-header"
      lines="full"
      v-if="isLargeDisplay"
    >
      <ion-label class="list-header-label header-label-selected">
        <ms-checkbox
          v-show="allowSelection"
          @change="users.selectAll($event)"
          :checked="allSelected"
          :indeterminate="someSelected && !allSelected"
        />
      </ion-label>
      <ion-label
        class="list-header-label cell-title header-label-name"
        @click="headerSortChange(SortProperty.Name)"
        :class="{ 'list-header-label--sorted': sortBy === SortProperty.Name }"
      >
        {{ $msTranslate('UsersPage.listDisplayTitles.name') }}
        <ion-icon
          v-show="sortBy === SortProperty.Name"
          :icon="sortAscending ? arrowUp : arrowDown"
          class="sort-icon"
        />
      </ion-label>
      <ion-label
        class="list-header-label cell-title header-label-profile"
        @click="headerSortChange(SortProperty.Profile)"
        :class="{ 'list-header-label--sorted': sortBy === SortProperty.Profile }"
      >
        {{ $msTranslate('UsersPage.listDisplayTitles.profile') }}
        <ion-icon
          v-show="sortBy === SortProperty.Profile"
          :icon="sortAscending ? arrowUp : arrowDown"
          class="sort-icon"
        />
      </ion-label>
      <ion-label
        class="list-header-label cell-title header-label-email"
        @click="headerSortChange(SortProperty.Email)"
        :class="{ 'list-header-label--sorted': sortBy === SortProperty.Email }"
      >
        {{ $msTranslate('UsersPage.listDisplayTitles.email') }}
        <ion-icon
          v-show="sortBy === SortProperty.Email"
          :icon="sortAscending ? arrowUp : arrowDown"
          class="sort-icon"
        />
      </ion-label>
      <ion-label
        class="list-header-label cell-title header-label-join-date"
        @click="headerSortChange(SortProperty.JoinedDate)"
        :class="{ 'list-header-label--sorted': sortBy === SortProperty.JoinedDate }"
      >
        {{ $msTranslate('UsersPage.listDisplayTitles.joinedOn') }}
        <ion-icon
          v-show="sortBy === SortProperty.JoinedDate"
          :icon="sortAscending ? arrowUp : arrowDown"
          class="sort-icon"
        />
      </ion-label>
      <ion-label class="list-header-label cell-title header-label-status">
        {{ $msTranslate('UsersPage.listDisplayTitles.status') }}
      </ion-label>
      <ion-label class="list-header-label list-item-end cell-title header-label-space" />
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
      :active-checkbox="!user.isCurrent && !user.isRevoked() && (someSelected || selectionEnabled === true)"
      :allow-selection="allowSelection"
      @menu-click="onMenuClick"
      @select="onUserSelected"
    />
  </ion-list>
</template>

<script setup lang="ts">
import { SortProperty, UserCollection, UserListItem, UserModel } from '@/components/users';
import { IonIcon, IonLabel, IonList, IonListHeader, IonText } from '@ionic/vue';
import { arrowDown, arrowUp } from 'ionicons/icons';
import { MsCheckbox, useWindowSize } from 'megashark-lib';
import { computed } from 'vue';

const { isLargeDisplay, isSmallDisplay } = useWindowSize();

const props = defineProps<{
  users: UserCollection;
  selectionEnabled?: boolean;
  sortBy: SortProperty;
  sortAscending: boolean;
  allowSelection: boolean;
}>();

const emits = defineEmits<{
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
  (e: 'checkboxClick'): void;
  (e: 'sortUpdate', property: SortProperty, ascending: boolean): void;
}>();

const allSelected = computed(() => {
  return props.users.selectedCount() > 0 && props.users.selectedCount() === props.users.selectableUsersCount();
});

const someSelected = computed(() => {
  return props.users.selectedCount() > 0;
});

async function headerSortChange(property: SortProperty): Promise<void> {
  if (property === props.sortBy) {
    emits('sortUpdate', property, !props.sortAscending);
  } else {
    emits('sortUpdate', property, props.sortAscending);
  }
}

async function onMenuClick(event: Event, user: UserModel, onFinished: () => void): Promise<void> {
  emits('menuClick', event, user, onFinished);
}

function onUserSelected(user: UserModel, selected: boolean): void {
  if (user.isRevoked()) {
    return;
  }
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
