// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { APP_VERSION } from '@/services/environment';
import AboutModal from '@/views/about/AboutModal.vue';
import LongPathsSupportModal from '@/views/about/LongPathsSupportModal.vue';
import UpdateAppModal from '@/views/about/UpdateAppModal.vue';
import { modalController } from '@ionic/vue';
import { Answer, MsModalResult } from 'megashark-lib';

export { AboutModal, LongPathsSupportModal, UpdateAppModal };

export async function openAboutModal(): Promise<void> {
  const modal = await modalController.create({
    component: AboutModal,
    cssClass: 'about-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
}

export async function openUpdateAppModal(newVersion: string): Promise<Answer> {
  const modal = await modalController.create({
    component: UpdateAppModal,
    canDismiss: true,
    cssClass: 'update-app-modal',
    backdropDismiss: false,
    componentProps: {
      currentVersion: APP_VERSION,
      targetVersion: newVersion,
    },
  });
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();

  return role === MsModalResult.Confirm ? Answer.Yes : Answer.No;
}
