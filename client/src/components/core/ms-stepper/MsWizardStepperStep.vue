<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="ms-wizard-stepper-step">
    <!-- default -->
    <div
      class="shape"
      :class="getClass(status)"
    >
      <div class="left-line" />
      <div class="circle">
        <div
          v-if="status === 'done'"
          class="inner-circle-done"
        />
        <div
          v-if="status === 'active'"
          class="inner-circle-active"
        />
        <ion-icon
          v-if="status === 'done'"
          class="icon-checkmark"
          :icon="checkmark"
          size="default"
        />
      </div>
      <div class="right-line" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { MsStepStatus } from '@/components/core/ms-stepper/types';
import { IonIcon } from '@ionic/vue';
import { checkmark } from 'ionicons/icons';

defineProps<{
  status: MsStepStatus;
}>();

function getClass(status: MsStepStatus): string {
  if (status === 'default') {
    return 'default';
  } else if (status === 'done') {
    return 'done';
  } else if (status === 'active') {
    return 'active';
  }
  return '';
}
</script>

<style scoped lang="scss">
.ms-wizard-stepper-step {
  width: 100%;
  display: flex;
}

.shape {
  display: flex;
  flex-direction: row;
  align-items: center;
  .left-line,
  .right-line {
    background: var(--parsec-color-light-primary-600);
    width: 53px; //change to rem
    height: 2px;
  }
  .circle {
    background: var(--parsec-color-light-secondary-background);
    width: 24px;
    height: 20px;
    border-radius: var(--parsec-radius-32);
    border: 2px solid var(--parsec-color-light-primary-600);
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
  }
}

.done {
  opacity: 0.4;
  .circle {
    background: var(--parsec-color-light-primary-600);
    .icon-checkmark {
      color: white;
    }
  }
}

.active {
  .left-line {
    opacity: 0.4;
  }
  .circle {
    background: var(--parsec-color-light-secondary-background);
    .inner-circle-active {
      background: var(--parsec-color-light-primary-600);
      width: 0.75rem;
      height: 0.5rem;
      border-radius: var(--parsec-radius-32);
      position: absolute;
    }
  }
}
</style>
