// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { alertController } from '@ionic/vue';
import { MsModalResult } from '@/components/core/ms-modal/MsModal.vue';

export async function createAlert(
  header: string,
  message: string,
  cancelLabel: string,
  okLabel: string,
): Promise<HTMLIonAlertElement> {
  return await alertController.create({
    header: header,
    message: message,
    buttons: [
      {
        text: cancelLabel,
        role: MsModalResult.Cancel,
      },
      {
        text: okLabel,
        role: MsModalResult.Confirm,
      },
    ],
  });
}
