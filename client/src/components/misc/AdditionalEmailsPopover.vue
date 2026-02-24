<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="emails-list">
    <ion-item
      class="emails-list-item body"
      lines="none"
      v-for="(email, index) in emails"
      :key="index"
    >
      <ion-text class="emails-list-item__text">
        {{ email }}
      </ion-text>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { IonItem, IonList, IonText } from '@ionic/vue';

defineProps<{
  emails: Array<string>;
}>();
</script>

<style lang="scss" scoped>
.emails-list {
  display: flex;
  flex-direction: column;
  padding: 0;

  &::part(native) {
    --padding-start: 0;
    --padding-end: 0;
  }

  &-item {
    padding: 0.5rem 0.75rem;
    --background: none;
    --background-hover: none;
    --background-hover-opacity: 1;
    --inner-padding-end: 0;
    color: var(--parsec-color-light-secondary-text);

    &::part(native) {
      padding: 0;
      cursor: default;
    }

    &__text {
      word-break: break-all;
    }

    &:not(:last-child) {
      border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
    }
  }
}
</style>
