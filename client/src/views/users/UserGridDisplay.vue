<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <user-card
    :disabled="true"
    :user="currentUser as UserModel"
    :show-checkbox="false"
    :show-options="false"
  />
  <user-card
    v-for="user in users.getUsers()"
    :key="user.id"
    :user="user"
    :show-checkbox="someSelected"
    @select="onSelectedChange"
    @menu-click="(event, user, onFinished) => $emit('menuClick', event, user, onFinished)"
    ref="userGridItemRefs"
  />
</template>

<script setup lang="ts">
import { UserCard, UserCollection, UserModel } from '@/components/users';
import { UserInfo } from '@/parsec';
import { computed, ref } from 'vue';

const props = defineProps<{
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

async function onSelectedChange(_entry: UserInfo, _checked: boolean): Promise<void> {
  selectedCount.value = props.users.selectedCount();
}
</script>

<style scoped lang="scss"></style>
