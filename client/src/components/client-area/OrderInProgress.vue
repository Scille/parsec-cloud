<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<!-- eslint-disable vue/html-indent -->
<template>
  <div
    class="order-progress"
    :class="{ open: open }"
  >
    <!-- order header -->
    <div
      class="order-header"
      v-if="customOrderBmsStatus"
      @click="toggleOpen"
    >
      <ion-text class="order-header__title title-h3">
        {{
          $msTranslate({
            key: 'clientArea.orders.progress.orderNumber',
            data: { number: request.id },
          })
        }}
        <span
          class="order-tag button-small"
          :class="getStatus(customOrderBmsStatus, customOrderSellsyStatus)?.stepName"
        >
          {{ $msTranslate(getStatus(customOrderBmsStatus, customOrderSellsyStatus)?.status) }}
        </span>
      </ion-text>
      <ion-text class="order-header__date subtitles-normal">
        <span class="date-title body-sm">
          {{ $msTranslate('clientArea.orders.new.date') }}
        </span>
        {{ $msTranslate(I18n.formatDate(request.orderDate, 'short')) }}
      </ion-text>
    </div>

    <!-- order details -->
    <div
      v-if="open && orderDetails?.type === DataType.CustomOrderDetails"
      class="order-content"
    >
      <div class="order-content-details">
        <ion-text class="order-content-details-header subtitles-sm">
          {{ $msTranslate('clientArea.orders.new.details') }}
        </ion-text>
        <!-- No organization created at this step, users and storage estimation -->
        <div
          class="details-list"
          v-if="getStatus(customOrderBmsStatus, customOrderSellsyStatus)?.stepName === (StatusStep.Received || StatusStep.Processing)"
        >
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="people" />
              {{ $msTranslate('clientArea.orders.new.usersNeed') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">{{ $msTranslate(getUserOptions(request.users)) }}</ion-text>
          </div>
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="pieChart" />
              {{ $msTranslate('clientArea.orders.new.dataNeeded') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">{{ $msTranslate(getStorageOptions(request.storage)) }}</ion-text>
          </div>
        </div>

        <!-- Organization created, users and storage are known -->
        <div
          class="details-list"
          v-if="
            getStatus(customOrderBmsStatus, customOrderSellsyStatus)?.stepName === (StatusStep.Confirmed || StatusStep.Available) &&
            orderDetails
          "
        >
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="people" />
              {{ $msTranslate('clientArea.orders.new.usersNeed') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">
              {{
                $msTranslate({
                  key: 'clientArea.orders.quantityOrdered.administrator',
                  data: { count: orderDetails.administrators.quantityOrdered },
                  count: orderDetails.administrators.quantityOrdered,
                })
              }}
              <ion-icon
                :icon="checkmark"
                class="checkmark-icon"
              />
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">
              {{
                $msTranslate({
                  key: 'clientArea.orders.quantityOrdered.standard',
                  data: { count: orderDetails.standards.quantityOrdered },
                  count: orderDetails.standards.quantityOrdered,
                })
              }}
              <ion-icon
                :icon="checkmark"
                class="checkmark-icon"
              />
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">
              {{
                $msTranslate({
                  key: 'clientArea.orders.quantityOrdered.outsider',
                  data: { count: orderDetails.outsiders.quantityOrdered },
                  count: orderDetails.outsiders.quantityOrdered,
                })
              }}
              <ion-icon
                :icon="checkmark"
                class="checkmark-icon"
              />
            </ion-text>
          </div>
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="pieChart" />
              {{ $msTranslate('clientArea.orders.new.dataNeeded') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">
              {{ $msTranslate(formatFileSize(orderDetails.storage.amountWithTaxes)) }}

              <ion-icon
                :icon="checkmark"
                class="checkmark-icon"
              />
            </ion-text>
          </div>
        </div>

        <div class="details-list">
          <!-- comment -->
          <div
            v-if="request.status === CustomOrderRequestStatus.Received || request.status === CustomOrderRequestStatus.Processing"
            class="details-list-item"
            id="detail-comment"
          >
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="chatbubbleEllipses" />
              {{ $msTranslate('clientArea.orders.new.comment') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">
              {{ $msTranslate(request.comment) }}
            </ion-text>
          </div>
          <!-- Starting date -->
          <div
            class="details-list-item"
            v-if="request.organizationId"
          >
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="time" />
              {{ $msTranslate('clientArea.orders.new.startDate') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">
              {{ $msTranslate(I18n.formatDate(orderDetails.created, 'short')) }}
              <ion-icon
                :icon="checkmark"
                class="checkmark-icon"
              />
            </ion-text>
          </div>
          <!-- Ending date -->
          <div
            class="details-list-item"
            v-if="request.organizationId"
          >
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="time" />
              {{ $msTranslate('clientArea.orders.new.endDate') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">
              {{ $msTranslate(I18n.formatDate(orderDetails.dueDate, 'short')) }}
              <ion-icon
                :icon="checkmark"
                class="checkmark-icon"
              />
            </ion-text>
          </div>
        </div>

        <div class="details-list">
          <!-- comment -->
          <div
            v-if="getStatus(customOrderBmsStatus, customOrderSellsyStatus)?.stepName === StatusStep.Confirmed && orderDetails"
            class="details-list-item"
          >
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="wallet" />
              {{ $msTranslate('clientArea.orders.invoiceToBePaid.paid') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">
              {{
                $msTranslate({
                  key: 'clientArea.orders.invoiceToBePaid.price',
                  data: { amount: orderDetails.amountDue },
                })
              }}
              <ion-icon
                :icon="checkmark"
                class="checkmark-icon"
              />
            </ion-text>
          </div>
        </div>

        <!-- invoice details -->
        <div
          v-if="
            getStatus(customOrderBmsStatus, customOrderSellsyStatus)?.stepName === (StatusStep.Confirmed || StatusStep.InvoiceToBePaid) &&
            orderDetails
          "
          class="details-invoice"
        >
          <div class="details-invoice-text">
            <ion-text class="details-invoice-text__title title-h4">{{ $msTranslate('clientArea.orders.invoiceToBePaid.title') }}</ion-text>
            <ion-text class="details-invoice-text__description body">
              {{ $msTranslate('clientArea.orders.invoiceToBePaid.description') }}
            </ion-text>
          </div>
          <ion-text class="details-invoice-price">
            <span
              v-if="getStatus(customOrderBmsStatus, customOrderSellsyStatus)?.stepName === StatusStep.Confirmed"
              class="body-lg"
            >
              {{ $msTranslate('clientArea.orders.invoiceToBePaid.waiting') }}
            </span>
            <span
              v-if="getStatus(customOrderBmsStatus, customOrderSellsyStatus)?.stepName === StatusStep.InvoiceToBePaid"
              class="title-h4"
            >
              {{
                $msTranslate({
                  key: 'clientArea.orders.invoiceToBePaid.price',
                  data: { amount: orderDetails.amountDue },
                })
              }}
            </span>
          </ion-text>
        </div>
      </div>
      <order-tracking-progress
        class="order-content-tracking"
        v-if="orderRequests"
        :sellsy-status="customOrderBmsStatus"
        :bms-status="customOrderSellsyStatus"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon } from '@ionic/vue';
import { chatbubbleEllipses, checkmark, people, pieChart, time, wallet } from 'ionicons/icons';
import { formatFileSize } from '@/common/file';
import {
  CustomOrderRequestStatus,
  CustomOrderDetailsResultData,
  GetCustomOrderRequestsResultData,
  BmsOrganization,
  CustomOrderStatus,
  BmsAccessInstance,
  CustomOrderRequest,
  DataType,
} from '@/services/bms';
import { ref, onMounted } from 'vue';
import { isDefaultOrganization } from '@/views/client-area/types';
import OrderTrackingProgress from '@/components/client-area/OrderTrackingProgress.vue';
import { Translatable, I18n } from 'megashark-lib';

const error = ref<string>('');
const querying = ref(true);
const orderRequests = ref<GetCustomOrderRequestsResultData | undefined>(undefined);
const customOrderBmsStatus = ref<CustomOrderRequestStatus>(CustomOrderRequestStatus.Received);
const orderDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);
const customOrderSellsyStatus = ref<CustomOrderStatus>(CustomOrderStatus.Unknown);
const open = ref(false);

const props = defineProps<{
  request: CustomOrderRequest;
  organization?: BmsOrganization;
}>();

enum StatusStep {
  Received = 'received',
  Processing = 'processing',
  Confirmed = 'confirmed',
  InvoiceToBePaid = 'invoiceToBePaid',
  Available = 'available',
  Standby = 'standby',
  Cancelled = 'cancelled',
  Unknown = 'unknown',
}

function getUserOptions(user: number): Translatable {
  switch (user) {
    case 50:
      return 'clientArea.orders.request.userNeeds.choices.50';
    case 100:
      return 'clientArea.orders.request.userNeeds.choices.100';
    case 300:
      return 'clientArea.orders.request.userNeeds.choices.300';
    case 9999:
      return 'clientArea.orders.request.userNeeds.choices.more';
    default:
      return '';
  }
}

function getStorageOptions(storage: number): Translatable {
  switch (storage) {
    case 100:
      return 'clientArea.orders.request.storageNeeds.choices.100';
    case 500:
      return 'clientArea.orders.request.storageNeeds.choices.500';
    case 1000:
      return 'clientArea.orders.request.storageNeeds.choices.1000';
    case 9999:
      return 'clientArea.orders.request.storageNeeds.choices.more';
    default:
      return '';
  }
}

onMounted(async () => {
  if (props.organization) {
    if (isDefaultOrganization(props.organization)) {
      querying.value = true;
      return;
    }
  }

  const orderRequestsRep = await BmsAccessInstance.get().getCustomOrderRequests();
  if (!orderRequestsRep.isError && orderRequestsRep.data && orderRequestsRep.data.type === DataType.GetCustomOrderRequests) {
    orderRequests.value = orderRequestsRep.data;

    const request = orderRequestsRep.data.requests.find((req) => req.id === props.request.id);
    if (request) {
      customOrderBmsStatus.value = request.status;
    }
  } else {
    error.value = 'clientArea.dashboard.processing.error.title';
  }

  if (props.organization) {
    const orderDetailsRep = await BmsAccessInstance.get().getCustomOrderDetails(props.organization);
    if (!orderDetailsRep.isError && orderDetailsRep.data && orderDetailsRep.data.type === DataType.CustomOrderDetails) {
      orderDetails.value = orderDetailsRep.data;
    } else {
      error.value = 'clientArea.contracts.errors.noInfo';
    }

    const orderDetailsStatus = await BmsAccessInstance.get().getCustomOrderStatus(props.organization);
    if (!orderDetailsStatus.isError && orderDetailsStatus.data && orderDetailsStatus.data.type === DataType.CustomOrderStatus) {
      customOrderSellsyStatus.value = orderDetailsStatus.data.status;
    } else {
      error.value = 'clientArea.contracts.errors.noInfo';
    }
  }

  querying.value = false;
});

interface StepNameAndStatus {
  stepName: StatusStep;
  status: Translatable;
}

function getStatus(
  statusBms?: CustomOrderRequestStatus | undefined,
  statusSellsy?: CustomOrderStatus | undefined,
): StepNameAndStatus | undefined {
  switch (statusBms) {
    case CustomOrderRequestStatus.Received:
      return {
        stepName: StatusStep.Received,
        status: 'clientArea.dashboard.step.requestSent.tag',
      };
    case CustomOrderRequestStatus.Processing:
      return {
        stepName: StatusStep.Processing,
        status: 'clientArea.dashboard.step.processing.tag',
      };
    case CustomOrderRequestStatus.Standby:
      return {
        stepName: StatusStep.Standby,
        status: 'clientArea.dashboard.step.standby.tag',
      };
    case CustomOrderRequestStatus.Cancelled:
      return {
        stepName: StatusStep.Cancelled,
        status: 'clientArea.dashboard.step.cancel.tag',
      };
    case CustomOrderRequestStatus.Finished:
      if (statusSellsy === CustomOrderStatus.EstimateLinked) {
        return {
          stepName: StatusStep.Confirmed,
          status: 'clientArea.dashboard.step.confirmed.tag',
        };
      }
      if (statusSellsy === CustomOrderStatus.NothingLinked || statusSellsy === CustomOrderStatus.InvoiceToBePaid) {
        return {
          stepName: StatusStep.InvoiceToBePaid,
          status: 'clientArea.dashboard.step.invoiceToBePaid.tag',
        };
      }
      if (statusSellsy === CustomOrderStatus.InvoicePaid) {
        return {
          stepName: StatusStep.Available,
          status: 'clientArea.dashboard.step.organizationAvailable.tag',
        };
      }
      break;
    default:
      return {
        stepName: StatusStep.Unknown,
        status: '',
      };
  }
}

function toggleOpen(): boolean {
  return (open.value = !open.value);
}
</script>

<style scoped lang="scss">
.order-progress {
  display: flex;
  flex-direction: column;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-inversed-contrast);
  outline: 1px solid var(--parsec-color-light-secondary-premiere);
  width: 100%;
  max-width: 80rem;

  &:hover {
    outline: 1px solid var(--parsec-color-light-secondary-light);
  }

  &.open {
    box-shadow: var(--parsec-shadow-light);
    outline-color: transparent;

    &:has(.order-header:hover) {
      outline: 1px solid var(--parsec-color-light-secondary-light);
    }
  }
}

.order-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  cursor: pointer;

  &__title {
    display: flex;
    align-items: center;
    color: var(--parsec-color-light-secondary-text);
    gap: 1rem;

    .order-tag {
      color: var(--parsec-color-tags-indigo-500);
      padding: 0.15rem 0.5rem;
      border-radius: var(--parsec-radius-32);
    }

    .received {
      background-color: var(--parsec-color-tags-indigo-50);
      color: var(--parsec-color-tags-indigo-700);
    }
    .processing {
      background-color: var(--parsec-color-light-secondary-premiere);
      color: var(--parsec-color-light-secondary-text);
    }
    .confirmed {
      background-color: var(--parsec-color-tags-blue-50);
      color: var(--parsec-color-tags-blue-700);
    }
    .invoice {
      background-color: var(--parsec-color-tags-orange-50);
      color: var(--parsec-color-tags-orange-700);
    }
    .available {
      background-color: var(--parsec-color-light-success-50);
      color: var(--parsec-color-light-success-500);
    }
    .standby {
      background-color: var(--parsec-color-tags-orange-50);
      color: var(--parsec-color-tags-orange-700);
    }
    .cancelled {
      background-color: var(--parsec-color-light-danger-50);
      color: var(--parsec-color-light-danger-500);
    }
  }

  &__date {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    text-align: end;

    color: var(--parsec-color-light-secondary-text);

    .date-title {
      color: var(--parsec-color-light-secondary-hard-grey);
    }
  }
}

.order-content {
  border-top: 1px solid var(--parsec-color-light-secondary-medium);
  display: flex;
  justify-content: space-between;

  &-details {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 28rem;
    margin: 2.5rem;
    width: 100%;

    &-header {
      color: var(--parsec-color-light-secondary-grey);
    }

    .details-list {
      display: flex;
      flex-direction: column;
      gap: 1rem;

      &-item {
        display: flex;
        align-items: center;
        gap: 1rem;
        justify-content: space-between;
        flex-wrap: wrap;

        &__title {
          display: flex;
          align-items: center;
          gap: 0.5rem;
          color: var(--parsec-color-light-secondary-hard-grey);
        }

        &__data {
          color: var(--parsec-color-light-secondary-soft-text);
          padding: 0.5rem 0.75rem;
          background: var(--parsec-color-light-secondary-premiere);
          border-radius: var(--parsec-radius-8);
          max-width: 15rem;
          margin-left: auto;
          text-align: end;
          width: 100%;
          position: relative;

          .checkmark-icon {
            position: absolute;
            right: -1.5rem;
            color: var(--parsec-color-light-primary-700);
            margin-left: 0.5rem;
          }
        }

        &#detail-comment {
          flex-direction: column;
          align-items: flex-start;

          .details-list-item__data {
            max-width: 100%;
            padding: 1rem;
            text-align: start;
          }
        }

        ion-icon {
          flex-shrink: 0;
        }
      }
    }

    .details-invoice {
      display: flex;
      justify-content: space-between;
      gap: 1rem;
      margin-top: 0.5rem;
      border-top: 1px solid var(--parsec-color-light-secondary-medium);
      padding-top: 1.5rem;

      &-text {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;

        &__title {
          color: var(--parsec-color-light-secondary-text);
        }

        &__description {
          color: var(--parsec-color-light-secondary-grey);
        }
      }

      &-price {
        color: var(--parsec-color-light-secondary-soft-text);
        text-align: end;
      }
    }
  }

  &-tracking {
    width: fit-content;
    margin: 0;
    border-left: 1px solid var(--parsec-color-light-secondary-medium);
  }
}
</style>
