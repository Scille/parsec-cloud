<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="upload-status">
    <template v-if="status.totalFiles > 0">
      <div class="upload-status-syncing">
        <ms-spinner
          class="upload-status-syncing__spinner"
          :size="16"
        />
        <div class="upload-status-syncing-text">
          <ion-text class="upload-status-syncing-text__title">
            {{ $msTranslate('FoldersPage.ImportFile.syncing.title') }}
          </ion-text>
          <ion-text class="upload-status-syncing-text__time">
            <span v-if="eta !== Infinity && eta > 0">
              {{
                $msTranslate({
                  key: 'FoldersPage.ImportFile.syncing.timeLeft',
                  data: { time: $msTranslate(formatETA(eta)) },
                })
              }}
            </span>
            <span v-else>
              {{ $msTranslate('FoldersPage.ImportFile.syncing.calculating') }}
            </span>
          </ion-text>
        </div>
        <ion-text class="upload-status-syncing__files">
          {{ $msTranslate({ key: 'FoldersPage.ImportFile.syncing.filesLeft', count: status.totalFiles }) }}
        </ion-text>
      </div>
    </template>
    <template v-else>
      <ion-icon
        :icon="checkmarkCircle"
        class="upload-status__icon checkmark-icon"
      />
      <ion-text class="upload-status__text">{{ $msTranslate('FoldersPage.ImportFile.synced') }}</ion-text>
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
import { onMounted, onUnmounted, ref } from 'vue';

const ETA_REFRESH_INTERVAL_MS = 2500;

const props = defineProps<{
  status: UploadProgress;
  rateCalculator: TransferRateCalculator;
}>();

const eta = ref(Infinity);
let refreshInterval: ReturnType<typeof setInterval> | undefined;

onMounted(() => {
  refreshInterval = setInterval(() => {
    eta.value = props.rateCalculator.getEta();
  }, ETA_REFRESH_INTERVAL_MS);
});

onUnmounted(() => {
  clearInterval(refreshInterval);
});

defineEmits<{
  (e: 'close'): void;
}>();
</script>

<style scoped lang="scss">
.upload-status {
  display: flex;
  align-items: center;
  padding: ms.spacing('padding-2xl') ms.spacing('padding-lg') ms.spacing('padding-2xl') ms.spacing('padding-2xl');
  background: ms.color('surface-base-page');
  gap: ms.spacing('gap-md');
  min-height: 3rem;

  &__icon {
    font-size: 1.25rem;

    &.checkmark-icon {
      color: ms.color('icon-brand-default');
    }

    &.step-icon {
      color: ms.color('icon-brand-default');
    }

    &.close-icon {
      color: ms.color('icon-neutral-default');
      margin-left: auto;
      padding: ms.spacing('padding-sm');

      &:hover {
        cursor: pointer;
        background: ms.color('surface-neutral-default-subtle-hover');
        border-radius: ms.radius('2xl');
      }
    }
  }

  &__text {
    @include ms.font('heading-h6');
    display: flex;
    align-items: center;
    color: ms.color('text-base-heading');
  }

  &-step {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-md');
    color: ms.color('text-base-description');

    &__icon {
      font-size: 1.125rem;
      padding: ms.spacing('padding-xs');
    }

    &__text {
      display: flex;
      align-items: center;
      color: ms.color('text-base-description');
      font-style: italic;
    }
  }

  &-syncing {
    display: flex;
    gap: ms.spacing('gap-lg');
    flex-grow: 1;

    &__spinner {
      align-self: start;
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-xs');

      &__title {
        @include ms.font('heading-h6');
        color: ms.color('text-base-heading');
      }

      &__time {
        @include ms.font('body-sm-regular');
        color: ms.color('text-base-description');
      }
    }

    &__files {
      @include ms.font('label-sm-medium');
      color: ms.color('text-base-body');
      margin-left: auto;
    }
  }
}
</style>
