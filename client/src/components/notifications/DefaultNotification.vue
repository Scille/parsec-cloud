<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <div class="element-icon">
      <!-- This icon is only a default placeholder, replace/add notification specific icons -->
      <ms-image
        :image="LogoIconGradient"
        class="file-icon"
      />
    </div>
    <div class="element-details">
      <ion-label class="element-details__message body">
        <span>{{ notification.information.message }}</span>
      </ion-label>
      <ion-text class="element-details__time body-sm">
        <span class="default-state">{{ formatTimeSince(notification.time, '', 'short') }}</span>
        <span class="hover-state">{{ $t('notificationCenter.browse') }}</span>
      </ion-text>
    </div>
    <ion-icon
      class="arrow-icon hover-state"
      :icon="arrowForward"
    />
  </notification-item>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import { LogoIconGradient, MsImage } from '@/components/core/ms-image';
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonLabel, IonText } from '@ionic/vue';
import { arrowForward } from 'ionicons/icons';

defineProps<{
  notification: Notification;
}>();
</script>

<style scoped lang="scss">
.element-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2.5rem;
  height: 2.5rem;
  padding: 0.5rem;
  background: var(--background-icon-info);
  border-radius: var(--parsec-radius-12);

  .file-icon {
    width: 1.5rem;
    height: 1.5rem;
  }
}

.element-details {
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

.arrow-icon {
  margin-left: auto;
}
</style>
