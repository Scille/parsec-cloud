// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import SettingsModal from '@/views/settings/SettingsModal.vue';
import { modalController } from '@ionic/vue';

export async function openSettingsModal(): Promise<void> {
  const modal = await modalController.create({
    component: SettingsModal,
    cssClass: 'settings-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
}
