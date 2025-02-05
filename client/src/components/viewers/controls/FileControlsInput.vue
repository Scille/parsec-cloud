<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-text
    v-if="!editing"
    class="file-controls-input text-only subtitles-normal"
    @click="onTextClick"
  >
    <span v-if="prefix">{{ $msTranslate(prefix) }}</span>
    {{ modelValue }}
    <span v-if="suffix">{{ $msTranslate(suffix) }}</span>
  </ion-text>
  <ion-input
    v-else
    class="file-controls-input editing-input"
    fill="outline"
    ref="inputRef"
    :value="modelValue"
    @ion-blur="onFocusChanged(false)"
    @ion-focus="onFocusChanged(true)"
    @ion-input="onChange($event.detail.value || '')"
    @keyup.enter="blur"
    @keyup.esc="blur"
    :disabled="$props.disabled"
  >
    <span
      class="file-controls-input-prefix"
      slot="start"
      v-if="prefix"
    >
      {{ $msTranslate(prefix) }}
    </span>
    <span
      class="file-controls-input-suffix"
      slot="end"
      v-if="suffix"
    >
      {{ $msTranslate(suffix) }}
    </span>
  </ion-input>
</template>

<script setup lang="ts">
import { IonInput, IonText } from '@ionic/vue';
import { Translatable } from 'megashark-lib';
import { ref } from 'vue';

const editing = ref(false);
const inputRef = ref();
const lostFocus = ref(false);

const props = defineProps<{
  modelValue: string;
  disabled?: boolean;
  restrictChange?: (value: string) => Promise<string>;
  prefix?: Translatable;
  suffix?: Translatable;
}>();

const emits = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
  (e: 'onSubmittedValue', value: string): void;
  (e: 'onFocusChanged', value: boolean): void;
}>();

defineExpose({
  setFocus,
  isEditing,
});

async function setFocus(): Promise<void> {
  // Doesn't work without setTimeout for some reason
  setTimeout(async () => {
    if (inputRef.value && inputRef.value.$el) {
      await inputRef.value.$el.setFocus();
    }
  }, 100);
}

async function onFocusChanged(focus: boolean): Promise<void> {
  if (focus === false) {
    await blur();
  }
  emits('onFocusChanged', focus);
}

async function onChange(value: string): Promise<void> {
  if (props.restrictChange) {
    const restrictedValue = await props.restrictChange(value);
    if (restrictedValue) {
      emitChange(restrictedValue);
    }
  } else {
    emitChange(value);
  }
  if (!value) {
    lostFocus.value = true;
  }
}

function emitChange(value: string): void {
  emits('update:modelValue', value);
  emits('change', value);
}

async function onSubmittedValue(value: string): Promise<void> {
  emits('onSubmittedValue', value);
}

async function onTextClick(): Promise<void> {
  editing.value = true;
  await setFocus();
}

function isEditing(): boolean {
  return editing.value;
}

async function blur(): Promise<void> {
  lostFocus.value = true;
  editing.value = false;
  await onSubmittedValue(props.modelValue || '');
}
</script>

<style lang="scss" scoped>
.file-controls-input {
  margin-inline: 0px;
  margin-top: 0px;
  margin-bottom: 0px;
  min-width: 3rem;
  text-align: center;
  color: var(--parsec-color-light-secondary-soft-text);
  transition: all 0.15ms ease-in-out;

  &.editing-input {
    --background: none !important;
    --background-hover: none !important;
    --padding-top: 0.5rem;
    --padding-bottom: 0.5rem;
    --padding-end: 0.5rem;
    --padding-start: 0.5rem;
    border-radius: var(--parsec-radius-6);
    --border-width: 0 !important;
    outline: 1px solid var(--parsec-color-light-secondary-text);
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }

  &.text-only {
    cursor: pointer;
    background: var(--parsec-color-light-secondary-background);
    padding: 0.5rem 0.625rem;
    outline: 1px solid var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-6);
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }

  &:hover {
    outline: 1px solid var(--parsec-color-light-secondary-soft-grey);
  }

  &-prefix {
    margin-inline-end: 0.25rem;
  }

  &-suffix {
    margin-inline-start: 0.25rem;
  }
}
</style>
