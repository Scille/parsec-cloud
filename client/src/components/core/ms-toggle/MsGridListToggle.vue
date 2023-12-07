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
      <span v-if="modelValue === DisplayState.Grid">
        {{ $t('WorkspacesPage.viewDisplay.grid') }}
      </span>
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
      <span v-if="modelValue === DisplayState.List">
        {{ $t('WorkspacesPage.viewDisplay.list') }}
      </span>
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonIcon } from '@ionic/vue';
import { defineEmits } from 'vue';
import { grid, list } from 'ionicons/icons';
import { DisplayState } from '@/components/core/ms-toggle/types';

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
}

.button-view {
  color: var(--parsec-color-light-primary-700);
  padding: 0.25rem;
  border-radius: 4px;
  height: auto;

  span {
    margin-left: 0.5rem;
  }

  &:not(.button-disabled) {
    cursor: pointer;
    --background-hover: none;
  }
}

.button-disabled {
  background: var(--parsec-color-light-secondary-inversed-contrast);
  opacity: 1;
}

ion-button::part(native) {
  padding-inline-start: 0px;
  padding-inline-end: 0px;
  padding-top: 0px;
  padding-bottom: 0px;
}
</style>
