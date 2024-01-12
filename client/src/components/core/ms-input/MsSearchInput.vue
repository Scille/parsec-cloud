<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- Here we still use ion-item to wrap ion-input because of lack of support of ion-icon with new v7 standalone ion-input -->
  <!-- TODO: Migrate from legacy to modern syntax following this issue: https://github.com/ionic-team/ionic-framework/issues/26297 -->
  <ion-item
    class="input-item ms-search-input"
    id="ms-search-input"
  >
    <ion-icon
      :icon="search"
      slot="start"
      class="icon"
    />
    <ion-input
      class="form-input input"
      v-model="searchRef"
      :value="modelValue"
      :placeholder="label"
      :clear-input="true"
      @ion-input="onChange($event.target.value)"
      @keyup.enter="onEnterPress()"
      mode="ios"
    />
    <!-- mode=ios to change the clear icon style -->
  </ion-item>
</template>

<script setup lang="ts">
import { IonIcon, IonInput, IonItem } from '@ionic/vue';
import { search } from 'ionicons/icons';
import { ref } from 'vue';

defineProps<{
  label: string;
  modelValue?: string;
}>();

const searchRef = ref('');

const emits = defineEmits<{
  (e: 'change', value: string): void;
  (e: 'enter'): void;
  (e: 'update:modelValue', value: string): void;
}>();

function onEnterPress(): void {
  if (searchRef.value.length > 0) {
    emits('enter');
  }
}

function onChange(value: any): void {
  emits('update:modelValue', value);
  emits('change', value);
}
</script>

<style scoped lang="scss">
.ms-search-input {
  border: 1px solid var(--parsec-color-light-secondary-light);
  flex-grow: 0;
  margin-right: 1rem;
  max-width: 30rem;

  &:focus-within {
    flex-grow: 1;
  }
  .input {
    --placeholder-color: var(--parsec-color-light-secondary-light);
    --placeholder-opacity: 0.8;
    min-height: 1rem;
  }

  .icon {
    font-size: 1.125em;
    margin-inline-end: 0.5rem;
    color: var(--parsec-color-light-secondary-light);
  }
}
</style>
