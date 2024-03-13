<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <div class="notification-icon-container">
      <ion-icon
        class="notification-icon"
        :icon="people"
      />
    </div>
    <div class="notification-details">
      <ion-text class="notification-details__message body">
        <i18n-t
          keypath="notification.multipleUsersJoinWorkspace"
          scope="global"
        >
          <template #count>
            <strong> {{ notificationData.roles.length }} {{ $t('notification.users') }} </strong>
          </template>
          <template #workspace>
            <strong>{{ workspaceInfo ? workspaceInfo.currentName : '' }}</strong>
          </template>
        </i18n-t>
      </ion-text>
      <ion-text class="notification-details__time body-sm">
        <span
          class="hover-state"
          @click="openPopover($event)"
        >
          {{ $t('notificationCenter.viewJoinedUsers') }}
        </span>
        <span class="default-state">{{ formatTimeSince(notification.time, '', 'short') }}</span>
      </ion-text>
    </div>
  </notification-item>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import MultipleUsersJoinPopover from '@/components/notifications/MultipleUsersJoinPopover.vue';
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import { getWorkspaceInfo, StartedWorkspaceInfo } from '@/parsec';
import { MultipleUsersJoinWorkspaceData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { people } from 'ionicons/icons';
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

const notificationData = props.notification.getData<MultipleUsersJoinWorkspaceData>();

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: MultipleUsersJoinPopover,
    cssClass: 'multiple-users-join-popover',
    componentProps: {
      notification: props.notification,
    },
    event: event,
    alignment: 'center',
    showBackdrop: false,
  });
  await popover.present();
}
</script>

<style scoped lang="scss">
.notification-icon-container {
  background: var(--background-icon-info);
  color: var(--color-icon-info);
}
</style>
