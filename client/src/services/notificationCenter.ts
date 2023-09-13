// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DateTime } from 'luxon';
import { v4 as uuid4 } from 'uuid';
import MsAlertModal, { MsAlertModalConfig } from '@/components/core/ms-modal/MsAlertModal.vue';
import { modalController } from '@ionic/vue';
import { ComposerTranslation } from 'vue-i18n';
import { MsModalResult, MsTheme } from '@/components/core/ms-types';

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

export class NotificationCenter {
  notifications: Notification[];

  constructor(public t: ComposerTranslation) {
    this.notifications = [];
    this.t = t;
  }

  async showModal({
    notification,
    addToList = false,
    trace = false,
  }: {
    notification: Notification,
    addToList?: boolean,
    trace?: boolean,
  }): Promise<void> {
    if (addToList) {
      this.addToList(notification);
    }
    if (trace) {
      this._trace(notification);
    }

    const alertModalConfig: MsAlertModalConfig = {
      theme: this._getModalTheme(notification.level),
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

  async showSnackbar({
    notification,
    addToList = false,
    trace = false,
  }: {
    notification: Notification,
    addToList?: boolean,
    trace?: boolean,
  }): Promise<void> {
    if (addToList) {
      this.addToList(notification);
    }
    if (trace) {
      this._trace(notification);
    }
  }

  private _getModalTheme(notificationLevel: NotificationLevel): MsTheme {
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
