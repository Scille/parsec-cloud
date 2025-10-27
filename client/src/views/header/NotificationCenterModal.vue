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
        <div class="notification-center-header-actions">
          <ion-toggle
            class="notification-center-header-actions__toggle dark small body-sm"
            @ion-change="onlyReadToggle = !onlyReadToggle"
          >
            {{ $msTranslate('notificationCenter.readOnly') }}
          </ion-toggle>
          <ion-icon
            class="notification-center-header-actions__close"
            :icon="close"
            @click="modalController.dismiss()"
          />
        </div>
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
          @click="onNotificationMouseOver(notification)"
        />
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Notifications } from '@/components/notifications';
import { EventDistributor } from '@/services/eventDistributor';
import { InformationDataType } from '@/services/informationManager';
import { Notification, NotificationManager } from '@/services/notificationManager';
import { IonIcon, IonList, IonText, IonToggle, modalController } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { MsImage, NoNotification } from 'megashark-lib';
import { Ref, computed, ref, type Component } from 'vue';

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
.notification-center {
  width: 100%;
  height: 100%;
  border-radius: var(--parsec-radius-12);
  overflow: hidden;

  &-header {
    &-actions {
      display: flex;
      align-items: center;
      gap: 0.25rem;

      &__close {
        color: var(--parsec-color-light-secondary-hard-grey);
        font-size: 1.25rem;
        cursor: pointer;
        border-radius: var(--parsec-radius-8);
        padding: 0.5rem;

        &:hover {
          color: var(--parsec-color-light-primary-30-opacity80);
        }
      }

      // eslint-disable-next-line vue-scoped-css/no-unused-selector
      &__toggle {
        color: var(--parsec-color-light-secondary-text);
        margin-left: auto;
        display: flex;
        align-items: center;
        cursor: pointer;

        &::part(label) {
          transition: opacity 150ms ease-in-out;
          margin-right: 0.5rem;
        }

        &::part(track) {
          background: var(--parsec-color-light-secondary-disabled);
          opacity: 0.8;
        }

        &::part(handle) {
          background: var(--parsec-color-light-secondary-text);
        }

        &:hover,
        &.toggle-checked {
          &::part(label) {
            opacity: 1;
          }

          &::part(track) {
            background: var(--parsec-color-light-secondary-disabled);
          }

          &::part(handle) {
            background: var(--parsec-color-light-secondary-text);
          }
        }
      }
    }
  }
}
</style>
