<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="small-display-header-title"
    :class="selectionEnabled ? 'small-display-header-title--selection' : ''"
  >
    <div v-if="selectionEnabled">
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
    </div>
    <ion-text
      v-if="title"
      class="title__text title-h2"
      :class="selectionEnabled ? 'title__text--selection' : ''"
    >
      <span class="title__text--content">{{ $msTranslate(props.title) }}</span>
    </ion-text>
    <slot />
    <ion-text
      v-if="selectionEnabled"
      class="button-medium title__button title__button-right"
      @click="$emit('cancelSelection', $event)"
    >
      {{ $msTranslate('FoldersPage.actions.cancel') }}
    </ion-text>
    <ion-icon
      v-if="!selectionEnabled && !optionsDisabled"
      class="title__icon"
      :icon="ellipsisHorizontal"
      @click="$emit('openContextualModal', $event)"
    />
  </div>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';
import { ellipsisHorizontal } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

const props = defineProps<{
  title: Translatable;
  selectionEnabled?: boolean;
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
.small-display-header-title {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.125rem 2rem 1rem;
  min-height: 3.75rem;
  gap: 0.5rem;
  background: var(--parsec-color-light-secondary-background);

  &--selection {
    padding: 0.125rem 0.25rem 1rem;
  }
}

.title__text {
  display: flex;
  color: var(--parsec-color-light-primary-800);
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;

  &--content {
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }

  &--selection {
    justify-content: center;
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
  padding: 0.8rem;
  flex-shrink: 0;

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
