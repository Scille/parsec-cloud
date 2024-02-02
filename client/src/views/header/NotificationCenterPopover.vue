<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="notification-center-container">
    <div class="notification-center">
      <div class="notification-center-header">
        <ion-text class="notification-center-header__title title-h4">
          {{ $t('notificationCenter.title') }}
        </ion-text>
        <span class="notification-center-header__counter body-sm">
          {{ unreadCount > 99 ? '+99' : unreadCount }}
        </span>
        <ion-toggle
          class="notification-center-header__toggle dark small body-sm"
          @ion-change="ReadOnlyToggle = !ReadOnlyToggle"
        >
          {{ console.log(ReadOnlyToggle) }}
          {{ $t('notificationCenter.readOnly') }}
        </ion-toggle>
      </div>
      <ion-list class="notification-center-content">
        <div
          class="notification-center-content__empty"
          v-if="notifications.length === 0"
        >
          <ms-image :image="NoNotification" />
          <ion-text class="body-lg">
            {{ $t('notificationCenter.noNotification') }}
          </ion-text>
        </div>
        <template v-if="ReadOnlyToggle">
          <notification-item
            v-for="notification in unReadNotification"
            :key="notification.information.id"
            :notification="notification"
            @click="onNotificationClick($event)"
            @mouse-over="onNotificationMouseOver($event)"
          />
        </template>
        <template v-else>
          <notification-item
            v-for="notification in notifications"
            :key="notification.information.id"
            :notification="notification"
            @click="onNotificationClick($event)"
            @mouse-over="onNotificationMouseOver($event)"
          />
        </template>
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MsImage, NoNotification } from '@/components/core/ms-image';
import NotificationItem from '@/components/header/NotificationItem.vue';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonList, IonText, IonToggle } from '@ionic/vue';
import { Ref, computed, inject, onMounted, ref } from 'vue';

const informationManager: InformationManager = inject(InformationKey)!;
const unreadCount = informationManager.notificationManager.getUnreadCount();
const notifications = computed(() => {
  return informationManager.notificationManager.getNotifications().sort((n1, n2) => n2.time.toMillis() - n1.time.toMillis());
});
const unReadNotification = computed(() => {
  return informationManager.notificationManager.getNotifications(true).sort((n1, n2) => n2.time.toMillis() - n1.time.toMillis());
});
const ReadOnlyToggle: Ref<boolean> = ref(false);

onMounted(() => {
  // only for testing purpose
  const testInformation = new Information({
    message: Math.random().toString(36).substring(2),
    level: InformationLevel.Error,
  });
  informationManager.present(testInformation, PresentationMode.Modal | PresentationMode.Console);
});

function onNotificationClick(notification: Notification): void {
  console.log(notification);
}

function onNotificationMouseOver(notification: Notification): void {
  if (!notification.read) {
    informationManager.notificationManager.markAsRead(notification.information.id);
  }
}
</script>

<style lang="scss" scoped>
.notification-center-container {
  display: flex;
  align-items: center;
  flex-direction: column;
  --fill-color: var(--parsec-color-light-primary-900);
  overflow: visible;
}

.notification-center {
  width: 100%;
  border-radius: var(--parsec-radius-12);
  overflow: hidden;

  &-header {
    background: var(--parsec-color-light-primary-800);
    color: var(--parsec-color-light-primary-30);
    padding: 1rem 1.5rem;
    display: flex;
    gap: 0.5rem;

    &__title {
      padding: 0;
      display: flex;
      align-items: center;
    }

    &__counter {
      margin-right: auto;
      padding: 0 0.25rem;
      background: var(--parsec-color-light-primary-30-opacity15);
      border-radius: var(--parsec-radius-12);
      display: flex;
      align-items: center;
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &__toggle {
      color: var(--parsec-color-light-secondary-medium);
      margin-left: auto;
      display: flex;
      align-items: center;
      cursor: pointer;

      &::part(label) {
        transition: opacity 150ms ease-in-out;
        opacity: 0.6;
        margin-right: 0.5rem;
      }

      &:hover,
      &.toggle-checked {
        &::part(label) {
          opacity: 1;
        }
      }
    }
  }

  &-content {
    background: var(--parsec-color-light-secondary-white);
    color: var(--parsec-color-light-primary-900);
    border-radius: 0 0 var(--parsec-radius-6) var(--parsec-radius-6);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    padding: 0;
    height: 40vh;
    max-height: 25rem;
    transition: all 250ms ease-in-out;

    &__empty {
      overflow: hidden;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 0.5rem;
      margin: auto;
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}
</style>
