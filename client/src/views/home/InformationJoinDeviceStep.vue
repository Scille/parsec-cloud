<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-informative-text>
    {{ $t('ClaimDeviceModal.subtitles.claimDevice') }}
  </ms-informative-text>
  <div class="info-device">
    <div class="content">
      <img
        src="@/assets/images/device.svg"
        alt="device with Parsec Cloud connected"
        class="content__device"
      >
      <ion-text class="body content__text">
        {{ $t('ClaimDeviceModal.currentDevice') }}
      </ion-text>
      <ion-text class="caption-caption content__subtext">
        {{ $t('ClaimDeviceModal.connected') }}
      </ion-text>
    </div>
    <div>
      <img
        src="@/assets/images/arrows.svg"
        alt="arrows between device"
      >
    </div>
    <div class="content">
      <img
        src="@/assets/images/device.svg"
        alt="device with Parsec Cloud connected"
        class="content__device"
      >
      <ion-text class="body content__text">
        {{ $t('ClaimDeviceModal.newDevice') }}
      </ion-text>
      <ion-text class="caption-caption content__subtext">
        {{ $t('ClaimDeviceModal.connected') }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  caretForward,
} from 'ionicons/icons';
import {
  IonText,
} from '@ionic/vue';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
</script>

<style scoped lang="scss">
.info-device {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 2rem;
  margin-top: 1rem;
}

.content {
  display: flex;
  flex-direction: column;
  align-items: center;

  &__device {
    width: 100%;
    height: auto;
    max-width: 9rem;
  }

  &__text {
    color: var(--parsec-color-light-primary-600);
  }

  &__subtext {
    color: var(--parsec-color-light-success-500);
    position: relative;
  }
}
</style>
