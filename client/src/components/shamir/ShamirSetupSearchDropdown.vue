<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="shamir-setup-search-dropdown">
    <ion-item
      v-for="user in users"
      :key="user.id"
      class="option"
      :class="{ 'option--selected': isSelected(user) }"
      :button="!isSelected(user)"
      lines="none"
      @click="onUserClick(user)"
    >
      <ion-icon
        v-if="isSelected(user)"
        :icon="shieldCheckmark"
        class="option__icon"
      />
      <ion-text class="option__text">
        <span class="option__text-label subtitles-sm">
          {{ user.humanHandle.label }}
        </span>
        <span class="option__text-description body">
          {{ user.humanHandle.email }}
        </span>
      </ion-text>
      <ion-icon
        :icon="close"
        class="option__icon"
      />
    </ion-item>

    <ion-item
      v-if="users.length === 0"
      class="option option--empty"
      lines="none"
    >
      <ion-text class="option__text option__text--empty subtitles-sm">
        {{ $msTranslate('UserSelect.noMatch') }}
      </ion-text>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { UserInfo } from '@/parsec';
import { IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { close, shieldCheckmark } from 'ionicons/icons';

const emits = defineEmits<{
  (e: 'userSelected', user: UserInfo): void;
  (e: 'deselectUser', user: UserInfo): void;
}>();

const props = defineProps<{
  users: Array<UserInfo>;
  selectedUsers: Array<UserInfo>;
}>();

function isSelected(user: UserInfo): boolean {
  return props.selectedUsers.some((selectedUser) => selectedUser.id === user.id);
}

function onUserClick(user: UserInfo): void {
  if (isSelected(user)) {
    emits('deselectUser', user);
  } else {
    emits('userSelected', user);
  }
}
</script>

<style scoped lang="scss">
.shamir-setup-search-dropdown {
  display: flex;
  flex-direction: column;
  position: absolute;
  width: 100%;
  border: 1px solid var(--parsec-color-light-secondary-medium);
  background: var(--parsec-color-light-secondary-white);
  border-radius: var(--parsec-radius-12);
  margin-top: 0.5rem;
  max-height: 18rem;
  overflow: auto;
  padding: 0;
  scrollbar-width: auto;
}

.option {
  --background-hover: none;
  flex-shrink: 0;
  --color: var(--parsec-color-light-secondary-hard-grey);
  --background: none;
  --min-height: 0;
  --inner-padding-end: 0;
  --ripple-color: transparent;
  position: relative;
  z-index: 2;
  pointer-events: auto;
  overflow: hidden;
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  cursor: pointer;
  transition: all 0.2s ease;

  &:last-child {
    border-bottom: none;
  }

  &::part(native) {
    padding: 0.625rem 1rem;
  }

  &::part(container) {
    gap: 0.5rem;
  }

  &__text {
    margin: 0;
    display: flex;
    align-items: center;
    gap: 0.375rem;
    width: 100%;
    overflow: hidden;

    &-label {
      color: var(--parsec-color-light-secondary-text);
    }

    &-description {
      color: var(--parsec-color-light-secondary-grey);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &--empty {
      color: var(--parsec-color-light-secondary-hard-grey);
    }
  }

  &__icon {
    flex-shrink: 0;
    font-size: 1rem;
    color: var(--parsec-color-light-secondary-grey);

    &:first-child {
      color: var(--parsec-color-light-primary-500);
    }

    &:last-child {
      opacity: 0;
      padding: 0.125rem;
      border-radius: var(--parsec-radius-18);
      color: var(--parsec-color-light-secondary-grey);
      cursor: pointer;
      transform: rotate(45deg);
    }
  }

  &:hover:not(.option--selected) {
    background: var(--parsec-color-light-primary-30);

    .option__icon {
      opacity: 1;
      transition: transform 0.2s ease;
    }
  }

  &--selected {
    background: var(--parsec-color-light-primary-50);

    .option__text-label {
      color: var(--parsec-color-light-primary-600);
    }

    .option__text-description {
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    .option__icon:last-child {
      transition: transform 0.2s ease;
      transform: rotate(0deg);
      opacity: 1;
    }

    &:hover {
      background: var(--parsec-color-light-secondary-premiere);

      .option__text-label {
        color: var(--parsec-color-light-secondary-text);
      }

      .option__icon:last-child {
        color: var(--parsec-color-light-danger-500);
        background: var(--parsec-color-light-danger-100);
      }
    }
  }

  &--empty {
    cursor: default;
  }
}
</style>
