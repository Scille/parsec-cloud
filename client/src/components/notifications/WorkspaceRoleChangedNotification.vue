<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <div class="notification-icon-container">
      <!-- This icon is only a default placeholder, replace/add notification specific icons -->
      <!-- prettier-ignore -->
      <ms-image
        :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
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
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import { StartedWorkspaceInfo, getWorkspaceInfo } from '@/parsec';
import { EventDistributor } from '@/services/eventDistributor';
import { WorkspaceRoleChangedData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { IonText } from '@ionic/vue';
import { LogoIconGradient, MsImage, formatTimeSince } from 'megashark-lib';
import { Ref, onMounted, ref } from 'vue';

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
