// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { ConnectionHandle } from '@/parsec';
import { getConnectionHandle } from '@/router';
import { modalController } from '@ionic/vue';
import { MsAlertModal, MsAlertModalConfig, MsReportTheme, ToastManager, Translatable } from 'megashark-lib';

export const InformationManagerKey = 'information';

export enum InformationLevel {
  Info = 'INFO',
  Success = 'SUCCESS',
  Warning = 'WARNING',
  Error = 'ERROR',
}

export interface InformationOptions {
  message: Translatable;
  level: InformationLevel;
  title?: Translatable;
}

export class Information {
  message: Translatable;
  level: InformationLevel;
  title?: Translatable;

  constructor({ message, level, title }: InformationOptions) {
    this.message = message;
    this.level = level;
    this.title = title;
  }

  get theme(): MsReportTheme {
    switch (this.level) {
      case InformationLevel.Info:
        return MsReportTheme.Info;
      case InformationLevel.Success:
        return MsReportTheme.Success;
      case InformationLevel.Warning:
        return MsReportTheme.Warning;
      case InformationLevel.Error:
        return MsReportTheme.Error;
    }
  }
}

export enum PresentationMode {
  Console = 1 << 0,
  Toast = 1 << 2,
  Modal = 1 << 3,
}

export class InformationManager {
  toastManager: ToastManager;
  handle: ConnectionHandle | null;

  constructor(handle: ConnectionHandle | null = null) {
    this.toastManager = new ToastManager();
    this.handle = handle;
  }

  private async showModal(information: Information): Promise<void> {
    const alertModalConfig: MsAlertModalConfig = {
      title: information.title,
      theme: information.theme,
      message: information.message,
    };

    await this._createAndPresentModal(alertModalConfig);
  }

  private async _createAndPresentModal(modalConfig: MsAlertModalConfig): Promise<any> {
    const top = await modalController.getTop();
    if (top) {
      // Should not an information modal if one is already
      // opened
      if (top.classList.contains('information-modal')) {
        console.log('Avoid overlapping information modals');
        return;
      }
      top.classList.add('overlapped-modal');
    }
    const modal = await modalController.create({
      component: MsAlertModal,
      canDismiss: true,
      cssClass: 'information-modal',
      componentProps: modalConfig,
    });
    await modal.present();
    const result = await modal.onWillDismiss();
    await modal.dismiss();

    if (top) {
      top.classList.remove('overlapped-modal');
    }
    return result;
  }

  private async showToast(information: Information): Promise<void> {
    this.toastManager.createAndPresent({
      theme: information.theme,
      message: information.message,
    });
  }

  async present(information: Information, presentationMode: PresentationMode): Promise<void> {
    if (presentationMode & PresentationMode.Modal && presentationMode & PresentationMode.Toast) {
      console.error('Error: forbidden usage, toasts and modals are not meant to be used at the same time.');
    }
    if (presentationMode & PresentationMode.Console) {
      this._trace(information);
    }
    if (presentationMode & PresentationMode.Toast) {
      // Only show toasts is the organization that generated it is the current one
      if (this.handle === getConnectionHandle()) {
        await this.showToast(information);
      } else {
        console.log('Organization is not currently focused, toast is ignored');
      }
    }
    if (presentationMode & PresentationMode.Modal) {
      // Only show modals is the organization that generated it is the current one
      if (this.handle === getConnectionHandle()) {
        await this.showModal(information);
      } else {
        console.log('Organization is not currently focused, modal is ignored');
      }
    }
  }

  private _trace(information: Information): void {
    console.trace(`[${information.level}] ${information.message}`);
  }
}
