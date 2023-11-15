<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="$t('WorkspaceSharing.title')"
      :close-button="{ visible: true }"
    >
      <!-- content -->
      <div>
        <ms-input
          v-model="search"
          :label="$t('WorkspaceSharing.searchLabel')"
          :placeholder="$t('WorkspaceSharing.searchPlaceholder')"
        />
        <ion-list class="user-list">
          <workspace-user-role
            :disabled="true"
            :user="{
              id: clientInfo ? clientInfo.userId : 'FAKE',
              humanHandle: clientInfo ? clientInfo.humanHandle : {
                label: $t('WorkspaceSharing.currentUserLabel'),
                email: '',
              },
              profile: UserProfile.Outsider,
            }"
            :role="ownRole"
            :client-profile="clientInfo ? clientInfo.currentProfile : UserProfile.Outsider"
            :client-role="ownRole"
          />
          <workspace-user-role
            :disabled="isSelectDisabled(entry[1])"
            v-for="entry in userRoles"
            :key="entry[0].id"
            :user="entry[0]"
            :role="entry[1]"
            :client-profile="clientInfo ? clientInfo.currentProfile : UserProfile.Outsider"
            :client-role="ownRole"
            @role-update="updateUserRole"
          />
        </ion-list>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonList } from '@ionic/vue';
import { ref, Ref, watch, onUnmounted, onMounted, inject } from 'vue';
import { WorkspaceID, WorkspaceRole, getWorkspaceSharing, UserTuple, shareWorkspace, UserProfile, ClientInfo, getClientInfo } from '@/parsec';
import { NotificationKey } from '@/common/injectionKeys';
import { NotificationCenter, Notification, NotificationLevel } from '@/services/notificationCenter';
import WorkspaceUserRole from '@/components/workspaces/WorkspaceUserRole.vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';
import { useI18n } from 'vue-i18n';
import { translateWorkspaceRole } from '@/common/translations';

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationCenter: NotificationCenter = inject(NotificationKey)!;
const search = ref('');
const { t } = useI18n();
const clientInfo: Ref<ClientInfo | null> = ref(null);

const props = defineProps<{
  workspaceId: WorkspaceID;
  ownRole: WorkspaceRole | null;
}>();

const userRoles: Ref<Array<[UserTuple, WorkspaceRole | null]>> = ref([]);

// Would prefere to use a computed instead of a watch but
// Vue doesn't handle async in computed.
const unwatchSearch = watch(search, async () => {
  await refreshSharingInfo(search.value);
});

function isSelectDisabled(role: WorkspaceRole | null): boolean {
  // Outsider should not be able to change anything
  if (!clientInfo.value || clientInfo.value.currentProfile === UserProfile.Outsider) {
    return true;
  }
  // If our role is not Manager or Owner, can't change anything
  if (props.ownRole === null || props.ownRole === WorkspaceRole.Reader || props.ownRole === WorkspaceRole.Contributor) {
    return true;
  }
  // If the user's role is Owner, can't change it
  if (role === WorkspaceRole.Owner) {
    return true;
  }
  // If our own role is Manager and the user's role is Manager, can't change it
  if (role === WorkspaceRole.Manager && props.ownRole === WorkspaceRole.Manager) {
    return true;
  }

  return false;
}

async function refreshSharingInfo(searchString = ''): Promise<void> {
  const result = await getWorkspaceSharing(props.workspaceId, true);

  if (result.ok) {
    if (searchString !== '') {
      const roles: Array<[UserTuple, WorkspaceRole | null]> = [];
      const lowerCaseSearch = search.value.toLocaleLowerCase();

      for (const entry of result.value) {
        if (entry[0].humanHandle.label.toLocaleLowerCase().includes(lowerCaseSearch)) {
          roles.push(entry);
        }
      }
      userRoles.value = roles;
    } else {
      userRoles.value = result.value;
    }
  } else {
    notificationCenter.showToast(
      new Notification({
        message: t('WorkspaceSharing.listFailure'),
        level: NotificationLevel.Error,
      }),
    );
  }
}

onMounted(async () => {
  const result = await getClientInfo();
  if (result.ok) {
    clientInfo.value = result.value;
  }
  await refreshSharingInfo();
});

onUnmounted(() => {
  unwatchSearch();
});

async function updateUserRole(user: UserTuple, role: WorkspaceRole | null): Promise<void> {
  const result = await shareWorkspace(props.workspaceId, user.id, role);
  if (result.ok) {
    if (!role) {
      notificationCenter.showToast(
        new Notification({
          message: t('WorkspaceSharing.unshareSuccess', {
            user: user.humanHandle.label,
          }),
          level: NotificationLevel.Success,
        }),
      );
    } else {
      notificationCenter.showToast(
        new Notification({
          message: t('WorkspaceSharing.updateRoleSuccess', {
            user: user.humanHandle.label,
            role: translateWorkspaceRole(t, role),
          }),
          level: NotificationLevel.Success,
        }),
      );
    }
  } else {
    notificationCenter.showToast(
      new Notification({
        message: t('WorkspaceSharing.updateRoleFailure', {
          user: user.humanHandle.label,
        }),
        level: NotificationLevel.Error,
      }),
    );
  }
  await refreshSharingInfo();
}
</script>

<!-- eslint-disable-next-line vue-scoped-css/enforce-style-type -->
<style lang="scss">
.ms-modal {
  display: flex;
  flex-direction: column;
  max-height: 45em;

  .inner-content {
    overflow-y: auto;
    height: 100%;
  }
}
</style>

<style scoped lang="scss">
.user-list {
  padding: 0.5rem;
}
</style>
