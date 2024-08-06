<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="time-container">
    <ion-list class="time-list">
      <!-- add class when a time is selected -->
      <ion-text
        class="time-list-item ion-no-padding button-medium"
        v-for="time in times"
        :key="time"
        @click="onTimeClick, selectedTime = time"
        :class="{
          selected: time === selectedTime,
        }"
      >
        {{ time }}
      </ion-text>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { IonList, IonText, popoverController } from '@ionic/vue';
import { ref } from 'vue';
import { MsModalResult } from 'megashark-lib';

const selectedTime = ref('');

const props = defineProps<{
  times: string[];
}>();

const emits = defineEmits<{
  (e: 'timeSelected', time: string): void;
}>();

async function onTimeClick(): Promise<void> {
  emits('timeSelected', selectedTime.value);
  await popoverController.dismiss(props.times, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
* {
  transition: all 0.150s ease;
}

.time-container {
  display: flex;
  --background: var(--parsec-color-light-secondary-inversed-contrast);
  padding: 0.25rem;

  .time-list {
    display: flex;
    flex-wrap: wrap;
    padding: 0;

    &-item {
      display: flex;
      justify-content: center;
      color: var(--parsec-color-light-secondary-hard-grey);
      padding: 0.5rem;
      max-width: 4rem;
      width: 100%;
      border-radius: var(--parsec-radius-6);
      cursor: pointer;
      user-select: none;

      &:hover {
        background: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-primary-700);
      }

      &.selected {
        background: var(--parsec-color-light-primary-100);
        color: var(--parsec-color-light-primary-700);
      }
    }
  }
}
</style>
