// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsReportTheme } from '@/components/core';
import { translate } from '@/services/translation';
import { toastController } from '@ionic/vue';
import { checkmark, closeCircle, information, warning } from 'ionicons/icons';

const DEFAULT_TOAST_DURATION = 5000;

export class ToastManager {
  toasts: HTMLIonToastElement[];
  currentToast: HTMLIonToastElement | null = null;

  constructor() {
    this.toasts = [];
  }

  async createAndPresent(toastConfig: {
    title?: string;
    icon?: string;
    message: string;
    theme: MsReportTheme;
    confirmButtonLabel?: string;
    duration?: number;
  }): Promise<any> {
    const duration = toastConfig.duration || DEFAULT_TOAST_DURATION;

    const toast = await toastController.create({
      header: toastConfig.title,
      message: toastConfig.message,
      cssClass: ['notification-toast body', toastConfig.theme],
      mode: 'ios',
      duration: duration,
      icon: toastConfig.theme ? this._getIcon(toastConfig.theme) : toastConfig.icon,
      buttons: [
        {
          text: toastConfig.confirmButtonLabel ?? translate('Notification.nextButton'),
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
    const currentToast = this.toasts.at(0);

    if (currentToast) {
      document.documentElement.style.setProperty('--ms-toast-duration', `${currentToast.duration}ms`);
      // when the class active is added progress bar width is set to 0%
      setTimeout(() => {
        currentToast.classList.add('active');
      }, 100);
      await currentToast.present();
    }
  }

  private _getIcon(theme: MsReportTheme): string {
    switch (theme) {
      case MsReportTheme.Info:
        return information;
      case MsReportTheme.Success:
        return checkmark;
      case MsReportTheme.Warning:
        return warning;
      case MsReportTheme.Error:
        return closeCircle;
    }
  }
}
