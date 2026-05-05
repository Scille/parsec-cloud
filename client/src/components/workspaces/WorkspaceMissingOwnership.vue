<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="missing-ownership-label">
    <ion-icon
      :icon="warning"
      class="missing-ownership-label__icon"
    />
    <ion-text
      class="missing-ownership-label__text button-medium"
      v-if="showText"
    >
      {{ $msTranslate('WorkspacesPage.missingOwnership.label') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';
import { warning } from 'ionicons/icons';

defineProps<{
  showText?: boolean;
}>();
</script>

<style scoped lang="scss">
.missing-ownership-label {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.375rem;
  border-radius: var(--parsec-radius-12);
  overflow: hidden;
  background: var(--parsec-color-light-danger-50);
  border: 1px solid var(--parsec-color-light-danger-100);
  transition: all 0.1s ease-in-out;
  flex-shrink: 0;

  &__icon {
    font-size: 1rem;
    color: var(--parsec-color-light-danger-500);
    flex-shrink: 0;
  }

  &__text {
    text-wrap: nowrap;
    text-align: left;
    color: var(--parsec-color-light-danger-500);
    text-shadow: var(--parsec-shadow-light);
  }

  &:hover {
    background: var(--parsec-color-light-danger-100);

    .missing-ownership-label__icon,
    .missing-ownership-label__text {
      color: var(--parsec-color-light-danger-700);
    }
  }
}
</style>
