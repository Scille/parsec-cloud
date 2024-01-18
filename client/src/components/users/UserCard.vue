<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="user-card-item ion-no-padding"
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
      <div class="user-profile">
        <tag-profile :profile="user.currentProfile" />
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import TagProfile from '@/components/users/TagProfile.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { UserInfo } from '@/parsec';
import { IonAvatar, IonCheckbox, IonIcon, IonItem, IonText } from '@ionic/vue';
import { ellipsisHorizontal } from 'ionicons/icons';
import { ref } from 'vue';

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
.user-card-item {
  --background: none;
  --background-hover: none;
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  width: 10.5rem;
  border-radius: var(--parsec-radius-12);
  position: relative;
  height: fit-content;

  &::part(native) {
    --inner-padding-end: 0px;
  }

  &:hover:not(.item-disabled) {
    background: var(--parsec-color-light-primary-30);
  }

  &.selected {
    --background: var(--parsec-color-light-primary-100);
    border: 1px solid var(--parsec-color-light-primary-100);
  }
}

.card-option,
.card-checkbox {
  position: absolute;
}

.card-checkbox {
  left: 0.5rem;
  top: 0.5rem;
}

.card-option {
  color: var(--parsec-color-light-secondary-grey);
  text-align: right;
  display: flex;
  align-items: center;
  top: 0;
  right: 0;
  font-size: 1.5rem;
  padding: 0.5rem;
  cursor: pointer;

  &:hover {
    color: var(--parsec-color-light-primary-500);
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 1.5rem 0.5rem;
  width: 100%;
  margin: auto;
  color: var(--parsec-color-light-secondary-text);
}
</style>
