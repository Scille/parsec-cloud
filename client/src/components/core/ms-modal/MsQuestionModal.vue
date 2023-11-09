<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="subtitle"
    :title-icon="helpCircle"
    :close-button="{ visible: false }"
    :cancel-button="{
      disabled: false,
      label: $t('QuestionModal.no'),
      onClick: onNo,
    }"
    :confirm-button="{
      disabled: false,
      label: $t('QuestionModal.yes'),
      onClick: onYes,
    }"
    @on-enter-keyup="onYes"
  />
</template>

<script lang="ts">
import MsQuestionModal from '@/components/core/ms-modal/MsQuestionModal.vue';

export enum Answer {
  No = 0,
  Yes = 1,
}

export async function askQuestion(title: string, subtitle?: string, redisplayMainModalOnYes = true): Promise<Answer> {
  const top = await modalController.getTop();
  if (top) {
    top.classList.add('overlapped-modal');
  }

  const modal = await modalController.create({
    component: MsQuestionModal,
    canDismiss: true,
    backdropDismiss: false,
    // Different if a modal is already opened
    cssClass: 'question-modal',
    componentProps: {
      title: title,
      subtitle: subtitle,
    },
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();

  const answer = result.role === MsModalResult.Confirm ? Answer.Yes : Answer.No;

  if (top) {
    if (answer === Answer.No) {
      top.classList.remove('overlapped-modal');
    }
    // In most cases, we use askQuestion to dismiss a main modal process,
    // If we don't keep the main modal hidden on Yes, there is a disgraceful blink before the dismiss.
    // It's not really pretty but worst case is you forget to set the argument and the main modal blinks, instead of causing potentiel bugs.
    if (answer === Answer.Yes && redisplayMainModalOnYes) {
      top.classList.remove('overlapped-modal');
    }
  }
  return answer;
}
</script>

<script setup lang="ts">
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { modalController } from '@ionic/vue';
import { MsModalResult } from '@/components/core/ms-types';
import { helpCircle } from 'ionicons/icons';

defineProps<{
  title: string;
  subtitle?: string;
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
