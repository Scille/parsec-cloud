<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-page-dashboard">
    <!-- summary + invoices section -->
    <div class="dashboard-section-container">
      <!-- month summary -->
      <div class="dashboard-section month-summary-container">
        <ion-title class="dashboard-section-title title-h2">
          {{ currentMonthYear }}
        </ion-title>
        <div class="month-summary">
          <!-- amount -->
          <div class="month-summary-item">
            <ion-title class="month-summary-item__title title-h4">
              {{ $msTranslate('clientArea.dashboard.summary.estimatedAmount') }}
            </ion-title>
            <div class="month-summary-item__data title-h1-xl">
              <div
                class="data-content"
                v-if="stats"
              >
                <ion-text class="data-content-text">
                  {{
                    $msTranslate({
                      key: 'clientArea.dashboard.invoices.price.amount',
                      data: { amount: calculateEstimatedAmount(stats.users, stats.dataSize) },
                    })
                  }}
                  {{ calculateEstimatedAmount(stats.users, stats.dataSize) }}
                </ion-text>
                <ms-information-tooltip
                  text="clientArea.dashboard.summary.withdrawalDescription"
                  slot="end"
                  class="data-content-icon"
                />
              </div>
              <span class="data-info title-h4 button-medium">
                {{
                  $msTranslate({
                    key: 'clientArea.dashboard.summary.withdrawal',
                    data: { date: getEndDayOfMonth(currentDate) },
                  })
                }}
              </span>
            </div>
          </div>

          <!-- users -->
          <div class="month-summary-item">
            <ion-title class="month-summary-item__title title-h4">
              {{ $msTranslate('clientArea.dashboard.summary.users') }}
            </ion-title>
            <div class="month-summary-item__data title-h1-xl">
              {{ stats?.users }}
            </div>
          </div>

          <!-- storage -->
          <div class="month-summary-item">
            <ion-title class="month-summary-item__title title-h4">
              {{ $msTranslate('clientArea.dashboard.summary.storage') }}
            </ion-title>
            <div class="month-summary-item__data title-h1-xl">
              {{ stats?.dataSize }}
            </div>
          </div>
        </div>
      </div>

      <!-- recent invoices -->
      <div class="dashboard-section invoices-container">
        <ion-title class="dashboard-section-title title-h2">
          {{ $msTranslate('clientArea.dashboard.invoices.title') }}
        </ion-title>
        <div
          class="invoices"
          v-if="invoices.length > 0"
        >
          <div class="invoices-header">
            <ion-list class="invoices-header-list ion-no-padding">
              <ion-item class="invoices-header-list-item invoices-date">
                <ion-text class="menu-active">{{ $msTranslate('clientArea.dashboard.invoices.date') }}</ion-text>
              </ion-item>
              <ion-item class="invoices-header-list-item invoices-organization">
                <ion-text class="menu-active">{{ $msTranslate('clientArea.dashboard.invoices.organization') }}</ion-text>
              </ion-item>
              <ion-item class="invoices-header-list-item invoices-amount">
                <ion-text class="menu-active">{{ $msTranslate('clientArea.dashboard.invoices.price.title') }}</ion-text>
              </ion-item>
              <ion-item class="invoices-header-list-item invoices-status">
                <ion-text class="menu-active">{{ $msTranslate('clientArea.dashboard.invoices.status') }}</ion-text>
              </ion-item>
            </ion-list>
          </div>

          <div class="invoices-content">
            <ion-list class="invoices-list ion-no-padding">
              <ion-item
                class="invoices-list-item ion-no-padding"
                v-for="invoice in invoices"
                :key="invoice.id"
              >
                <ion-text class="invoices-list-item__data invoices-date subtitles-sm">{{ invoice.start.toFormat('LLLL yyyy') }}</ion-text>
                <ion-text class="invoices-list-item__data invoices-organization body">{{ invoice.organizationId }}</ion-text>
                <ion-text class="invoices-list-item__data invoices-amount body">{{ invoice.total }}</ion-text>
                <ion-text class="invoices-list-item__data invoices-status">
                  <span class="badge-status body-sm incoming">{{ invoice.status }}</span>
                  <ion-text class="custom-button custom-button-ghost button-medium">
                    <ion-icon :icon="download" />
                    {{ $msTranslate('clientArea.dashboard.invoices.download') }}
                  </ion-text>
                </ion-text>
              </ion-item>
            </ion-list>
          </div>
        </div>
        <ion-text
          class="body-lg"
          v-else
        >
          {{ $msTranslate('clientArea.dashboard.invoices.noInvoices') }}
        </ion-text>
      </div>
    </div>

    <!-- payment section -->
    <div class="dashboard-section payment-container">
      <div class="dashboard-section-title">
        <ion-title class="title-h2">{{ $msTranslate('clientArea.dashboard.payment.title') }}</ion-title>
        <ion-text class="payment-text body">
          {{ $msTranslate('clientArea.dashboard.payment.description') }}
        </ion-text>
      </div>
      <div class="payment">
        <ms-stripe-card-details
          v-if="defaultCard"
          :card="defaultCard"
          class="payment-card"
        />
        <ion-text
          class="custom-button custom-button-fill button-medium"
          @click="switchPage(ClientAreaPages.PaymentMethods)"
        >
          {{ $msTranslate('clientArea.dashboard.payment.update') }}
        </ion-text>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonTitle, IonText, IonList, IonItem, IonIcon } from '@ionic/vue';
import { download } from 'ionicons/icons';
import { ClientAreaPages } from '@/views/client-area/types';
import {
  BmsAccessInstance,
  DataType,
  BmsOrganization,
  OrganizationStatsResultData,
  BmsInvoice,
  BillingDetailsResultData,
  BillingDetailsPaymentMethodCard,
  PaymentMethod,
} from '@/services/bms';
import { MsInformationTooltip, I18n, MsStripeCardDetails } from 'megashark-lib';
import { onMounted, ref } from 'vue';
import { PaymentMethod as MsPaymentMethod } from 'megashark-lib';

const props = defineProps<{
  organization: BmsOrganization;
}>();

const stats = ref<OrganizationStatsResultData | undefined>(undefined);
const invoices = ref<Array<BmsInvoice>>([]);
const billingDetails = ref<BillingDetailsResultData | undefined>(undefined);
const defaultCard = ref<MsPaymentMethod.Card | undefined>(undefined);
const currentPage = ref<ClientAreaPages>(ClientAreaPages.Dashboard);
const currentMonthYear = new Date().toLocaleDateString(I18n.getLocale(), { month: 'long', year: 'numeric' });
const currentDate = new Date();

// should be fetched from the backend
const ADMIN_USER_PRICE = 15;

onMounted(async () => {
  const orgStatsResponse = await BmsAccessInstance.get().getOrganizationStats(props.organization.bmsId);
  if (!orgStatsResponse.isError && orgStatsResponse.data && orgStatsResponse.data.type === DataType.OrganizationStats) {
    stats.value = orgStatsResponse.data;
  }

  const invoicesResponse = await BmsAccessInstance.get().getInvoices();
  if (!invoicesResponse.isError && invoicesResponse.data && invoicesResponse.data.type === DataType.Invoices) {
    invoices.value = invoicesResponse.data.invoices.filter((invoice) => invoice.organizationId === props.organization.parsecId);
  }

  const billingDetailsResponse = await BmsAccessInstance.get().getBillingDetails();
  if (!billingDetailsResponse.isError && billingDetailsResponse.data && billingDetailsResponse.data.type === DataType.BillingDetails) {
    billingDetails.value = billingDetailsResponse.data;
    const card = billingDetails.value.paymentMethods.find(
      (method) => method.isDefault && method.type === PaymentMethod.Card,
    ) as BillingDetailsPaymentMethodCard;
    if (card) {
      defaultCard.value = {
        last4: card.lastDigits,
        // eslint-disable-next-line camelcase
        exp_year: card.expirationDate.year,
        // eslint-disable-next-line camelcase
        exp_month: card.expirationDate.month,
        brand: card.brand,
      } as MsPaymentMethod.Card;
    }
  }
});

function getEndDayOfMonth(date: Date): string {
  return new Date(date.getFullYear(), date.getMonth() + 1, 0).toLocaleDateString(I18n.getLocale(), {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
  });
}

async function switchPage(page: ClientAreaPages): Promise<void> {
  currentPage.value = page;
}

// temporary function, should be fetched from the backend
function calculateEstimatedAmount(users: number, storageSize: number): number {
  return users * ADMIN_USER_PRICE + storageSize * 0.1;
}
</script>

<style scoped lang="scss">
.client-page-dashboard {
  display: flex;
  gap: 1.5rem;
}

// summary + invoices section
.dashboard-section-container {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-width: 60rem;
  width: 100%;
}

//payment section
.payment-container {
  max-width: 20rem;
  height: fit-content;
}

//every sections
.dashboard-section {
  background: var(--parsec-color-light-secondary-inversed-contrast);
  display: flex;
  flex-direction: column;
  gap: 2rem;
  padding: 1.5rem 2rem;
  border-radius: var(--parsec-radius-12);
  border: 1px solid var(--parsec-color-light-secondary-premiere);

  &-title {
    color: var(--parsec-color-light-primary-700);
  }
}

.month-summary {
  display: flex;

  &-item {
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: 100%;
    height: min-content;
    gap: 0.75rem;

    &__title {
      height: fit-content;
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    &__data {
      display: flex;
      flex-direction: column;
      gap: 0.625rem;
      color: var(--parsec-color-light-secondary-contrast);

      .data-content {
        display: flex;
        gap: 0.5rem;
        align-items: center;

        &-text {
          color: var(--parsec-color-light-secondary-contrast);
        }

        &-icon {
          color: var(--parsec-color-light-secondary-grey);
          font-size: 1.5rem;
          cursor: pointer;

          &:hover {
            color: var(--parsec-color-light-secondary-text);
          }
        }
      }

      .data-info {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}

.invoices {
  display: flex;
  flex-direction: column;
  overflow: hidden;
  gap: 0.5rem;

  &-date {
    width: 100%;
    max-width: 10rem;
  }

  &-organization {
    width: 100%;
    max-width: 12rem;
  }

  &-amount {
    width: 100%;
    max-width: 5.625rem;
  }

  &-status {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  &-header {
    display: flex;
    justify-content: space-between;
    background: var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-8);

    .invoices-header-list {
      display: flex;
      padding: 0.75rem 0.5rem;
      width: 100%;

      &-item {
        --padding-start: 0;
        --padding-end: 0;
        --padding-top: 0;
        --padding-bottom: 0;
        --background: none;
        color: var(--parsec-color-light-secondary-soft-text);
      }
    }
  }

  &-content {
    display: flex;
    flex-direction: column;

    .invoices-list {
      display: flex;
      flex-direction: column;

      &-item {
        display: flex;
        justify-content: space-between;
        --background: none;
        padding: 0.75rem 0.5rem;
        border-radius: var(--parsec-radius-8);
        position: relative;

        &:hover {
          .invoices-list-item__data {
            color: var(--parsec-color-light-secondary-contrast);
          }
        }

        &:not(:last-child) {
          &::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
          }
        }

        &__data {
          color: var(--parsec-color-light-secondary-hard-grey);

          .custom-button-ghost {
            color: var(--parsec-color-light-primary-500);

            &:hover {
              color: var(--parsec-color-light-primary-600);
            }
          }
        }

        // eslint-disable-next-line vue-scoped-css/no-unused-selector
        .badge-status {
          border-radius: var(--parsec-radius-32);
          padding-inline: 0.5rem;

          &.incoming {
            background: var(--parsec-color-light-info-100);
            color: var(--parsec-color-light-info-700);
          }

          &.paid {
            background: var(--parsec-color-light-success-100);
            color: var(--parsec-color-light-success-700);
          }
        }
      }
    }
  }
}

.payment-container .dashboard-section-title {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.payment {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &-text {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &-card {
    margin-bottom: 1rem;
  }
}
</style>
