<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-grid>
    <ion-row>
      <ion-col class="column">
        <ion-button
          class="button-choice caption-code"
          fill="outline"
          @click="$emit('select', choices[0])"
        >
          {{ choices[0] }}
        </ion-button>
      </ion-col>
      <ion-col class="column">
        <ion-button
          class="button-choice caption-code"
          fill="outline"
          @click="$emit('select', choices[1])"
        >
          {{ choices[1] }}
        </ion-button>
      </ion-col>
    </ion-row>
    <ion-row>
      <ion-col class="column">
        <ion-button
          class="button-choice caption-code"
          fill="outline"
          @click="$emit('select', choices[2])"
        >
          {{ choices[2] }}
        </ion-button>
      </ion-col>
      <ion-col class="column">
        <ion-button
          class="button-choice caption-code"
          fill="outline"
          @click="$emit('select', choices[3])"
        >
          {{ choices[3] }}
        </ion-button>
      </ion-col>
    </ion-row>
    <ion-row>
      <ion-col class="container-clear">
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
import { IonGrid, IonRow, IonCol, IonButton } from '@ionic/vue';

defineProps<{
  choices: string[];
}>();

defineEmits<{
  (e: 'select', value: string | null): void;
}>();
</script>

<style scoped lang="scss">
.column {
  padding: 0;
}

.button-choice {
  width: 8em;
  height: 4em;
  background-color: var(--parsec-color-light-primary-30);
  margin: 0.75rem;
  border-radius: var(--parsec-radius-4);

  &::part(native) {
    border: 2px solid var(--parsec-color-light-primary-200);
  }

  &:hover {
    background-color: var(--parsec-color-light-primary-100);

    &::part(native) {
      border-color: var(--parsec-color-light-primary-600);
    }
  }
}

.container-clear {
  text-align: center;
}
</style>
