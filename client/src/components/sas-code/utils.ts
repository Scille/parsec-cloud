// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import SasCodeInfoModal from '@/components/sas-code/SasCodeInfoModal.vue';
import { modalController } from '@ionic/vue';
import { Answer, MsModalResult, useWindowSize } from 'megashark-lib';

const { isLargeDisplay: isLargeDisplay } = useWindowSize();

export async function openSasCodeInfoModal(): Promise<Answer> {
  const modal = await modalController.create({
    component: SasCodeInfoModal,
    handle: false,
    breakpoints: isLargeDisplay ? undefined : [0, 1],
    expandToScroll: false,
    initialBreakpoint: isLargeDisplay ? undefined : 1,
    cssClass: 'question-modal',
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();

  return result.role === MsModalResult.Confirm ? Answer.Yes : Answer.No;
}
