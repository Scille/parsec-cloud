<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="toggle-settings">
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
      :checked="modelValue"
      @ion-change="$emit('update:modelValue', $event.target.checked)"
    />
  </div>
</template>

<script setup lang="ts">
import {
  IonToggle,
  IonText
} from '@ionic/vue';
import { defineProps, defineEmits } from 'vue';

defineProps<{
  title: string
  description?: string,
  modelValue: boolean
}>();

defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>();
</script>

<style scoped lang="scss">
.toggle-settings {
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

  &__toggle {
    width: auto;
    --handle-width: 22px;
    --handle-height: 22px;
    --handle-max-height: auto;
    --handle-spacing: 3px;

    // uncheck
    &::part(track) {
      height: 28px;
      width: 50px;
      border-radius: 20px;
      background: var(--parsec-color-light-secondary-inversed-contrast);
      box-shadow:inset 0px 0px 0px 1px var(--parsec-color-light-primary-600);
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
