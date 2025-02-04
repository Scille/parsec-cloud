<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <template
    v-if="!querying && !error && customOrderBmsStatus && customOrderBmsStatus !== CustomOrderStatus.Unknown && customOrderSellsyStatus"
  >
    <div class="process-container">
      <div
        class="process-step"
        v-for="(step, index) in steps"
        :key="index"
        :class="{
          'process-step-todo': getStep(customOrderBmsStatus, customOrderSellsyStatus) < getStep(step.statusBms, step.statusRequest),
          'process-step-active': getStep(customOrderBmsStatus, customOrderSellsyStatus) === getStep(step.statusBms, step.statusRequest),
          'process-step-done': getStep(customOrderBmsStatus, customOrderSellsyStatus) > getStep(step.statusBms, step.statusRequest),
        }"
      >
        <div class="process-step-icon">
          <div
            class="dot-todo"
            v-if="getStep(customOrderBmsStatus, customOrderSellsyStatus) < getStep(step.statusBms, step.statusRequest)"
          />
          <div
            class="dot-active"
            v-if="getStep(customOrderBmsStatus, customOrderSellsyStatus) === getStep(step.statusBms, step.statusRequest)"
          />
          <ion-icon
            class="process-step-icon__item"
            :icon="checkmarkCircle"
            v-if="getStep(customOrderBmsStatus, customOrderSellsyStatus) > getStep(step.statusBms, step.statusRequest)"
          />
        </div>
        <div class="process-step-text">
          <ion-text class="process-step-text__title title-h3">
            {{ $msTranslate(step.title) }}
          </ion-text>
          <ion-text
            class="process-step-text__info body"
            v-if="getStep(customOrderBmsStatus, customOrderSellsyStatus) === getStep(step.statusBms, step.statusRequest)"
          >
            {{ $msTranslate(step.description) }}
          </ion-text>
        </div>
      </div>
    </div>
  </template>
  <template v-else-if="querying">
    <ms-spinner />
  </template>
  <template v-else-if="error || customOrderBmsStatus === CustomOrderStatus.Unknown">
    <ms-report-text :theme="MsReportTheme.Error">
      {{ $msTranslate(error) }}
    </ms-report-text>
  </template>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { BmsAccessInstance, DataType, BmsOrganization, CustomOrderStatus, CustomOrderRequestStatus } from '@/services/bms';
import { isDefaultOrganization } from '@/views/client-area/types';
import { ref, onMounted } from 'vue';
import { MsSpinner, MsReportText, MsReportTheme } from 'megashark-lib';
import { getCustomOrderStatusTranslationKey } from '@/services/translation';

const customOrderBmsStatus = ref<CustomOrderStatus>(CustomOrderStatus.Unknown);
const customOrderSellsyStatus = ref<CustomOrderRequestStatus>(CustomOrderRequestStatus.Received);

const querying = ref(true);
const error = ref('');

const props = defineProps<{
  organization: BmsOrganization;
}>();

const steps = [
  {
    id: 1,
    statusBms: CustomOrderStatus.NothingLinked,
    statusRequest: CustomOrderRequestStatus.Received,
    title: getCustomOrderStatusTranslationKey(CustomOrderStatus.NothingLinked, CustomOrderRequestStatus.Received).title,
    description: getCustomOrderStatusTranslationKey(CustomOrderStatus.NothingLinked, CustomOrderRequestStatus.Received).description,
  },
  {
    id: 2,
    statusBms: CustomOrderStatus.NothingLinked,
    statusRequest: CustomOrderRequestStatus.Processing,
    title: getCustomOrderStatusTranslationKey(CustomOrderStatus.NothingLinked, CustomOrderRequestStatus.Processing).title,
    description: getCustomOrderStatusTranslationKey(CustomOrderStatus.NothingLinked, CustomOrderRequestStatus.Processing).description,
  },
  {
    id: 3,
    statusBms: CustomOrderStatus.NothingLinked,
    statusRequest: CustomOrderRequestStatus.Finished,
    title: getCustomOrderStatusTranslationKey(CustomOrderStatus.NothingLinked, CustomOrderRequestStatus.Finished).title,
    description: getCustomOrderStatusTranslationKey(CustomOrderStatus.NothingLinked, CustomOrderRequestStatus.Finished).description,
  },
  {
    id: 4,
    statusBms: CustomOrderStatus.InvoiceToBePaid,
    statusRequest: CustomOrderRequestStatus.Finished,
    title: getCustomOrderStatusTranslationKey(CustomOrderStatus.InvoiceToBePaid, CustomOrderRequestStatus.Finished).title,
    description: getCustomOrderStatusTranslationKey(CustomOrderStatus.InvoiceToBePaid, CustomOrderRequestStatus.Finished).description,
  },
  {
    id: 5,
    statusBms: CustomOrderStatus.InvoicePaid,
    statusRequest: CustomOrderRequestStatus.Finished,
    title: getCustomOrderStatusTranslationKey(CustomOrderStatus.InvoicePaid, CustomOrderRequestStatus.Finished).title,
    description: getCustomOrderStatusTranslationKey(CustomOrderStatus.InvoicePaid, CustomOrderRequestStatus.Finished).description,
  },
];

const CustomOrderStatusSteps = {
  [`${CustomOrderStatus.NothingLinked}-${CustomOrderStatus.Unknown}`]: 0,
  [`${CustomOrderStatus.NothingLinked}-${CustomOrderRequestStatus.Received}`]: 1,
  [`${CustomOrderStatus.NothingLinked}-${CustomOrderRequestStatus.Processing}`]: 2,
  [`${CustomOrderStatus.NothingLinked}-${CustomOrderRequestStatus.Finished}`]: 3,
  [`${CustomOrderStatus.InvoiceToBePaid}-${CustomOrderRequestStatus.Finished}`]: 4,
  [`${CustomOrderStatus.InvoicePaid}-${CustomOrderRequestStatus.Finished}`]: 5,
  [`${CustomOrderStatus.ContractEnded}-${CustomOrderRequestStatus.Finished}`]: 6,
};

function getStep(customOrderStatus: CustomOrderStatus, customOrderRequestStatus: CustomOrderRequestStatus): number {
  const key = `${customOrderStatus}-${customOrderRequestStatus}`;
  console.log(key);
  return CustomOrderStatusSteps[key as keyof typeof CustomOrderStatusSteps] ?? 0;
}

onMounted(async () => {
  if (isDefaultOrganization(props.organization)) {
    querying.value = true;
    return;
  }

  const orgStatusResponse = await BmsAccessInstance.get().getCustomOrderStatus(props.organization);

  if (!orgStatusResponse.isError && orgStatusResponse.data && orgStatusResponse.data.type === DataType.CustomOrderStatus) {
    customOrderBmsStatus.value = orgStatusResponse.data.status;
  } else {
    error.value = 'clientArea.dashboard.processing.error.title';
  }

  querying.value = false;
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
  border-radius: var(--parsec-radius-12);
  background-color: var(--parsec-color-light-primary-30-opacity15);
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
      background-color: var(--parsec-color-light-primary-600);
      border-radius: 50%;
      position: relative;
      background-color: var(--parsec-color-light-secondary-light);
    }

    .dot-active {
      width: 0.5rem;
      height: 0.5rem;
      background-color: var(--parsec-color-light-primary-600);
      border-radius: 50%;
      position: relative;
      background-color: var(--parsec-color-light-primary-600);

      &::before {
        content: '';
        position: absolute;
        transform: translate(-50%, -50%);
        width: 100%;
        height: 100%;
        background-color: var(--parsec-color-light-primary-600);
        z-index: -2;
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
    z-index: -1;
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
