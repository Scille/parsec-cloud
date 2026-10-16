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
        <span class="option__text-label">
          {{ user.humanHandle.label }}
        </span>
        <span class="option__text-description">
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
      <ion-text class="option__text option__text--empty">
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
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  background: ms.color('surface-base-default');
  border-radius: ms.radius('2xl');
  margin-top: 0.5rem;
  max-height: 18rem;
  overflow: auto;
  padding: ms.spacing('padding-none');
  scrollbar-width: auto;
}

.option {
  --background-hover: none;
  flex-shrink: 0;
  --color: #{ms.color('text-base-description')};
  --background: none;
  --min-height: 0;
  --inner-padding-end: #{ms.spacing('padding-none')};
  --ripple-color: transparent;
  position: relative;
  z-index: 2;
  pointer-events: auto;
  overflow: hidden;
  border-bottom: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  cursor: pointer;
  transition: all 0.2s ease;

  &:last-child {
    border-bottom: none;
  }

  &::part(native) {
    padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
  }

  &::part(container) {
    gap: ms.spacing('gap-lg');
  }

  &__text {
    margin: 0;
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-md');
    width: 100%;
    overflow: hidden;

    &-label {
      @include ms.font('body-md-medium');
      color: ms.color('text-base-body');
    }

    &-description {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &--empty {
      @include ms.font('body-md-medium');
      color: ms.color('text-base-description');
    }
  }

  &__icon {
    flex-shrink: 0;
    font-size: 1rem;
    color: ms.color('icon-neutral-default');

    &:first-child {
      color: ms.color('icon-brand-default');
    }

    &:last-child {
      opacity: 0;
      padding: ms.spacing('padding-xs');
      border-radius: ms.radius('4xl');
      color: ms.color('icon-neutral-default');
      cursor: pointer;
      transform: rotate(45deg);
    }
  }

  &:hover:not(.option--selected) {
    background: ms.color('surface-brand-default-subtle');

    .option__icon {
      opacity: ms.opacity('10');
      transition: transform 0.2s ease;
    }
  }

  &--selected {
    background: ms.color('surface-brand-default-subtle-hover');

    .option__text-label {
      color: ms.color('text-brand-default-hover');
    }

    .option__text-description {
      color: ms.color('text-base-description');
    }

    .option__icon:last-child {
      transition: transform 0.2s ease;
      transform: rotate(0deg);
      opacity: ms.opacity('10');
    }

    &:hover {
      background: ms.color('surface-base-page-secondary');

      .option__text-label {
        color: ms.color('text-base-body');
      }

      .option__icon:last-child {
        color: ms.color('icon-error-default');
        background: ms.color('surface-error-default-subtle-pressed');
      }
    }
  }

  &--empty {
    cursor: default;
  }
}
</style>
