<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item
    :notification="notification"
    @click="navigateToDevices"
  >
    <div class="notification-icon">
      <ion-icon
        class="file-icon"
        :icon="desktop"
      />
    </div>
    <div class="notification-details">
      <ion-text class="notification-details__message body">
        {{ $t('notification.newDeviceAdded') }}
      </ion-text>
      <ion-text class="notification-details__time body-sm">
        <span class="hover-state">
          {{ $t('notificationCenter.goToDevices') }}
        </span>
        <span class="default-state">{{ formatTimeSince(notification.time, '', 'short') }}</span>
        <ion-icon
          class="arrow-icon hover-state"
          :icon="arrowForward"
        />
      </ion-text>
    </div>
  </notification-item>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import { Routes, navigateTo } from '@/router';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { arrowForward, desktop } from 'ionicons/icons';

defineProps<{
  notification: Notification;
}>();

async function navigateToDevices(): Promise<void> {
  await popoverController.dismiss();
  await navigateTo(Routes.Devices);
}
</script>

<style scoped lang="scss">
.notification-icon {
  background: var(--background-icon-info);
  color: var(--color-icon-info);
}

.arrow-icon {
  margin-left: auto;
}

.hover-state {
  display: none;
}
</style>
