<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="container">
    <ion-avatar
      class="avatar person-avatar body-sm"
      v-for="person in people.slice(0, maxDisplay)"
      :key="person"
    >
      {{ person.substring(0, 2) }}
    </ion-avatar>
    <ion-avatar
      class="avatar extra-avatar"
      v-if="people.length > maxDisplay"
    >
      + {{ people.length - maxDisplay }}
    </ion-avatar>
  </div>
</template>

<script setup lang="ts">
import { IonAvatar } from '@ionic/vue';
import { defineProps } from 'vue';

defineProps<{
  people: string[],
  maxDisplay: number
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

.person-avatar {
  background-color: var(--parsec-color-tags-indigo100);
  color: var(--parsec-color-tags-indigo500);
}

.extra-avatar {
  background-color: var(--parsec-color-light-secondary-background);
  color: var(--parsec-color-light-secondary-grey);
}
</style>
