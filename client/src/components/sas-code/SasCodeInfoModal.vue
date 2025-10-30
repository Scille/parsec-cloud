<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="SasCodeChoice.info.title"
    :close-button="{ visible: false }"
    :cancel-button="{
      disabled: false,
      label: 'SasCodeChoice.info.noText',
      onClick: onNo,
    }"
    :confirm-button="{
      disabled: false,
      label: 'SasCodeChoice.info.yesText',
      onClick: onYes,
    }"
  >
    <div class="content">
      <ion-text class="body">
        {{ $msTranslate('SasCodeChoice.info.message.line1') }}
      </ion-text>
      <ion-text class="body">
        {{ $msTranslate('SasCodeChoice.info.message.line2') }}
      </ion-text>
      <ion-text class="body">
        {{ $msTranslate('SasCodeChoice.info.message.line3') }}
      </ion-text>
      <ion-text class="body">
        {{ $msTranslate('SasCodeChoice.info.message.line4') }}
      </ion-text>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { MsModal, MsModalResult } from 'megashark-lib';
import { IonText, modalController } from '@ionic/vue';

async function onYes(): Promise<boolean> {
  return await modalController.dismiss(null, MsModalResult.Confirm);
}

async function onNo(): Promise<boolean> {
  return await modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style lang="scss" scoped>
.content {
  margin: 1rem;
  display: flex;
  flex-direction: column;
}
</style>
