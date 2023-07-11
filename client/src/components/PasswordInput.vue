<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-grid>
    <ion-row>
      <ion-col class="input-container">
        <ion-text
          id="passwordLabel"
          class="form-label"
        >
          {{ label }}
        </ion-text>
        <ion-item class="input">
          <ion-input
            aria-labelledby="passwordLabel"
            :type="passwordVisible ? 'text' : 'password'"
            v-model="passwordRef"
            @ion-input="onChange($event.target.value)"
            :value="modelValue"
            @keyup.enter="onEnterPress()"
            :autofocus="true"
            id="password-input"
          />
          <ion-button
            @click="passwordVisible = !passwordVisible"
            slot="end"
            fill="clear"
          >
            <ion-icon
              slot="icon-only"
              :icon="passwordVisible ? eyeOff : eye"
            />
          </ion-button>
        </ion-item>
      </ion-col>
    </ion-row>
  </ion-grid>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { IonGrid, IonCol, IonRow, IonButton, IonItem, IonInput, IonIcon, IonText } from '@ionic/vue';
import {
  eye,
  eyeOff
} from 'ionicons/icons';

defineProps<{
  label: string,
  modelValue?: string
}>();

const emits = defineEmits<{
  (e: 'change', value: string): void
  (e: 'enter'): void,
  (e: 'update:modelValue', value: string): void
}>();

const passwordVisible = ref(false);
const passwordRef = ref('');

function onChange(value: any) : void {
  emits('update:modelValue', value);
  emits('change', value);
}

function onEnterPress() : void {
  if (passwordRef.value.length > 0) {
    emits('enter');
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
}
</style>
