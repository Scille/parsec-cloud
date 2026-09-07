// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DeviceSaveStrategy, ParsecAddr } from '@/parsec';
import ChooseAuthenticationModal from '@/views/authentication/ChooseAuthenticationModal.vue';
import { modalController } from '@ionic/vue';

export async function openChooseAuthenticationModal(serverAddr: ParsecAddr): Promise<DeviceSaveStrategy | undefined> {
  const modal = await modalController.create({
    cssClass: 'choose-authentication-modal',
    showBackdrop: true,
    component: ChooseAuthenticationModal,
    componentProps: {
      serverAddr: serverAddr,
      title: 'CHOOSE AUTHENTICATION',
    },
  });
  await modal.present();
  const { data } = await modal.onDidDismiss();
  await modal.dismiss();
  return data.saveStrategy;
}
