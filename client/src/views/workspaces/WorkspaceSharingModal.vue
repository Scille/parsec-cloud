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
      <ms-report-text
        :theme="MsReportTheme.Info"
        v-if="isOnlyOwner()"
        class="only-owner-warning"
      >
        <i18n-t
          keypath="WorkspaceSharing.onlyOwnerWarning"
          scope="global"
        >
          <template #owner>
            <strong> {{ $msTranslate('WorkspaceSharing.owner') }} </strong>
          </template>
        </i18n-t>
      </ms-report-text>
      <!-- content -->
      <div class="modal-container">
        <ms-search-input
          v-model="search"
          :placeholder="'WorkspaceSharing.searchPlaceholder'"
        />
        <ion-list class="user-list">
          <workspace-user-role
            v-if="clientInfo"
            v-show="currentUserMatchSearch()"
            :disabled="true"
            :user="{ id: clientInfo.userId, humanHandle: clientInfo.humanHandle, profile: clientInfo.currentProfile }"
            :role="ownRole"
            :client-profile="ownProfile"
            :client-role="ownRole"
            :is-current-user="true"
            class="current-user"
          />
          <ion-text
            class="no-match-result body"
            v-show="userRoles.length > 0 && filteredUserRoles.length === 0 && !currentUserMatchSearch()"
          >
            {{ $msTranslate({ key: 'WorkspaceSharing.noMatch', data: { query: search } }) }}
          </ion-text>
          <workspace-user-role
            v-for="entry in filteredUserRoles"
            :disabled="isSelectDisabled(entry[1])"
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
import { MsModal, MsSearchInput } from 'megashark-lib';
import WorkspaceUserRole from '@/components/workspaces/WorkspaceUserRole.vue';
import {
  ClientInfo,
  UserProfile,
  UserTuple,
  WorkspaceID,
  WorkspaceName,
  WorkspaceRole,
  getClientProfile,
  getClientInfo,
  getWorkspaceSharing,
  shareWorkspace,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { I18n, MsReportText, MsReportTheme } from 'megashark-lib';
import { IonList, IonPage, IonText } from '@ionic/vue';
import { Ref, onMounted, ref, computed } from 'vue';

const search = ref('');
let ownProfile = UserProfile.Outsider;

const props = defineProps<{
  workspaceId: WorkspaceID;
  workspaceName: WorkspaceName;
  ownRole: WorkspaceRole | null;
  informationManager: InformationManager;
}>();

const clientInfo: Ref<ClientInfo | null> = ref(null);

const userRoles: Ref<Array<[UserTuple, WorkspaceRole | null]>> = ref([]);
const filteredUserRoles = computed(() => {
  const searchString = search.value.toLocaleLowerCase();
  return userRoles.value.filter((item) => {
    return (
      item[0].humanHandle.email.toLocaleLowerCase().includes(searchString) ||
      item[0].humanHandle.label.toLocaleLowerCase().includes(searchString)
    );
  });
});

function currentUserMatchSearch(): boolean {
  const searchString = search.value.toLocaleLowerCase();
  if (!clientInfo.value) {
    return false;
  }
  return (
    clientInfo.value.humanHandle.email.toLocaleLowerCase().includes(searchString) ||
    clientInfo.value.humanHandle.label.toLocaleLowerCase().includes(searchString)
  );
}

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

async function refreshSharingInfo(): Promise<void> {
  const result = await getWorkspaceSharing(props.workspaceId, true);

  if (result.ok) {
    userRoles.value = result.value;
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
  const result = await getClientInfo();
  if (result.ok) {
    clientInfo.value = result.value;
  }
  ownProfile = await getClientProfile();
  await refreshSharingInfo();
});

function isOnlyOwner(): boolean {
  if (props.ownRole !== WorkspaceRole.Owner) {
    return false;
  }
  for (const role of userRoles.value) {
    if (role[1] === WorkspaceRole.Owner) {
      return false;
    }
  }
  return true;
}

async function updateUserRole(
  user: UserTuple,
  _oldRole: WorkspaceRole | null,
  newRole: WorkspaceRole | null,
  reject: () => void,
): Promise<void> {
  const current = userRoles.value.find((item) => item[0].id === user.id);

  // Trying to set the same role again
  if (current && current[1] === newRole) {
    if (newRole === null) {
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
              role: I18n.translate(getWorkspaceRoleTranslationKey(newRole).label),
            },
          },
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    }
    return;
  }
  const result = await shareWorkspace(props.workspaceId, user.id, newRole);
  if (result.ok) {
    if (!newRole) {
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
              role: I18n.translate(getWorkspaceRoleTranslationKey(newRole).label),
            },
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    reject();
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
  padding-bottom: 1.5rem;
  color: var(--parsec-color-light-secondary-text);
}

.only-owner-warning {
  margin-bottom: 1rem;
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
