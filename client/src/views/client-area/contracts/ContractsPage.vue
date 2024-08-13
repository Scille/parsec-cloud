<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-contract-page">
    <ion-title class="contract__title title-h2"> {{ $msTranslate('clientArea.contracts.title') }}ART14516 </ion-title>
    <div class="contract-header">
      <div class="contract-header-data">
        <!-- duration -->
        <div class="data-item">
          <ion-text class="data-item__title title-h2">{{ $msTranslate('clientArea.contracts.duration') }}</ion-text>
          <div class="data-item-text">
            <ion-text class="data-item-text__value title-h1">2 mois</ion-text>
            <ion-text class="data-item-text__description breadcrumb-normal">15 jours restants</ion-text>
          </div>
          <div class="data-item-divider" />
          <div class="data-item-time">
            <div class="time-column">
              <ion-text class="time-column__title subtitles-sm">{{ $msTranslate('clientArea.contracts.from') }}</ion-text>
              <ion-text class="time-column__value title-h4">26/02/2023</ion-text>
            </div>
            <ion-icon
              class="time-icon"
              :icon="arrowForward"
            />
            <div class="time-column">
              <ion-text class="time-column__title subtitles-sm">{{ $msTranslate('clientArea.contracts.to') }}</ion-text>
              <ion-text class="time-column__value title-h4">26/04/2023</ion-text>
            </div>
          </div>
        </div>
        <!-- amount -->
        <div class="data-item">
          <ion-text class="data-item__title title-h2">{{ $msTranslate('clientArea.contracts.amount') }}</ion-text>
          <div class="data-item-text">
            <ion-text class="data-item-text__value title-h1">674,00 €</ion-text>
            <ion-text class="data-item-text__description breadcrumb-normal">15 jours restants</ion-text>
          </div>
          <div class="data-item-divider" />
        </div>
        <!-- invoice -->
        <div class="data-invoice">
          <div class="data-invoice-item">
            <ms-image
              class="data-invoice-item__image"
              alt="pdf"
              :image="File.Pdf"
            />
            <ion-text class="data-invoice-item__name title-h4">{{ $msTranslate('clientArea.contracts.invoice.title') }} 14541</ion-text>
          </div>
          <div class="data-invoice-text">
            <ion-text class="data-invoice-text__value subtitles-normal">
              <span>{{ $msTranslate('clientArea.contracts.invoice.date') }}</span>
              16 février 2024
            </ion-text>
            <ion-text class="data-invoice-text__value subtitles-normal">
              <span>{{ $msTranslate('clientArea.contracts.invoice.payment') }}</span>
              {{ $msTranslate('clientArea.contracts.invoice.transfer') }}
            </ion-text>
          </div>
          <div class="data-invoice-button">
            <a
              class="custom-button custom-button-fill button-medium"
              download
            >
              {{ $msTranslate('clientArea.invoices.cell.download') }}
            </a>
          </div>
        </div>
      </div>
    </div>

    <div class="contract-content">
      <ion-title class="contract-content__title title-h3">{{ $msTranslate('clientArea.contracts.previewOrganization') }}</ion-title>
      <div class="contract-users">
        <!-- admins-->
        <div class="contract-users-item">
          <div class="item-licence">
            <ion-icon
              :icon="person"
              class="item-licence__icon"
            />
            <div class="item-licence-text">
              <ion-text class="item-licence-text__title title-h4">{{ $msTranslate('clientArea.contracts.user.admin') }}</ion-text>
              <ion-text class="item-licence-text__subtitle subtitles-sm">
                {{ $msTranslate('clientArea.contracts.user.fullAccess') }}
              </ion-text>
            </div>
          </div>
          <div class="item-active">
            <ion-text class="item-active__number title-h1-xl">
              10
              <span class="item-active__label subtitles-normal">/ 15 membres</span>
            </ion-text>
            <div class="progress">
              <div class="progress-bar">
                <div class="progress-bar-used" />
                <div class="progress-bar-unused" />
              </div>
              <ion-text class="progress-text subtitles-sm">5 {{ $msTranslate('clientArea.contracts.user.remaining') }}</ion-text>
            </div>
          </div>
        </div>
        <!-- standards-->
        <div class="contract-users-item">
          <div class="item-licence">
            <ion-icon
              :icon="person"
              class="item-licence__icon"
            />
            <div class="item-licence-text">
              <ion-text class="item-licence-text__title title-h4">{{ $msTranslate('clientArea.contracts.user.standard') }}</ion-text>
              <ion-text class="item-licence-text__subtitle subtitles-sm">
                {{ $msTranslate('clientArea.contracts.user.allFeaturesAccess') }}
              </ion-text>
            </div>
          </div>
          <div class="item-active">
            <ion-text class="item-active__number title-h1-xl">
              98
              <span class="item-active__label subtitles-normal">/ 100 membres</span>
            </ion-text>
            <div class="progress">
              <div class="progress-bar">
                <div class="progress-bar-used" />
                <div class="progress-bar-unused" />
              </div>
              <ion-text class="progress-text subtitles-sm">2 {{ $msTranslate('clientArea.contracts.user.remaining') }}</ion-text>
            </div>
          </div>
        </div>
        <!-- externals-->
        <div class="contract-users-item">
          <div class="item-licence">
            <ion-icon
              :icon="person"
              class="item-licence__icon"
            />
            <div class="item-licence-text">
              <ion-text class="item-licence-text__title title-h4">{{ $msTranslate('clientArea.contracts.user.external') }}</ion-text>
              <ion-text class="item-licence-text__subtitle subtitles-sm">
                {{ $msTranslate('clientArea.contracts.user.rightLimited') }}
              </ion-text>
            </div>
          </div>
          <div class="item-active">
            <ion-text class="item-active__number title-h1-xl">
              356
              <span class="item-active__label subtitles-normal">/ 2000 membres</span>
            </ion-text>
            <div class="progress">
              <div class="progress-bar">
                <div class="progress-bar-used" />
                <div class="progress-bar-unused" />
              </div>
              <ion-text class="progress-text subtitles-sm">1644 {{ $msTranslate('clientArea.contracts.user.remaining') }}</ion-text>
            </div>
          </div>
        </div>
      </div>

      <div class="contract-storage">
        <!-- storage-->
        <div class="contract-storage-item">
          <div class="item-licence">
            <ion-icon
              :icon="pieChart"
              class="item-licence__icon"
            />
            <div class="item-licence-text">
              <ion-text class="item-licence-text__title title-h4">{{ $msTranslate('clientArea.contracts.storage.title') }}</ion-text>
              <ion-text class="item-licence-text__subtitle subtitles-sm">
                {{ $msTranslate('clientArea.contracts.storage.stack') }}
              </ion-text>
            </div>
          </div>
          <div class="item-active">
            <ion-text class="item-active__number title-h1-xl">
              927,8 <span class="title-h4">Go</span>
              <span class="item-active__label subtitles-normal">/ 1 024 Go</span>
            </ion-text>
            <div class="progress">
              <div class="progress-bar">
                <div class="progress-bar-used" />
                <div class="progress-bar-unused" />
              </div>
              <ion-text class="progress-text subtitles-sm">81 % {{ $msTranslate('clientArea.contracts.storage.used') }}</ion-text>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon, IonTitle } from '@ionic/vue';
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { MsImage, File } from 'megashark-lib';
import { arrowForward, person, pieChart } from 'ionicons/icons';
import { BmsOrganization } from '@/services/bms';
import { onMounted } from 'vue';

defineProps<{
  organization: BmsOrganization;
}>();

onMounted(async () => {});
</script>

<style scoped lang="scss">
.client-contract-page {
  display: flex;
  flex-direction: column;
  gap: 2.5rem;
  padding-bottom: 3rem;
}

.contract__title {
  color: var(--parsec-color-light-primary-800);
}

.contract-header {
  display: flex;
  justify-content: space-between;

  &-data {
    display: flex;
    gap: 4rem;
    width: 100%;
  }

  .data-item {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    max-width: 15rem;
    width: 100%;

    &__title {
      color: var(--parsec-color-light-primary-700);
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &__value {
        color: var(--parsec-color-light-secondary-contrast);
      }

      &__description {
        color: var(--parsec-color-light-secondary-grey);
      }
    }

    &-divider {
      width: 100%;
      height: 1px;
      background-color: var(--parsec-color-light-secondary-medium);
    }

    &-time {
      display: flex;
      gap: 1rem;

      .time-column {
        display: flex;
        flex-direction: column;
        gap: 0.125rem;

        &__title {
          color: var(--parsec-color-light-secondary-grey);
        }

        &__value {
          color: var(--parsec-color-light-secondary-800);
        }
      }

      .time-icon {
        color: var(--parsec-color-light-secondary-light);
        font-size: 1.25rem;
        align-self: self-end;
      }
    }
  }

  .data-invoice {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    background: var(--parsec-color-light-secondary-background);
    padding: 1.5rem;
    border-radius: var(--parsec-radius-12);
    max-width: 18rem;
    width: 100%;

    &-item {
      display: flex;
      gap: 0.75rem;
      align-items: center;

      &__image {
        width: 2rem;
        height: 2rem;
      }

      &__name {
        color: var(--parsec-color-light-secondary-soft-text);
      }
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: 1rem;

      &__value {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }

    &-button {
      margin-top: 1rem;

      .custom-button {
        background: var(--parsec-color-light-secondary-contrast);
        color: var(--parsec-color-light-secondary-white);

        &:hover {
          background: var(--parsec-color-light-secondary-text);
        }
      }
    }
  }
}

.contract-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  &__title {
    color: var(--parsec-color-light-primary-700);
  }
}

.contract-users,
.contract-storage {
  display: flex;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  gap: 3rem;
  padding: 1.5rem;
  border-radius: var(--parsec-radius-12);
  width: fit-content;

  &-item {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    width: 275px;
  }

  .item-licence {
    display: flex;
    gap: 1.25rem;

    &__icon {
      color: var(--parsec-color-light-primary-600);
      font-size: 1.5rem;
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
      margin-top: 0.125rem;
      flex: 1;

      &__title {
        color: var(--parsec-color-light-secondary-text);
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }

  .item-active {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    border-radius: var(--parsec-radius-8);
    flex: 1;

    &__number {
      display: flex;
      gap: 0.25rem;
      align-items: first baseline;
      color: var(--parsec-color-light-primary-600);
    }

    &__label {
      color: var(--parsec-color-light-secondary-grey);
      line-height: 0;
    }

    .progress {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &-bar {
        display: flex;
        gap: 0.25rem;
        height: 0.5rem;
        background: var(--parsec-color-light-secondary-background);
        border-radius: var(--parsec-radius-8);
        height: 0.25rem;

        &-used {
          width: 70%;
          background: var(--parsec-color-light-primary-500);
          border-radius: var(--parsec-radius-8);
        }

        &-unused {
          width: 30%;
          background: var(--parsec-color-light-secondary-light);
          border-radius: var(--parsec-radius-8);
        }
      }

      &-text {
        color: var(--parsec-color-light-secondary-hard-grey);
        align-self: flex-end;
      }
    }
  }
}
</style>
