// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { createDeviceInvitation } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import GreetDeviceModal from '@/views/devices/GreetDeviceModal.vue';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';

export async function openDeviceGreetModal(informationManager: InformationManager): Promise<MsModalResult> {
  const result = await createDeviceInvitation(false);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: 'DevicesPage.greet.errors.startFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return MsModalResult.Cancel;
  }

  const [_, invitationAddr] = result.value.addr;
  const modal = await modalController.create({
    component: GreetDeviceModal,
    canDismiss: true,
    backdropDismiss: false,
    cssClass: 'greet-organization-modal',
    componentProps: {
      informationManager,
      invitationLink: invitationAddr,
      token: result.value.token,
    },
  });
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();
  return role ? (role as MsModalResult) : MsModalResult.Cancel;
}
