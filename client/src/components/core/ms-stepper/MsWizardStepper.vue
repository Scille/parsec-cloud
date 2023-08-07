<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="ms-wizard-stepper">
    <div
      class="ms-wizard-stepper__step"
      v-for="(title, index) in titles"
      :key="title"
    >
      <ms-wizard-stepper-step
        :status="index < currentIndex ? MsStepStatus.DONE : index === currentIndex ? MsStepStatus.ACTIVE : MsStepStatus.DEFAULT"
      />
      <ion-text
        class="caption-caption step-title"
        :class="index < currentIndex ? MsStepStatus.DONE : ''"
      >
        {{ title }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import MsWizardStepperStep, { MsStepStatus } from '@/components/core/ms-stepper/MsWizardStepperStep.vue';
import { defineProps } from 'vue';

defineProps<{
  titles: string[]
  currentIndex: number
}>();
</script>

<!-- "setup" removed to interact with child component style (ms-wizard-stepper-step) -->
<!-- eslint-disable-next-line vue-scoped-css/enforce-style-type -->
<style lang="scss">
.ms-wizard-stepper {
  background: var(--parsec-color-light-secondary-background);
  display: flex;
  padding: 2.5rem 6rem;
  justify-content: center;

  &__step {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    width: 8.125rem;

    .step-title {
      color: var(--parsec-color-light-primary-600);
    }

    .done {
      opacity: 0.4;
    }

    &:first-of-type {
      .ms-wizard-stepper-step{
        justify-content: end;
      }
      .left-line {
        display: none;
      }
    }
    &:last-of-type {
      .ms-wizard-stepper-step{
        justify-content: start;
      };
      .right-line {
        display: none;
      }
    }
  }
}
</style>
