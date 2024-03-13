<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <!-- @click="openImportedMenu" -->
    <div class="notification-icon-container">
      <ion-icon
        class="notification-icon"
        :icon="checkmarkCircle"
      />
    </div>
    <div class="notification-details">
      <ion-text class="notification-details__message body">
        {{ $t('notification.allImportedElements') }}
      </ion-text>
      <ion-text class="notification-details__time body-sm">
        <span
          class="hover-state"
          @click="openImportedMenu"
        >
          {{ $t('notificationCenter.openImportedMenu') }}
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
import { Routes, currentRouteIsFileRoute, navigateTo } from '@/router';
import useUploadMenu from '@/services/fileUploadMenu';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { arrowForward, checkmarkCircle } from 'ionicons/icons';

const menu = useUploadMenu();

defineProps<{
  notification: Notification;
}>();

async function openImportedMenu(): Promise<void> {
  await popoverController.dismiss();
  if (!currentRouteIsFileRoute()) {
    await navigateTo(Routes.Workspaces);
  }
  menu.show();
  menu.expand();
}
</script>

<style scoped lang="scss">
.notification-icon-container {
  background: var(--background-icon-success);
  color: var(--color-icon-success);
}

.arrow-icon {
  margin-left: auto;
}

.hover-state {
  display: none;
}
</style>
