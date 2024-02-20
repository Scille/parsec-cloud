<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <div class="notification-icon">
      <ion-icon
        class="file-icon"
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
            <strong>{{ workspaceName }}</strong>
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
import { getWorkspaceName } from '@/parsec';
import { currentRouteIsWorkspaceRoute, navigateToWorkspace } from '@/router';
import { NewWorkspaceAccessData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { arrowForward, business } from 'ionicons/icons';
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

async function navigateToNewWorkspace(): Promise<void> {
  await popoverController.dismiss();
  if (!currentRouteIsWorkspaceRoute(notificationData.workspaceId)) {
    await navigateToWorkspace(notificationData.workspaceId);
  }
}

const notificationData = props.notification.getData<NewWorkspaceAccessData>();
</script>

<style scoped lang="scss">
.notification-icon {
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
