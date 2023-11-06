<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="settings-option">
    <div class="settings-option__content">
      <ion-text class="body title">
        {{ title }}
      </ion-text>
      <ion-text class="body-sm description">
        {{ description }}
      </ion-text>
    </div>
    <slot />
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';

defineProps<{
  title: string
  description?: string
}>();
</script>

<style scoped lang="scss">
.settings-option {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem 0.75rem 0;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  gap: 1.5rem;

  &__content {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    gap: 0.25rem;

    .title {
      color: var(--parsec-color-light-primary-700);
    }

    .description {
      color: var(--parsec-color-light-secondary-grey)
    }
  }
}
</style>
