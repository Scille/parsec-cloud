<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="client-page-orders">
    <!-- header -->
    <div class="orders-new">
      <div class="orders-new-title">
        <ion-text class="orders-new-title__text">
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
        <ion-text class="loading-text">{{ $msTranslate('clientArea.orders.loading') }}</ion-text>
      </div>
    </template>

    <!-- error -->
    <template v-if="!querying && error">
      <ion-text class="no-orders">
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
  height: auto;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
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
  gap: ms.spacing('gap-lg');
  flex-direction: column;
  justify-content: center;
  align-items: center;

  .loading-text {
    @include ms.font('body-lg-regular');
    color: ms.color('text-neutral-default');
  }
}

.orders-new {
  display: flex;
  align-items: center;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');

  &-title {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: ms.spacing('gap-3xl');
    width: 100%;
    padding: ms.spacing('padding-3xl') ms.spacing('padding-4xl');
    border-radius: ms.radius('lg');

    &__button::part(native) {
      --background: #{ms.color('surface-neutral-default')};
      --background-hover: #{ms.color('surface-neutral-default-hover')};
      color: ms.color('text-neutral-on-color');
    }

    &__text {
      @include ms.font('body-lg-medium');
      color: ms.color('text-neutral-default');
    }
  }
}

.no-orders {
  @include ms.font('body-lg-regular');
}
</style>
