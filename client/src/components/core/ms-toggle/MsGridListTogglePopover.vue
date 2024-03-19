<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="ms-grid-list-toggle-popover">
    <!-- grid -->
    <ion-list class="view-list">
      <ion-label class="view-list__title body-sm">
        {{ $t('common.view.display') }}
      </ion-label>
      <ion-item
        class="view-list-item"
        :class="{ selected: modelValue === DisplayState.Grid }"
        button
        id="grid-view"
        @click="$emit('update:modelValue', modelValue === DisplayState.Grid ? DisplayState.List : DisplayState.Grid)"
      >
        <ion-icon
          :icon="grid"
          class="view-list-item__icon"
        />
        <span class="body">{{ $t('common.view.grid') }}</span>
        <ion-icon
          class="checked-icon"
          :icon="checkmark"
          v-if="modelValue === DisplayState.Grid"
        />
      </ion-item>
      <!-- list -->
      <ion-item
        class="view-list-item"
        :class="{ selected: modelValue === DisplayState.List }"
        id="list-view"
        @click="$emit('update:modelValue', modelValue === DisplayState.Grid ? DisplayState.List : DisplayState.Grid)"
      >
        <ion-icon
          :icon="list"
          class="view-list-item__icon"
        />
        <span class="body">{{ $t('common.view.list') }}</span>
        <ion-icon
          class="checked-icon"
          :icon="checkmark"
          v-if="modelValue === DisplayState.List"
        />
      </ion-item>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { DisplayState } from '@/components/core/ms-toggle/types';
import { IonIcon, IonLabel, IonList, IonItem } from '@ionic/vue';
import { grid, list, checkmark } from 'ionicons/icons';

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
.view-list {
  padding: 0.75rem;
  display: flex;
  flex-direction: column;

  &__title {
    color: var(--parsec-color-light-secondary-grey);
    opacity: 0.7;
    margin-bottom: 0.5rem;
  }

  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  &-item {
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
    --color: var(--parsec-color-light-secondary-grey);
    --color-hover: var(--parsec-color-light-primary-700);
    --border-radius: var(--parsec-radius-4);
    --padding-top: 0.375rem;
    --padding-bottom: 0.375rem;
    --padding-start: 0.5rem;
    --padding-end: 0.5rem;
    --inner-padding-end: 0;

    &__icon {
      margin-right: 0.5em;
      font-size: 1.25rem;
      color: var(--parsec-color-light-secondary-grey);
    }

    .checked-icon {
      margin-left: auto;
    }

    &:hover {
      ion-icon {
        color: var(--parsec-color-light-primary-700);
      }
    }

    &.selected {
      color: var(--parsec-color-light-primary-700);
      --color-hover: var(--parsec-color-light-primary-700);
      --background-hover: none;

      span {
        font-weight: 600;
      }

      ion-icon {
        color: var(--parsec-color-light-primary-700);
      }
    }

    ion-icon {
      font-size: 1.25rem;
    }
  }
}
</style>
