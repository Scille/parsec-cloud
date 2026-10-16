// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import BugReportModal from '@/components/misc/BugReportModal.vue';
import LogDisplayModal from '@/components/misc/LogDisplayModal.vue';
import { isWeb } from '@/parsec';
import { formatLogEntry, GUILogger, libparsecLogger } from '@/services/webLogger';
import { modalController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';

export async function openLogDisplayModal(): Promise<void> {
  const top = await modalController.getTop();
  if (top) {
    top.classList.add('overlapped-modal');
  }
  const modal = await modalController.create({
    component: LogDisplayModal,
    cssClass: 'log-modal',
  });
  await modal.present();
  await modal.onDidDismiss();
  await modal.dismiss();
  if (top) {
    top.classList.remove('overlapped-modal');
  }
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

export async function getLogs(source: 'GUI' | 'libparsec'): Promise<string> {
  if (isWeb()) {
    const logEntries = source === 'GUI' ? await GUILogger.getEntries() : await libparsecLogger.getEntries();
    return logEntries.map((entry) => formatLogEntry(entry)).join('\n');
  } else {
    const logs = await new Promise<string>((resolve) => {
      const timeoutId = setTimeout(() => {
        window.nativeAPI.log('warn', 'Took too long to retrieve the logs.');
        resolve('');
      }, 10000);

      window.nativeAPI.receive('parsec-log-records', async (resSource: 'GUI' | 'libparsec', logs: string) => {
        if (resSource === source) {
          clearTimeout(timeoutId);
          resolve(logs);
        }
      });

      window.nativeAPI.getLogs(source);
    });
    return logs;
  }
}
