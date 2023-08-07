<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="container">
    <ion-avatar
      class="avatar person-avatar body-sm"
    >
      {{ userAvatar.substring(0, 2) }}
    </ion-avatar>
    <ion-text
      class="person-name"
      v-show="userName"
    >
      {{ userName }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonAvatar, IonText } from '@ionic/vue';
import { defineProps } from 'vue';

defineProps<{
  userAvatar: string,
  userName?: string
}>();
</script>

<style lang="scss" scoped>
.container {
  display: flex;
  flex-wrap: nowrap;

  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    text-transform: uppercase;
    height: 2rem;
    width: 2rem;
    border: 2px solid var(--parsec-color-light-secondary-inversed-contrast);

    &:first-of-type{
      margin-left: 0;
    }
  }

  &.large {
    .avatar {
      height: 4rem;
      width: 4rem;
    }
    .person-avatar {
      font-size: 1.125rem;
    }

    .person-name {
      padding-top: 0;
      padding-left: 0;
    }
  }

  &:not(.main-cell) .person-name {
    color: var(--parsec-color-light-secondary-grey);
  }
}

.person-name {
  padding-top: 0.3em;
  padding-left: 0.5em;
  font-size: 1em;
  font-weight: 500;
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
