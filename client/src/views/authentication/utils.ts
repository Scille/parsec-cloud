// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { DeviceSaveStrategy, ParsecAddr } from '@/parsec';
import ChooseAuthenticationModal from '@/views/authentication/ChooseAuthenticationModal.vue';
import { modalController } from '@ionic/vue';

export async function openChooseAuthenticationModal(serverAddr: ParsecAddr): Promise<DeviceSaveStrategy | undefined> {
  const modal = await modalController.create({
    cssClass: 'choose-authentication-modal',
    showBackdrop: true,
    backdropDismiss: false,
    component: ChooseAuthenticationModal,
    componentProps: {
      serverAddr: serverAddr,
      title: 'Authentication.chooseModal.title',
      subtitle: 'Authentication.chooseModal.subtitle',
    },
  });
  await modal.present();
  const { data } = await modal.onDidDismiss();
  await modal.dismiss();
  return data.saveStrategy;
}
