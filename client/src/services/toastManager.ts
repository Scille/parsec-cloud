// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { toastController } from '@ionic/vue';
import { MsTheme } from '@/components/core/ms-types';
import {
  informationCircle,
  checkmarkCircle,
  warning,
  closeCircle,
} from 'ionicons/icons';
import { ComposerTranslation } from 'vue-i18n';

const DEFAULT_TOAST_DURATION = 3000;

export class ToastManager {
  toasts: HTMLIonToastElement[];
  currentToast: HTMLIonToastElement | null = null;

  constructor(public t: ComposerTranslation) {
    this.toasts = [];
    this.t = t;
  }

  async createAndPresent(toastConfig: {
    title?: string,
    icon?: string,
    message: string,
    theme: MsTheme,
    confirmButtonLabel?: string,
    duration?: number,
  }): Promise<any> {
    const toast = await toastController.create({
      header: toastConfig.title,
      message: toastConfig.message,
      cssClass: [
        'notification-toast',
        toastConfig.theme,
      ],
      mode: 'ios',
      duration: toastConfig.duration ?? DEFAULT_TOAST_DURATION,
      icon: toastConfig.theme ? this._getIcon(toastConfig.theme) : toastConfig.icon,
      buttons: [
        {
          text: toastConfig.confirmButtonLabel ?? this.t('Notification.nextButton'),
          role: 'confirm',
          side: 'end',
        },
      ],
    });
    this.toasts.push(toast);

    await this.managePresentQueue();

    const result = await toast.onDidDismiss();
    if (result) {
      this.toasts.splice(this.toasts.indexOf(toast), 1);
      await this.managePresentQueue();
    }
    return toast.onWillDismiss();
  }

  async managePresentQueue(): Promise<any> {
    await this.toasts.at(0)?.present();
  }

  private _getIcon(theme: MsTheme): string {
    switch(theme) {
      case MsTheme.Info:
        return informationCircle;
      case MsTheme.Success:
        return checkmarkCircle;
      case MsTheme.Warning:
        return warning;
      case MsTheme.Error:
        return closeCircle;
    }
    return '';
  }
}
