<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item
    :notification="notification"
    @click="navigateToDevices"
  >
    <div class="notification-icon-container">
      <ion-icon
        class="notification-icon"
        :icon="desktop"
      />
    </div>
    <div class="notification-details">
      <ion-text class="notification-details__message body">
        {{ $msTranslate('notification.newDeviceAdded') }}
      </ion-text>
      <ion-text class="notification-details__time body-sm">
        <span class="hover-state">
          {{ $msTranslate('notificationCenter.goToDevices') }}
        </span>
        <span class="default-state">{{ $msTranslate(formatTimeSince(notification.time, '', 'short')) }}</span>
        <ion-icon
          class="arrow-icon hover-state"
          :icon="arrowForward"
        />
      </ion-text>
    </div>
  </notification-item>
</template>

<script setup lang="ts">
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import { Routes, navigateTo } from '@/router';
import { EventDistributor } from '@/services/eventDistributor';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { arrowForward, desktop } from 'ionicons/icons';
import { formatTimeSince } from 'megashark-lib';

defineProps<{
  notification: Notification;
  eventDistributor: EventDistributor;
}>();

async function navigateToDevices(): Promise<void> {
  await popoverController.dismiss();
  await navigateTo(Routes.MyProfile);
}
</script>

<style scoped lang="scss">
.notification-icon-container {
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
