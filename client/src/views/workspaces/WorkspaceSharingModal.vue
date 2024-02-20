<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="$t('WorkspaceSharing.title')"
      :close-button="{ visible: true }"
    >
      <!-- content -->
      <div class="modal-container">
        <ms-input
          v-model="search"
          :label="$t('WorkspaceSharing.searchLabel')"
          :placeholder="$t('WorkspaceSharing.searchPlaceholder')"
        />
        <ion-list class="user-list">
          <workspace-user-role
            :disabled="true"
            :user="{
              id: 'FAKE',
              humanHandle: {
                label: $t('WorkspaceSharing.currentUserLabel'),
                email: '',
              },
              profile: UserProfile.Outsider,
            }"
            :role="ownRole"
            :client-profile="ownProfile"
            :client-role="ownRole"
          />
          <workspace-user-role
            :disabled="isSelectDisabled(entry[1])"
            v-for="entry in userRoles"
            :key="entry[0].id"
            :user="entry[0]"
            :role="entry[1]"
            :client-profile="ownProfile"
            :client-role="ownRole"
            @role-update="updateUserRole"
          />
        </ion-list>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { MsInput, MsModal } from '@/components/core';
import WorkspaceUserRole from '@/components/workspaces/WorkspaceUserRole.vue';
import { UserProfile, UserTuple, WorkspaceID, WorkspaceRole, getClientProfile, getWorkspaceSharing, shareWorkspace } from '@/parsec';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { translate, translateWorkspaceRole } from '@/services/translation';
import { IonList, IonPage } from '@ionic/vue';
import { Ref, inject, onMounted, onUnmounted, ref, watch } from 'vue';

const informationManager: InformationManager = inject(InformationKey)!;
const search = ref('');
let ownProfile = UserProfile.Outsider;

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
  if (ownProfile === UserProfile.Outsider) {
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
    informationManager.present(
      new Information({
        message: translate('WorkspaceSharing.listFailure.message'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

onMounted(async () => {
  ownProfile = await getClientProfile();
  await refreshSharingInfo();
});

onUnmounted(() => {
  unwatchSearch();
});

async function updateUserRole(user: UserTuple, role: WorkspaceRole | null): Promise<void> {
  const current = userRoles.value.find((item) => item[0].id === user.id);

  // Trying to set the same role again
  if (current && current[1] === role) {
    if (role === null) {
      informationManager.present(
        new Information({
          message: translate('WorkspaceSharing.listFailure.alreadyNotShared', { user: user.humanHandle.label }),
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: translate('WorkspaceSharing.listFailure.alreadyHasRole', {
            user: user.humanHandle.label,
            role: translateWorkspaceRole(role).label,
          }),
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    }
    return;
  }
  const result = await shareWorkspace(props.workspaceId, user.id, role);
  if (result.ok) {
    if (!role) {
      informationManager.present(
        new Information({
          message: translate('WorkspaceSharing.unshareSuccess', {
            user: user.humanHandle.label,
          }),
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: translate('WorkspaceSharing.updateRoleSuccess', {
            user: user.humanHandle.label,
            role: translateWorkspaceRole(role).label,
          }),
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: translate('WorkspaceSharing.updateRoleFailure', {
          user: user.humanHandle.label,
        }),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
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

  .inner-content {
    overflow-y: hidden;
    height: 100%;
  }
}
</style>

<style scoped lang="scss">
.modal-container {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.user-list {
  margin-top: 0.5rem;
  padding-right: 0.5rem;
  overflow-y: auto;
}
</style>
