<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-grid>
    <ion-row>
      <ion-col>
        <ion-item>
          <ion-input
            :label="label"
            label-placement="floating"
            :type="passwordVisible ? 'text' : 'password'"
            v-model="passwordRef"
            @ion-input="$emit('change', $event.detail.value)"
            @keyup.enter="onEnterPress()"
            :autofocus="true"
          />
          <ion-button
            @click="passwordVisible = !passwordVisible"
            slot="end"
            fill="clear"
          >
            <ion-icon
              slot="icon-only"
              :icon="passwordVisible ? eyeOffOutline : eyeOutline"
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
  eyeOutline,
  eyeOffOutline
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
  ion-item {
    align-items: center;
  }
</style>
