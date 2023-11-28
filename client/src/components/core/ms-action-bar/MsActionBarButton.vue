<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    class="ms-action-bar-button button-medium"
    size="default"
  >
    <ion-icon
      v-show="icon"
      slot="start"
      :icon="icon"
    />
    {{ buttonLabel }}
  </ion-button>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';
import { IonButton, IonIcon } from '@ionic/vue';

defineProps<{
  buttonLabel?: any;
  icon?: string;
}>();
</script>

<style lang="scss" scoped>
.ms-action-bar-button {
  --background: none !important;
  --background-hover: var(--parsec-color-light-primary-50) !important;
  margin-inline: 0px;
  margin-top: 0px;
  margin-bottom: 0px;
  --padding-top: 0.5rem;
  --padding-end: 0.5rem;
  --padding-bottom: 0.5rem;
  --padding-start: 0.5rem;
  color: var(--parsec-color-light-secondary-grey);

  &:hover {
    color: var(--parsec-color-light-primary-600);
  }
}

.danger:hover {
  color: var(--parsec-color-light-danger-700);
  --background-hover: var(--parsec-color-light-danger-100) !important;
}
</style>
