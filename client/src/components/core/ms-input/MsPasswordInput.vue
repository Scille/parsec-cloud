<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-grid class="input-grid">
    <ion-row>
      <ion-col class="input-container">
        <ion-text
          id="passwordLabel"
          class="form-label"
        >
          {{ label }}
        </ion-text>
        <ion-item class="input ion-no-padding">
          <ion-input
            aria-labelledby="passwordLabel"
            :type="passwordVisible ? 'text' : 'password'"
            @ion-input="onChange($event.target.value)"
            :value="modelValue"
            @keyup.enter="onEnterPress()"
            id="ms-password-input"
            :clear-on-edit="false"
            :autofocus="autofocus || false"
          />
          <div
            class="input-icon"
            @click="passwordVisible = !passwordVisible"
          >
            <ion-icon
              slot="icon-only"
              :icon="passwordVisible ? eyeOff : eye"
            />
          </div>
        </ion-item>
      </ion-col>
    </ion-row>
  </ion-grid>
</template>

<script setup lang="ts">
import { IonCol, IonGrid, IonIcon, IonInput, IonItem, IonRow, IonText } from '@ionic/vue';
import { eye, eyeOff } from 'ionicons/icons';
import { ref } from 'vue';

const props = defineProps<{
  label: string;
  modelValue?: string;
  autofocus?: boolean;
}>();

const emits = defineEmits<{
  (e: 'change', value: string): void;
  (e: 'onEnterKeyup', value: string): void;
  (e: 'update:modelValue', value: string): void;
}>();

const passwordVisible = ref(false);

function onChange(value: any): void {
  emits('update:modelValue', value);
  emits('change', value);
}

function onEnterPress(): void {
  if (props.modelValue && props.modelValue.length > 0) {
    emits('onEnterKeyup', props.modelValue);
  }
}
</script>

<style lang="scss" scoped>
#ms-password-input {
  width: auto;
  flex-grow: 1;
}

.input-grid {
  width: 100%;
}

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
    border: 1px solid var(--parsec-color-light-primary-300);
    border-radius: var(--parsec-radius-6);
    overflow: hidden;
    color: var(--parsec-color-light-primary-800);
    padding-left: 1rem;

    &:focus-within {
      --background: var(--parsec-color-light-secondary-white);
      outline: var(--offset) solid var(--parsec-color-light-primary-300);
    }

    .input-icon {
      padding: 0.5rem 0 0.5rem 1rem;
      display: flex;
      align-items: center;
      justify-content: center;

      &:hover {
        ion-icon {
          color: var(--parsec-color-light-primary-700);
        }
      }

      ion-icon {
        font-size: 1.2rem;
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}
</style>
