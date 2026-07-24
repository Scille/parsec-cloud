<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item
    :notification="notification"
    @click="update"
  >
    <div class="notification-icon-container">
      <ion-icon
        class="notification-icon"
        :icon="sparkles"
      />
    </div>
    <div class="notification-details">
      <ion-text class="notification-details__message body">
        {{ $msTranslate('notificationCenter.newVersionAvailable') }}
        <span class="notification-details__message-version">
          {{ $msTranslate({ key: 'formatter.version', data: { version: data.newVersion } }) }}
        </span>
      </ion-text>
      <ion-text class="notification-details__time body-sm">
        <span class="hover-state">
          {{ $msTranslate('notificationCenter.update') }}
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
import { EventDistributor, Events } from '@/services/eventDistributor';
import { NewVersionAvailableData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { openUpdateAppModal } from '@/views/about';
import { IonIcon, IonText, modalController, popoverController } from '@ionic/vue';
import { arrowForward, sparkles } from 'ionicons/icons';
import { Answer, formatTimeSince } from 'megashark-lib';

const props = defineProps<{
  notification: Notification;
  eventDistributor: EventDistributor;
}>();

const data: NewVersionAvailableData = props.notification.getData<NewVersionAvailableData>();

async function update(): Promise<void> {
  if (await popoverController.getTop()) {
    await popoverController.dismiss();
  } else if (await modalController.getTop()) {
    await modalController.dismiss();
  }

  const answer = await openUpdateAppModal(data.newVersion);

  if (answer === Answer.Yes) {
    await props.eventDistributor.dispatchEvent(Events.LogoutRequested);
    window.electronAPI.prepareUpdate();
  }
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

.notification-details__message-version {
  color: var(--parsec-color-light-secondary-grey);
}
</style>
