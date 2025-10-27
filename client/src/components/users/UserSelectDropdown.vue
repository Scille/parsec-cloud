<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list-container">
    <ion-item
      class="option body"
      :class="{ 'item-disabled': user.id === currentUser.id }"
      button
      lines="none"
      v-for="user in users"
      :key="user.id"
      @click="onUserClick(user)"
      :disabled="user.id === currentUser.id"
    >
      <div class="option-content">
        <ion-text class="option-text">
          <span class="option-text__label body">
            {{ user.humanHandle.label }}
          </span>
          <span class="option-text__description body"> ({{ user.humanHandle.email }}) </span>
        </ion-text>
        <ion-text
          v-if="user.id === currentUser.id"
          class="subtitles-sm option-warning"
        >
          <ion-icon :icon="warning" />
          {{ $msTranslate('UsersPage.userContextMenu.cannotAssignRolesToSelf') }}
        </ion-text>
      </div>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { UserInfo } from '@/parsec';
import { IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { warning } from 'ionicons/icons';

const props = defineProps<{
  users: UserInfo[];
  currentUser: UserInfo;
}>();

const emits = defineEmits<{
  (e: 'select', user: UserInfo): void;
}>();

function onUserClick(user: UserInfo): void {
  if (user.id === props.currentUser.id) {
    return;
  }
  emits('select', user);
}
</script>

<style lang="scss" scoped>
.list-container {
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  max-height: 11rem;
  overflow: auto;

  @include ms.responsive-breakpoint('sm') {
    max-height: 15rem;
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.option {
  --background-hover: none;
  --color: var(--parsec-color-light-secondary-grey);
  padding: 0.375rem 0.75rem;
  --background: none;
  border-radius: var(--parsec-radius-6);
  --min-height: 0;
  --inner-padding-end: 0;
  position: relative;
  z-index: 2;
  pointer-events: auto;
  flex-shrink: 0;
  cursor: pointer;

  &:hover:not(.item-disabled) {
    background: var(--parsec-color-light-primary-50);
    --background-hover: var(--parsec-color-light-primary-50);
  }

  &-content {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    width: 100%;
    gap: 0.25rem;
  }

  &::part(native) {
    padding: 0;
  }

  &-text {
    margin: 0;
    display: flex;
    gap: 0.5rem;

    &__label {
      color: var(--parsec-color-light-secondary-text);
    }

    &__description {
      color: var(--parsec-color-light-secondary-hard-grey);
    }
  }

  &.item-disabled {
    opacity: 1;
    background: var(--parsec-color-light-secondary-premiere);
    cursor: not-allowed;

    &::part(native) {
      cursor: not-allowed;
    }

    .option-text__label,
    .option-text__description {
      opacity: 0.75;
    }
  }

  &-warning {
    border-radius: var(--parsec-radius-6);
    color: var(--parsec-color-light-secondary-soft-text);
    display: flex;
    align-items: center;
    gap: 0.25rem;

    ion-icon {
      font-size: 0.875rem;
    }
  }
}
</style>
