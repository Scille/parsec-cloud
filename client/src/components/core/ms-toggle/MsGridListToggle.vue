<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="ms-grid-list-toggle">
    <!-- grid -->
    <ion-button
      fill="clear"
      class="button-view"
      id="grid-view"
      :disabled="modelValue === DisplayState.Grid"
      @click="$emit('update:modelValue', modelValue === DisplayState.Grid ? DisplayState.List : DisplayState.Grid)"
    >
      <ion-icon :icon="grid" />
    </ion-button>
    <!-- list -->
    <ion-button
      fill="clear"
      class="button-view"
      id="list-view"
      :disabled="modelValue === DisplayState.List"
      @click="$emit('update:modelValue', modelValue === DisplayState.Grid ? DisplayState.List : DisplayState.Grid)"
    >
      <ion-icon :icon="list" />
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import { DisplayState } from '@/components/core/ms-toggle/types';
import { IonButton, IonIcon } from '@ionic/vue';
import { grid, list } from 'ionicons/icons';
import { defineEmits } from 'vue';

defineProps<{
  modelValue: DisplayState;
}>();

defineEmits<{
  (e: 'update:modelValue', value: DisplayState): void;
}>();

defineExpose({
  DisplayState,
});
</script>

<style scoped lang="scss">
.ms-grid-list-toggle {
  display: flex;
  align-items: center;
  background: var(--parsec-color-light-secondary-white);
  padding: 0;
  gap: 0.25rem;
}

.button-view {
  color: var(--parsec-color-light-secondary-light);
  margin: 0;
  border-radius: var(--parsec-radius-8);
  border: 1px solid var(--parsec-color-light-secondary-background);
  height: auto;

  &:not(.button-disabled) {
    cursor: pointer;
    --background-hover: none;

    &:hover {
      color: var(--parsec-color-light-primary-700);
      opacity: 0.5;
    }
  }

  &::part(native) {
    padding-inline-start: 0px;
    padding-inline-end: 0px;
    padding-top: 0px;
    padding-bottom: 0px;
  }

  ion-icon {
    font-size: 1.25rem;
    padding: 0.25rem;
  }
}

.button-disabled {
  border: 1px solid var(--parsec-color-light-primary-700);
  background: var(--parsec-color-light-secondary-premiere);
  color: var(--parsec-color-light-primary-700);
  opacity: 1;
}
</style>
