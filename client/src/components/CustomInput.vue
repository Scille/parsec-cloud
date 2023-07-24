<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div
    class="input-container"
  >
    <span
      id="label"
      class="form-label"
    >
      {{ label }}
    </span>

    <ion-item
      class="input"
      :class="{'form-input-disabled': $props.disabled}"
    >
      <ion-input
        class="form-input"
        :autofocus="true"
        :placeholder="$props.placeholder"
        :value="modelValue"
        @ion-input="$emit('update:modelValue', $event.detail.value); $emit('change', $event.detail.value)"
        :disabled="$props.disabled"
      />
    </ion-item>
    <!-- We need to had a info informative text for the custom input issue-->
    <!-- <span
      class="form-error caption-caption"
    >
      <ion-icon
        class="form-error-icon"
        :icon="errorIcon"
      />
      {{ errorMessage }}
    </span> -->
  </div>
</template>

<script setup lang="ts">
import {IonItem, IonInput } from '@ionic/vue';

defineProps<{
  label?: string,
  placeholder?: string,
  errorMessage?: string,
  errorIcon?: string,
  modelValue?: string,
  disabled?: boolean
}>();

defineEmits<{
  (e: 'update:modelValue', value: string): void,
  (e: 'change', value: string): void
}>();
</script>

<style lang="scss" scoped>
.input-container {
  // offset necessary to simulate border 3px on focus with outline (outline 2px + border 1px)
  --offset: 2px;
  padding: var(--offset);
  display: flex;
  flex-direction: column;
  gap: .5rem;

  .form-label{
    color: var(--parsec-color-light-primary-700);
  }

  .input {
    border: 1px solid var(--parsec-color-light-primary-300);
    border-radius: var(--parsec-radius-6);
    overflow: hidden;
    color: var(--parsec-color-light-primary-800);

    &:focus-within {
      --background: var(--parsec-color-light-secondary-background);
      outline: var(--offset) solid var(--parsec-color-light-primary-300);
    }
  }

  .form-input-disabled {
    --background: var(--parsec-color-light-secondary-disabled);
    border: var(--parsec-color-light-secondary-disabled);
  }
}
</style>
