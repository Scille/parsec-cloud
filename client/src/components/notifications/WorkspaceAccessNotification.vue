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
      <ion-text class="notification-details__message body">
        <i18n-t
          keypath="notification.workspaceAccess"
          scope="global"
        >
          <template #workspace>
            <strong>{{ workspaceInfo ? workspaceInfo.currentName : '' }}</strong>
          </template>
        </i18n-t>
      </ion-text>
      <ion-text class="notification-details__time body-sm">
        <span
          class="hover-state"
          @click="navigateToNewWorkspace"
        >
          {{ $t('notificationCenter.goToWorkspace') }}
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
import { getWorkspaceInfo, StartedWorkspaceInfo } from '@/parsec';
import { currentRouteIsWorkspaceRoute, navigateToWorkspace } from '@/router';
import { NewWorkspaceAccessData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { arrowForward, business } from 'ionicons/icons';
import { onMounted, ref, Ref } from 'vue';

const workspaceInfo: Ref<StartedWorkspaceInfo | null> = ref(null);

const props = defineProps<{
  notification: Notification;
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
