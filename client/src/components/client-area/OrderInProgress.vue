<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<!-- eslint-disable vue/html-indent -->
<template>
  <div
    class="order-progress"
    :class="{ open: open }"
  >
    <template v-if="!querying && !error">
      <!-- order header -->
      <div
        class="order-header"
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
            :class="getStatus(request.status, customOrderSellsyStatus).status"
          >
            {{ $msTranslate(getStatus(request.status, customOrderSellsyStatus).name) }}
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
        v-if="open"
        class="order-content"
      >
        <div class="order-content-details">
          <ion-text class="order-content-details-header subtitles-sm">
            {{ $msTranslate('clientArea.orders.new.details') }}
          </ion-text>
          <!-- No organization created at this step, users and storage estimation -->
          <div
            class="details-list"
            v-if="!orderDetails"
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
                    data: { count: request.adminUsers },
                    count: request.adminUsers,
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
                    data: { count: request.standardUsers },
                    count: request.standardUsers,
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
                    data: { count: request.outsiderUsers },
                    count: request.outsiderUsers,
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
              <ion-text class="details-list-item__data subtitles-normal">{{ $msTranslate(getStorageOptions(request.storage)) }}</ion-text>
            </div>
          </div>

          <!-- Organization created, users and storage are known -->
          <div
            class="details-list"
            v-if="
              orderDetails &&
              [StatusStep.Confirmed, StatusStep.InvoiceToBePaid, StatusStep.Available].includes(
                getStatus(request.status, customOrderSellsyStatus).status,
              )
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

          <div
            class="details-list"
            v-if="orderDetails"
          >
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
              <ion-text
                v-if="request.formula"
                class="details-list-item__data subtitles-normal"
              >
                {{ $msTranslate(request.formula) }}
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
              v-if="getStatus(request.status, customOrderSellsyStatus).status === StatusStep.Confirmed && orderDetails"
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
              orderDetails &&
              [StatusStep.Confirmed, StatusStep.InvoiceToBePaid].includes(getStatus(request.status, customOrderSellsyStatus).status)
            "
            class="details-invoice"
          >
            <div class="details-invoice-text">
              <ion-text class="details-invoice-text__title title-h4">
                {{ $msTranslate('clientArea.orders.invoiceToBePaid.title') }}
              </ion-text>
              <ion-text class="details-invoice-text__description body">
                {{ $msTranslate('clientArea.orders.invoiceToBePaid.description') }}
              </ion-text>
            </div>
            <ion-text class="details-invoice-price">
              <span
                v-if="getStatus(request.status, customOrderSellsyStatus).status === StatusStep.Confirmed"
                class="body-lg"
              >
                {{ $msTranslate('clientArea.orders.invoiceToBePaid.waiting') }}
              </span>
              <span
                v-if="getStatus(request.status, customOrderSellsyStatus).status === StatusStep.InvoiceToBePaid"
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
          :sellsy-status="request.status"
          :bms-status="customOrderSellsyStatus"
        />
      </div>
    </template>

    <template v-if="error">
      <ion-text class="error-message">
        {{ $msTranslate(error) }}
      </ion-text>
    </template>

    <template v-if="querying">
      <div class="skeleton-list">
        <ion-skeleton-text
          :animated="true"
          class="skeleton-item"
        />
        <ion-skeleton-text
          :animated="true"
          class="skeleton-item"
        />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon, IonSkeletonText } from '@ionic/vue';
import { chatbubbleEllipses, checkmark, people, pieChart, time, wallet } from 'ionicons/icons';
import { formatFileSize } from '@/common/file';
import {
  CustomOrderRequestStatus,
  CustomOrderDetailsResultData,
  BmsOrganization,
  CustomOrderStatus,
  BmsAccessInstance,
  CustomOrderRequest,
  DataType,
} from '@/services/bms';
import { ref, onMounted } from 'vue';
import OrderTrackingProgress from '@/components/client-area/OrderTrackingProgress.vue';
import { Translatable, I18n } from 'megashark-lib';

const error = ref<string>('');
const querying = ref(true);
const orderDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);
const customOrderSellsyStatus = ref<CustomOrderStatus>(CustomOrderStatus.Unknown);
const open = ref(false);

const props = defineProps<{
  request: CustomOrderRequest;
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
  try {
    querying.value = true;

    if (!props.request.organizationId) {
      return;
    }
    let organization: BmsOrganization | undefined = undefined;
    const orgResp = await BmsAccessInstance.get().listOrganizations();
    if (!orgResp.isError && orgResp.data?.type === DataType.ListOrganizations) {
      organization = orgResp.data.organizations.find((org) => org.parsecId === props.request.organizationId);
    }
    if (!organization) {
      return;
    }
    const orderDetailsRep = await BmsAccessInstance.get().getCustomOrderDetails(organization);
    if (orderDetailsRep.isError || orderDetailsRep.data?.type !== DataType.CustomOrderDetails) {
      error.value = 'clientArea.contracts.errors.noInfo';
      return;
    }
    orderDetails.value = orderDetailsRep.data;

    const orderDetailsStatus = await BmsAccessInstance.get().getCustomOrderStatus(organization);
    if (orderDetailsStatus.isError || orderDetailsStatus.data?.type !== DataType.CustomOrderStatus) {
      error.value = 'clientArea.contracts.errors.noInfo';
      return;
    }
    customOrderSellsyStatus.value = orderDetailsStatus.data.status;
  } finally {
    querying.value = false;
  }
});

interface StepNameAndStatus {
  status: StatusStep;
  name: Translatable;
}

function getStatus(statusBms: CustomOrderRequestStatus | undefined, statusSellsy: CustomOrderStatus | undefined): StepNameAndStatus {
  switch (statusBms) {
    case CustomOrderRequestStatus.Received:
      return {
        status: StatusStep.Received,
        name: 'clientArea.dashboard.step.requestSent.tag',
      };
    case CustomOrderRequestStatus.Processing:
      return {
        status: StatusStep.Processing,
        name: 'clientArea.dashboard.step.processing.tag',
      };
    case CustomOrderRequestStatus.Standby:
      return {
        status: StatusStep.Standby,
        name: 'clientArea.dashboard.step.standby.tag',
      };
    case CustomOrderRequestStatus.Cancelled:
      return {
        status: StatusStep.Cancelled,
        name: 'clientArea.dashboard.step.cancel.tag',
      };
    case CustomOrderRequestStatus.Finished:
      if (statusSellsy === CustomOrderStatus.EstimateLinked) {
        return {
          status: StatusStep.Confirmed,
          name: 'clientArea.dashboard.step.confirmed.tag',
        };
      }
      if (statusSellsy === CustomOrderStatus.NothingLinked || statusSellsy === CustomOrderStatus.InvoiceToBePaid) {
        return {
          status: StatusStep.InvoiceToBePaid,
          name: 'clientArea.dashboard.step.invoiceToBePaid.tag',
        };
      }
      if (statusSellsy === CustomOrderStatus.InvoicePaid) {
        return {
          status: StatusStep.Available,
          name: 'clientArea.dashboard.step.organizationAvailable.tag',
        };
      }
      break;
    default:
      return {
        status: StatusStep.Unknown,
        name: I18n.valueAsTranslatable(''),
      };
  }
  return {
    status: StatusStep.Unknown,
    name: I18n.valueAsTranslatable(''),
  };
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
    .invoiceToBePaid {
      background-color: var(--parsec-color-tags-orange-50);
      color: var(--parsec-color-tags-orange-700);
    }
    .available {
      background-color: var(--parsec-color-light-success-50);
      color: var(--parsec-color-light-success-700);
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

.skeleton-list {
  display: flex;
  gap: 1rem;
  justify-content: space-between;
  padding: 1.5rem;

  .skeleton-item {
    width: 100%;
    height: 2rem;
    border-radius: var(--parsec-radius-8);

    &:nth-child(1) {
      width: 50%;
    }

    &:nth-child(2) {
      width: 10%;
    }
  }
}

.error-message {
  color: var(--parsec-color-light-danger-500);
  padding: 1rem;
  text-align: center;
}
</style>
