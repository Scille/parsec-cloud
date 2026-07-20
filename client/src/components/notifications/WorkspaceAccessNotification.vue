<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <div class="notification-icon-container">
      <ion-icon
        class="notification-icon"
        :icon="business"
      />
    </div>
    <div class="notification-details">
      <ms-rich-text
        class="notification-details__message body"
        :text="{ key: 'notification.workspaceAccess', data: { workspace: workspaceInfo ? workspaceInfo.name : '' } }"
      />
      <ion-text class="notification-details__time body-sm">
        <span
          class="hover-state"
          @click="navigateToNewWorkspace"
        >
          {{ $msTranslate('notificationCenter.goToWorkspace') }}
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
import { getWorkspaceInfo, WorkspaceInfo } from '@/parsec';
import { currentRouteIsWorkspaceRoute, navigateToWorkspace } from '@/router';
import { EventDistributor } from '@/services/eventDistributor';
import { NewWorkspaceAccessData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { arrowForward, business } from 'ionicons/icons';
import { formatTimeSince, MsRichText } from 'megashark-lib';
import { onMounted, ref, Ref } from 'vue';

const workspaceInfo: Ref<WorkspaceInfo | null> = ref(null);

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

async function navigateToNewWorkspace(): Promise<void> {
  await popoverController.dismiss();
  if (!currentRouteIsWorkspaceRoute(notificationData.workspaceHandle)) {
    await navigateToWorkspace(notificationData.workspaceHandle);
  }
}

const notificationData = props.notification.getData<NewWorkspaceAccessData>();
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
