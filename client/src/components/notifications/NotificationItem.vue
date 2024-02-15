<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="notification-container ion-no-padding"
    :class="{
      unread: notification.read === false,
      info: notification.information.level === InformationLevel.Info,
      success: notification.information.level === InformationLevel.Success,
      warning: notification.information.level === InformationLevel.Warning,
      error: notification.information.level === InformationLevel.Error,
    }"
    @mouseover="$emit('mouseOver', notification)"
  >
    <div
      class="notification"
      @click="onClick"
    >
      <slot />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { InformationLevel } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonItem } from '@ionic/vue';

const props = defineProps<{
  notification: Notification;
}>();

const emits = defineEmits<{
  (event: 'click', notification: Notification): void;
  (event: 'mouseOver', notification: Notification): void;
}>();

async function onClick(): Promise<void> {
  emits('click', props.notification);
}
</script>

<style scoped lang="scss"></style>
