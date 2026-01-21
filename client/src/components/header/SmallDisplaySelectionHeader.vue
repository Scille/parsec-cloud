<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="small-display-selection-header">
    <ion-text
      v-if="someSelected"
      class="button-medium title__button title__button-left"
      @click="$emit('unselect', $event)"
    >
      {{ $msTranslate('FoldersPage.actions.unselect') }}
    </ion-text>
    <ion-text
      v-else
      class="button-medium title__button title__button-left"
      @click="$emit('select', $event)"
    >
      {{ $msTranslate('FoldersPage.actions.select') }}
    </ion-text>
    <ion-text class="title__text title-h3">
      <span class="title__text--content">{{ $msTranslate(props.title) }}</span>
    </ion-text>
    <slot />
    <ion-text
      class="button-medium title__button title__button-right"
      @click="$emit('cancelSelection', $event)"
    >
      {{ $msTranslate('FoldersPage.actions.cancel') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import { Translatable } from 'megashark-lib';

const props = defineProps<{
  title: Translatable;
  someSelected?: boolean;
  optionsDisabled?: boolean;
}>();

defineEmits<{
  (e: 'openContextualModal', event: Event): void;
  (e: 'select', event: Event): void;
  (e: 'unselect', event: Event): void;
  (e: 'cancelSelection', event: Event): void;
}>();
</script>

<style scoped lang="scss">
.small-display-selection-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 0.5rem;
  background: var(--parsec-color-light-secondary-background);
  padding: 1.5rem 1rem;
}

.title__text {
  display: flex;
  color: var(--parsec-color-light-primary-800);
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  justify-content: center;

  &--content {
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }
}

.title__icon {
  font-size: 1.5rem;
  color: var(--parsec-color-light-secondary-grey);
  padding: 0.5rem;
  flex-shrink: 0;
  cursor: pointer;

  &:hover {
    color: var(--parsec-color-light-primary-500);
  }

  &:active {
    color: var(--parsec-color-light-primary-500);
  }
}

.title__button {
  padding: 0.625rem 0.825rem;
  flex-shrink: 0;
  background: var(--parsec-color-light-secondary-white);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12);
  box-shadow: var(--parsec-shadow-soft);

  &:hover {
    cursor: pointer;
  }

  &-left {
    color: var(--parsec-color-light-primary-500);
  }

  &-right {
    color: var(--parsec-color-light-secondary-text);
  }
}
</style>
