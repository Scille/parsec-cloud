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
        <ion-item class="input-item ion-no-padding">
          <ion-input
            class="form-input input"
            ref="inputRef"
            aria-labelledby="passwordLabel"
            :type="passwordVisible ? 'text' : 'password'"
            @ion-input="onChange($event.target.value)"
            :value="modelValue"
            @keyup.enter="onEnterPress()"
            id="ms-password-input"
            :clear-on-edit="false"
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

const inputRef = ref();

const props = defineProps<{
  label: string;
  modelValue?: string;
}>();

const emits = defineEmits<{
  (e: 'change', value: string): void;
  (e: 'onEnterKeyup', value: string): void;
  (e: 'update:modelValue', value: string): void;
}>();

const passwordVisible = ref(false);

defineExpose({
  setFocus,
});

function setFocus(): void {
  setTimeout(() => {
    inputRef.value.$el.setFocus();
  }, 200);
}

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
.input-grid {
  width: 100%;
}
</style>
