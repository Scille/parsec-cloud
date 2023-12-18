<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="card"
    :detail="false"
    :class="{ selected: isSelected, 'no-padding-end': !isSelected }"
    @click="$emit('click', $event, user)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="card-checkbox">
      <ion-checkbox
        aria-label=""
        v-model="isSelected"
        v-if="isSelected || isHovered || showCheckbox"
        class="checkbox"
        @click.stop
        @ion-change="$emit('select', user, isSelected)"
      />
    </div>
    <div
      class="card-option"
      v-show="isHovered || menuOpened"
      @click.stop="onOptionsClick($event)"
    >
      <ion-icon :icon="ellipsisHorizontal" />
    </div>
    <div class="card-content">
      <ion-avatar class="card-content-avatar">
        <user-avatar-name
          class="user-avatar large"
          :user-avatar="user.humanHandle.label"
        />
      </ion-avatar>
      <ion-text class="user-name body">
        {{ user.humanHandle.label }}
      </ion-text>
      <ion-title class="user-profile body-lg">
        <tag-profile :profile="user.currentProfile" />
      </ion-title>
    </div>
  </div>
</template>

<script setup lang="ts">
import TagProfile from '@/components/users/TagProfile.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { UserInfo } from '@/parsec';
import { IonAvatar, IonCheckbox, IonIcon, IonText, IonTitle } from '@ionic/vue';
import { ellipsisHorizontal } from 'ionicons/icons';
import { defineEmits, defineExpose, defineProps, ref } from 'vue';

const isHovered = ref(false);
const isSelected = ref(false);
const menuOpened = ref(false);

const emits = defineEmits<{
  (e: 'click', event: Event, user: UserInfo): void;
  (e: 'menuClick', event: Event, user: UserInfo, onFinished: () => void): void;
  (e: 'select', user: UserInfo, selected: boolean): void;
}>();

const props = defineProps<{
  user: UserInfo;
  showCheckbox: boolean;
}>();

function getUser(): UserInfo {
  return props.user;
}

defineExpose({
  isHovered,
  isSelected,
  getUser,
});

async function onOptionsClick(event: Event): Promise<void> {
  menuOpened.value = true;
  emits('menuClick', event, props.user, () => {
    menuOpened.value = false;
  });
}
</script>

<style scoped lang="scss">
.card {
  border: 1px solid var(--parsec-color-light-secondary-light);
  padding: 1rem 0.5rem 1.5rem;
  min-width: 11.25rem;
  border-radius: var(--parsec-radius-6);
  overflow: hidden;
  position: relative;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  transition: box-shadow 150ms ease-in-out;
  overflow: visible;
  padding: 1rem;

  &:hover {
    background: var(--parsec-color-light-primary-30);
    box-shadow: var(--parsec-shadow-light);
  }

  &.item-checkbox-checked {
    background: var(--parsec-color-light-primary-100);
  }
}

.card-option,
.card-checkbox {
  position: absolute;
}

.card-checkbox {
  left: 1rem;
}

.card-option {
  right: 1rem;
  font-size: 1.5rem;
  color: var(--parsec-color-light-secondary-grey);
}

.card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  color: var(--parsec-color-light-secondary-text);

  .card-content-avatar {
    margin: 1rem;
  }
}
</style>
