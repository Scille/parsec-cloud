// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import BugReportModal from '@/components/misc/BugReportModal.vue';
import LogDisplayModal from '@/components/misc/LogDisplayModal.vue';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';

export async function openLogDisplayModal(): Promise<void> {
  const modal = await modalController.create({
    component: LogDisplayModal,
    cssClass: 'log-modal',
  });
  await modal.present();
  await modal.onDidDismiss();
  await modal.dismiss();
}

export async function openBugReportModal(): Promise<MsModalResult> {
  const modal = await modalController.create({
    component: BugReportModal,
    cssClass: 'bug-report-modal',
  });
  await modal.present();
  const { role } = await modal.onDidDismiss();
  await modal.dismiss();
  return (role as MsModalResult) ?? MsModalResult.Cancel;
}
