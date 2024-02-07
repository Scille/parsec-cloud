// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Information } from '@/services/informationManager';
import { DateTime } from 'luxon';
import { Ref, computed, ref } from 'vue';

export const NotificationKey = 'notification';

export class Notification {
  information: Information;
  time: DateTime;
  read: boolean;
  constructor(information: Information) {
    this.information = information;
    this.time = DateTime.now();
    this.read = false;
  }

  getData<Type>(): Type {
    return this.information.data as Type;
  }
}

export interface NotificationOptions {
  addToList?: boolean;
  trace?: boolean;
}

export class NotificationManager {
  notifications: Ref<Notification[]>;
  unreadCount: Ref<number> = computed(() => this.notifications.value.filter((n) => n.read === false).length);

  constructor() {
    this.notifications = ref([]);
  }

  add(information: Information): void {
    const notification = new Notification(information);
    this.notifications.value.unshift(notification);
  }

  markAsRead(id: string, read = true): void {
    const notification = this.notifications.value.find((notif) => notif.information.id === id);

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
}
