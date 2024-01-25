<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="notification-center-container">
    <ms-image :image="WavyCaretUp" />
    <div class="notification-center">
      <div class="notification-center-header">
        <ion-text class="tooltip-text body-sm">
          {{ $t('notificationCenter.title') }}
        </ion-text>
        <ion-label class="notification-center-header-unread body-sm">
          <span class="default-state">{{ $t('notificationCenter.unread', { count: unreadCount }) }}</span>
        </ion-label>
      </div>
      <ion-list
        class="notification-center-content"
        lines="full"
      >
        <notification-item
          v-for="notification in notifications"
          :key="notification.id"
          :notification="notification"
          @click="onNotificationClick($event)"
          @mouse-over="onNotificationMouseOver($event)"
        />
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MsImage, WavyCaretUp } from '@/components/core/ms-image';
import NotificationItem from '@/components/header/NotificationItem.vue';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { translate } from '@/services/translation';
import { IonLabel, IonList, IonText } from '@ionic/vue';
import { computed, inject, onMounted } from 'vue';

const notificationManager: NotificationManager = inject(NotificationKey)!;
const unreadCount = notificationManager.getUnreadCount();
const notifications = computed(() => {
  return notificationManager.getNotifications().sort((n1, n2) => n2.time.toMillis() - n1.time.toMillis());
});

onMounted(() => {
  // only for testing purpose
  const testNotification = new Notification({
    title: Math.random().toString(36).substring(7),
    message: translate('link.invalid.message'),
    level: NotificationLevel.Error,
  });
  notificationManager.addToList(testNotification);
});

function onNotificationClick(notification: Notification): void {
  console.log(notification);
}

function onNotificationMouseOver(notification: Notification): void {
  if (!notification.read) {
    notificationManager.markAsRead(notification.id);
  }
}
</script>

<style lang="scss" scoped>
.notification-center-container {
  display: flex;
  align-items: center;
  flex-direction: column;
  --fill-color: var(--parsec-color-light-primary-900);
}

.notification-center {
  width: 100%;
  border-radius: var(--parsec-radius-6);
  // @SharkDesigner: boxShadow is not working atm; border here as a development workaround
  box-shadow: 0px 0px 4px 12px var(--parsec-shadow-strong);
  border: 1px solid var(--parsec-color-light-primary-900);

  &-header {
    background: var(--parsec-color-light-primary-900);
    color: var(--parsec-color-light-primary-30);
    padding: 1rem;
    border-radius: var(--parsec-radius-6) var(--parsec-radius-6) 0 0;
    display: flex;

    &-unread {
      color: var(--parsec-color-light-secondary-grey);
      margin-left: auto;
    }
  }

  &-content {
    background: var(--parsec-color-light-secondary-white);
    color: var(--parsec-color-light-primary-900);
    border-radius: 0 0 var(--parsec-radius-6) var(--parsec-radius-6);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    height: 40vh;
    max-height: 25rem;
    transition: all 250ms ease-in-out;
  }
}
</style>
