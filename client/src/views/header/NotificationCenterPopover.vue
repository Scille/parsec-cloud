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
          @ion-change="onlyReadToggle = !onlyReadToggle"
        >
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
        <component
          v-for="notification in notifications"
          :is="getComponentType(notification)"
          :key="notification.information.id"
          :notification="notification"
          @click="onNotificationClick($event)"
          @mouse-over="onNotificationMouseOver($event)"
        />
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MsImage, NoNotification } from '@/components/core/ms-image';
import { Notifications } from '@/components/notifications/index';
import { WorkspaceRole } from '@/parsec';
import {
  Information,
  InformationDataType,
  InformationKey,
  InformationLevel,
  InformationManager,
  PresentationMode,
} from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonList, IonText, IonToggle } from '@ionic/vue';
import { Ref, computed, inject, onMounted, ref, type Component } from 'vue';

const informationManager: InformationManager = inject(InformationKey)!;
const unreadCount = informationManager.notificationManager.getUnreadCount();
const notifications = computed(() => {
  return informationManager.notificationManager
    .getNotifications()
    .filter((n) => (onlyReadToggle.value ? !n.read : true))
    .sort((n1, n2) => n2.time.toMillis() - n1.time.toMillis());
});
const onlyReadToggle: Ref<boolean> = ref(false);

onMounted(() => {
  // Add a new notification every 5 seconds, for a total of 10 notifications
  for (let i = 1; i < 10; i++) {
    setTimeout(() => {
      const info = new Information({
        message: `Notification ${i}`,
        level: InformationLevel.Info,
        data: {
          type: InformationDataType.WorkspaceRoleChanged,
          workspaceHandle: 2,
          oldRole: WorkspaceRole.Contributor,
          newRole: WorkspaceRole.Manager,
        },
      });
      informationManager.present(info, PresentationMode.Notification);

      const info1 = new Information({
        message: `Notification ${i}`,
        level: InformationLevel.Info,
        data: {
          type: InformationDataType.NewWorkspaceAccess,
          workspaceHandle: 1,
          role: WorkspaceRole.Manager,
        },
      });
      informationManager.present(info1, PresentationMode.Notification);

      const info2 = new Information({
        message: `Notification ${i}`,
        level: InformationLevel.Info,
        data: {
          type: InformationDataType.NewWorkspaceAccess,
          workspaceHandle: 3,
          role: WorkspaceRole.Manager,
        },
      });
      informationManager.present(info2, PresentationMode.Notification);

      const info3 = new Information({
        message: `Notification ${i}`,
        level: InformationLevel.Info,
        data: {
          type: InformationDataType.UserJoinWorkspace,
          workspaceHandle: 1,
          role: WorkspaceRole.Manager,
          userId: 'id1',
        },
      });
      informationManager.present(info3, PresentationMode.Notification);

      const info4 = new Information({
        message: `Notification ${i}`,
        level: InformationLevel.Info,
        data: {
          type: InformationDataType.MultipleUsersJoinWorkspace,
          workspaceHandle: 1,
          roles: [
            {
              userId: 'id1',
              role: WorkspaceRole.Manager,
            },
            {
              userId: 'id1',
              role: WorkspaceRole.Contributor,
            },
          ],
        },
      });
      informationManager.present(info4, PresentationMode.Notification);

      const info5 = new Information({
        message: `Notification ${i}`,
        level: InformationLevel.Info,
        data: {
          type: InformationDataType.UserSharedDocument,
          workspaceHandle: 3,
          userId: 'id1',
          fileName: 'Encrypted-file.txt',
          filePath: '/',
          fileSize: 1024,
        },
      });
      informationManager.present(info5, PresentationMode.Notification);

      const info6 = new Information({
        message: `Notification ${i}`,
        level: InformationLevel.Success,
        data: {
          type: InformationDataType.AllImportedElements,
        },
      });
      informationManager.present(info6, PresentationMode.Notification);

      const info7 = new Information({
        message: `Notification ${i}`,
        level: InformationLevel.Success,
        data: {
          type: InformationDataType.NewDevice,
        },
      });
      informationManager.present(info7, PresentationMode.Notification);
    }, i * 2000);
  }
});

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
    default:
      return Notifications.DefaultNotification;
  }
}

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
