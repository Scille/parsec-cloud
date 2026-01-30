<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <user-card
    v-for="user in users.getUsers()"
    :key="user.id"
    :user="user"
    :disabled="user.isCurrent"
    :show-checkbox="someSelected || selectionEnabled === true"
    @menu-click="onMenuClick"
    @select="onUserSelected"
  />
  <ion-text
    class="no-match-result body"
    v-show="users.getUsers().length === 0"
  >
    {{ $msTranslate('UsersPage.noMatch') }}
  </ion-text>
</template>

<script setup lang="ts">
import { UserCard, UserCollection, UserModel } from '@/components/users';
import { IonText } from '@ionic/vue';
import { computed, ref } from 'vue';

defineProps<{
  users: UserCollection;
  selectionEnabled?: boolean;
}>();

const emits = defineEmits<{
  (e: 'menuClick', event: Event, user: UserModel, onFinished: () => void): void;
  (e: 'checkboxClick'): void;
}>();

const selectedCount = ref(0);

const someSelected = computed(() => {
  return selectedCount.value > 0;
});

async function onMenuClick(event: Event, user: UserModel, onFinished: () => void): Promise<void> {
  emits('menuClick', event, user, onFinished);
}

function onUserSelected(user: UserModel, selected: boolean): void {
  user.isSelected = selected;
  emits('checkboxClick');
}
</script>

<style scoped lang="scss"></style>
