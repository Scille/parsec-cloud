<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="users-list">
    <ion-item
      class="users-list-item body"
      button
      lines="none"
      v-for="user in userRoles"
      :key="user.info.id"
    >
      <div class="users-list-item-content">
        <user-avatar-name
          :user-avatar="user.info.humanHandle.label"
          :user-name="user.info.humanHandle.label"
          class="users-list-item-content__avatar body"
        />
        <workspace-tag-role
          :role="user.role"
          class="users-list-item-content__body body"
        />
      </div>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import WorkspaceTagRole from '@/components/workspaces/WorkspaceTagRole.vue';
import { UserInfo, WorkspaceRole, getUserInfo } from '@/parsec';
import { MultipleUsersJoinWorkspaceData } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonItem, IonList } from '@ionic/vue';
import { Ref, onMounted, ref } from 'vue';

const userRoles: Ref<{ info: UserInfo; role: WorkspaceRole }[]> = ref([]);
const props = defineProps<{
  notification: Notification;
}>();

onMounted(async () => {
  for (const user of notificationData.roles) {
    const resultUser = await getUserInfo(user.userId);

    if (resultUser.ok) {
      userRoles.value.push({ info: resultUser.value, role: user.role });
    }
  }
});

const notificationData = props.notification.getData<MultipleUsersJoinWorkspaceData>();
</script>

<style lang="scss" scoped>
.users-list {
  display: flex;
  flex-direction: column;
  padding: 0.25rem;

  &-item {
    padding: 0.5rem 0.75rem;
    border-radius: 0.5rem;
    --background: none;
    --background-hover: none;
    --background-hover-opacity: 1;
    --inner-padding-end: 0;
    color: var(--parsec-color-light-secondary-grey);
    position: relative;

    &:hover {
      background: var(--parsec-color-light-secondary-premiere);
    }

    &::part(native) {
      padding: 0;
      cursor: default;
    }

    &-content {
      display: flex;
      align-items: center;
      justify-content: space-between;
      width: 100%;

      &__avatar {
        flex-grow: 1;
      }

      &__body {
        flex: 0;
        text-align: right;
      }
    }

    &:not(:last-child)::after {
      content: '';
      position: absolute;
      bottom: 0;
      left: 3rem;
      right: 0;
      height: 1px;
      background-color: var(--parsec-color-light-secondary-premiere);
    }
  }
}
</style>
