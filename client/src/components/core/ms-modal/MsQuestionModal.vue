<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="subtitle"
    :title-icon="helpCircle"
    :close-button="{visible: false}"
    :cancel-button="{disabled: false, label: $t('QuestionModal.no'), onClick: onNo}"
    :confirm-button="{disabled: false, label: $t('QuestionModal.yes'), onClick: onYes}"
    @on-enter-keyup="onYes"
  />
</template>

<script lang="ts">
import MsQuestionModal from '@/components/core/ms-modal/MsQuestionModal.vue';

export enum Answer {
  No = 0,
  Yes = 1,
}

export async function askQuestion(title: string, subtitle?: string): Promise<Answer> {
  const top = await modalController.getTop();
  if (top) {
    top.classList.add('overlapped-modal');
  }

  const modal = await modalController.create({
    component: MsQuestionModal,
    canDismiss: true,
    backdropDismiss: false,
    // Different if a modal is already opened
    cssClass: top ? 'question-modal-ontop' : 'question-modal',
    componentProps: {
      title: title,
      subtitle: subtitle,
    },
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();

  if (top) {
    top.classList.remove('overlapped-modal');
  }
  return result.role === MsModalResult.Confirm ? Answer.Yes : Answer.No;
}
</script>

<script setup lang="ts">
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { modalController } from '@ionic/vue';
import { MsModalResult } from '@/components/core/ms-types';
import {
  helpCircle,
} from 'ionicons/icons';

defineProps<{
  title: string,
  subtitle?: string,
}>();

async function onYes(): Promise<boolean> {
  const res = await modalController.dismiss(null, MsModalResult.Confirm);
  return res;
}

async function onNo(): Promise<boolean> {
  return await modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style lang="scss" scoped>
.question-modal {
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.question-modal-ontop {
  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  .modal {
    background-color: green;
  }
}
</style>
