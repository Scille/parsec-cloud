<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<!-- eslint-disable vue/html-indent -->
<template>
  <div class="order-content-details">
    <ion-text class="order-content-details-header subtitles-sm">
      {{ $msTranslate('clientArea.orders.new.details') }}
    </ion-text>
    <!-- No contract created at this step, users and storage estimation requested only -->
    <div
      class="details-list"
      v-if="[OrderStep.Received, OrderStep.Processing, OrderStep.Confirmed].includes(orderStep)"
    >
      <div class="details-list-item">
        <ion-text class="details-list-item__title body-lg">
          <ion-icon :icon="people" />
          {{ $msTranslate('clientArea.orders.new.usersNeed') }}
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">{{ $msTranslate(getUserOptions(request.standardUsers)) }}</ion-text>
      </div>
      <div class="details-list-item">
        <ion-text class="details-list-item__title body-lg">
          <ion-icon :icon="pieChart" />
          {{ $msTranslate('clientArea.orders.new.dataNeeded') }}
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">{{ $msTranslate(getStorageOptions(request.storage)) }}</ion-text>
      </div>
      <!-- comment -->
      <div
        class="details-list-item"
        id="detail-comment"
      >
        <ion-text class="details-list-item__title body-lg">
          <ion-icon :icon="chatbubbleEllipses" />
          {{ $msTranslate('clientArea.orders.new.comment') }}
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">
          {{ request.describedNeeds }}
        </ion-text>
      </div>
    </div>

    <!-- Contract created, users and storage are known -->
    <div
      class="details-list"
      v-if="orderDetails && [OrderStep.InvoiceToBePaid, OrderStep.Available].includes(orderStep)"
    >
      <div class="details-list-item">
        <ion-text class="details-list-item__title body-lg">
          <ion-icon :icon="people" />
          {{ $msTranslate('clientArea.orders.new.usersNeed') }}
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">
          {{
            $msTranslate({
              key: 'clientArea.orders.quantityOrdered.administrator',
              data: { count: orderDetails.administrators.quantityOrdered },
              count: orderDetails.administrators.quantityOrdered,
            })
          }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">
          {{
            $msTranslate({
              key: 'clientArea.orders.quantityOrdered.standard',
              data: { count: orderDetails.standards.quantityOrdered },
              count: orderDetails.standards.quantityOrdered,
            })
          }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">
          {{
            $msTranslate({
              key: 'clientArea.orders.quantityOrdered.outsider',
              data: { count: orderDetails.outsiders.quantityOrdered },
              count: orderDetails.outsiders.quantityOrdered,
            })
          }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
      <div class="details-list-item">
        <ion-text class="details-list-item__title body-lg">
          <ion-icon :icon="pieChart" />
          {{ $msTranslate('clientArea.orders.new.dataNeeded') }}
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">
          {{ $msTranslate(formatFileSize(orderDetails.storage.quantityOrdered)) }}

          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
      <!-- Starting date -->
      <div
        class="details-list-item"
        v-if="request.organizationId"
      >
        <ion-text class="details-list-item__title body-lg">
          <ion-icon :icon="time" />
          {{ $msTranslate('clientArea.orders.new.startDate') }}
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">
          {{ $msTranslate(I18n.formatDate(orderDetails.created, 'short')) }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
      <!-- Ending date -->
      <div
        class="details-list-item"
        v-if="request.organizationId"
      >
        <ion-text class="details-list-item__title body-lg">
          <ion-icon :icon="time" />
          {{ $msTranslate('clientArea.orders.new.endDate') }}
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">
          {{ $msTranslate(I18n.formatDate(orderDetails.dueDate, 'short')) }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
      <div
        v-if="orderStep === OrderStep.Available && orderDetails"
        class="details-list-item"
      >
        <ion-text class="details-list-item__title body-lg">
          <ion-icon :icon="wallet" />
          {{ $msTranslate('clientArea.orders.invoiceToBePaid.paid') }}
        </ion-text>
        <ion-text class="details-list-item__data subtitles-normal">
          {{
            $msTranslate({
              key: 'clientArea.orders.invoiceToBePaid.price',
              data: { amount: orderDetails.amountDue },
            })
          }}
          <ion-icon
            :icon="checkmark"
            class="checkmark-icon"
          />
        </ion-text>
      </div>
    </div>

    <!-- invoice details -->
    <div
      v-if="orderStep === OrderStep.InvoiceToBePaid && orderDetails"
      class="details-invoice"
    >
      <div class="details-invoice-text">
        <ion-text class="details-invoice-text__title title-h4">
          {{ $msTranslate('clientArea.orders.invoiceToBePaid.title') }}
        </ion-text>
        <ion-text class="details-invoice-text__description body">
          {{ $msTranslate('clientArea.orders.invoiceToBePaid.description') }}
        </ion-text>
      </div>
      <ion-text class="details-invoice-price">
        <span class="title-h4">
          {{
            $msTranslate({
              key: 'clientArea.orders.invoiceToBePaid.price',
              data: { amount: orderDetails.amountDue },
            })
          }}
        </span>
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import { OrderStep } from '@/components/client-area/orders/utils';
import { BmsAccessInstance, BmsOrganization, CustomOrderDetailsResultData, CustomOrderRequest, DataType } from '@/services/bms';
import { IonIcon, IonText } from '@ionic/vue';
import { chatbubbleEllipses, checkmark, people, pieChart, time, wallet } from 'ionicons/icons';
import { I18n, Translatable } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const error = ref<string>('');
const querying = ref(true);
const orderDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);

const props = defineProps<{
  orderStep: OrderStep;
  request: CustomOrderRequest;
  organization: BmsOrganization | undefined;
}>();

function getUserOptions(userNeeds: number): Translatable {
  switch (userNeeds) {
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

onMounted(async () => {
  try {
    querying.value = true;

    // If the order is in a step where no organization is created yet, we don't need to fetch details
    if ([OrderStep.Received, OrderStep.Processing, OrderStep.Confirmed].includes(props.orderStep) || !props.organization) {
      return;
    }

    const orderDetailsRep = await BmsAccessInstance.get().getCustomOrderDetails(props.organization);
    if (orderDetailsRep.isError || orderDetailsRep.data?.type !== DataType.CustomOrderDetails) {
      error.value = 'clientArea.contracts.errors.noInfo';
      return;
    }
    orderDetails.value = orderDetailsRep.data;
  } finally {
    querying.value = false;
  }
});
</script>

<style scoped lang="scss">
.order-content-details {
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
      flex-wrap: wrap;

      &__title {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      &__data {
        color: var(--parsec-color-light-secondary-soft-text);
        padding: 0.5rem 0.75rem;
        background: var(--parsec-color-light-secondary-premiere);
        border-radius: var(--parsec-radius-8);
        max-width: 15rem;
        margin-left: auto;
        text-align: end;
        width: 100%;
        position: relative;

        .checkmark-icon {
          position: absolute;
          right: -1.5rem;
          color: var(--parsec-color-light-primary-700);
          margin-left: 0.5rem;
        }
      }

      &#detail-comment {
        flex-direction: column;
        align-items: flex-start;

        .details-list-item__data {
          max-width: 100%;
          padding: 1rem;
          text-align: start;
        }
      }

      ion-icon {
        flex-shrink: 0;
      }
    }
  }

  .details-invoice {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    margin-top: 0.5rem;
    border-top: 1px solid var(--parsec-color-light-secondary-medium);
    padding-top: 1.5rem;

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &__title {
        color: var(--parsec-color-light-secondary-text);
      }

      &__description {
        color: var(--parsec-color-light-secondary-grey);
      }
    }

    &-price {
      color: var(--parsec-color-light-secondary-soft-text);
      text-align: end;
    }
  }
}
</style>
