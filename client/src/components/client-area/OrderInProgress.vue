<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="order-progress">
    <!-- order header -->
    <div class="order-header">
      <ion-text class="order-header__title title-h3">
        {{
          $msTranslate({
            key: 'clientArea.orders.progress.orderNumber',
            data: { number: orderNumber },
          })
        }}
        <span class="order-tag button-small">Reçue</span>
      </ion-text>
      <ion-text class="order-header__date subtitles-normal">
        <span class="date-title body-sm">
          {{ $msTranslate('Date de la commande') }}
        </span>
        {{ $msTranslate('17 Nov. 2024') }}
      </ion-text>
    </div>

    <!-- order details -->
    <div class="order-content">
      <div class="order-content-details">
        <ion-text class="order-content-details-header subtitles-sm">
          {{ $msTranslate('Détails de la commande') }}
        </ion-text>
        <div class="details-list">
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="people" />
              {{ $msTranslate('Nombre de personnes') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">{{ $msTranslate('100 - 200 utilisateurs') }}</ion-text>
          </div>
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="pieChart" />
              {{ $msTranslate('Volume de données') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">{{ $msTranslate('500 Go - 1 To (stockage)') }}</ion-text>
          </div>
        </div>

        <div class="details-list">
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="time" />
              {{ $msTranslate('Date de début') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">{{ $msTranslate('-') }}</ion-text>
          </div>
          <div class="details-list-item">
            <ion-text class="details-list-item__title body-lg">
              <ion-icon :icon="time" />
              {{ $msTranslate('Date de fin') }}
            </ion-text>
            <ion-text class="details-list-item__data subtitles-normal">{{ $msTranslate('-') }}</ion-text>
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

const orderNumber = 13453;
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

  &__title {
    display: flex;
    align-items: center;
    color: var(--parsec-color-light-secondary-text);
    gap: 1rem;

    .order-tag {
      background-color: var(--parsec-color-tags-indigo100);
      color: var(--parsec-color-tags-indigo500);
      padding: 0.15rem 0.5rem;
      border-radius: var(--parsec-radius-32);
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
