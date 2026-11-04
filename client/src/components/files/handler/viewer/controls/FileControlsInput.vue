<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="file-controls-input-container">
    <span
      class="file-controls-input-prefix"
      v-if="prefix"
    >
      {{ $msTranslate(prefix) }}
    </span>
    <ion-text
      v-if="!editing"
      class="file-controls-input text-only"
      @click="onTextClick"
    >
      {{ modelValue }}
      <span v-if="suffix">{{ $msTranslate(suffix) }}</span>
    </ion-text>
    <ion-input
      v-else
      class="file-controls-input editing-input"
      fill="outline"
      ref="input"
      :value="modelValue"
      @ion-blur="onFocusChanged(false)"
      @ion-focus="onFocusChanged(true)"
      @ion-input="onChange($event.detail.value || '')"
      @keyup.enter="blur"
      @keyup.esc="blur"
      :disabled="$props.disabled"
    >
      <span
        class="file-controls-input-suffix"
        slot="end"
        v-if="suffix"
      >
        {{ $msTranslate(suffix) }}
      </span>
    </ion-input>
  </div>
</template>

<script setup lang="ts">
import { IonInput, IonText } from '@ionic/vue';
import { Translatable } from 'megashark-lib';
import { ref, useTemplateRef } from 'vue';

const editing = ref(false);
const inputRef = useTemplateRef<InstanceType<typeof IonInput>>('input');
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
.file-controls-input-container {
  display: flex;
  align-items: center;
  flex-shrink: 0;
  gap: ms.spacing('gap-lg');
}

.file-controls-input {
  margin-inline: 0px;
  margin-top: 0px;
  margin-bottom: 0px;
  min-width: 3rem;
  text-align: center;
  color: ms.color('text-neutral-default');
  transition: all 0.15ms ease-in-out;

  &.editing-input {
    --background: none !important;
    --background-hover: none !important;
    --padding-top: #{ms.spacing('padding-lg')};
    --padding-bottom: #{ms.spacing('padding-lg')};
    --padding-end: #{ms.spacing('padding-lg')};
    --padding-start: #{ms.spacing('padding-lg')};
    border-radius: ms.radius('md');
    --border-width: #{ms.border('none')} !important;
    outline: ms.border('thin') solid ms.color('border-neutral-default');
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }

  &.text-only {
    @include ms.font('body-lg-medium');
    cursor: pointer;
    background: ms.color('surface-base-default-secondary');
    padding: ms.spacing('padding-lg') ms.spacing('padding-xl');
    outline: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    border-radius: ms.radius('md');
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
    min-width: 4rem;
    width: 100%;
  }

  &:hover {
    outline: ms.border('thin') solid ms.color('border-base-default-hover');
  }

  &-prefix {
    @include ms.font('body-md-medium');
    color: ms.color('text-neutral-default');
  }

  &-suffix {
    margin-inline-start: ms.spacing('gap-sm') !important;
  }
}
</style>
