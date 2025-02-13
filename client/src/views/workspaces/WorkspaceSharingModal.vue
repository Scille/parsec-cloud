<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="I18n.valueAsTranslatable(workspaceName)"
      :close-button="{ visible: true }"
    >
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
      <div class="modal-head-content">
        <ms-search-input
          class="modal-head-content__search"
          v-model="search"
          :placeholder="'WorkspaceSharing.searchPlaceholder'"
        />
        <div class="modal-head-content-right">
          <ion-text
            class="selected-counter"
            v-show="showCheckboxes && selectedUsers.length > 0"
          >
            {{ $msTranslate({ key: 'WorkspaceSharing.batchSharing.counter', data: { count: selectedUsers.length } }) }}
          </ion-text>
          <ion-text
            id="batch-activate-button"
            @click="onBatchSharingActivate()"
            v-show="batchSharingEnabled"
            fill="clear"
            class="button-small"
          >
            {{ $msTranslate(showCheckboxes ? 'WorkspaceSharing.batchSharing.buttonFinish' : 'WorkspaceSharing.batchSharing.buttonSelect') }}
          </ion-text>
          <ms-dropdown
            v-show="showCheckboxes"
            ref="dropdownRef"
            class="dropdown"
            :options="options"
            :disabled="selectedUsers.length === 0"
            :label="'WorkspaceSharing.batchSharing.chooseRole'"
            :appearance="MsAppearance.Outline"
            @change="onBatchRoleChange($event.option)"
          />
        </div>
      </div>
      <ms-report-text
        :theme="MsReportTheme.Info"
        id="profile-assign-info"
        v-if="showCheckboxes && orgHasExternalUsers"
      >
        <i18n-t
          keypath="WorkspaceSharing.batchSharing.outsiderRoleWarning"
          scope="global"
        >
          <template #external>
            <strong> {{ $msTranslate('WorkspaceSharing.batchSharing.external') }} </strong>
          </template>
          <template #contributor>
            <strong> {{ $msTranslate('WorkspaceSharing.batchSharing.contributor') }} </strong>
          </template>
          <template #reader>
            <strong> {{ $msTranslate('WorkspaceSharing.batchSharing.reader') }} </strong>
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
                filteredUserRoles.length === 0 &&
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
            <ion-text class="user-list-title title-h5">
              <ms-checkbox
                class="checkbox"
                id="all-members-checkbox"
                :indeterminate="someMembersSelected && !allMembersSelected"
                :checked="allMembersSelected"
                @change="selectAllMembers()"
                v-show="showCheckboxes && selectableFilteredMembers.length > 0"
              />
              {{ $msTranslate({ key: 'workspaceRoles.members', data: { count: countSharedUsers }, count: countSharedUsers }) }}
            </ion-text>
            <workspace-user-role
              v-if="clientInfo"
              :class="{ 'checkbox-space': showCheckboxes }"
              v-show="currentUserMatchSearch()"
              :disabled="true"
              :user="{ id: clientInfo.userId, humanHandle: clientInfo.humanHandle, profile: clientInfo.currentProfile }"
              :role="ownRole"
              :client-profile="ownProfile"
              :client-role="ownRole"
              :is-current-user="true"
              class="current-user"
            />
            <div
              v-for="entry in filteredSharedUserRoles"
              :key="`${entry.user.id}-${entry.role}`"
              class="user-list-members-item"
            >
              <ms-checkbox
                id="member-checkbox"
                v-show="showCheckboxes"
                :disabled="!(entry.role && canSelectUser(entry.user.profile, entry.role))"
                v-model="entry.isSelected"
              />
              <workspace-user-role
                class="workspace-user-role"
                :class="showCheckboxes ? 'current-user' : ''"
                :disabled="isSelectDisabled(entry.role) || showCheckboxes"
                :user="entry.user"
                :role="entry.role"
                :client-profile="ownProfile"
                :client-role="ownRole"
                :show-checkbox="showCheckboxes"
                :checkbox-value="entry.isSelected"
                @role-update="updateUserRole"
              />
            </div>
          </div>

          <!-- user suggestions -->
          <div
            class="user-list-suggestions"
            v-if="(ownRole === WorkspaceRole.Manager || ownRole === WorkspaceRole.Owner) && filteredNotSharedUserRoles.length > 0"
          >
            <ion-text class="user-list-title title-h5">
              {{ $msTranslate('workspaceRoles.suggestion') }}
            </ion-text>
            <div
              v-for="entry in filteredNotSharedUserRoles"
              :key="`${entry.user.id}-${entry.role}`"
              class="user-list-suggestions-item"
            >
              <ms-checkbox
                id="suggested-checkbox"
                v-show="showCheckboxes"
                v-model="entry.isSelected"
              />
              <workspace-user-role
                class="workspace-user-role"
                :class="showCheckboxes ? 'current-user' : ''"
                :disabled="isSelectDisabled(entry.role) || showCheckboxes"
                :user="entry.user"
                :role="entry.role"
                :client-profile="ownProfile"
                :client-role="ownRole"
                :show-checkbox="showCheckboxes"
                :checkbox-value="entry.isSelected"
                @role-update="updateUserRole"
              />
            </div>
          </div>
        </ion-list>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { MsAppearance, MsCheckbox, MsDropdown, MsModal, MsOption, MsOptions, MsSearchInput } from 'megashark-lib';
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
  ClientShareWorkspaceError,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { I18n, MsReportText, MsReportTheme } from 'megashark-lib';
import { IonList, IonPage, IonText } from '@ionic/vue';
import { Ref, onMounted, ref, computed } from 'vue';
import { canChangeRole } from '@/components/workspaces/utils';

const search = ref('');
let ownProfile = UserProfile.Outsider;

const props = defineProps<{
  workspaceId: WorkspaceID;
  workspaceName: WorkspaceName;
  ownRole: WorkspaceRole | null;
  informationManager: InformationManager;
}>();

interface UserRole {
  user: UserTuple;
  role: WorkspaceRole | null;
  isSelected: boolean;
}

const clientInfo: Ref<ClientInfo | null> = ref(null);

const showCheckboxes = ref<boolean>(false);
const dropdownRef = ref();

const userRoles: Ref<Array<UserRole>> = ref([]);

const filteredUserRoles = computed(() => {
  const searchString = search.value.toLocaleLowerCase();
  return userRoles.value.filter((userRole: UserRole) => {
    return (
      userRole.user.humanHandle.email.toLocaleLowerCase().includes(searchString) ||
      userRole.user.humanHandle.label.toLocaleLowerCase().includes(searchString)
    );
  });
});

const filteredSharedUserRoles = computed(() => {
  return filteredUserRoles.value
    .filter((userRole: UserRole) => userRole.role !== null)
    .sort((item1, item2) => item1.user.humanHandle.label.localeCompare(item2.user.humanHandle.label));
});

const filteredNotSharedUserRoles = computed(() => {
  return filteredUserRoles.value
    .filter((userRole: UserRole) => userRole.role === null)
    .sort((item1, item2) => item1.user.humanHandle.label.localeCompare(item2.user.humanHandle.label));
});

const selectedUsers = computed(() => userRoles.value.filter((user) => user.isSelected === true));
const countSharedUsers = computed(() => filteredSharedUserRoles.value.length + (clientInfo.value ? 1 : 0));
const selectableFilteredMembers = computed(() => {
  return filteredSharedUserRoles.value.filter((user) => user.role && canSelectUser(user.user.profile, user.role));
});
const someMembersSelected = computed(() => selectableFilteredMembers.value.some((user) => user.isSelected === true));
const allMembersSelected = computed(() => selectableFilteredMembers.value.every((user) => user.isSelected === true));
const orgHasExternalUsers = computed(() => userRoles.value.some((user) => user.user.profile === UserProfile.Outsider));
const batchSharingEnabled = computed(() => {
  return (
    (props.ownRole === WorkspaceRole.Owner || props.ownRole === WorkspaceRole.Manager) &&
    filteredNotSharedUserRoles.value.length + selectableFilteredMembers.value.length > 1
  );
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
  // If our own role is Manager and the user's role is Manager or owner, can't change it
  if ((role === WorkspaceRole.Manager || role === WorkspaceRole.Owner) && props.ownRole === WorkspaceRole.Manager) {
    return true;
  }

  return false;
}

async function onBatchSharingActivate(): Promise<void> {
  if (showCheckboxes.value === true) {
    for (const user of userRoles.value) {
      user.isSelected = false;
    }
  }
  showCheckboxes.value = !showCheckboxes.value;
}

function getLowestSelectedProfile(): UserProfile {
  let lowestProfile = UserProfile.Admin;
  for (const user of selectedUsers.value) {
    if (user.user.profile === UserProfile.Outsider) {
      return UserProfile.Outsider;
    }
    if (user.user.profile === UserProfile.Standard) {
      lowestProfile = UserProfile.Standard;
    }
  }
  return lowestProfile;
}

function getHighestSelectedRole(): WorkspaceRole {
  let highestRole = WorkspaceRole.Reader;
  for (const user of selectedUsers.value) {
    if (!user.role) {
      continue;
    }
    if (user.role === WorkspaceRole.Manager || user.role === WorkspaceRole.Owner) return user.role;
    if (user.role === WorkspaceRole.Contributor) {
      highestRole = WorkspaceRole.Contributor;
    }
  }
  return highestRole;
}

const options = computed((): MsOptions => {
  return new MsOptions(
    [WorkspaceRole.Owner, WorkspaceRole.Manager, WorkspaceRole.Contributor, WorkspaceRole.Reader, null].map(
      (role: WorkspaceRole | null) => {
        return {
          key: role,
          label: getWorkspaceRoleTranslationKey(role).label,
          description: getWorkspaceRoleTranslationKey(role).description,
          disabled: !canChangeRole(ownProfile, getLowestSelectedProfile(), props.ownRole, getHighestSelectedRole(), role).authorized,
          disabledReason: canChangeRole(ownProfile, getLowestSelectedProfile(), props.ownRole, getHighestSelectedRole(), role).reason,
        };
      },
    ),
  );
});

function canSelectUser(userProfile: UserProfile, userRole: WorkspaceRole): boolean {
  return canChangeRole(ownProfile, userProfile, props.ownRole, userRole, userRole).authorized;
}

function selectAllMembers(): void {
  const value = !allMembersSelected.value;
  for (const user of selectableFilteredMembers.value) {
    user.isSelected = value;
  }
}

async function refreshSharingInfo(): Promise<void> {
  const result = await getWorkspaceSharing(props.workspaceId, true);

  if (result.ok) {
    userRoles.value = result.value.map((userInfo) => {
      return {
        user: userInfo[0],
        role: userInfo[1],
        isSelected: false,
      };
    });
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
  for (const user of userRoles.value) {
    if (user.role === WorkspaceRole.Owner) {
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
  const current = userRoles.value.find((item) => item.user.id === user.id);

  // Trying to set the same role again
  if (current && current.role === newRole) {
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

async function onBatchRoleChange(newRoleOption: MsOption): Promise<void> {
  let changesMade: number = 0;
  let errorCount: number = 0;
  let error: ClientShareWorkspaceError | undefined = undefined;

  for (const userRole of selectedUsers.value) {
    if (!userRole.role || newRoleOption.key !== userRole.role) {
      const result = await shareWorkspace(props.workspaceId, userRole.user.id, newRoleOption.key);
      if (result.ok) {
        changesMade += 1;
      } else {
        if (errorCount === 0) {
          error = result.error;
        }
        errorCount += 1;
      }
    }
  }

  if (errorCount > 0) {
    if (changesMade === 0) {
      console.error(error);
      props.informationManager.present(
        new Information({
          message: { key: 'WorkspaceSharing.batchSharing.allFailed', data: { reason: error?.tag } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      props.informationManager.present(
        new Information({
          message: 'WorkspaceSharing.batchSharing.someFailed',
          level: InformationLevel.Warning,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    if (newRoleOption.key) {
      props.informationManager.present(
        new Information({
          message: {
            key: 'WorkspaceSharing.batchSharing.updateRoleSuccess',
            data: {
              role: I18n.translate(getWorkspaceRoleTranslationKey(newRoleOption.key).label),
            },
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    } else {
      props.informationManager.present(
        new Information({
          message: 'WorkspaceSharing.batchSharing.unshareSuccess',
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  }

  dropdownRef.value.setCurrentKey(0);
  showCheckboxes.value = false;
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

  &-right {
    display: flex;
    gap: 1rem;

    #batch-activate-button {
      font-size: 0.875rem;
      padding: 0.5rem 1rem;
      color: var(--parsec-color-light-secondary-text);
      background: var(--parsec-color-light-secondary-premiere);
      border-radius: var(--parsec-radius-6);
      cursor: pointer;
      transition: background 0.2s;

      &:hover {
        background: var(--parsec-color-light-secondary-medium);
      }
    }
  }

  .selected-counter {
    color: var(--parsec-color-light-secondary-grey);
    font-size: 0.9em;
    margin: auto;
    text-align: center;
  }
}

#profile-assign-info {
  margin-bottom: 0;
  margin-top: 0.5rem;
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
  position: relative;

  &-title {
    color: var(--parsec-color-light-secondary-grey);
    background: var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-6);
    padding: 0.375rem 0.75rem;
    display: flex;
    position: sticky;
    align-items: center;
    gap: 0.5rem;
    top: 0;
    z-index: 3;
  }

  &-members,
  &-suggestions {
    display: flex;
    flex-direction: column;

    #member-checkbox,
    #suggested-checkbox {
      padding-left: 0.75rem;
    }

    &-item {
      display: flex;
      align-items: center;
      gap: 0.5rem;

      &:last-child .workspace-user-role::after {
        border-bottom: none;
      }

      .workspace-user-role {
        flex: 1;
      }
    }
  }

  .checkbox-space {
    padding-left: 3rem;
  }
}
</style>
