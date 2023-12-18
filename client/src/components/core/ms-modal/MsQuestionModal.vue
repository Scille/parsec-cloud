<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="subtitle"
    :close-button="{ visible: false }"
    :cancel-button="{
      disabled: false,
      label: noText || $t('QuestionModal.no'),
      onClick: onNo,
    }"
    :confirm-button="{
      disabled: false,
      label: yesText || $t('QuestionModal.yes'),
      onClick: onYes,
      theme: yesIsDangerous ? MsReportTheme.Error : undefined,
    }"
    @on-enter-keyup="onYes"
  />
</template>

<script setup lang="ts">
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { MsModalResult } from '@/components/core/ms-modal/types';
import { MsReportTheme } from '@/components/core/ms-types';
import { modalController } from '@ionic/vue';

defineProps<{
  title: string;
  subtitle: string;
  yesText?: string;
  noText?: string;
  yesIsDangerous?: boolean;
}>();

async function onYes(): Promise<boolean> {
  const res = await modalController.dismiss(null, MsModalResult.Confirm);
  return res;
}

async function onNo(): Promise<boolean> {
  return await modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style lang="scss" scoped></style>
