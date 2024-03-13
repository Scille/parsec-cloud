<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="notification-container ion-no-padding"
    :class="{
      unread: notification.read === false,
    }"
    @mouseover="$emit('mouseOver', notification)"
  >
    <div class="notification">
      <slot />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { Notification } from '@/services/notificationManager';
import { IonItem } from '@ionic/vue';

defineProps<{
  notification: Notification;
}>();

defineEmits<{
  (event: 'mouseOver', notification: Notification): void;
}>();
</script>

<style scoped lang="scss"></style>
