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
      ref="inputRef"
      :value="modelValue"
      :placeholder="placeholder"
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

const props = defineProps<{
  modelValue?: string;
  placeholder?: string;
}>();

const inputRef = ref();

const emits = defineEmits<{
  (e: 'change', value: string): void;
  (e: 'enter'): void;
  (e: 'update:modelValue', value: string): void;
}>();

defineExpose({
  setFocus,
  selectText,
});

function setFocus(): void {
  setTimeout(() => {
    if (inputRef.value && inputRef.value.$el) {
      inputRef.value.$el.setFocus();
    }
  }, 200);
}

async function selectText(range?: [number, number]): Promise<void> {
  const input = await inputRef.value.$el.getInputElement();

  let begin = 0;
  let end = props.modelValue ? props.modelValue.length : 0;
  if (range) {
    begin = range[0];
    end = range[1];
  }
  input.setSelectionRange(begin, end);
}

function onEnterPress(): void {
  if (props.modelValue && props.modelValue.length > 0) {
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
  margin-right: 1rem;
  max-width: 20rem;
  padding: 0.25rem 0 0.25rem 1rem;
  flex-grow: 1;

  .input {
    --placeholder-color: var(--parsec-color-light-secondary-light);
    --placeholder-opacity: 0.8;
    min-height: 1rem;
  }

  .icon {
    font-size: 1.125em;
    margin: 0 0.5rem 0 0;
    color: var(--parsec-color-light-secondary-light);
  }
}
</style>
