<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="'WorkspaceSharing.title'"
      :close-button="{ visible: true }"
    >
      <div class="modal-head-content">
        <ion-text class="modal-title title-h4">
          {{ workspaceName }}
        </ion-text>
        <ms-search-input
          class="modal-head-content__search"
          v-model="search"
          :placeholder="'WorkspaceSharing.searchPlaceholder'"
        />
      </div>
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
        <ion-list class="user-list">
          <!-- prettier-ignore -->
          <ion-text
            class="no-match-result body"
            v-show="
              userRoles.length > 0 &&
                filteredSharedUserRoles.length === 0 &&
                filteredNotSharedUserRoles.length === 0 &&
                !currentUserMatchSearch()
            "
          >
            {{ $msTranslate({ key: 'WorkspaceSharing.noMatch', data: { query: search } }) }}
          </ion-text>

          <!-- workspaces members -->
          <div
            class="user-list-members"
            v-show="currentUserMatchSearch() || filteredSharedUserRoles.length > 0"
          >
            <ion-text class="user-list__title title-h5">
              {{ $msTranslate({ key: 'workspaceRoles.members', data: { count: countSharedUsers }, count: countSharedUsers }) }}
            </ion-text>
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

            <workspace-user-role
              v-for="entry in filteredSharedUserRoles"
              :disabled="isSelectDisabled(entry[1])"
              :key="entry[0].id"
              :user="entry[0]"
              :role="entry[1]"
              :client-profile="ownProfile"
              :client-role="ownRole"
              @role-update="updateUserRole"
            />
          </div>

          <!-- user suggestions -->
          <div
            class="user-list-suggestions"
            v-if="filteredNotSharedUserRoles.length > 0"
          >
            <ion-text class="user-list__title title-h5">{{ $msTranslate('workspaceRoles.suggestion') }}</ion-text>
            <workspace-user-role
              v-for="entry in filteredNotSharedUserRoles"
              :disabled="isSelectDisabled(entry[1])"
              :key="entry[0].id"
              :user="entry[0]"
              :role="entry[1]"
              :client-profile="ownProfile"
              :client-role="ownRole"
              @role-update="updateUserRole"
            />
          </div>
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
let ownProfile = UserProfile.External;

const props = defineProps<{
  workspaceId: WorkspaceID;
  workspaceName: WorkspaceName;
  ownRole: WorkspaceRole | null;
  informationManager: InformationManager;
}>();

const clientInfo: Ref<ClientInfo | null> = ref(null);

const userRoles: Ref<Array<[UserTuple, WorkspaceRole | null]>> = ref([]);
const filteredSharedUserRoles = computed(() => {
  const searchString = search.value.toLocaleLowerCase();
  return userRoles.value
    .filter(([user, role]) => {
      const isSharedUser = role !== null;
      const matchesSearch =
        user.humanHandle.email.toLocaleLowerCase().includes(searchString) ||
        user.humanHandle.label.toLocaleLowerCase().includes(searchString);
      return isSharedUser && matchesSearch;
    })
    .sort((item1, item2) => item1[0].humanHandle.label.localeCompare(item2[0].humanHandle.label));
});

const filteredNotSharedUserRoles = computed(() => {
  const searchString = search.value.toLocaleLowerCase();
  return userRoles.value
    .filter(([user, role]) => {
      const isNotSharedUser = role === null;
      const matchesSearch =
        user.humanHandle.email.toLocaleLowerCase().includes(searchString) ||
        user.humanHandle.label.toLocaleLowerCase().includes(searchString);
      return isNotSharedUser && matchesSearch;
    })
    .sort((item1, item2) => item1[0].humanHandle.label.localeCompare(item2[0].humanHandle.label));
});

const countSharedUsers = computed(() => filteredSharedUserRoles.value.length + (clientInfo.value ? 1 : 0));

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
  // External should not be able to change anything
  if (ownProfile === UserProfile.External) {
    return true;
  }
  // If our role is not Manager or Owner, can't change anything
  if (props.ownRole === null || props.ownRole === WorkspaceRole.Reader || props.ownRole === WorkspaceRole.Contributor) {
    return true;
  }
  // If our own role is Manager and the user's role is Manager or owner, can't change it
  if ((role === WorkspaceRole.Manager || role === WorkspaceRole.Owner) && props.ownRole === WorkspaceRole.Manager) {
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

  &-header {
    padding: 0 !important;
  }
}

.modal-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--parsec-color-light-secondary-hard-grey);
}

.only-owner-warning {
  margin-bottom: 1rem;
}
</style>

<style scoped lang="scss">
.modal-head-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin: 0.5rem 0;

  &__search {
    max-height: 2.25rem;
    max-width: 15rem;
    margin: 0;
  }
}

.modal-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.user-list {
  padding: 0;
  margin-top: 1rem;
  overflow-y: auto;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  border-radius: var(--parsec-radius-6);
  position: relative;

  &__title {
    color: var(--parsec-color-light-secondary-grey);
    background: var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-6);
    padding: 0.375rem 0.5rem;
    display: flex;
    position: sticky;
    top: 0;
    z-index: 3;
  }
}
</style>
