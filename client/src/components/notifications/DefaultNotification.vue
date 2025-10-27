<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <div
      class="notification-icon-container"
      :class="{
        info: notification.information.level === InformationLevel.Info,
        success: notification.information.level === InformationLevel.Success,
        warning: notification.information.level === InformationLevel.Warning,
        error: notification.information.level === InformationLevel.Error,
      }"
    >
      <ion-icon
        class="notification-icon"
        :icon="getIcon()"
      />
    </div>
    <div class="notification-details">
      <ion-label class="notification-details__message body">
        <span>{{ $msTranslate(notification.information.message) }}</span>
      </ion-label>
      <ion-text class="notification-details__time body-sm">
        <span>{{ $msTranslate(formatTimeSince(notification.time, '', 'short')) }}</span>
      </ion-text>
    </div>
  </notification-item>
</template>

<script setup lang="ts">
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import { EventDistributor } from '@/services/eventDistributor';
import { InformationLevel } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonLabel, IonText } from '@ionic/vue';
import { alertCircle, checkmarkCircle, informationCircle, warning } from 'ionicons/icons';
import { formatTimeSince } from 'megashark-lib';

const props = defineProps<{
  notification: Notification;
  eventDistributor: EventDistributor;
}>();

function getIcon(): string {
  switch (props.notification.information.level) {
    case InformationLevel.Info:
      return informationCircle;
    case InformationLevel.Success:
      return checkmarkCircle;
    case InformationLevel.Warning:
      return warning;
    case InformationLevel.Error:
      return alertCircle;
    default:
      return '';
  }
}
</script>

<style scoped lang="scss">
.notification-icon-container {
  &.info {
    background: var(--background-icon-info);
    color: var(--color-icon-info);
  }
  &.success {
    background: var(--background-icon-success);
    color: var(--color-icon-success);
  }
  &.warning {
    background: var(--background-icon-warning);
    color: var(--color-icon-warning);
  }
  &.error {
    background: var(--background-icon-danger);
    color: var(--color-icon-danger);
  }
  .notification-icon {
    width: 1.5rem;
    height: 1.5rem;
  }
}

.notification-details {
  display: flex;
  flex-direction: column;
  position: relative;
  margin-left: 0.875rem;

  &__message {
    color: var(--parsec-color-light-secondary-text);
  }

  &__time {
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
