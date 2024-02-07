<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    {{ 'Role changed on workspace' }}
    {{ notificationData.workspaceId }}
  </notification-item>
</template>

<script setup lang="ts">
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import { WorkspaceRoleChangedData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';

const props = defineProps<{
  notification: Notification;
}>();

const notificationData = props.notification.getData<WorkspaceRoleChangedData>();
</script>

<style scoped lang="scss"></style>
