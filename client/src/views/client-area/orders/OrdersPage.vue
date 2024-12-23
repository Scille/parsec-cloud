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

    <!-- condition à ajouter v-if="orderInProgress" -->
    <order-in-progress />

    <div class="orders-done">
      <ion-text class="orders-done__title title-h3">
        {{ $msTranslate('Commandes passées') }}
        <span class="orders-done-count subtitles-sm">{{ '4' }}</span>
      </ion-text>
      <div class="orders-done-content">
        <!-- row header -->
        <ion-list class="orders-done-header-list ion-no-padding">
          <ion-item class="orders-done-header-list-item orders-number">
            <ion-text class="menu-active">{{ $msTranslate('Numéro de commande') }}</ion-text>
          </ion-item>
          <ion-item class="orders-done-header-list-item orders-date">
            <ion-text class="menu-active">{{ $msTranslate('Période') }}</ion-text>
          </ion-item>
          <ion-item class="orders-done-header-list-item orders-members">
            <ion-text class="menu-active">{{ $msTranslate('Nombre de membres') }}</ion-text>
          </ion-item>
          <ion-item class="orders-done-header-list-item orders-storage">
            <ion-text class="menu-active">{{ $msTranslate('Stockage') }}</ion-text>
          </ion-item>
          <ion-item class="orders-done-header-list-item orders-status">
            <ion-text class="menu-active">{{ $msTranslate('Statut') }}</ion-text>
          </ion-item>
        </ion-list>

        <ion-list class="orders-done-list ion-no-padding">
          <ion-item
            class="orders-done-list-item ion-no-padding"
            v-for="order in 4"
            :key="order"
          >
            <ion-text class="orders-done-list-item__data orders-number subtitles-normal">
              {{ $msTranslate('1234567') }}
            </ion-text>
            <ion-text class="orders-done-list-item__data orders-date subtitles-normal">
              {{ $msTranslate('17 Nov. 2024') }}
              <ion-icon
                class="orders-done-list-item__icon"
                :icon="arrowForward"
              />
              {{ $msTranslate('28 Fév. 2024') }}
            </ion-text>
            <ion-text class="orders-done-list-item__data orders-members subtitles-normal">{{ $msTranslate('XXX membres') }}</ion-text>
            <ion-text class="orders-done-list-item__data orders-storage subtitles-normal">{{ $msTranslate('350 Go') }}</ion-text>
            <ion-text class="orders-done-list-item__data orders-status">
              <span class="badge-status body-sm">
                {{ $msTranslate('Terminée') }}
              </span>
            </ion-text>
          </ion-item>
        </ion-list>
      </div>
    </div>

    <!-- orders -->
    <template v-if="!querying">
      <!-- v-if="orders.length > 0" -->
      <div
        class="orders-container"
      >
        <p>commande supérieur à 0 titre</p>
      </div>
      <!-- skeleton orders -->
      <div
        class="skeleton"
        v-if="querying"
      >
        <!-- skeleton year -->
        <ion-skeleton-text
          :animated="true"
          class="skeleton-year"
        />

        <!-- skeleton row header -->
        <div class="skeleton-header-list">
          <ion-skeleton-text
            :animated="true"
            class="orders-date"
          />
        </div>

        <!-- skeleton row orders -->
        <div class="skeleton-content-list">
          <ion-skeleton-text
            :animated="true"
            class="orders-date"
          />
        </div>
      </div>
    </template>

    <!-- no orders -->
    <template v-if="!querying && !error">
      <ion-text class="body-lg no-orders">
        {{ $msTranslate('clientArea.orders.noInvoice') }}
      </ion-text>
    </template>
    <span
      v-show="error"
      class="form-error body"
    >
      {{ $msTranslate(error) }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon, IonButton, IonSkeletonText, modalController } from '@ionic/vue';
import { arrowForward } from 'ionicons/icons';
import {
  BmsAccessInstance,
  BmsOrganization,
  CustomOrderStatus,
  CustomOrderDetailsResultData,
  DataType,
  OrganizationStatsResultData,
} from '@/services/bms';
import NewOrderModal from '@/views/client-area/orders/NewOrderModal.vue';
import OrderInProgress from '@/components/client-area/OrderInProgress.vue';
import { ref, onMounted } from 'vue';

const error = ref<string>('');
const querying = ref(true);
const contractDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);

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
  console.log(props.organization);
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
  gap: 1.5rem;
  position: relative;

  &::after {
    content: '-';
    height: 2rem;
    color: transparent;
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
      --background-hover: var(--parsec-color-light-secondary-constrast);
      color: var(--parsec-color-light-secondary-white);
    }

    &__text {
      color: var(--parsec-color-light-secondary-soft-text);
    }
  }
}

.orders-done {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &__title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding-left: 0.5rem;
    color: var(--parsec-color-light-secondary-soft-text);

    .orders-done-count {
      color: var(--parsec-color-light-secondary-soft-text);
      background: var(--parsec-color-light-secondary-medium);
      padding: 0.125rem 0.25rem;
      border-radius: var(--parsec-radius-8);
    }
  }

  &-content {
    display: flex;
    background: var(--parsec-color-light-secondary-premiere);
    flex-direction: column;
    border-radius: var(--parsec-radius-12);
    padding: 1rem 0.5rem;
    --max-width-number: 12rem;
    --max-width-date: 30rem;
    --max-width-members: 25rem;
    --max-width-storage: 20rem;
    --max-width-amount: 20rem;
    transition: padding 0.2s;
  }

  &-header-list {
    display: flex;
    width: 100%;
    background: none;
    margin-bottom: 0.5rem;
    position: sticky;
    top: 0rem;
    z-index: 2;

    &-item {
      --padding-start: 0;
      --padding-end: 0;
      --padding-top: 0;
      --padding-bottom: 0;
      --background: none;
      padding: 0.375rem 1rem;
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  .orders {
    &-number {
      width: 100%;
      max-width: var(--max-width-number);

      &:not(.orders-done-header-list-item) {
        color: var(--parsec-color-light-secondary-text);
      }
    }

    &-date {
      width: 100%;
      max-width: var(--max-width-date);
    }

    &-members {
      width: 100%;
      max-width: var(--max-width-members);
    }

    &-storage {
      width: 100%;
      max-width: var(--max-width-amount);
    }

    &-status {
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 0.5rem;
    }
  }

  &-list {
    display: flex;
    flex-direction: column;
    --ion-safe-area-left: 0;
    gap: 0.5rem;
    background: none;

    &-item {
      background: var(--parsec-color-light-secondary-white);
      display: flex;
      justify-content: space-between;
      --background: none;
      border-radius: var(--parsec-radius-8);
      position: relative;
      --inner-padding-end: 0;
      align-items: center;

      &:hover {
        background: var(--parsec-color-light-secondary-background);
      }

      &__data {
        color: var(--parsec-color-light-secondary-hard-grey);
        align-self: stretch;
        display: flex;
        align-items: center;
        padding: 0.825rem 1rem;
      }

      // eslint-disable-next-line vue-scoped-css/no-unused-selector
      .badge-status {
        border-radius: var(--parsec-radius-32);
        padding-inline: 0.5rem;
        background: var(--parsec-color-light-secondary-disabled);
        color: var(--parsec-color-light-secondary-text);

        &.paid {
          background: var(--parsec-color-light-info-100);
          color: var(--parsec-color-light-info-700);
        }

        &.open {
          background: var(--parsec-color-tags-orange100);
          color: var(--parsec-color-tags-orange500);
        }
      }
    }
  }
}

.skeleton {
  display: flex;
  flex-direction: column;
  padding: 0.5rem;
  margin-top: 0.5rem;
  background: var(--parsec-color-light-secondary-premiere);
  --max-width-date: 12rem;
  --max-width-number: 12rem;
  --max-width-organization: 20rem;
  --max-width-amount: 10rem;
  transition: padding 0.2s;
  border-radius: var(--parsec-radius-12);

  [class^='orders-'] {
    height: 1rem;
    width: 100%;
  }

  .orders-date {
    max-width: calc(var(--max-width-date) - 1rem);
  }

  ion-skeleton-text {
    border-radius: var(--parsec-radius-8);
  }

  &-year {
    margin: 0.5rem 1rem;
    height: 1.5rem;
    width: 5rem;
  }

  &-header-list {
    display: flex;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-8);
    position: relative;
    align-items: center;
  }

  &-content-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
}
</style>
