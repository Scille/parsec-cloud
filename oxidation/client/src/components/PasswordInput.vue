<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-grid>
    <ion-row>
      <ion-col>
        <ion-item>
          <ion-label position="stacked">{{ label }}</ion-label>
          <ion-input :type="passwordVisible ? 'text' : 'password'"></ion-input>
        </ion-item>
      </ion-col>
      <ion-col size="auto">
        <ion-button @click="passwordVisible = !passwordVisible">
          <ion-icon
            slot="icon-only"
            :icon="passwordVisible ? eyeOffOutline : eyeOutline">
          </ion-icon>
        </ion-button>
      </ion-col>
    </ion-row>
  </ion-grid>
</template>

<script setup lang="ts">
import { defineProps, ref } from 'vue';
import { IonGrid, IonCol, IonRow, IonButton, IonItem, IonInput, IonIcon, IonLabel } from '@ionic/vue';
import {
  eyeOutline,
  eyeOffOutline
} from 'ionicons/icons';

defineProps<{
  label: string
}>();

const passwordVisible = ref(false);

</script>

<style lang="scss" scoped>

</style>
