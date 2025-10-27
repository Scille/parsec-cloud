<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-method">
    <div class="method-header">
      <ion-title class="method-header__title title-h2">{{ $msTranslate('clientArea.paymentMethodsPage.title') }}</ion-title>
      <ion-text class="method-header__description body-lg">{{ $msTranslate('clientArea.paymentMethodsPage.description') }}</ion-text>
    </div>

    <!-- credit cards -->
    <div class="method-cards">
      <!-- active -->
      <div
        v-if="billingDetails && activeCard"
        class="method-cards-active"
        v-show="activeCard.isDefault"
      >
        <ms-stripe-card-details
          :card="getCardDetails(activeCard)"
          class="method-cards-active__card"
        />
        <div class="method-cards-active-text button-medium">
          <ion-text class="method-cards-active-text__description title-h5">
            {{ $msTranslate('clientArea.paymentMethodsPage.activeDefault') }}
          </ion-text>
          <span class="method-cards-active-text__badge button-small">
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

      <!-- saved  -->
      <div class="method-cards-saved">
        <div class="method-cards-saved-header">
          <ion-text
            class="method-cards-saved-header__title title-h3"
            v-if="savedCards"
          >
            {{ $msTranslate('clientArea.paymentMethodsPage.savedCard') }}
          </ion-text>
          <!-- add button -->
          <ion-button
            class="custom-button method-cards-saved-header__add-button"
            @click="onAddPaymentMethodClicked"
          >
            {{ $msTranslate('clientArea.paymentMethodsPage.addPaymentMethod.addCard') }}
          </ion-button>
        </div>
        <span
          v-show="!querying && !error && cards && cards.length === 0"
          class="no-payment-method"
        >
          {{ $msTranslate('clientArea.paymentMethodsPage.noPaymentMethods') }}
        </span>
        <span
          class="no-additional-payment-method"
          v-show="!querying && !error && savedCards && savedCards.length === 0 && activeCard"
        >
          {{ $msTranslate('clientArea.paymentMethodsPage.noAdditionalPaymentMethods') }}
        </span>
        <template v-if="savedCards">
          <div class="method-cards-saved-list">
            <div
              class="card-item"
              v-for="card in savedCards"
              :key="card.id"
            >
              <div class="card-item-data">
                <ms-image
                  class="card-item-data__image"
                  alt="credit card"
                  :image="card.brand === 'visa' ? VisaCardSmall : MasterCardSmall"
                />
                <div class="card-item-data-text">
                  <!-- name should be update with the credit card holder issue #7964-->
                  <ion-text class="card-item-data-text__name title-h5">{{ getUserName() }}</ion-text>
                  <ion-text class="card-item-data-text__info title-h5">
                    <span>{{ '**** **** **** ' + card.lastDigits }}</span>
                    <span>{{ card.expirationDate.toFormat('MM/yy') }}</span>
                  </ion-text>
                </div>
              </div>
              <div class="card-item-buttons">
                <ion-button
                  class="card-item-buttons__set-default"
                  @click="setDefaultCard(card)"
                  v-show="!card.isExpired()"
                  fill="clear"
                >
                  {{ $msTranslate('clientArea.paymentMethodsPage.setDefault') }}
                </ion-button>
                <ion-button
                  class="card-item-buttons__delete"
                  @click="deleteCard(card)"
                  fill="clear"
                >
                  {{ $msTranslate('clientArea.paymentMethodsPage.delete') }}
                </ion-button>
              </div>
            </div>
          </div>
        </template>
      </div>

      <div v-if="!billingDetails && error">
        {{ $msTranslate(error) }}
      </div>
    </div>

    <template v-if="!isDefaultOrganization(organization)">
      <!-- stop subscription -->
      <div
        v-if="isSubscribed"
        class="method-stop"
      >
        <ion-title class="method-stop-title title-h3">
          {{ $msTranslate('clientArea.paymentMethodsPage.stopSubscription.title') }}
        </ion-title>
        <ion-text class="method-stop-description body">
          {{ $msTranslate('clientArea.paymentMethodsPage.stopSubscription.description') }}
        </ion-text>
        <ion-text
          class="custom-button-outline button-medium"
          @click="stopSubscription"
        >
          {{ $msTranslate('clientArea.paymentMethodsPage.stopSubscription.button') }}
        </ion-text>
      </div>
      <!-- resume subscription -->
      <div
        v-else
        class="method-stop"
      >
        <ion-title class="method-stop-title title-h3">
          {{ $msTranslate('clientArea.paymentMethodsPage.restartSubscription.title') }}
        </ion-title>
        <ion-text class="method-stop-description body">
          {{ $msTranslate('clientArea.paymentMethodsPage.restartSubscription.description') }}
        </ion-text>
        <ion-button
          class="method-stop-button-restart"
          @click="restartSubscription"
        >
          {{ $msTranslate('clientArea.paymentMethodsPage.restartSubscription.button') }}
        </ion-button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import {
  BillingDetailsPaymentMethodCard,
  BillingDetailsResultData,
  BmsAccessInstance,
  BmsOrganization,
  DataType,
  PaymentMethod,
  PersonalInformationResultData,
} from '@/services/bms';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import CreditCardModal from '@/views/client-area/payment-methods/CreditCardModal.vue';
import { isDefaultOrganization } from '@/views/client-area/types';
import { IonButton, IonSkeletonText, IonText, IonTitle, modalController } from '@ionic/vue';
import {
  Answer,
  MasterCardSmall,
  MsImage,
  MsModalResult,
  PaymentMethod as MsPaymentMethod,
  MsStripeCardDetails,
  VisaCardSmall,
  askQuestion,
} from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const props = defineProps<{
  organization: BmsOrganization;
  informationManager: InformationManager;
}>();

const personalInformation = ref<PersonalInformationResultData | null>(null);
const billingDetails = ref<BillingDetailsResultData | undefined>(undefined);
const error = ref('');
const querying = ref(false);
const isSubscribed = ref(props.organization.isSubscribed());
const cards = computed(() => {
  if (!billingDetails.value) {
    return [];
  }
  return billingDetails.value.paymentMethods
    .filter(
      (method) => method.type === PaymentMethod.Card,
      // Always put the active card first
    )
    .sort((method1, method2) => Number(method2.isDefault) - Number(method1.isDefault)) as Array<BillingDetailsPaymentMethodCard>;
});

const activeCard = computed(() => cards.value.find((card) => card.isDefault));
const savedCards = computed(() => cards.value.filter((card) => !card.isDefault));

onMounted(async () => {
  if (BmsAccessInstance.get().isLoggedIn()) {
    personalInformation.value = BmsAccessInstance.get().getPersonalInformation();
  }
  await queryBillingDetails();
});

async function queryBillingDetails(): Promise<void> {
  querying.value = true;
  const currentBillingDetails = billingDetails.value;
  billingDetails.value = undefined;
  const response = await BmsAccessInstance.get().getBillingDetails();
  if (!response.isError && response.data && response.data.type === DataType.BillingDetails) {
    billingDetails.value = response.data;
  } else {
    billingDetails.value = currentBillingDetails;
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
    componentProps: {
      informationManager: props.informationManager,
    },
  });

  await modal.present();
  const { role } = await modal.onDidDismiss();
  await modal.dismiss();
  if (role === MsModalResult.Confirm) {
    await queryBillingDetails();
  }
}

async function setDefaultCard(card: BillingDetailsPaymentMethodCard): Promise<void> {
  const response = await BmsAccessInstance.get().setDefaultPaymentMethod(card.id);

  props.informationManager.present(
    new Information({
      message: response.isError
        ? 'clientArea.paymentMethodsPage.setPaymentMethodDefaultFailed'
        : 'clientArea.paymentMethodsPage.setPaymentMethodDefaultSuccess',
      level: response.isError ? InformationLevel.Error : InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
  await queryBillingDetails();
}

async function deleteCard(card: BillingDetailsPaymentMethodCard): Promise<void> {
  const answer = await askQuestion(
    'clientArea.paymentMethodsPage.deletePaymentMethod.questionTitle',
    'clientArea.paymentMethodsPage.deletePaymentMethod.questionMessage',
    {
      yesText: 'clientArea.paymentMethodsPage.deletePaymentMethod.confirm',
      noText: 'clientArea.paymentMethodsPage.deletePaymentMethod.cancel',
      yesIsDangerous: true,
    },
  );

  if (answer === Answer.No) {
    return;
  }
  const response = await BmsAccessInstance.get().deletePaymentMethod(card.id);

  props.informationManager.present(
    new Information({
      message: response.isError
        ? 'clientArea.paymentMethodsPage.deletePaymentMethod.failed'
        : 'clientArea.paymentMethodsPage.deletePaymentMethod.success',
      level: response.isError ? InformationLevel.Error : InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
  await queryBillingDetails();
}

async function stopSubscription(): Promise<void> {
  const answer = await askQuestion(
    'clientArea.paymentMethodsPage.stopSubscription.questionTitle',
    'clientArea.paymentMethodsPage.stopSubscription.questionMessage',
    {
      yesText: 'clientArea.paymentMethodsPage.stopSubscription.confirm',
      noText: 'clientArea.paymentMethodsPage.stopSubscription.cancel',
      yesIsDangerous: true,
    },
  );

  if (answer === Answer.No) {
    return;
  }
  const response = await BmsAccessInstance.get().unsubscribeOrganization(props.organization.bmsId);

  props.informationManager.present(
    new Information({
      message: response.isError
        ? 'clientArea.paymentMethodsPage.stopSubscription.failed'
        : 'clientArea.paymentMethodsPage.stopSubscription.success',
      level: response.isError ? InformationLevel.Error : InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
  isSubscribed.value = response.isError;
}

async function restartSubscription(): Promise<void> {
  const answer = await askQuestion(
    'clientArea.paymentMethodsPage.restartSubscription.questionTitle',
    'clientArea.paymentMethodsPage.restartSubscription.questionMessage',
    {
      yesText: 'clientArea.paymentMethodsPage.restartSubscription.confirm',
      noText: 'clientArea.paymentMethodsPage.restartSubscription.cancel',
    },
  );

  if (answer === Answer.No) {
    return;
  }
  const response = await BmsAccessInstance.get().subscribeOrganization(props.organization.bmsId);

  props.informationManager.present(
    new Information({
      message: response.isError
        ? 'clientArea.paymentMethodsPage.restartSubscription.failed'
        : 'clientArea.paymentMethodsPage.restartSubscription.success',
      level: response.isError ? InformationLevel.Error : InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
  isSubscribed.value = !response.isError;
}

function getUserName(): string {
  const info = personalInformation.value;
  if (info === null || !info.firstName || !info.lastName) {
    return '';
  }
  return `${info.firstName} ${info.lastName}`;
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
  flex-direction: column;
  gap: 3rem;

  &-active {
    display: flex;
    gap: 2rem;
    align-items: center;

    &__card {
      box-shadow: var(--parsec-shadow-strong);
      width: 15.625rem;
      height: 8.625rem;
      margin-bottom: 0;
      border-radius: 0.9375rem;
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.75rem;

      &__description {
        color: var(--parsec-color-light-secondary-text);
      }

      &__badge {
        color: var(--parsec-color-light-primary-700);
        background-color: var(--parsec-color-light-primary-50);
        border-radius: var(--parsec-radius-32);
        padding: 0.2rem 0.5rem;
        text-align: center;
        width: fit-content;
      }
    }
  }

  &-saved {
    display: flex;
    max-width: 100rem;
    flex-direction: column;
    gap: 2rem;

    &::after {
      content: '';
      position: absolute;
      right: 0;
      width: 10rem;
      height: 100%;
      background: linear-gradient(to right, transparent 0%, var(--parsec-color-light-secondary-white) 100%);
    }

    &-header {
      display: flex;
      align-items: center;
      width: fit-content;
      gap: 1.5rem;

      &__title {
        color: var(--parsec-color-light-primary-700);
      }

      &__add-button {
        padding: 0;
      }
    }

    &-list {
      display: flex;
      overflow: auto;
      gap: 1rem;
    }
  }

  .card-item {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    background-color: var(--parsec-color-light-secondary-inversed-contrast);
    border: 1px solid var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-8);
    padding: 0.75rem;
    width: 100%;
    max-width: 25rem;
    flex-shrink: 0;

    &-data {
      display: flex;
      align-items: center;
      gap: 1rem;

      &__image {
        width: 100px;
        height: 55.8px;
        border-radius: var(--parsec-radius-8);
        background: red;
      }

      &-text {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;

        &__name {
          color: var(--parsec-color-light-secondary-text);
        }

        &__info {
          display: flex;
          gap: 0.75rem;
          color: var(--parsec-color-light-secondary-grey);
        }
      }
    }

    &-buttons {
      display: flex;
      gap: 1rem;

      &__delete {
        color: var(--parsec-color-light-danger-500);
        --background-hover: var(--parsec-color-light-danger-100);
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
  margin-top: 4rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1.5rem;
  border-radius: var(--parsec-radius-12);
  max-width: 30rem;
  background-color: var(--parsec-color-light-secondary-background);
  position: relative;

  &::before {
    content: '';
    position: absolute;
    top: -3rem;
    left: 0;
    width: 100%;
    height: 1px;
    background-color: var(--parsec-color-light-secondary-disabled);
    border-radius: var(--parsec-radius-8) var(--parsec-radius-8) 0 0;
  }

  &-title {
    color: var(--parsec-color-light-secondary-text);
  }

  &-description {
    color: var(--parsec-color-light-secondary-soft-text);
  }

  &-button-restart {
    width: fit-content;
  }
}

.custom-button-outline {
  border: 1px solid var(--parsec-color-light-secondary-text);
  color: var(--parsec-color-light-secondary-text);
  border-radius: var(--parsec-radius-8);
  padding: 0.625rem 0.75rem;
  cursor: pointer;
  height: fit-content;
  width: fit-content;

  &:hover {
    border: 1px solid var(--parsec-color-light-secondary-contrast);
    color: var(--parsec-color-light-secondary-contrast);
  }
}
</style>
