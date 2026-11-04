<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      <ion-text class="content__text">
        {{ $msTranslate('ClaimDeviceModal.currentDevice') }}
      </ion-text>
      <ion-text class="content__subtext">
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
      <ion-text class="content__text">
        {{ $msTranslate('ClaimDeviceModal.newDevice') }}
      </ion-text>
      <ion-text class="content__subtext">
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
  gap: ms.spacing('gap-5xl');
  margin-top: ms.spacing('padding-3xl');
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
    @include ms.font('body-md-regular');
    color: ms.color('text-brand-default');
  }

  &__subtext {
    @include ms.font('body-md-medium');
    color: ms.color('text-success-default');
    position: relative;
  }
}
</style>
