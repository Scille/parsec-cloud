<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="process-container">
    <ms-report-text
      v-if="orderStep === OrderStep.Standby"
      :theme="MsReportTheme.Warning"
      class="process-stop-container"
    >
      <div class="process-stop-text">
        <ion-text class="process-stop-text__title">{{ $msTranslate(getOrderStepTranslations(OrderStep.Standby).title) }}</ion-text>
        <ion-text class="process-stop-text__description">
          {{ $msTranslate(getOrderStepTranslations(OrderStep.Standby).description) }}
        </ion-text>
      </div>
    </ms-report-text>

    <ms-report-text
      v-if="orderStep === OrderStep.Cancelled"
      :theme="MsReportTheme.Error"
      class="process-stop-container"
    >
      <div class="process-stop-text">
        <ion-text class="process-stop-text__title">{{ $msTranslate(getOrderStepTranslations(OrderStep.Cancelled).title) }}</ion-text>
        <ion-text class="process-stop-text__description">
          {{ $msTranslate(getOrderStepTranslations(OrderStep.Cancelled).description) }}
        </ion-text>
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
        <ion-text class="process-step-text__title">
          {{ $msTranslate(getOrderStepTranslations(step).title) }}
        </ion-text>
        <ion-text
          class="process-step-text__info"
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
  gap: ms.spacing('gap-6xl');
  padding: ms.spacing('padding-6xl');
  background-color: ms.color('surface-base-page');
  overflow: hidden;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.process-stop-container {
  .process-stop-text {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');

    &__title {
      @include ms.font('heading-h5');
    }

    &__description {
      @include ms.font('body-md-regular');
    }
  }

  &.ms-warning {
    .process-stop-text__title {
      color: ms.color('text-warning-default');
    }
  }

  &.ms-error {
    .process-stop-text__title {
      color: ms.color('text-error-default');
    }
  }
}

.process-step {
  display: flex;
  gap: ms.spacing('gap-4xl');
  border-radius: ms.radius('lg');
  position: relative;

  &-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.125rem;
    height: 1.125rem;
    color: ms.color('icon-brand-default');
    position: relative;
    flex-shrink: 0;

    &__item {
      font-size: 1.125rem;
    }

    @keyframes pulse {
      0% {
        transform: scale(0.8);
        opacity: ms.opacity('2');
      }

      50% {
        transform: scale(2.8);
        opacity: ms.opacity('2');
      }

      100% {
        transform: scale(0.8);
        opacity: ms.opacity('2');
      }
    }

    .dot-todo {
      width: 0.5rem;
      height: 0.5rem;
      border-radius: ms.radius('full');
      position: relative;
      background-color: ms.color('surface-disabled-default');
    }

    .dot-active {
      width: 0.5rem;
      height: 0.5rem;
      border-radius: ms.radius('full');
      position: relative;
      background-color: ms.color('surface-brand-default');

      &::before {
        content: '';
        position: absolute;
        transform: translate(-50%, -50%);
        width: 100%;
        height: 100%;
        background-color: ms.color('surface-brand-default');
        z-index: 0;
        border-radius: ms.radius('full');
        animation: pulse 1.5s ease-in-out infinite;
      }
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');

    &__title {
      @include ms.font('heading-h4');
      color: ms.color('text-brand-default');
    }

    &__info {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');
    }
  }

  &-todo {
    .process-step-text {
      .process-step-text__title {
        color: ms.color('text-disabled-default');
      }
    }
  }

  &-done {
    opacity: ms.opacity('6');
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
    background-color: ms.color('surface-disabled-default');
    border-radius: ms.radius('lg');
  }

  &:is(.process-step-done)::after {
    background-color: ms.color('surface-brand-default');
  }
}
</style>
