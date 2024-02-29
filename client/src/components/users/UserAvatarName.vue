<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="avatar-container">
    <ion-avatar class="avatar person-avatar body-sm">
      {{ userAvatar.substring(0, 2) }}
    </ion-avatar>
    <ion-text
      class="person-name body"
      v-if="userName"
    >
      {{ userName }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonAvatar, IonText } from '@ionic/vue';

defineProps<{
  userAvatar: string;
  userName?: string;
}>();
</script>

<style lang="scss" scoped>
.avatar-container {
  display: flex;
  flex-wrap: nowrap;
  align-items: center;
  gap: 0.5rem;

  --width: 2rem;
  --height: 2rem;

  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    text-transform: uppercase;
    height: var(--height);
    min-width: var(--width);
    max-width: var(--width);
    border: 2px solid var(--parsec-color-light-secondary-inversed-contrast);

    &:first-of-type {
      margin-left: 0;
    }
  }

  &.large {
    .avatar {
      height: 4rem;
      min-width: 4rem;
    }
    .person-avatar {
      font-size: 1.125rem;
    }

    .person-name {
      padding-top: 0;
      padding-left: 0;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }

  &:not(.main-cell) .person-name {
    color: var(--parsec-color-light-secondary-grey);
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.main-cell {
  color: var(--parsec-color-light-secondary-text);
}

.person-avatar {
  color: var(--parsec-color-light-primary-500);
  background-color: var(--parsec-color-light-primary-100);
}
</style>
