<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item :notification="notification">
    <user-avatar-name
      class="notification-avatar"
      :user-avatar="userInfo ? userInfo.humanHandle.label : ''"
    />
    <div class="notification-details">
      <ion-text class="notification-details__message body">
        <i18n-t
          keypath="notification.userJoinWorkspace"
          scope="global"
        >
          <template #name>
            <strong>{{ userInfo ? userInfo.humanHandle.label : '' }}</strong>
          </template>
          <template #workspace>
            <strong>{{ workspaceName }}</strong>
          </template>
        </i18n-t>
      </ion-text>
      <ion-text class="notification-details__time body-sm">
        <span>{{ formatTimeSince(notification.time, '', 'short') }}</span>
      </ion-text>
    </div>
  </notification-item>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { UserInfo, getUserInfo, getWorkspaceName } from '@/parsec';
import { UserJoinWorkspaceData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonText } from '@ionic/vue';
import { Ref, onMounted, ref } from 'vue';

const userInfo: Ref<UserInfo | null> = ref(null);
const workspaceName = ref('');
const props = defineProps<{
  notification: Notification;
}>();

onMounted(async () => {
  const resultWorkspace = await getWorkspaceName(notificationData.workspaceId);
  const resultUser = await getUserInfo(notificationData.userId);

  if (resultWorkspace.ok) {
    workspaceName.value = resultWorkspace.value;
  }

  if (resultUser.ok) {
    userInfo.value = resultUser.value;
  }
});

const notificationData = props.notification.getData<UserJoinWorkspaceData>();
</script>

<style scoped lang="scss"></style>
