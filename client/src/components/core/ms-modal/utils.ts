// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { modalController } from '@ionic/vue';
import MsQuestionModal from '@/components/core/ms-modal/MsQuestionModal.vue';
import MsPasswordInputModal from '@/components/core/ms-modal/MsPasswordInputModal.vue';
import MsTextInputModal from '@/components/core/ms-modal/MsTextInputModal.vue';
import FolderSelectionModal from '@/components/core/ms-modal/FolderSelectionModal.vue';
import { FsPath } from '@/parsec';
import { FolderSelectionOptions, GetPasswordOptions, GetTextOptions, Answer, MsModalResult } from '@/components/core/ms-modal/types';

export async function askQuestion(title: string, subtitle?: string, redisplayMainModalOnYes = true): Promise<Answer> {
  const top = await modalController.getTop();
  if (top) {
    top.classList.add('overlapped-modal');
  }

  const modal = await modalController.create({
    component: MsQuestionModal,
    canDismiss: true,
    backdropDismiss: false,
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

export async function getPasswordFromUser(options: GetPasswordOptions): Promise<string | null> {
  const modal = await modalController.create({
    component: MsPasswordInputModal,
    canDismiss: true,
    cssClass: 'password-input-modal',
    componentProps: options,
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();
  return result.role === MsModalResult.Confirm ? result.data : null;
}

export async function getTextInputFromUser(options: GetTextOptions): Promise<string | null> {
  const modal = await modalController.create({
    component: MsTextInputModal,
    canDismiss: true,
    cssClass: 'text-input-modal',
    componentProps: {
      title: options.title,
      subtitle: options.subtitle,
      trim: options.trim,
      validator: options.validator,
      inputLabel: options.inputLabel,
      placeholder: options.placeholder,
      okButtonText: options.okButtonText,
      defaultValue: options.defaultValue,
    },
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();
  return result.role === MsModalResult.Confirm ? result.data : null;
}

export async function selectFolder(options: FolderSelectionOptions): Promise<FsPath | null> {
  const modal = await modalController.create({
    component: FolderSelectionModal,
    canDismiss: true,
    cssClass: 'folder-selection-modal',
    componentProps: options,
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();
  return result.role === MsModalResult.Confirm ? result.data : null;
}
