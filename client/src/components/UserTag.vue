<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="container">
    <ion-avatar
      class="avatar person-avatar body-sm"
    >
      {{ user.substring(0, 2) }}
    </ion-avatar>
    <ion-text class="person-name">
      {{ user }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonAvatar, IonText } from '@ionic/vue';
import { defineProps } from 'vue';

defineProps<{
  user: string
}>();
</script>

<style lang="scss" scoped>
.container {
  display: flex;
  flex-wrap: nowrap;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  border-radius: 32px;

  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    text-transform: uppercase;
    margin-left: -0.6em;
    height: 2rem;
    width: 2rem;
    border: 2px solid var(--parsec-color-light-secondary-inversed-contrast);

    &:first-of-type{
      margin-left: 0;
    }
  }
}

.person-name {
  padding-top: 0.3em;
  padding-left: 0.5em;
  font-size: 1em;
  font-weight: 500;
  color: var(--parsec-color-light-secondary-grey);
}

.person-avatar {
  color: var(--parsec-color-light-primary-500);
  background-color: var(--parsec-color-light-primary-100);
}
</style>
