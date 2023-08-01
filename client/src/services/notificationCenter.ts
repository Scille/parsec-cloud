// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DateTime } from 'luxon';
import { v4 as uuid4 } from 'uuid';

export enum NotificationLevel {
  Info = 'INFO',
  Success = 'SUCCESS',
  Warning = 'WARNING',
  Error = 'ERROR',
}

export interface Notification {
  id: string,
  title: string,
  message: string,
  level: NotificationLevel,
  read: boolean,
  time: DateTime,
  data: object,
}

export class NotificationCenter {
  notifications: Notification[];

  constructor() {
    this.notifications = [];
  }

  async showModal(
    title: string,
    message: string,
    level: NotificationLevel,
    addToList: boolean,
    trace = false,
    data = {},
  ): Promise<void> {
    const notif: Notification = {
      id: uuid4(),
      title: title,
      message: message,
      level: level,
      read: false,
      time: DateTime.now(),
      data: data,
    };
    if (trace) {
      this.trace(notif);
    }
    if (addToList) {
      this.notifications.push(notif);
    }
  }

  async showSnackbar(
    title: string,
    message: string,
    level: NotificationLevel,
    addToList: boolean,
    trace: false,
    data = {},
  ): Promise<void> {
    const notif: Notification = {
      id: uuid4(),
      title: title,
      message: message,
      level: level,
      read: false,
      time: DateTime.now(),
      data: data,
    };
    if (trace) {
      this.trace(notif);
    }
    if (addToList) {
      this.notifications.push(notif);
    }
  }

  async addToList(title: string, message: string, level: NotificationLevel, trace = false, data = {}): Promise<void> {
    const notif: Notification = {
      id: uuid4(),
      title: title,
      message: message,
      level: level,
      read: false,
      time: DateTime.now(),
      data: data,
    };
    if (trace) {
      this.trace(notif);
    }
    this.notifications.push(notif);
  }

  markAsRead(id: string, read = true): void {
    const notif = this.notifications.find((n) => n.id === id);

    if (notif) {
      notif.read = read;
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

  private trace(notif: Notification): void {
    console.trace(`[${notif.level}] ${notif.title} : ${notif.message}`);
  }
}
