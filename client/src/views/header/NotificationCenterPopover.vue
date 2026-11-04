<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="notification-center-container">
    <div class="notification-center">
      <div class="notification-center-header">
        <ion-text class="notification-center-header__title">
          {{ $msTranslate('notificationCenter.title') }}
        </ion-text>
        <span class="notification-center-header__counter">
          {{ unreadCount > 99 ? '+99' : unreadCount }}
        </span>
        <ion-toggle
          class="notification-center-header__toggle dark small"
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
          <ion-text class="notification-center-content__empty-text">
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
import { Notifications } from '@/components/notifications';
import { EventDistributor } from '@/services/eventDistributor';
import { InformationDataType } from '@/services/informationManager';
import { Notification, NotificationManager } from '@/services/notificationManager';
import { IonList, IonText, IonToggle } from '@ionic/vue';
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
.notification-center-container {
  display: flex;
  align-items: center;
  flex-direction: column;
  --fill-color: #{ms.color('icon-brand-default')};
  overflow: visible;
}

.notification-center {
  width: 100%;
  border-radius: ms.radius('2xl');
  overflow: hidden;

  &-header {
    background: ms.color('surface-base-default-secondary');
    color: ms.color('text-base-body');
    padding: ms.spacing('padding-3xl') ms.spacing('padding-4xl');
    display: flex;
    gap: ms.spacing('gap-lg');

    &__title {
      @include ms.font('heading-h5');
      padding: ms.spacing('padding-none');
      display: flex;
      align-items: center;
    }

    &__counter {
      @include ms.font('body-sm-regular');
      margin-right: auto;
      padding: ms.spacing('padding-none') ms.spacing('padding-sm');
      background: ms.color('surface-neutral-default-subtle-pressed');
      border-radius: ms.radius('2xl');
      display: flex;
      align-items: center;
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &__toggle {
      @include ms.font('body-sm-regular');
      color: ms.color('text-neutral-default');
      margin-left: auto;
      display: flex;
      align-items: center;
      cursor: pointer;

      &::part(label) {
        transition: opacity 150ms ease-in-out;
        opacity: ms.opacity('6');
        margin-right: ms.spacing('padding-lg');
      }

      &:hover,
      &.toggle-checked {
        &::part(label) {
          opacity: ms.opacity('10');
        }
      }
    }
  }

  &-content {
    background: ms.color('surface-base-default');
    color: ms.color('text-brand-default');
    border-radius: ms.radius('none') ms.radius('none') ms.radius('md') ms.radius('md');
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    padding: ms.spacing('padding-none');
    height: 40vh;
    max-height: 25rem;
    transition: all 250ms ease-in-out;

    &__empty {
      overflow: hidden;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: ms.spacing('gap-lg');
      margin: auto;
      color: ms.color('text-base-description');
    }

    &__empty-text {
      @include ms.font('body-lg-regular');
    }
  }
}
</style>
