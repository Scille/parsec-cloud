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
              data: { number: request.label },
            })
          }}
          <span
            class="order-tag button-small"
            :class="orderStep"
          >
            {{ $msTranslate(getOrderStepTranslations(orderStep!).tag) }}
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
        <template v-if="orderStep && orderStep !== OrderStep.Unknown">
          <order-details
            :request="request"
            :order-step="orderStep"
            :organization="organization"
          />
          <order-tracking-progress
            class="order-content-tracking"
            :order-step="orderStep"
          />
        </template>
        <template v-else>
          <ion-text class="error-message">
            {{ $msTranslate(getOrderStepTranslations(OrderStep.Unknown).description) }}
          </ion-text>
        </template>
      </div>
    </template>

    <template v-if="error && !querying">
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
import OrderDetails from '@/components/client-area/orders/OrderDetails.vue';
import OrderTrackingProgress from '@/components/client-area/orders/OrderTrackingProgress.vue';
import { getOrderStep, getOrderStepTranslations, OrderStep } from '@/components/client-area/orders/utils';
import { BmsAccessInstance, BmsOrganization, CustomOrderRequest, DataType } from '@/services/bms';
import { IonSkeletonText, IonText } from '@ionic/vue';
import { I18n } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const error = ref<string>('');
const querying = ref(true);
const orderStep = ref<OrderStep | undefined>();
const organization = ref<BmsOrganization | undefined>(undefined);
const open = ref(false);

const props = defineProps<{
  request: CustomOrderRequest;
}>();

onMounted(async () => {
  try {
    querying.value = true;

    // Set the order step based on the custom order request status
    orderStep.value = getOrderStep(props.request.status);

    // Here we aim to fetch the organization details based on the request's organizationId.
    // If the organizationId is not present, we skip the fetching.
    if (!props.request.organizationId) {
      return;
    }

    // Fetch the organization details
    const orgResp = await BmsAccessInstance.get().listOrganizations();
    if (!orgResp.isError && orgResp.data?.type === DataType.ListOrganizations) {
      organization.value = orgResp.data.organizations.find((org) => org.parsecId === props.request.organizationId);
    }

    // If the organization is not found, we exit early.
    if (!organization.value) {
      return;
    }

    // Fetch the custom order status for the organization
    const customOrderStatusResp = await BmsAccessInstance.get().getCustomOrderStatus(organization.value);
    if (customOrderStatusResp.isError || customOrderStatusResp.data?.type !== DataType.CustomOrderStatus) {
      error.value = 'clientArea.contracts.errors.noInfo';
      return;
    }

    // Update the order step based on the fetched custom order status
    orderStep.value = getOrderStep(props.request.status, customOrderStatusResp.data.status);
  } finally {
    querying.value = false;
  }
});

function toggleOpen(): void {
  open.value = !open.value;
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
    .cancelled,
    .unknown {
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
