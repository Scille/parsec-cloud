<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-informative-text>
    {{ $msTranslate('ClaimDeviceModal.subtitles.claimDevice') }}
  </ms-informative-text>
  <div class="info-device">
    <div class="content">
      <ms-image
        :image="Device"
        class="content__device"
      />
      <ion-text class="body content__text">
        {{ $msTranslate('ClaimDeviceModal.currentDevice') }}
      </ion-text>
      <ion-text class="subtitles-sm content__subtext">
        {{ $msTranslate('ClaimDeviceModal.connected') }}
      </ion-text>
    </div>
    <div>
      <ms-image :image="SwapArrows" />
    </div>
    <div class="content">
      <ms-image
        :image="Device"
        class="content__device"
      />
      <ion-text class="body content__text">
        {{ $msTranslate('ClaimDeviceModal.newDevice') }}
      </ion-text>
      <ion-text class="subtitles-sm content__subtext">
        {{ $msTranslate('ClaimDeviceModal.connected') }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import { Device, MsImage, MsInformativeText, SwapArrows } from 'megashark-lib';
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
