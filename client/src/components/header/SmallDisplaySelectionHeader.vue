<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="small-display-selection-header">
    <ion-button
      v-if="someSelected"
      fill="outline"
      class="title__button title__button-left"
      @click="$emit('unselect', $event)"
    >
      {{ $msTranslate('FoldersPage.actions.unselect') }}
    </ion-button>
    <ion-button
      v-else
      fill="outline"
      class="title__button title__button-left"
      @click="$emit('select', $event)"
    >
      {{ $msTranslate('FoldersPage.actions.select') }}
    </ion-button>
    <ion-text class="title__text">
      <span class="title__text--content">{{ $msTranslate(props.title) }}</span>
    </ion-text>
    <slot />
    <ion-button
      class="title__button title__button-right"
      @click="$emit('cancelSelection', $event)"
    >
      {{ $msTranslate('FoldersPage.actions.cancel') }}
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonText } from '@ionic/vue';
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
  gap: ms.spacing('gap-lg');
  background: ms.color('surface-base-default-secondary');
  padding: ms.spacing('padding-3xl') ms.spacing('padding-2xl');
}

.title__text {
  @include ms.font('heading-h4');
  display: flex;
  color: ms.color('text-neutral-default');
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
  color: ms.color('text-neutral-default');
  flex-shrink: 0;
  cursor: pointer;

  &:hover {
    color: ms.color('text-brand-default');
  }

  &:active {
    color: ms.color('text-brand-default');
  }
}
</style>
