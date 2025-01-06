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
  </div>
</template>

<script></script>

<script setup lang="ts">
import { IonText, IonButton, modalController } from '@ionic/vue';
import { BmsAccessInstance, BmsOrganization, CustomOrderDetailsResultData, DataType } from '@/services/bms';
import NewOrderModal from '@/views/client-area/orders/NewOrderModal.vue';
import { ref, onMounted } from 'vue';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { MsModalResult } from 'megashark-lib';

const error = ref<string>('');
const querying = ref(true);
const contractDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);

async function openNewOrderModal(): Promise<void> {
  const modal = await modalController.create({
    component: NewOrderModal,
    cssClass: 'new-order-modal',
  });
  await modal.present();
  const { role } = await modal.onDidDismiss();
  await modal.dismiss();
  if (role === MsModalResult.Confirm) {
    await props.informationManager.present(
      new Information({
        message: 'clientArea.orders.new.sent',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
  await modal.dismiss();
}

const props = defineProps<{
  organization: BmsOrganization;
  informationManager: InformationManager;
}>();

onMounted(async () => {
  querying.value = true;
  const detailsRep = await BmsAccessInstance.get().getCustomOrderDetails(props.organization);
  if (!detailsRep.isError && detailsRep.data && detailsRep.data.type === DataType.CustomOrderDetails) {
    contractDetails.value = detailsRep.data;
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
