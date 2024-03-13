<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <notification-item
    :notification="notification"
    id="user-shared-document-notification"
  >
    <div class="notification-content">
      <user-avatar-name
        class="notification-avatar"
        :user-avatar="userInfo ? userInfo.humanHandle.label : ''"
      />
      <div class="notification-details">
        <ion-text class="notification-details__message body">
          <i18n-t
            keypath="notification.userSharedDocument"
            scope="global"
          >
            <template #name>
              <strong>{{ userInfo ? userInfo.humanHandle.label : '' }}</strong>
            </template>
          </i18n-t>
        </ion-text>
        <ion-text class="notification-details__time body-sm">
          <span>{{ formatTimeSince(notification.time, '', 'short') }}</span>
        </ion-text>
      </div>
    </div>
    <div
      class="notification-file"
      @click="goToEnclosingFolder"
    >
      <ms-image
        :image="getFileIcon(notificationData.fileName)"
        class="default-state notification-icon"
      />
      <ms-image
        :image="Folder"
        class="hover-state notification-icon"
      />
      <ion-text class="notification-file__name subtitles-sm">
        <span class="default-state">{{ notificationData.fileName }}</span>
        <span class="hover-state">{{ $t('notificationCenter.openEnclosingFolder') }}</span>
      </ion-text>
      <ion-text class="notification-file__size default-state subtitles-sm">
        {{ formatFileSize(notificationData.fileSize) }}
      </ion-text>
    </div>
  </notification-item>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import { formatFileSize, getFileIcon } from '@/common/file';
import { Folder, MsImage } from '@/components/core/ms-image';
import NotificationItem from '@/components/notifications/NotificationItem.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { StartedWorkspaceInfo, UserInfo, getUserInfo, getWorkspaceInfo } from '@/parsec';
import { navigateToWorkspace } from '@/router';
import { UserSharedDocumentData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonText, popoverController } from '@ionic/vue';
import { Ref, onMounted, ref } from 'vue';

const userInfo: Ref<UserInfo | null> = ref(null);
const workspaceInfo: Ref<StartedWorkspaceInfo | null> = ref(null);

const props = defineProps<{
  notification: Notification;
}>();

onMounted(async () => {
  const resultWorkspace = await getWorkspaceInfo(notificationData.workspaceHandle);
  const resultUser = await getUserInfo(notificationData.userId);

  if (resultWorkspace.ok) {
    workspaceInfo.value = resultWorkspace.value;
  }

  if (resultUser.ok) {
    userInfo.value = resultUser.value;
  }
});

async function goToEnclosingFolder(): Promise<void> {
  await popoverController.dismiss();
  await navigateToWorkspace(notificationData.workspaceHandle, notificationData.filePath);
}

const notificationData = props.notification.getData<UserSharedDocumentData>();
</script>

<style scoped lang="scss">
.notification-content {
  display: flex;
  align-items: center;
  gap: 0.5re;
}

.notification-file {
  display: flex;
  align-items: center;
  background: var(--parsec-color-light-secondary-premiere);
  width: fit-content;
  padding: 0.5rem;
  margin-top: 0.5rem;
  margin-left: 3rem;
  border-radius: var(--parsec-radius-8);
  gap: 0.5rem;

  .notification-icon {
    width: 1.5rem;
    height: 1.5rem;
  }

  &__name {
    color: var(--parsec-color-light-secondary-text);
  }

  &__size {
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
