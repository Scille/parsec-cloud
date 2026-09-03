<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="HOW DOES IT WORK"
    :cancel-button="{
      label: 'PREVIOUS',
      disabled: false,
      onClick: onPreviousClicked,
    }"
    :confirm-button="{
      label: 'NEXT',
      disabled: false,
      onClick: onNextClicked,
    }"
  >
    <div
      class="step"
      v-if="step === 0"
    >
      FIRST TEXT
    </div>
    <div
      class="step"
      v-if="step === 1"
    >
      SECOND TEXT
    </div>
    <div
      class="step"
      v-if="step === 2"
    >
      THIRD TEXT
    </div>
    <span>{{ `STEP ${step + 1} OF 3` }}</span>
  </ms-modal>
</template>

<script setup lang="ts">
import { modalController } from '@ionic/core';
import { MsModal, MsModalResult } from 'megashark-lib';
import { ref } from 'vue';

const step = ref(0);

async function onPreviousClicked(): Promise<boolean> {
  if (step.value <= 0) {
    return await modalController.dismiss(undefined, MsModalResult.Cancel);
  } else {
    step.value -= 1;
    return false;
  }
}

async function onNextClicked(): Promise<boolean> {
  if (step.value >= 2) {
    return await modalController.dismiss(undefined, MsModalResult.Confirm);
  } else {
    step.value += 1;
    return false;
  }
}
</script>

<style scoped lang="scss">
.step {
  height: 200px;
  background-color: pink;
}
</style>
