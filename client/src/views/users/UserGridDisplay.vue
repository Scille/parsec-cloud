<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <user-card
    :disabled="true"
    :user="currentUser as UserModel"
    :show-checkbox="false"
    :show-options="false"
    :is-current-user="true"
  />
  <user-card
    v-for="user in users.getUsers()"
    :key="user.id"
    :user="user"
    :show-checkbox="someSelected"
    @menu-click="(event, user, onFinished) => $emit('menuClick', event, user, onFinished)"
    ref="userGridItemRefs"
  />
  <ion-text v-show="users.getUsers().length === 0 && users.totalUsersCount() > 0">
    {{ $msTranslate('UsersPage.noMatch') }}
  </ion-text>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import { UserCard, UserCollection, UserModel } from '@/components/users';
import { UserInfo } from '@/parsec';
import { computed, ref } from 'vue';

defineProps<{
  users: UserCollection;
  currentUser: UserInfo;
}>();

defineEmits<{
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
}>();

const selectedCount = ref(0);

const someSelected = computed(() => {
  return selectedCount.value > 0;
});
</script>

<style scoped lang="scss"></style>
