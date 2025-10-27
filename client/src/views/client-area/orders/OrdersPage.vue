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

    <orders-list
      v-if="!querying && !error && orderRequests"
      :orders="orderRequests"
    />

    <!-- loading -->
    <template v-if="querying">
      <div class="loading-container">
        <ms-spinner />
        <ion-text class="body-lg loading-text">{{ $msTranslate('clientArea.orders.loading') }}</ion-text>
      </div>
    </template>

    <!-- error -->
    <template v-if="!querying && error">
      <ion-text class="body-lg no-orders">
        {{ $msTranslate(error) }}
      </ion-text>
    </template>
  </div>
</template>

<script setup lang="ts">
import { OrdersList } from '@/components/client-area';
import { BmsAccessInstance, CustomOrderRequest, DataType } from '@/services/bms';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import NewOrderModal from '@/views/client-area/orders/NewOrderModal.vue';
import { IonButton, IonText, modalController } from '@ionic/vue';
import { MsModalResult, MsSpinner } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  informationManager: InformationManager;
}>();

const error = ref<string>('');
const querying = ref(true);
const orderRequests = ref<Array<CustomOrderRequest>>([]);

onMounted(async () => {
  await getCustomOrderRequests();
});

async function getCustomOrderRequests(): Promise<void> {
  querying.value = true;
  try {
    const orderRequestsRep = await BmsAccessInstance.get().getCustomOrderRequests();

    if (!orderRequestsRep.isError && orderRequestsRep.data && orderRequestsRep.data.type === DataType.GetCustomOrderRequests) {
      orderRequests.value = orderRequestsRep.data.requests.reverse();
    } else {
      error.value = 'clientArea.contracts.errors.noInfo';
    }
  } finally {
    querying.value = false;
  }
}

async function openNewOrderModal(): Promise<void> {
  const modal = await modalController.create({
    component: NewOrderModal,
    cssClass: 'new-order-modal',
  });
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role === MsModalResult.Confirm) {
    props.informationManager.present(
      new Information({
        message: 'clientArea.orders.new.sent',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await getCustomOrderRequests();
  }
}
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
