<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="avatar-container"
    :class="{ clickable: clickable }"
  >
    <ion-avatar class="avatar person-avatar button-small">
      {{ userAvatar.substring(0, 2) }}
    </ion-avatar>

    <div class="person-name-container">
      <ion-text
        class="person-name button-medium"
        v-if="userName"
        :title="userName"
      >
        {{ userName }}
      </ion-text>

      <ion-text
        class="person-description button-medium"
        :title="userAvatar"
        v-if="userDescription"
      >
        {{ $msTranslate(userDescription) }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonAvatar, IonText } from '@ionic/vue';
import { Translatable } from 'megashark-lib';
defineProps<{
  userAvatar: string;
  userName?: string;
  userDescription?: Translatable;
  clickable?: boolean;
}>();
</script>

<style lang="scss" scoped>
.avatar-container {
  display: flex;
  flex-wrap: nowrap;
  align-items: center;
  gap: 0.75rem;
  overflow: hidden;

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
    border: 1px solid var(--parsec-color-light-secondary-medium);

    &:first-of-type {
      margin-left: 0;
    }
  }

  .person-name {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  &.medium {
    --width: 2.5rem;
    --height: 2.5rem;
  }

  &.large {
    .avatar {
      height: 3rem;
      min-width: 3rem;
    }
    .person-avatar {
      width: 100%;
    }

    .person-name {
      padding-top: 0;
      padding-left: 0;
    }
  }

  &:not(.main-cell) .person-name {
    color: var(--parsec-color-light-secondary-text);
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }

  &.clickable {
    cursor: pointer;
    &:hover {
      .person-name {
        color: var(--parsec-color-light-secondary-text);
        text-decoration: underline;
        text-decoration-color: var(--parsec-color-light-primary-500);
      }
    }
  }

  .person-name-container {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;

    .person-description {
      color: var(--parsec-color-light-secondary-grey);
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.main-cell {
  color: var(--parsec-color-light-secondary-text);
}

.person-avatar {
  color: var(--parsec-color-light-primary-500);
  background-color: var(--parsec-color-light-primary-50);
}
</style>
