<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <div class="notification-icon">
      <!-- This icon is only a default placeholder, replace/add notification specific icons -->
      <ms-image
        :image="LogoIconGradient"
        class="file-icon"
      />
    </div>
    <div class="notification-details">
      <ion-text class="notification-details__message body">
        <i18n-t
          keypath="notification.changeRole"
          scope="global"
        >
          <template #role>
            <strong>{{ translateWorkspaceRole(notificationData.newRole).label }}</strong>
          </template>
          <template #workspace>
            <strong>{{ workspaceName }}</strong>
          </template>
        </i18n-t>
      </ion-text>
      <ion-text class="notification-details__time body-sm">
        <span class="default-state">{{ formatTimeSince(notification.time, '', 'short') }}</span>
      </ion-text>
    </div>
  </notification-item>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import { LogoIconGradient, MsImage } from '@/components/core/ms-image';
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import { getWorkspaceName } from '@/parsec';
import { WorkspaceRoleChangedData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { translateWorkspaceRole } from '@/services/translation';
import { IonText } from '@ionic/vue';
import { onMounted, ref } from 'vue';

const workspaceName = ref('');
const props = defineProps<{
  notification: Notification;
}>();

onMounted(async () => {
  const result = await getWorkspaceName(notificationData.workspaceId);
  if (result.ok) {
    workspaceName.value = result.value;
  }
});

const notificationData = props.notification.getData<WorkspaceRoleChangedData>();
</script>

<style scoped lang="scss"></style>
