<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="notification-center-container">
    <div class="notification-center">
      <div class="notification-center-header">
        <ion-text class="notification-center-header__title title-h4">
          {{ $msTranslate('notificationCenter.title') }}
        </ion-text>
        <span class="notification-center-header__counter body-sm">
          {{ unreadCount > 99 ? '+99' : unreadCount }}
        </span>
        <ion-toggle
          class="notification-center-header__toggle dark small body-sm"
          @ion-change="onlyReadToggle = !onlyReadToggle"
        >
          {{ $msTranslate('notificationCenter.readOnly') }}
        </ion-toggle>
      </div>
      <ion-list class="notification-center-content">
        <div
          class="notification-center-content__empty"
          v-if="notifications.length === 0"
        >
          <ms-image :image="NoNotification" />
          <ion-text class="body-lg">
            {{ $msTranslate('notificationCenter.noNotification') }}
          </ion-text>
        </div>
        <component
          v-for="notification in notifications"
          :is="getComponentType(notification)"
          :key="notification.information.id"
          :notification="notification"
          :event-distributor="eventDistributor"
          @mouse-over="onNotificationMouseOver($event)"
        />
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MsImage, NoNotification } from 'megashark-lib';
import { Notifications } from '@/components/notifications';
import { InformationDataType } from '@/services/informationManager';
import { Notification, NotificationManager } from '@/services/notificationManager';
import { IonList, IonText, IonToggle } from '@ionic/vue';
import { Ref, computed, ref, type Component } from 'vue';
import { EventDistributor } from '@/services/eventDistributor';

const props = defineProps<{
  notificationManager: NotificationManager;
  eventDistributor: EventDistributor;
}>();

const unreadCount = props.notificationManager.getUnreadCount();
const notifications = computed(() => {
  if (!props.notificationManager || !props.notificationManager.getNotifications()) {
    return [];
  }
  return props.notificationManager
    .getNotifications()
    .filter((n) => (onlyReadToggle.value ? !n.read : true))
    .sort((n1, n2) => n2.time.toMillis() - n1.time.toMillis());
});
const onlyReadToggle: Ref<boolean> = ref(false);

function getComponentType(notification: Notification): Component {
  if (!notification.information.data) {
    return Notifications.DefaultNotification;
  }
  switch (notification.information.data.type) {
    case InformationDataType.WorkspaceRoleChanged:
      return Notifications.WorkspaceRoleChangedNotification;
    case InformationDataType.NewWorkspaceAccess:
      return Notifications.WorkspaceAccessNotification;
    case InformationDataType.UserJoinWorkspace:
      return Notifications.UserJoinWorkspaceNotification;
    case InformationDataType.MultipleUsersJoinWorkspace:
      return Notifications.MultipleUsersJoinWorkspaceNotification;
    case InformationDataType.UserSharedDocument:
      return Notifications.UserSharedDocumentNotification;
    case InformationDataType.AllImportedElements:
      return Notifications.AllImportedElementsNotification;
    case InformationDataType.NewDevice:
      return Notifications.NewDeviceNotification;
    case InformationDataType.NewVersionAvailable:
      return Notifications.NewVersionAvailableNotification;
    default:
      return Notifications.DefaultNotification;
  }
}

function onNotificationMouseOver(notification: Notification): void {
  if (!notification.read) {
    props.notificationManager.markAsRead(notification.information.id);
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
