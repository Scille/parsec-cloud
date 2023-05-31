<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-grid>
    <ion-row>
      <ion-col class="input-col">
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
            @ion-input="$emit('change', $event.detail.value)"
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
import { IonGrid, IonCol, IonRow, IonButton, IonItem, IonInput, IonIcon } from '@ionic/vue';
import {
  eye,
  eyeOff
} from 'ionicons/icons';

defineProps<{
  label: string
}>();

const emits = defineEmits<{
  (e: 'change', value: string): void
  (e: 'enter'): void
}>();

const passwordVisible = ref(false);
const passwordRef = ref('');

function onEnterPress() : void {
  if (passwordRef.value.length > 0) {
    emits('enter');
  }
}
</script>

<style lang="scss" scoped>
.input-col {
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: .5rem;
  .form-label {
    color: var(--parsec-color-light-primary-700);
  }
  .input {
    border-radius: 6px;
    overflow: hidden;
  }
}
</style>
