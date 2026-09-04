<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
        <ion-text class="order-header__title">
          {{
            $msTranslate({
              key: 'clientArea.orders.progress.orderNumber',
              data: { number: request.label },
            })
          }}
          <span
            class="order-tag"
            :class="orderStep"
          >
            {{ $msTranslate(getOrderStepTranslations(orderStep!).tag) }}
          </span>
        </ion-text>
        <ion-text class="order-header__date">
          <span class="date-title">
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
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-default');
  outline: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  width: 100%;
  max-width: 80rem;
  overflow: hidden;
  height: fit-content;

  &:hover {
    outline: ms.border('thin') solid ms.color('border-base-default-hover');
  }

  &.open {
    box-shadow: ms.shadow('elevation-sm');
    outline-color: transparent;

    &:has(.order-header:hover) {
      outline: ms.border('thin') solid ms.color('border-base-default-hover');
    }
  }
}

.order-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: ms.spacing('padding-3xl') ms.spacing('padding-4xl');
  cursor: pointer;

  &__title {
    @include ms.font('heading-h4');
    display: flex;
    align-items: center;
    color: ms.color('text-base-body');
    gap: ms.spacing('gap-3xl');

    .order-tag {
      @include ms.font('label-sm-medium');
      color: ms.color('text-extra-purple-default');
      padding: ms.spacing('padding-xs') ms.spacing('padding-lg');
      border-radius: ms.radius('6xl');
    }

    .received {
      background-color: ms.color('surface-extra-purple-default-subtle');
      color: ms.color('text-extra-purple-default');
    }
    .processing {
      background-color: ms.color('surface-base-default-secondary');
      color: ms.color('text-base-body');
    }
    .confirmed {
      background-color: ms.color('surface-information-default-subtle');
      color: ms.color('text-information-default');
    }
    .invoiceToBePaid {
      background-color: ms.color('surface-extra-orange-default-subtle');
      color: ms.color('text-extra-orange-default');
    }
    .available {
      background-color: ms.color('surface-success-default-subtle');
      color: ms.color('text-success-default');
    }
    .standby {
      background-color: ms.color('surface-extra-orange-default-subtle');
      color: ms.color('text-extra-orange-default');
    }
    .cancelled,
    .unknown {
      background-color: ms.color('surface-error-default-subtle');
      color: ms.color('text-error-default');
    }
  }

  &__date {
    @include ms.font('body-lg-medium');
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');
    text-align: end;

    color: ms.color('text-base-body');

    .date-title {
      @include ms.font('body-sm-regular');
      color: ms.color('text-base-description');
    }
  }
}

.order-content {
  border-top: ms.border('thin') solid ms.color('border-base-default');
  display: flex;
  justify-content: space-between;

  &-tracking {
    width: fit-content;
    margin: 0;
  }
}

.skeleton-list {
  display: flex;
  gap: ms.spacing('gap-3xl');
  justify-content: space-between;
  padding: ms.spacing('padding-4xl');

  .skeleton-item {
    width: 100%;
    height: 2rem;
    border-radius: ms.radius('lg');

    &:nth-child(1) {
      width: 50%;
    }

    &:nth-child(2) {
      width: 10%;
    }
  }
}

.error-message {
  color: ms.color('text-error-default');
  padding: ms.spacing('padding-3xl');
  text-align: center;
}
</style>
