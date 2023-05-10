<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item class="toggle-settings">
    <div class="toggle-settings__content">
      <ion-text class="body title">
        {{ title }}
      </ion-text>
      <ion-text class="body-sm description">
        {{ description }}
      </ion-text>
    </div>
    <ion-toggle
      class="toggle-settings__toggle"
      value=""
    />
  </ion-item>
</template>

<script setup lang="ts">
import {
  IonItem,
  IonToggle
} from '@ionic/vue';

defineProps<{
  title: string
  description?: string
}>();
</script>

<style scoped lang="scss">
.toggle-settings {
  display: flex;
  justify-content: space-between;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);

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

  &__toggle {
    width: auto;
    --handle-width: 30px;
    --handle-height: 30px;
    --handle-max-height: auto;
    --handle-spacing: 3px;

    // uncheck
    &::part(track) {
      height: 36px;
      width: 60px;
      border-radius: 20px;
      background: var(--parsec-color-light-secondary-inversed-contrast);
      border: 1.5px solid var(--parsec-color-light-primary-600);
      /* Required for iOS handle to overflow the height of the track */
      overflow: visible;
      opacity: .4;
    }

    &::part(handle) {
      background: var(--parsec-color-light-primary-600);
      box-shadow: none;
    }

    // check
    &.toggle-checked::part(track){
      background: var(--parsec-color-light-primary-600);
      border: none;
      opacity: 1;
    }

    &.toggle-checked::part(handle) {
      background: var(--parsec-color-light-secondary-inversed-contrast);
      box-shadow: -2px 1px 6px rgba(0, 0, 0, 0.24);
    }
  }
}
</style>
