<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-list class="list-container">
    <ion-item
      class="option"
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
          <span class="option-text__label">
            {{ user.humanHandle.label }}
          </span>
          <span class="option-text__description"> ({{ user.humanHandle.email }}) </span>
        </ion-text>
        <ion-text
          v-if="user.id === currentUser.id"
          class="option-warning"
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
  padding: ms.spacing('padding-lg');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
  max-height: 15rem;
  overflow: auto;

  @include ms.responsive-breakpoint('sm') {
    max-height: 15rem;
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.option {
  --background-hover: none;
  @include ms.font('body-md-regular');
  --color: #{ms.color('text-base-description')};
  padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
  --background: none;
  border-radius: ms.radius('md');
  --min-height: 0;
  --inner-padding-end: #{ms.spacing('padding-none')};
  position: relative;
  z-index: 2;
  pointer-events: auto;
  flex-shrink: 0;
  cursor: pointer;

  &:hover:not(.item-disabled) {
    background: ms.color('surface-brand-default-subtle-hover');
    --background-hover: #{ms.color('surface-brand-default-subtle-hover')};
  }

  &-content {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    width: 100%;
    gap: ms.spacing('gap-sm');
  }

  &::part(native) {
    padding: ms.spacing('padding-none');
  }

  &-text {
    margin: 0;
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');

    &__label {
      @include ms.font('label-md-medium');
      color: ms.color('text-base-body');
    }

    &__description {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');
    }
  }

  &.item-disabled {
    opacity: 1;
    background: ms.color('surface-base-page-secondary');
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
    @include ms.font('body-md-medium');
    border-radius: ms.radius('md');
    color: ms.color('text-base-label');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-sm');

    ion-icon {
      font-size: 0.875rem;
    }
  }
}
</style>
