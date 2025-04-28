<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-orders">
    <!-- header -->
    <div class="orders-new">
      <div class="orders-new-title">
        <ion-text class="orders-new-title__text subtitles-normal">
          {{ $msTranslate('clientArea.orders.new.title') }}
        </ion-text>
        <ion-button
          class="orders-new-title__button"
          @click="openNewOrderModal"
        >
          {{ $msTranslate('clientArea.orders.new.button') }}
        </ion-button>
      </div>
    </div>
    {{ orderRequests }}

    <template v-if="orderRequests?.type === DataType.GetCustomOrderRequests && organization && orderRequests && !error">
      <order-in-progress
        v-for="order in orderRequests.requests"
        :key="order.id"
        :request="order"
        :organization="organization"
      />
    </template>

    <!-- loading -->
    <template v-if="querying">
      <div class="loading-container">
        <ms-spinner />
        <ion-text class="body-lg loading-text">{{ $msTranslate('clientArea.orders.loading') }}</ion-text>
      </div>
    </template>

    <!-- no orders -->
    <template v-if="!querying && !error && !orderRequests">
      <ion-text class="body-lg no-orders">
        {{ $msTranslate('clientArea.orders.noOrders') }}
      </ion-text>
    </template>

    <!-- error -->
    <template v-if="error">
      <ion-text class="body-lg no-orders">
        {{ $msTranslate(error) }}
      </ion-text>
    </template>
  </div>
</template>

<script setup lang="ts">
import { IonText, IonButton, modalController } from '@ionic/vue';
import {
  BmsAccessInstance,
  BmsOrganization,
  CustomOrderDetailsResultData,
  GetCustomOrderRequestsResultData,
  DataType,
} from '@/services/bms';
import NewOrderModal from '@/views/client-area/orders/NewOrderModal.vue';
import OrderInProgress from '@/components/client-area/OrderInProgress.vue';
import { ref, onMounted } from 'vue';
import { MsSpinner } from 'megashark-lib';

const error = ref<string>('');
const querying = ref(true);
const orderDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);
const orderRequests = ref<GetCustomOrderRequestsResultData | undefined>(undefined);

async function openNewOrderModal(): Promise<void> {
  const modal = await modalController.create({
    component: NewOrderModal,
    cssClass: 'new-order-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
}

const props = defineProps<{
  organization: BmsOrganization;
}>();

onMounted(async () => {
  querying.value = true;

  const orderRequestsRep = await BmsAccessInstance.get().getCustomOrderRequests();
  const orderDetailsRep = await BmsAccessInstance.get().getCustomOrderDetails(props.organization);

  if (!orderRequestsRep.isError && orderRequestsRep.data && orderRequestsRep.data.type === DataType.GetCustomOrderRequests) {
    orderRequests.value = orderRequestsRep.data;
    orderRequests.value.requests = orderRequestsRep.data.requests;
  } else {
    error.value = 'clientArea.contracts.errors.noInfo';
  }

  if (!orderDetailsRep.isError && orderDetailsRep.data && orderDetailsRep.data.type === DataType.CustomOrderDetails) {
    orderDetails.value = orderDetailsRep.data;
  } else {
    error.value = 'clientArea.contracts.errors.noInfo';
  }

  querying.value = false;
});
</script>

<style scoped lang="scss">
* {
  transition: all 0.2s ease;
}

.client-page-orders {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  position: relative;

  &::after {
    content: '-';
    height: 2rem;
    color: transparent;
  }

  &:has(.loading-container) {
    height: 100%;
  }
}

.loading-container {
  width: 100%;
  height: 100%;
  display: flex;
  gap: 0.5rem;
  flex-direction: column;
  justify-content: center;
  align-items: center;

  .loading-text {
    color: var(--parsec-color-light-secondary-soft-text);
  }
}

.orders-new {
  display: flex;
  align-items: center;
  flex-direction: column;
  gap: 1rem;

  &-title {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    width: 100%;
    padding: 1rem 1.5rem;
    background: var(--parsec-color-light-secondary-background);
    border-radius: var(--parsec-radius-8);

    &__button::part(native) {
      --background: var(--parsec-color-light-secondary-text);
      --background-hover: var(--parsec-color-light-secondary-contrast);
      color: var(--parsec-color-light-secondary-white);
    }

    &__text {
      color: var(--parsec-color-light-secondary-soft-text);
    }
  }
}
</style>
