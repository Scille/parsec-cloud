<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="process-container">
    <ms-report-text
      v-if="orderStep === OrderStep.Standby"
      :theme="MsReportTheme.Warning"
      class="process-stop-container"
    >
      <div class="process-stop-text">
        <ion-text class="title-h4">{{ $msTranslate(getOrderStepTranslations(OrderStep.Standby).title) }}</ion-text>
        <ion-text class="body">{{ $msTranslate(getOrderStepTranslations(OrderStep.Standby).description) }}</ion-text>
      </div>
    </ms-report-text>

    <ms-report-text
      v-if="orderStep === OrderStep.Cancelled"
      :theme="MsReportTheme.Error"
      class="process-stop-container"
    >
      <div class="process-stop-text">
        <ion-text class="title-h4">{{ $msTranslate(getOrderStepTranslations(OrderStep.Cancelled).title) }}</ion-text>
        <ion-text class="body">{{ $msTranslate(getOrderStepTranslations(OrderStep.Cancelled).description) }}</ion-text>
      </div>
    </ms-report-text>
    <div
      class="process-step"
      v-for="(step, index) in orderSteps"
      :key="index"
      :class="{
        'process-step-todo': customOrderIndex < index,
        'process-step-active': customOrderIndex === index,
        'process-step-done': customOrderIndex > index,
      }"
    >
      <div class="process-step-icon">
        <div
          class="dot-todo"
          v-if="customOrderIndex < index"
        />
        <div
          class="dot-active"
          v-if="customOrderIndex === index"
        />
        <ion-icon
          class="process-step-icon__item"
          :icon="checkmarkCircle"
          v-if="customOrderIndex > index"
        />
      </div>
      <div class="process-step-text">
        <ion-text class="process-step-text__title title-h3">
          {{ $msTranslate(getOrderStepTranslations(step).title) }}
        </ion-text>
        <ion-text
          class="process-step-text__info body"
          v-if="customOrderIndex === index"
        >
          {{ $msTranslate(getOrderStepTranslations(step).description) }}
        </ion-text>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getOrderStepTranslations, OrderStep } from '@/components/client-area/orders/utils';
import { IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const customOrderIndex = ref<number>(0);

const props = defineProps<{
  orderStep: OrderStep;
}>();

const orderSteps: OrderStep[] = [
  OrderStep.Received,
  OrderStep.Processing,
  OrderStep.Confirmed,
  OrderStep.InvoiceToBePaid,
  OrderStep.Available,
];

onMounted(async () => {
  customOrderIndex.value = orderSteps.findIndex((step: OrderStep) => step === props.orderStep);
});
</script>

<style scoped lang="scss">
.process-container {
  display: flex;
  max-width: 28rem;
  width: 100%;
  margin-top: 2.5rem;
  margin-inline: auto;
  flex-direction: column;
  gap: 2.5rem;
  padding: 2.5rem;
  background-color: var(--parsec-color-light-primary-30-opacity15);
  overflow: hidden;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.process-stop-container {
  .process-stop-text {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  &.ms-warning {
    .title-h4 {
      color: var(--parsec-color-light-warning-700);
    }
  }

  &.ms-error {
    .title-h4 {
      color: var(--parsec-color-light-danger-700);
    }
  }
}

.process-step {
  display: flex;
  gap: 1.5rem;
  border-radius: var(--parsec-radius-8);
  position: relative;

  &-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.125rem;
    height: 1.125rem;
    color: var(--parsec-color-light-primary-600);
    position: relative;
    flex-shrink: 0;

    &__item {
      font-size: 1.125rem;
    }

    @keyframes pulse {
      0% {
        transform: scale(0.8);
        opacity: 0.2;
      }

      50% {
        transform: scale(2.8);
        opacity: 0.2;
      }

      100% {
        transform: scale(0.8);
        opacity: 0.2;
      }
    }

    .dot-todo {
      width: 0.5rem;
      height: 0.5rem;
      border-radius: var(--parsec-radius-circle);
      position: relative;
      background-color: var(--parsec-color-light-secondary-light);
    }

    .dot-active {
      width: 0.5rem;
      height: 0.5rem;
      border-radius: var(--parsec-radius-circle);
      position: relative;
      background-color: var(--parsec-color-light-primary-600);

      &::before {
        content: '';
        position: absolute;
        transform: translate(-50%, -50%);
        width: 100%;
        height: 100%;
        background-color: var(--parsec-color-light-primary-600);
        z-index: 0;
        border-radius: var(--parsec-radius-circle);
        animation: pulse 1.5s ease-in-out infinite;
      }
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &__title {
      color: var(--parsec-color-light-primary-600);
    }

    &__info {
      color: var(--parsec-color-light-secondary-hard-grey);
    }
  }

  &-todo {
    .process-step-text {
      .process-step-text__title {
        color: var(--parsec-color-light-secondary-light);
      }
    }
  }

  &-done {
    opacity: 0.6;
  }

  // line between steps
  &:not(:last-child)::after {
    transform-origin: 50% -100%;
    z-index: 1;
    content: '';
    position: absolute;
    top: 1.5rem;
    left: 0.44rem;
    width: 0.25rem;
    height: calc(100% + 0.5rem);
    background-color: var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-8);
  }

  &:is(.process-step-done)::after {
    background-color: var(--parsec-color-light-primary-600);
  }
}
</style>
