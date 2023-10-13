// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DateTime } from 'luxon';
import { v4 as uuid4 } from 'uuid';
import MsAlertModal, { MsAlertModalConfig } from '@/components/core/ms-modal/MsAlertModal.vue';
import { modalController } from '@ionic/vue';
import { ComposerTranslation } from 'vue-i18n';
import { MsModalResult, MsTheme } from '@/components/core/ms-types';
import { ToastManager } from '@/services/toastManager';

// Re-export so everything can be imported from this file
export { NotificationKey } from '@/common/injectionKeys';

export enum NotificationLevel {
  Info = 'INFO',
  Success = 'SUCCESS',
  Warning = 'WARNING',
  Error = 'ERROR',
}

export class Notification {
  id: string;
  time: DateTime;
  read: boolean;
  message: string;
  level: NotificationLevel;
  data?: object;
  title?: string;
  constructor({
    message,
    level,
    data = {},
    title,
  }: {
    message: string,
    level: NotificationLevel,
    data?: object,
    title?: string,
  }) {
    this.id = uuid4();
    this.time = DateTime.now();
    this.read = false;
    this.message = message;
    this.level = level;
    this.data = data;
    this.title = title;
  }
}

export interface NotificationOptions {
  addToList?: boolean,
  trace?: boolean,
}

export interface NotificationToastOptions extends NotificationOptions {
  duration?: number,
}

const DEFAULT_NOTIFICATION_DURATION = 5000;

export class NotificationCenter {
  notifications: Notification[];
  toastManager: ToastManager;

  constructor(public t: ComposerTranslation) {
    this.notifications = [];
    this.t = t;
    this.toastManager = new ToastManager(this.t);
  }

  async showModal(
    notification: Notification,
    options?: NotificationOptions,
  ): Promise<void> {
    if (options && options.addToList) {
      this.addToList(notification);
    }
    if (options && options.trace) {
      this._trace(notification);
    }

    const alertModalConfig: MsAlertModalConfig = {
      theme: this._getMsTheme(notification.level),
      message: notification.message,
    };
    if (notification.title) {
      alertModalConfig.title = notification.title;
    }

    const result = await this._createAndPresentModal(alertModalConfig);
    if (result && result.role === MsModalResult.Confirm) {
      this.markAsRead(notification.id);
    }
  }

  async _createAndPresentModal(modalConfig: MsAlertModalConfig): Promise<any> {
    const modal = await modalController.create({
      component: MsAlertModal,
      canDismiss: true,
      cssClass: 'notification-modal',
      componentProps: modalConfig,
    });
    await modal.present();
    return modal.onWillDismiss();
  }

  async showToast(
    notification: Notification,
    options?: NotificationToastOptions,
  ): Promise<void> {
    if (options && options.addToList) {
      this.addToList(notification);
    }
    if (options && options.trace) {
      this._trace(notification);
    }

    this.toastManager.createAndPresent({
      theme: this._getMsTheme(notification.level),
      message: notification.message,
      title: notification.title,
      duration: DEFAULT_NOTIFICATION_DURATION,
    }).then(async (result) => {
      if (result && result.role === 'confirm') {
        this.markAsRead(notification.id);
      }
    });
  }

  private _getMsTheme(notificationLevel: NotificationLevel): MsTheme {
    switch(notificationLevel) {
      case NotificationLevel.Info:
        return MsTheme.Info;
      case NotificationLevel.Success:
        return MsTheme.Success;
      case NotificationLevel.Warning:
        return MsTheme.Warning;
      case NotificationLevel.Error:
        return MsTheme.Error;
    }
  }

  async addToList(notification: Notification): Promise<void> {
    this.notifications.push(notification);
  }

  markAsRead(id: string, read = true): void {
    const notification = this.notifications.find((n) => n.id === id);

    if (notification) {
      notification.read = read;
    }
  }

  getNotifications(): Notification[] {
    return this.notifications;
  }

  hasUnreadNotifications(): boolean {
    return this.notifications.some((n) => n.read === false);
  }

  clear(): void {
    this.notifications = [];
  }

  private _trace(notification: Notification): void {
    console.trace(`[${notification.level}] ${notification.title} : ${notification.message}`);
  }
}
