<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="BEGIN RECOVERY"
    :close-button="{
      visible: true,
    }"
  >
    <ms-input
      v-model="link"
      :validator="shamirRecoveryLinkValidator"
    />
    <ion-button
      :disabled="!canStart"
      @click="startClicked"
    >
      START RECOVERY
    </ion-button>
    <div>SOME ADDITIONAL TEXT</div>
  </ms-modal>
</template>

<script setup lang="ts">
import { shamirRecoveryLinkValidator } from '@/common/validators';
import { modalController } from '@ionic/core';
import { IonButton } from '@ionic/vue';
import { asyncComputed, MsInput, MsModal, MsModalResult, Validity } from 'megashark-lib';
import { ref } from 'vue';

const link = ref('');

const canStart = asyncComputed(async () => {
  const result = await shamirRecoveryLinkValidator(link.value);
  return result.validity === Validity.Valid;
});

async function startClicked(): Promise<void> {
  await modalController.dismiss({ link: link.value }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss"></style>
