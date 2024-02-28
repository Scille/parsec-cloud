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
import { Groups, HotkeyManager, HotkeyManagerKey, Hotkeys, Modifiers, Platforms } from '@/services/hotkeyManager';
import { computed, inject, onMounted, onUnmounted, ref } from 'vue';

const props = defineProps<{
  users: UserCollection;
  currentUser: UserInfo;
}>();

defineEmits<{
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
}>();

const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
let hotkeys: Hotkeys | null = null;
const selectedCount = ref(0);

const someSelected = computed(() => {
  return selectedCount.value > 0;
});

onMounted(async () => {
  hotkeys = hotkeyManager.newHotkeys(Groups.Users);
  hotkeys.add('a', Modifiers.Ctrl, Platforms.Desktop, async () => props.users.selectAll(true));
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
});

async function onSelectedChange(_entry: UserInfo, _checked: boolean): Promise<void> {
  selectedCount.value = props.users.selectedCount();
}
</script>

<style scoped lang="scss"></style>
