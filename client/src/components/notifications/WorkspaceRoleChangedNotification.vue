<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <div class="notification-icon-container">
      <!-- This icon is only a default placeholder, replace/add notification specific icons -->
      <ms-image
        :image="LogoIconGradient"
        class="notification-icon"
      />
    </div>
    <div class="notification-details">
      <ion-text class="notification-details__message body">
        <i18n-t
          keypath="notification.changeRole"
          scope="global"
        >
          <template #role>
            <strong>{{ $msTranslate(getWorkspaceRoleTranslationKey(notificationData.newRole).label) }}</strong>
          </template>
          <template #workspace>
            <strong>{{ workspaceInfo ? workspaceInfo.currentName : '' }}</strong>
          </template>
        </i18n-t>
      </ion-text>
      <ion-text class="notification-details__time body-sm">
        <span>{{ $msTranslate(formatTimeSince(notification.time, '', 'short')) }}</span>
      </ion-text>
    </div>
  </notification-item>
</template>

<script setup lang="ts">
import { formatTimeSince, LogoIconGradient, MsImage } from 'megashark-lib';
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import { StartedWorkspaceInfo, getWorkspaceInfo } from '@/parsec';
import { WorkspaceRoleChangedData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { IonText } from '@ionic/vue';
import { Ref, onMounted, ref } from 'vue';
import { EventDistributor } from '@/services/eventDistributor';

const workspaceInfo: Ref<StartedWorkspaceInfo | null> = ref(null);

const props = defineProps<{
  notification: Notification;
  eventDistributor: EventDistributor;
}>();

onMounted(async () => {
  const result = await getWorkspaceInfo(notificationData.workspaceHandle);
  if (result.ok) {
    workspaceInfo.value = result.value;
  }
});

const notificationData = props.notification.getData<WorkspaceRoleChangedData>();
</script>

<style scoped lang="scss">
.notification-icon-container {
  background: var(--background-icon-info);
}
</style>
