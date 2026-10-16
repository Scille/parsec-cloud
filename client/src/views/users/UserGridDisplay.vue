<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="grid-container users-grid-container">
    <user-card
      v-for="user in users.getUsers()"
      :key="user.id"
      :user="user"
      :disabled="user.isCurrent"
      :active-checkbox="someSelected || selectionEnabled === true"
      :allow-selection="allowSelection"
      @menu-click="onMenuClick"
      @select="onUserSelected"
    />
    <ion-text
      class="no-match-result"
      v-show="users.getUsers().length === 0"
    >
      {{ $msTranslate('UsersPage.noMatch') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { UserCard, UserCollection, UserModel } from '@/components/users';
import { IonText } from '@ionic/vue';
import { computed, ref } from 'vue';

defineProps<{
  users: UserCollection;
  selectionEnabled?: boolean;
  allowSelection: boolean;
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
  if (user.isRevoked()) {
    return;
  }
  user.isSelected = selected;
  emits('checkboxClick');
}
</script>

<style scoped lang="scss">
.users-grid-container {
  display: flex;
  flex-wrap: wrap;
  gap: ms.spacing('gap-4xl');
  overflow-y: auto;

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-4xl') ms.spacing('padding-3xl') ms.spacing('padding-none');
    gap: ms.spacing('gap-4xl');
  }

  @include ms.responsive-breakpoint('xs') {
    gap: ms.spacing('gap-3xl');
  }
}

.no-match-result {
  @include ms.font('body-md-regular');
}
</style>
