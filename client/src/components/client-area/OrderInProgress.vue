<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="order-progress"
    @click="toggleOpen"
  >
    <!-- order header -->
    <div
      class="order-header"
      v-if="customOrderRequestStatus"
    >
      <ion-text class="order-header__title title-h3">
        {{
          $msTranslate({
            key: 'clientArea.orders.progress.orderNumber',
            data: { number: request.id },
          })
        }}
        <span
          class="order-tag button-small"
          :class="getStatusClass(customOrderRequestStatus ,customOrderStatus)"
        >
          {{ orderStepInfo.tag }}
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
      v-if="open && contractDetails?.type === DataType.CustomOrderDetails"
      class="order-content"
    >
      <div class="order-content-details">
        <ion-text class="order-content-details-header subtitles-sm">
          {{ $msTranslate('clientArea.orders.new.details') }}
        </ion-text>
        <div class="details-list">
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="people" />
              {{ $msTranslate('clientArea.orders.new.usersNeed') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">{{ getUserOptions(request.users) }}</ion-text>
          </div>
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="pieChart" />
              {{ $msTranslate('clientArea.orders.new.dataNeeded') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">{{ getStorageOptions(request.storage) }}</ion-text>
          </div>
        </div>

        <div class="details-list">
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="time" />
              {{ $msTranslate('clientArea.orders.new.startDate') }}
            </ion-text>
            <ion-text
              class="details-list-item__data subtitles-normal"
              v-if="organization"
            >
              {{ $msTranslate(I18n.formatDate(contractDetails.created, 'short')) }}
            </ion-text>
            <ion-text
              class="details-list-item__data subtitles-normal"
              v-else
            >
              {{ $msTranslate('-') }}
            </ion-text>
          </div>
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="time" />
              {{ $msTranslate('clientArea.orders.new.endDate') }}
            </ion-text>
            <ion-text
              class="details-list-item__data subtitles-normal"
              v-if="organization"
            >
              {{ $msTranslate(I18n.formatDate(contractDetails.dueDate, 'short')) }}
            </ion-text>
            <ion-text
              class="details-list-item__data subtitles-normal"
              v-else
            >
              {{ $msTranslate('-') }}
            </ion-text>
          </div>
        </div>
      </div>

      <div class="order-content-follow">
        <!-- waiting https://github.com/Scille/parsec-cloud/issues/7966 to be merge -->
        <!-- we need to create a component which includes request progression -->
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon } from '@ionic/vue';
import { people, pieChart, time } from 'ionicons/icons';
import {
  CustomOrderRequestStatus,
  CustomOrderDetailsResultData,
  GetCustomOrderRequestsResultData,
  BmsOrganization,
  CustomOrderStatus,
  BmsAccessInstance,
  DataType,
} from '@/services/bms';
import { ref, computed, onMounted } from 'vue';
import { DateTime } from 'luxon';
import { isDefaultOrganization } from '@/views/client-area/types';
import { Translatable, I18n } from 'megashark-lib';
import { MockedBmsApi } from '@/services/bms/mockApi';

const error = ref<string>('');
const querying = ref(true);
const contractRequests = ref<GetCustomOrderRequestsResultData | undefined>(undefined);
const customOrderRequestStatus = ref<CustomOrderRequestStatus>(CustomOrderRequestStatus.Received);
const contractDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);
const customOrderStatus = ref<CustomOrderStatus>(CustomOrderStatus.Unknown);
const open = ref(false);

const props = defineProps<{
  request: CustomOrderRequest;
  organization?: BmsOrganization;
}>();

function getUserOptions(user: number): Translatable {
  switch (user) {
    case 50:
      return 'clientArea.orders.request.userNeeds.choices.50';
    case 100:
      return 'clientArea.orders.request.userNeeds.choices.100';
    case 300:
      return 'clientArea.orders.request.userNeeds.choices.300';
    case 9999:
      return 'clientArea.orders.request.userNeeds.choices.more';
    default:
      return '';
  }
}

function getStorageOptions(storage: number): Translatable {
  switch (storage) {
    case 100:
      return 'clientArea.orders.request.storageNeeds.choices.100';
    case 500:
      return 'clientArea.orders.request.storageNeeds.choices.500';
    case 1000:
      return 'clientArea.orders.request.storageNeeds.choices.1000';
    case 9999:
      return 'clientArea.orders.request.storageNeeds.choices.more';
    default:
      return '';
  }
}

interface CustomOrderRequest {
  id: string;
  describedNeeds: string;
  users: number;
  storage: number;
  status: CustomOrderRequestStatus;
  comment: string;
  orderDate: DateTime;
}

interface RequestStatusStep {
  title: Translatable;
  subtitle?: Translatable;
  tag: Translatable;
}

onMounted(async () => {
  if (props.organization) {
    if (isDefaultOrganization(props.organization)) {
      querying.value = true;
      return;
    }
  }

  const orderRequestsRep = await MockedBmsApi.getCustomOrderRequests();
  if (!orderRequestsRep.isError && orderRequestsRep.data && orderRequestsRep.data.type === DataType.GetCustomOrderRequests) {
    contractRequests.value = orderRequestsRep.data;

    const request = orderRequestsRep.data.requests.find((req) => req.id === props.request.id);
    console.log('request', request);
    if (request) {
      customOrderRequestStatus.value = request.status;
    }
  } else {
    error.value = 'clientArea.dashboard.processing.error.title';
  }

  if (props.organization) {
    const orderDetailsRep = await BmsAccessInstance.get().getCustomOrderDetails(props.organization);
    if (!orderDetailsRep.isError && orderDetailsRep.data && orderDetailsRep.data.type === DataType.CustomOrderDetails) {
      contractDetails.value = orderDetailsRep.data;
    } else {
      error.value = 'clientArea.contracts.errors.noInfo';
    }
  }
  const orderDetailsStatus = await MockedBmsApi.getCustomOrderStatus();
  if (!orderDetailsStatus.isError && orderDetailsStatus.data && orderDetailsStatus.data.type === DataType.CustomOrderStatus) {
    customOrderStatus.value = orderDetailsStatus.data.status;
  } else {
    error.value = 'clientArea.contracts.errors.noInfo';
  }

  querying.value = false;
});

const orderStepInfo = computed<RequestStatusStep>(() => {

  switch (customOrderRequestStatus.value) {
    case CustomOrderRequestStatus.Received:
      if (CustomOrderStatus.NothingLinked) {
        return {
          title: 'Commande reçue',
          subtitle: 'Votre demande a bien été reçue, nous allons la traiter dans les plus brefs délais.',
          tag: 'Reçue',
        };
      }
      break;
    case CustomOrderRequestStatus.Processing:
      if (CustomOrderStatus.NothingLinked) {
        return {
          title: 'Demande traitée par le commercial',
          subtitle: 'Notre équipe commerciale traite votre demande, nous allons vous recontacter pour définir précisément vos besoins.',
          tag: 'En attente de traitement',
        };
      }
      break;
    case CustomOrderRequestStatus.Finished:
      if (CustomOrderStatus.NothingLinked) {
        return {
          title: 'Validation de la commande',
          subtitle: 'Votre commande est confirmée, vous allez bientôt recevoir la facture à payer.',
          tag: 'Confirmé',
        };
      }
      if (CustomOrderStatus.EstimateLinked || CustomOrderStatus.InvoiceToBePaid) {
        return {
          title: 'En attente de paiement',
          subtitle: 'En attente de la réception de votre virement bancaire pour rendre disponible votre organisation.',
          tag: 'En attente de paiement',
        };
      }
      if (CustomOrderStatus.InvoicePaid) {
        return {
          title: 'Organisation disponible',
          tag: 'Disponible',
        };
      }
      break;
    case CustomOrderRequestStatus.Standby:
      return {
        title: 'Commande suspendue',
        tag: 'Suspendue',
      };
    case CustomOrderRequestStatus.Cancelled:
      return {
        title: 'Commande annulée',
        tag: 'Annulée',
      };
    default:
      return {
        title: 'Aucune information',
        subtitle: 'Votre demande est en cours de traitement, nous vous tiendrons informé de son avancement.',
        tag: 'Inconnu',
      };
  }
  return {
    title: 'Aucune information',
    subtitle: 'Votre demande est en cours de traitement, nous vous tiendrons informé de son avancement.',
    tag: 'Inconnu',
  };
});

function getStatusClass (statusBms?: CustomOrderRequestStatus | undefined,
  statusSellsy?:CustomOrderStatus | undefined): string | undefined {
  switch (statusBms) {
    case CustomOrderRequestStatus.Received :
      return 'order-tag-received';
    case CustomOrderRequestStatus.Processing:
      return 'order-tag-processing';
    case CustomOrderRequestStatus.Standby:
      return 'order-tag-standby';
    case CustomOrderRequestStatus.Cancelled:
      return 'order-tag-cancelled';
    case CustomOrderRequestStatus.Finished:
      if (statusSellsy === CustomOrderStatus.NothingLinked) {
        return 'order-tag-finished';
      }
      if (statusSellsy === CustomOrderStatus.EstimateLinked || statusSellsy === CustomOrderStatus.InvoiceToBePaid) {
        return 'order-tag-finished';
      }
      if (statusSellsy === CustomOrderStatus.InvoicePaid) {
        return 'order-tag-available';
      }
      break;
    default:
      return '';
  }
}

function toggleOpen(): boolean {
  return open.value = !open.value;
}
</script>

<style scoped lang="scss">
.order-progress {
  display: flex;
  flex-direction: column;
  box-shadow: var(--parsec-shadow-light);
  border-radius: var(--parsec-radius-12);
  width: 100%;
}

.order-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
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

      &-received {
        background-color: var(--parsec-color-tags-indigo-50);
        color: var(--parsec-color-tags-indigo-700);
      }
      &-processing {
        background-color: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-secondary-text);
      }
      &-finished {
        background-color: var(--parsec-color-tags-blue-50);
        color: var(--parsec-color-tags-blue-700);
      }
      &-invoice {
        background-color: var(--parsec-color-tags-orange-50);
        color: var(--parsec-color-tags-orange-700);
      }
      &-available {
        background-color: var(--parsec-color-light-success-50);
        color: var(--parsec-color-light-success-500);
      }
      &-standby {
        background-color: var(--parsec-color-tags-orange-50);
        color: var(--parsec-color-tags-orange-700);
      }
      &-cancelled {
        background-color: var(--parsec-color-light-danger-50);
        color: var(--parsec-color-light-danger-500);
      }
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
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  &-details {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 28rem;
    margin: 2.5rem;
    width: 100%;

    &-header {
      color: var(--parsec-color-light-secondary-grey);
    }

    .details-list {
      display: flex;
      flex-direction: column;
      gap: 1rem;

      &-item {
        display: flex;
        align-items: center;
        gap: 1rem;
        justify-content: space-between;

        &__title {
          display: flex;
          align-items: center;
          gap: 0.5rem;
          color: var(--parsec-color-light-secondary-hard-grey);
        }

        &__data {
          color: var(--parsec-color-light-secondary-text);
          padding: 0.5rem 0.75rem;
          background: var(--parsec-color-light-secondary-background);
          border-radius: var(--parsec-radius-8);
          max-width: 15rem;
          text-align: end;
          width: 100%;
        }
      }
    }
  }
}
</style>
