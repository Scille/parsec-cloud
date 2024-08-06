<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-method">
    <div class="method-header">
      <ion-title class="method-header__title title-h2">{{ $msTranslate('clientArea.paymentMethodsPage.title') }}</ion-title>
      <ion-text class="method-header__description body-lg">{{ $msTranslate('clientArea.paymentMethodsPage.description') }}</ion-text>
    </div>

    <!-- credit cards -->
    <div class="method-cards">
      <!-- credit card -->
      <div
        v-if="billingDetails"
        class="method-cards-list"
      >
        <div
          class="method-cards-list__item"
          v-for="card in cards"
          :key="card.id"
        >
          <ms-stripe-card-details
            :card="getCardDetails(card)"
            class="card"
          />
          <span
            v-show="card.isDefault"
            class="card-active button-medium"
          >
            {{ $msTranslate('clientArea.paymentMethodsPage.activeMethodLabel') }}
          </span>
        </div>
      </div>
      <div v-if="querying">
        <ion-skeleton-text
          :animated="true"
          class="skeleton-loading-card"
        />
      </div>
      <!-- add button -->
      <ion-button
        class="custom-button-outline"
        @click="onAddPaymentMethodClicked"
        fill="outline"
      >
        {{ $msTranslate('clientArea.paymentMethodsPage.addCard') }}
      </ion-button>
      <div v-if="!billingDetails && error">
        {{ $msTranslate(error) }}
      </div>
    </div>

    <!-- stop subscription -->
    <div
      class="method-stop"
      v-show="false"
    >
      <ion-title class="method-stop-title title-h3">
        {{ $msTranslate('clientArea.paymentMethodsPage.stopSubscription.title') }}
      </ion-title>
      <ion-text class="method-stop-description body">
        {{ $msTranslate('clientArea.paymentMethodsPage.stopSubscription.description') }}
      </ion-text>
      <ion-button
        class="custom-button-outline"
        fill="outline"
      >
        {{ $msTranslate('clientArea.paymentMethodsPage.stopSubscription.button') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  BmsOrganization,
  BmsAccessInstance,
  DataType,
  BillingDetailsResultData,
  PaymentMethod,
  BillingDetailsPaymentMethodCard,
} from '@/services/bms';
import { onMounted, ref, computed, inject } from 'vue';
import { MsStripeCardDetails, PaymentMethod as MsPaymentMethod, MsModalResult } from 'megashark-lib';
import { IonButton, IonText, IonTitle, IonSkeletonText, modalController } from '@ionic/vue';
import CreditCardModal from '@/views/client-area/payment-methods/CreditCardModal.vue';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';

defineProps<{
  organization: BmsOrganization;
}>();

const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const informationManager: InformationManager = injectionProvider.getDefault().informationManager;
const billingDetails = ref<BillingDetailsResultData | undefined>(undefined);
const error = ref('');
const querying = ref(false);
const cards = computed(() => {
  if (!billingDetails.value) {
    return [];
  }
  return billingDetails.value.paymentMethods.filter(
    (method) => method.type === PaymentMethod.Card,
  ) as Array<BillingDetailsPaymentMethodCard>;
});

onMounted(async () => {
  await queryBillingDetails();
});

async function queryBillingDetails(): Promise<void> {
  querying.value = true;
  const response = await BmsAccessInstance.get().getBillingDetails();
  if (!response.isError && response.data && response.data.type === DataType.BillingDetails) {
    billingDetails.value = response.data;
  } else {
    console.log(`Failed to retrieve billing details: ${response.errors}`);
    error.value = 'clientArea.paymentMethodsPage.retrieveFailed';
  }
  querying.value = false;
}

function getCardDetails(card: BillingDetailsPaymentMethodCard): MsPaymentMethod.Card {
  return {
    last4: card.lastDigits,
    // eslint-disable-next-line camelcase
    exp_year: card.expirationDate.year,
    // eslint-disable-next-line camelcase
    exp_month: card.expirationDate.month,
    brand: card.brand,
  } as MsPaymentMethod.Card;
}

async function onAddPaymentMethodClicked(): Promise<void> {
  const modal = await modalController.create({
    component: CreditCardModal,
    canDismiss: true,
    backdropDismiss: false,
    cssClass: 'credit-card-modal',
  });

  await modal.present();
  const { data, role } = await modal.onDidDismiss();
  await modal.dismiss();
  if (role !== MsModalResult.Confirm) {
    return;
  }
  const card = data.card;
  const setDefault = data.setDefault;
  const response = await BmsAccessInstance.get().addPaymentMethod(card.id);
  if (response.isError) {
    informationManager.present(
      new Information({
        message: 'clientArea.paymentMethodsPage.addPaymentMethodFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }
  let setDefaultFailed = false;
  if (setDefault) {
    const defaultResponse = await BmsAccessInstance.get().setDefaultPaymentMethod(card.id);
    setDefaultFailed = defaultResponse.isError;
  }
  informationManager.present(
    new Information({
      message: setDefaultFailed
        ? 'clientArea.paymentMethodsPage.addPaymentMethodSuccessSetDefaultFailed'
        : 'clientArea.paymentMethodsPage.addPaymentMethodSuccess',
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
  await queryBillingDetails();
}
</script>

<style scoped lang="scss">
.client-page-method {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.method-header {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &__title {
    color: var(--parsec-color-light-primary-700);
  }

  &__description {
    color: var(--parsec-color-light-secondary-soft-text);
  }
}

.method-cards {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  margin-bottom: 3em;

  &-list {
    display: flex;
    flex-wrap: wrap;
    max-width: 50rem;
    gap: 1rem;

    &__item {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 0.75rem;
      position: relative;

      .card {
        margin: 0;
      }

      .card-active {
        position: absolute;
        bottom: -2.5rem;
        color: var(--parsec-color-light-primary-700);
        background-color: var(--parsec-color-light-primary-50);
        border-radius: var(--parsec-radius-32);
        padding: 0.25rem 0.5rem;
        text-align: center;
        width: fit-content;
      }
    }
  }

  .skeleton-loading-card {
    width: 15.625rem;
    height: 8.625rem;
    border-radius: var(--parsec-radius-12);
  }
}

.method-stop {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1.5rem;
  border-radius: var(--parsec-radius-12);
  max-width: 30rem;
  background-color: var(--parsec-color-light-secondary-background);

  &-title {
    color: var(--parsec-color-light-secondary-text);
  }

  &-description {
    color: var(--parsec-color-light-secondary-soft-text);
  }
}

.custom-button-outline {
  --color: var(--parsec-color-light-secondary-contrast);
  --background: none;
  --background-hover: var(--parsec-color-light-secondary-background);
  --color-hover: var(--parsec-color-light-secondary-contrast);
  --border-color: var(--parsec-color-light-secondary-contrast);
  --border-width: 1px;
  --border-radius: var(--parsec-radius-8);
  height: fit-content;
  width: fit-content;
}
</style>
