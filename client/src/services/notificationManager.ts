// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsAlertModal, MsAlertModalConfig, MsModalResult, MsReportTheme } from '@/components/core';
import { ToastManager } from '@/services/toastManager';
import { modalController } from '@ionic/vue';
import { DateTime } from 'luxon';
import { v4 as uuid4 } from 'uuid';
import { Ref, computed, ref } from 'vue';

export const NotificationKey = 'notification';

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
  title: string;
  message: string;
  level: NotificationLevel;
  data?: object;
  constructor({ title, level, data = {}, message }: { title: string; message: string; level: NotificationLevel; data?: object }) {
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
  addToList?: boolean;
  trace?: boolean;
}

export interface NotificationToastOptions extends NotificationOptions {
  duration?: number;
}

const DEFAULT_NOTIFICATION_DURATION = 5000;

export class NotificationManager {
  notifications: Ref<Notification[]>;
  unreadCount: Ref<number> = computed(() => this.notifications.value.filter((n) => n.read === false).length);
  toastManager: ToastManager;

  constructor() {
    this.notifications = ref([]);
    this.toastManager = new ToastManager();
  }

  async showModal(notification: Notification, options?: NotificationOptions): Promise<void> {
    if (options && options.addToList) {
      this.addToList(notification);
    }
    if (options && options.trace) {
      this._trace(notification);
    }

    const alertModalConfig: MsAlertModalConfig = {
      theme: this._getMsReportTheme(notification.level),
      title: notification.title,
      message: notification.message,
    };

    const result = await this._createAndPresentModal(alertModalConfig);
    if (result && result.role === MsModalResult.Confirm) {
      this.markAsRead(notification.id);
    }
  }

  async _createAndPresentModal(modalConfig: MsAlertModalConfig): Promise<any> {
    const top = await modalController.getTop();
    if (top) {
      top.classList.add('overlapped-modal');
    }

    const modal = await modalController.create({
      component: MsAlertModal,
      canDismiss: true,
      cssClass: 'notification-modal',
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

  async showToast(notification: Notification, options?: NotificationToastOptions): Promise<void> {
    if (options && options.addToList) {
      this.addToList(notification);
    }
    if (options && options.trace) {
      this._trace(notification);
    }

    this.toastManager
      .createAndPresent({
        theme: this._getMsReportTheme(notification.level),
        title: notification.title,
        message: notification.message,
        duration: DEFAULT_NOTIFICATION_DURATION,
      })
      .then(async (result) => {
        if (result && result.role === 'confirm') {
          this.markAsRead(notification.id);
        }
      });
  }

  private _getMsReportTheme(notificationLevel: NotificationLevel): MsReportTheme {
    switch (notificationLevel) {
      case NotificationLevel.Info:
        return MsReportTheme.Info;
      case NotificationLevel.Success:
        return MsReportTheme.Success;
      case NotificationLevel.Warning:
        return MsReportTheme.Warning;
      case NotificationLevel.Error:
        return MsReportTheme.Error;
    }
  }

  addToList(notification: Notification): void {
    this.notifications.value.unshift(notification);
  }

  markAsRead(id: string, read = true): void {
    const notification = this.notifications.value.find((n) => n.id === id);

    if (notification) {
      notification.read = read;
    }
  }

  getNotifications(): Notification[] {
    return this.notifications.value;
  }

  hasUnreadNotifications(): boolean {
    return this.unreadCount.value > 0;
  }

  getUnreadCount(): Ref<number> {
    return this.unreadCount;
  }

  clear(): void {
    this.notifications.value = [];
  }

  private _trace(notification: Notification): void {
    console.trace(`[${notification.level}] ${notification.title} : ${notification.message}`);
  }
}
