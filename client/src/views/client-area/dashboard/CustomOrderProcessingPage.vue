<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-processing">
    <template v-if="!querying && !error && customOrderStatus && customOrderDetail">
      <div class="process-container">
        <div
          class="process-step"
          v-for="(step, index) in steps"
          :key="index"
          :class="{
            'process-step--todo': currentStatusStep < step.status,
            'process-step--active': currentStatusStep === step.status,
            'process-step--done': currentStatusStep > step.status,
          }"
        >
          <div class="process-step-icon">
            <div
              class="dot-todo"
              v-if="currentStatusStep < step.status"
            />
            <div
              class="dot-active"
              v-if="currentStatusStep === step.status"
            />
            <ion-icon
              class="process-step-icon__item"
              :icon="checkmarkCircle"
              v-if="currentStatusStep > step.status"
            />
          </div>
          <div class="process-step-text">
            <ion-text class="process-step-text__title title-h3">
              {{ $msTranslate(step.title) }}
            </ion-text>
            <ion-text
              class="process-step-text__info body"
              v-if="currentStatusStep === step.status"
            >
              {{ $msTranslate(step.description) }}
              <a
                class="custom-button custom-button-ghost button-medium"
                :href="invoice.pdfLink"
                download
              >
                <ion-icon :icon="download" />
                {{ $msTranslate('clientArea.invoices.cell.download') }}
              </a>
            </ion-text>
          </div>
        </div>
      </div>
      <div class="process-request">
        <div class="div">
          <ion-text class="form-label">
            {{ $msTranslate('Nombre de personnes') }}
          </ion-text>
          <ion-text class="title-h1">
            {{ customOrderDetail.amountWithoutTaxes }}
          </ion-text>
        </div>
      </div>
    </template>
    <template v-else-if="querying">
      <ms-spinner />
    </template>
    <template v-else-if="error">
      <ms-report-text :theme="MsReportTheme.Error">
        {{ $msTranslate(error) }}
      </ms-report-text>
    </template>
  </div>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { BmsAccessInstance, CustomOrderStatusResultData, CustomOrderDetailsResultData,
  DataType, BmsOrganization, CustomOrderStatus } from '@/services/bms';
import { isDefaultOrganization } from '@/views/client-area/types';
import { ref, onMounted } from 'vue';
import { MsSpinner, MsReportText, MsReportTheme } from 'megashark-lib';
import { getCustomOrderStatusTranslationKey } from '@/services/translation';

const customOrderStatus = ref<CustomOrderStatusResultData>();
const customOrderDetail = ref<CustomOrderDetailsResultData>();
const querying = ref(true);
const error = ref('');

const props = defineProps<{
  organization: BmsOrganization;
}>();

enum CustomOrderStatusStep {
  NothingLinked = 0,
  EstimateLinked = 1,
  InvoiceToBePaid = 2,
  InvoicePaid = 3,
}

const currentStatusStep = ref<CustomOrderStatusStep>(CustomOrderStatusStep.EstimateLinked);

const steps = [
  {
    status: CustomOrderStatusStep.NothingLinked,
    title: getCustomOrderStatusTranslationKey(CustomOrderStatus.NothingLinked).title,
    description: getCustomOrderStatusTranslationKey(CustomOrderStatus.NothingLinked).description,
  },
  {
    status: CustomOrderStatusStep.EstimateLinked,
    title: getCustomOrderStatusTranslationKey(CustomOrderStatus.EstimateLinked).title,
    description: getCustomOrderStatusTranslationKey(CustomOrderStatus.EstimateLinked).description,
  },
  {
    status: CustomOrderStatusStep.InvoiceToBePaid,
    title: getCustomOrderStatusTranslationKey(CustomOrderStatus.InvoiceToBePaid).title,
    description: getCustomOrderStatusTranslationKey(CustomOrderStatus.InvoiceToBePaid).description,
  },
  {
    status: CustomOrderStatusStep.InvoicePaid,
    title: getCustomOrderStatusTranslationKey(CustomOrderStatus.InvoicePaid).title,
    description: getCustomOrderStatusTranslationKey(CustomOrderStatus.InvoicePaid).description,
  },
];

onMounted(async () => {
  if (isDefaultOrganization(props.organization)) {
    querying.value = false;
    return;
  }

  const orgStatusResponse = await BmsAccessInstance.get().getCustomOrderStatus(props.organization);
  const orgDetailsResponse = await BmsAccessInstance.get().getCustomOrderDetails(props.organization);

  if (!orgStatusResponse.isError && orgStatusResponse.data && orgStatusResponse.data.type === DataType.CustomOrderStatus) {
    customOrderStatus.value = orgStatusResponse.data;
  } else {
    error.value = 'clientArea.dashboard.processing.error.title';
  }

  if(!orgDetailsResponse.isError && orgDetailsResponse.data && orgDetailsResponse.data.type === DataType.CustomOrderDetails) {
    customOrderDetail.value = orgDetailsResponse.data;
  } else {
    error.value = 'clientArea.dashboard.processing.error.title';
  }

  querying.value = false;

  if (customOrderStatus.value) {
    switch (customOrderStatus.value.status) {
      case CustomOrderStatus.NothingLinked:
        currentStatusStep.value = CustomOrderStatusStep.NothingLinked;
        break;
      case CustomOrderStatus.EstimateLinked:
        currentStatusStep.value = CustomOrderStatusStep.EstimateLinked;
        break;
      case CustomOrderStatus.InvoiceToBePaid:
        currentStatusStep.value = CustomOrderStatusStep.InvoiceToBePaid;
        break;
      case CustomOrderStatus.InvoicePaid:
        currentStatusStep.value = CustomOrderStatusStep.InvoicePaid;
        break;
    }
  }
});
</script>

<style scoped lang="scss">
.client-page-processing {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

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
  box-shadow: var(--parsec-shadow-light);
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

  &:is(.process-step--done)::after {
    background-color: var(--parsec-color-light-primary-600);
  }
}
</style>
