<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="upload-status">
    <template v-if="status.totalBytes > 0">
      <div class="upload-status-syncing">
        <ms-spinner
          class="upload-status-syncing__spinner"
          :size="16"
        />
        <div class="upload-status-syncing-text">
          <ion-text class="title-h5 upload-status-syncing-text__title">
            {{ $msTranslate('FoldersPage.ImportFile.syncing.title') }}
          </ion-text>
          <ion-text class="body-sm upload-status-syncing-text__time">
            {{
              $msTranslate({
                key: 'FoldersPage.ImportFile.syncing.timeLeft',
                data: { time: $msTranslate(formatETA(rateCalculator.getEta(status.totalBytes))) },
              })
            }}
          </ion-text>
        </div>
        <ion-text class="button-small upload-status-syncing__files">
          {{ $msTranslate({ key: 'FoldersPage.ImportFile.syncing.filesLeft', count: status.totalFiles }) }}
        </ion-text>
      </div>
    </template>
    <template v-else>
      <ion-icon
        :icon="checkmarkCircle"
        class="upload-status__icon checkmark-icon"
      />
      <ion-text class="title-h5 upload-status__text">{{ $msTranslate('FoldersPage.ImportFile.synced') }}</ion-text>
      <ion-icon
        :icon="close"
        class="upload-status__icon close-icon"
        @click="$emit('close')"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import { TransferRateCalculator } from '@/common/transferRate';
import { UploadProgress } from '@/parsec';
import { formatETA } from '@/services/translation';
import { IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle, close } from 'ionicons/icons';
import { MsSpinner } from 'megashark-lib';

defineProps<{
  status: UploadProgress;
  rateCalculator: TransferRateCalculator;
}>();

defineEmits<{
  (e: 'close'): void;
}>();
</script>

<style scoped lang="scss">
.upload-status {
  display: flex;
  align-items: center;
  padding: 0.75rem 0.5rem 0.75rem 0.75rem;
  background: var(--parsec-color-light-secondary-background);
  gap: 0.375rem;
  min-height: 3rem;

  &__icon {
    font-size: 1.25rem;

    &.checkmark-icon {
      color: var(--parsec-color-light-primary-600);
    }

    &.step-icon {
      color: var(--parsec-color-light-primary-600);
    }

    &.close-icon {
      color: var(--parsec-color-light-secondary-grey);
      margin-left: auto;
      padding: 0.25rem;

      &:hover {
        cursor: pointer;
        background: var(--parsec-color-light-secondary-medium);
        border-radius: var(--parsec-radius-12);
      }
    }
  }

  &__text {
    display: flex;
    align-items: center;
    color: var(--parsec-color-light-secondary-soft-text);
  }

  &-step {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    color: var(--parsec-color-light-secondary-hard-grey);

    &__icon {
      font-size: 1.125rem;
      padding: 0.125rem;
    }

    &__text {
      display: flex;
      align-items: center;
      color: var(--parsec-color-light-secondary-hard-grey);
      font-style: italic;
    }
  }

  &-syncing {
    display: flex;
    gap: 0.5rem;
    flex-grow: 1;

    &__spinner {
      align-self: start;
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.125rem;

      &__title {
        color: var(--parsec-color-light-secondary-soft-text);
      }

      &__time {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }

    &__files {
      color: var(--parsec-color-light-secondary-text);
      margin-left: auto;
    }
  }
}
</style>
