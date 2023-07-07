<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-grid>
    <ion-row>
      <ion-col>
        <ion-button
          class="button-choice"
          fill="outline"
          @click="$emit('select', choices[0])"
        >
          {{ choices[0] }}
        </ion-button>
      </ion-col>
      <ion-col>
        <ion-button
          class="button-choice"
          fill="outline"
          @click="$emit('select', choices[1])"
        >
          {{ choices[1] }}
        </ion-button>
      </ion-col>
    </ion-row>
    <ion-row>
      <ion-col>
        <ion-button
          class="button-choice"
          fill="outline"
          @click="$emit('select', choices[2])"
        >
          {{ choices[2] }}
        </ion-button>
      </ion-col>
      <ion-col>
        <ion-button
          class="button-choice"
          fill="outline"
          @click="$emit('select', choices[3])"
        >
          {{ choices[3] }}
        </ion-button>
      </ion-col>
    </ion-row>
    <ion-row>
      <ion-col
        class="container-clear"
      >
        <ion-button
          class="button-clear"
          fill="clear"
          @click="$emit('select', null)"
        >
          {{ $t('SasCodeChoice.noneOfTheChoices') }}
        </ion-button>
      </ion-col>
    </ion-row>
  </ion-grid>
</template>

<script setup lang="ts">
import {
  IonGrid,
  IonRow,
  IonCol,
  IonButton
} from '@ionic/vue';

defineProps<{
  choices: string[]
}>();

defineEmits<{
  (e: 'select', value: string | null): void
}>();
</script>

<style scoped lang="scss">
.button-choice {
  width: 8em;
  height: 4em;
  background-color: var(--parsec-color-light-primary-30);
  font-weight: bold;
  font-size: 1.3em;
  text-align: center;
}

.container-clear {
  text-align: center;
}
</style>
