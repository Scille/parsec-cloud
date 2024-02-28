<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list">
    <ion-list-header
      class="user-list-header"
      lines="full"
    >
      <ion-label class="user-list-header__label label-selected">
        <ion-checkbox
          aria-label=""
          class="checkbox"
          @ion-change="selectAll($event.detail.checked)"
          :checked="allSelected"
          :indeterminate="someSelected && !allSelected"
        />
      </ion-label>
      <ion-label class="user-list-header__label cell-title label-name">
        {{ $t('UsersPage.listDisplayTitles.name') }}
      </ion-label>
      <ion-label class="user-list-header__label cell-title label-status">
        {{ $t('UsersPage.listDisplayTitles.status') }}
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
      :user="currentUser as UserModel"
      :show-checkbox="false"
      :disabled="true"
      :hide-options="true"
    />
    <user-list-item
      v-for="user in users.getUsers()"
      :key="user.id"
      :user="user"
      :show-checkbox="someSelected"
      @select="onSelectedChange"
      @menu-click="(event, user, onFinished) => $emit('menuClick', event, user, onFinished)"
    />
  </ion-list>
</template>

<script setup lang="ts">
import { UserCollection, UserListItem, UserModel } from '@/components/users';
import { UserInfo } from '@/parsec';
import { IonCheckbox, IonLabel, IonList, IonListHeader } from '@ionic/vue';
import { computed, ref } from 'vue';

const props = defineProps<{
  users: UserCollection;
  currentUser: UserInfo;
}>();

defineEmits<{
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
}>();

const selectedCount = ref(0);

const allSelected = computed(() => {
  return selectedCount.value === props.users.usersCount();
});

const someSelected = computed(() => {
  return selectedCount.value > 0;
});

async function onSelectedChange(_entry: UserInfo, _checked: boolean): Promise<void> {
  selectedCount.value = props.users.selectedCount();
}

async function selectAll(selected: boolean): Promise<void> {
  props.users.selectAll(selected);
  if (selected) {
    selectedCount.value = props.users.usersCount();
  } else {
    selectedCount.value = 0;
  }
}
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

  .label-status {
    min-width: 3rem;
    flex-grow: 0;
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
</style>
