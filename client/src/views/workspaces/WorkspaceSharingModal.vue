<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="'WorkspaceSharing.title'"
      :close-button="{ visible: true }"
    >
      <ion-text class="modal-title">
        {{ workspaceName }}
      </ion-text>
      <!-- content -->
      <div class="modal-container">
        <ms-search-input
          v-model="search"
          :placeholder="'WorkspaceSharing.searchPlaceholder'"
        />
        <ion-list class="user-list">
          <workspace-user-role
            :disabled="true"
            :user="{
              id: 'FAKE',
              humanHandle: {
                label: $msTranslate('WorkspaceSharing.currentUserLabel'),
                email: '',
              },
              profile: UserProfile.Outsider,
            }"
            :role="ownRole"
            :client-profile="ownProfile"
            :client-role="ownRole"
            class="current-user"
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
import { MsModal, MsSearchInput } from '@/components/core';
import WorkspaceUserRole from '@/components/workspaces/WorkspaceUserRole.vue';
import {
  UserProfile,
  UserTuple,
  WorkspaceID,
  WorkspaceName,
  WorkspaceRole,
  getClientProfile,
  getWorkspaceSharing,
  shareWorkspace,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { I18n } from 'megashark-lib';
import { IonList, IonPage, IonText } from '@ionic/vue';
import { Ref, onMounted, onUnmounted, ref, watch } from 'vue';

const search = ref('');
let ownProfile = UserProfile.Outsider;

const props = defineProps<{
  workspaceId: WorkspaceID;
  workspaceName: WorkspaceName;
  ownRole: WorkspaceRole | null;
  informationManager: InformationManager;
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
    props.informationManager.present(
      new Information({
        message: 'WorkspaceSharing.listFailure.message',
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
      props.informationManager.present(
        new Information({
          message: { key: 'WorkspaceSharing.listFailure.alreadyNotShared', data: { user: user.humanHandle.label } },
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    } else {
      props.informationManager.present(
        new Information({
          message: {
            key: 'WorkspaceSharing.listFailure.alreadyHasRole',
            data: {
              user: user.humanHandle.label,
              role: I18n.translate(getWorkspaceRoleTranslationKey(role).label),
            },
          },
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
      props.informationManager.present(
        new Information({
          message: {
            key: 'WorkspaceSharing.unshareSuccess',
            data: {
              user: user.humanHandle.label,
            },
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    } else {
      props.informationManager.present(
        new Information({
          message: {
            key: 'WorkspaceSharing.updateRoleSuccess',
            data: {
              user: user.humanHandle.label,
              role: I18n.translate(getWorkspaceRoleTranslationKey(role).label),
            },
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    props.informationManager.present(
      new Information({
        message: {
          key: 'WorkspaceSharing.updateRoleFailure',
          data: {
            user: user.humanHandle.label,
          },
        },
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
    height: 100%;
  }
}

.modal-title {
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  margin-bottom: 1.5rem;
  color: var(--parsec-color-light-secondary-text);
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
  height: 100%;
}
</style>
