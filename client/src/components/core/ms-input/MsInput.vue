<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="input-container">
    <span
      id="label"
      class="form-label"
    >
      {{ label }}
    </span>

    <ion-item
      class="input"
      :class="{
        'form-input-disabled': $props.disabled,
        'input-valid': validity === Validity.Valid,
        'input-invalid': validity === Validity.Invalid,
        'input-default': validity === Validity.Intermediate,
      }"
    >
      <ion-input
        class="form-input"
        :autofocus="true"
        :placeholder="$props.placeholder"
        :value="modelValue"
        @ion-input="onChange($event.detail.value || '')"
        @keyup.enter="$emit('onEnterKeyup', $event.target.value)"
        :disabled="$props.disabled"
      />
    </ion-item>
    <span
      v-show="errorMessage !== ''"
      class="form-error caption-caption"
    >
      <ion-icon
        class="form-error-icon"
        :icon="warning"
      />
      {{ errorMessage }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { IValidator, Validity } from '@/common/validators';
import { IonIcon, IonInput, IonItem } from '@ionic/vue';
import { warning } from 'ionicons/icons';
import { ref } from 'vue';

const props = defineProps<{
  label?: string;
  placeholder?: string;
  modelValue?: string;
  disabled?: boolean;
  validator?: IValidator;
}>();

const errorMessage = ref('');
const validity = ref(Validity.Intermediate);

const emits = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
  (e: 'onEnterKeyup', value: string): void;
}>();

async function onChange(value: string): Promise<void> {
  emits('update:modelValue', value);
  emits('change', value);
  if (props.validator) {
    const result = await props.validator(value);
    validity.value = result.validity;
    errorMessage.value = result.reason || '';
  }
}
</script>

<style lang="scss" scoped>
.input-container {
  // offset necessary to simulate border 3px on focus with outline (outline 2px + border 1px)
  --offset: 2px;
  padding: var(--offset);
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  .form-label {
    color: var(--parsec-color-light-primary-700);
  }

  .input {
    border-radius: var(--parsec-radius-6);
    overflow: hidden;
    color: var(--parsec-color-light-primary-800);
  }

  .form-input-disabled {
    --background: var(--parsec-color-light-secondary-disabled);
    background: var(--parsec-color-light-secondary-disabled);
    border: var(--parsec-color-light-secondary-disabled);
  }
}

.input-default {
  border: 1px solid var(--parsec-color-light-primary-300);

  &:focus-within {
    --background: var(--parsec-color-light-secondary-background);
    outline: var(--offset) solid var(--parsec-color-light-primary-300);
  }
}

.input-valid {
  border: 1px solid var(--parsec-color-light-primary-300);

  &:not(:focus-within) {
    --background: var(--parsec-color-light-secondary-background);
    outline: var(--offset) solid var(--parsec-color-light-success-500);
    border: 1px solid var(--parsec-color-light-success-500);
  }
}

.input-invalid {
  border: 1px solid var(--parsec-color-light-danger-500);

  &:focus-within {
    --background: var(--parsec-color-light-secondary-background);
    outline: var(--offset) solid var(--parsec-color-light-danger-500);
  }
}

.form-error {
  color: var(--parsec-color-light-danger-500);
  display: flex;
  align-items: center;
  gap: 0.2rem;
}
</style>
