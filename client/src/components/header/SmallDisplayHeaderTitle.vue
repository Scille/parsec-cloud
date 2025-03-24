<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="small-display-header-title">
    <ion-text
      v-if="title"
      class="title__text title-h2"
    >
      {{ $msTranslate(props.title) }}
    </ion-text>
    <slot />
    <ion-icon
      class="title__icon"
      :icon="ellipsisHorizontal"
      @click="$emit('openContextualModal', $event)"
    />
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon } from '@ionic/vue';
import { ellipsisHorizontal } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

const props = defineProps<{
  title: Translatable;
}>();

defineEmits<{
  (e: 'openContextualModal', event: Event): void;
}>();
</script>

<style scoped lang="scss">
.small-display-header-title {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 0 1rem;
  padding: 0.125rem 1rem;
  border-radius: var(--parsec-radius-12);
  border-bottom: 1px solid var(--parsec-color-light-secondary-premiere);
  background-color: var(--parsec-color-light-secondary-background);
}

.title__text {
  color: var(--parsec-color-light-primary-800);
  flex-grow: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.title__icon {
  font-size: 1.5rem;
  color: var(--parsec-color-light-secondary-grey);
  padding: 0.5rem;
  flex-shrink: 0;
}
</style>
