<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <span
    v-if="!editing"
    class="file-controls-input text-only"
    @click="onTextClick"
  >
    {{ modelValue }}
    {{ $msTranslate(suffix) }}
  </span>
  <ion-input
    v-else
    class="file-controls-input editing-input"
    fill="outline"
    ref="inputRef"
    :value="modelValue"
    @ion-blur="onFocusChanged(false)"
    @ion-focus="onFocusChanged(true)"
    @ion-input="onChange($event.detail.value || '')"
    @keyup.enter="enterPressed($event.target.value)"
    @keyup.esc="onFocusChanged(false)"
    :disabled="$props.disabled"
  >
    <span
      class="file-controls-input-suffix"
      slot="end"
    >
      {{ $msTranslate(suffix) }}
    </span>
  </ion-input>
</template>

<script setup lang="ts">
import { IonInput } from '@ionic/vue';
import { Translatable } from 'megashark-lib';
import { ref } from 'vue';

const editing = ref(false);
const inputRef = ref();
const lostFocus = ref(false);

const props = defineProps<{
  modelValue: string;
  disabled?: boolean;
  restrictChange?: (value: string) => Promise<string>;
  suffix?: Translatable;
}>();

const emits = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
  (e: 'onSubmittedValue', value: string): void;
  (e: 'onEnterKeyup', value: string): void;
  (e: 'onFocusChanged', value: boolean): void;
}>();

defineExpose({
  setFocus,
});

async function setFocus(): Promise<void> {
  if (inputRef.value && inputRef.value.$el) {
    await inputRef.value.$el.setFocus();
  }
}

async function onFocusChanged(focus: boolean): Promise<void> {
  if (focus === false) {
    lostFocus.value = true;
    editing.value = false;
    await onSubmittedValue(props.modelValue || '');
  }
  emits('onFocusChanged', focus);
}

async function enterPressed(value: string): Promise<void> {
  editing.value = false;
  emits('onEnterKeyup', value);
  await onSubmittedValue(value);
}

async function onChange(value: string): Promise<void> {
  if (props.restrictChange) {
    const restrictedValue = await props.restrictChange(value);
    if (restrictedValue) {
      emits('update:modelValue', restrictedValue);
      emits('change', restrictedValue);
    }
  } else {
    emits('update:modelValue', value);
    emits('change', value);
  }
  if (!value) {
    lostFocus.value = true;
  }
}

async function onSubmittedValue(value: string): Promise<void> {
  emits('onSubmittedValue', value);
}

async function onTextClick(): Promise<void> {
  editing.value = true;
  await setFocus();
}
</script>

<style lang="scss" scoped>
.file-controls-input {
  margin-inline: 0px;
  margin-top: 0px;
  margin-bottom: 0px;
  min-width: 3rem;
  text-align: center;
  color: var(--parsec-color-light-primary-600);
  opacity: 0.6;
  transition: all 0.2s ease-in-out;

  &.editing-input {
    --background: none !important;
    --background-hover: none !important;
    --padding-top: 0.5rem;
    --padding-bottom: 0.5rem;
    --padding-end: 0.5rem;
    --padding-start: 0.5rem;
    border-radius: var(--parsec-radius-6);
    --border-color: var(--parsec-color-light-primary-600) !important;
    --border-width: 1px !important;
  }

  &.text-only {
    cursor: pointer;
    padding: 0.5rem;
  }

  &:hover {
    opacity: 1;
  }

  &-suffix {
    margin-inline-start: 0.25rem;
  }
}
</style>
