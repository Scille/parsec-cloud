<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="$msTranslate('WorkspaceSharing.title')"
      :close-button="{ visible: true }"
    >
      <ion-text class="sharing-modal__title body">{{ workspaceName }}</ion-text>
      <div class="modal-head-content">
        <ms-search-input
          class="modal-head-content__search"
          v-model="search"
          :placeholder="'WorkspaceSharing.searchPlaceholder'"
        />
        <div class="modal-head-content-right">
          <ion-text
            class="selected-counter body"
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
            :class="{ 'done-button': showCheckboxes }"
          >
            <ion-icon
              v-if="!showCheckboxes"
              class="checkbox-icon"
              :icon="checkmarkCircle"
            />
            {{ $msTranslate(showCheckboxes ? 'WorkspaceSharing.batchSharing.buttonFinish' : 'WorkspaceSharing.batchSharing.buttonSelect') }}
          </ion-text>
          <ms-dropdown
            v-show="showCheckboxes"
            ref="dropdown"
            class="dropdown"
            :options="options"
            :disabled="selectedUsers.length === 0"
            :label="'WorkspaceSharing.batchSharing.chooseRole'"
            :appearance="MsAppearance.Outline"
            @change="onBatchRoleChange($event.option)"
          />
        </div>
      </div>

      <div
        class="report-text-content"
        v-if="isOnlyOwner() || orgHasExternalUsers"
      >
        <ms-report-text
          :theme="MsReportTheme.Info"
          id="only-owner-warning"
          v-if="isOnlyOwner()"
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
        <ms-report-text
          :theme="MsReportTheme.Info"
          id="profile-assign-info"
          v-if="orgHasExternalUsers"
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
      </div>

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
              v-if="clientInfo && currentUserMatchSearch()"
              :class="{ 'checkbox-space': showCheckboxes }"
              :disabled="true"
              :user="{ id: clientInfo.userId, humanHandle: clientInfo.humanHandle, profile: clientInfo.currentProfile }"
              :role="ownRole"
              :client-profile="ownProfile"
              :client-role="ownRole"
              :is-current-user="true"
              class="current-user user-member-item"
            />
            <div
              v-for="entry in filteredSharedUserRoles"
              :key="`${entry.user.id}-${entry.role}`"
              class="user-list-members-item"
            >
              <ms-checkbox
                class="member-checkbox"
                :class="isSmallDisplay ? 'checkbox-mobile' : ''"
                v-show="showCheckboxes"
                :disabled="!(entry.role && canSelectUser(entry.user.profile, entry.role))"
                v-model="entry.isSelected"
              />
              <workspace-user-role
                class="workspace-user-role user-member-item"
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
                class="suggested-checkbox"
                :class="isSmallDisplay ? 'checkbox-mobile' : ''"
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
import { canChangeRole } from '@/components/workspaces/utils';
import WorkspaceUserRole from '@/components/workspaces/WorkspaceUserRole.vue';
import {
  ClientInfo,
  ClientShareWorkspaceError,
  UserProfile,
  UserTuple,
  WorkspaceID,
  WorkspaceName,
  WorkspaceRole,
  getClientInfo,
  getClientProfile,
  getWorkspaceSharing,
  shareWorkspace,
} from '@/parsec';
import { EventDistributor, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { IonIcon, IonList, IonPage, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import {
  I18n,
  MsAppearance,
  MsCheckbox,
  MsDropdown,
  MsModal,
  MsOption,
  MsOptions,
  MsReportText,
  MsReportTheme,
  MsSearchInput,
  useWindowSize,
} from 'megashark-lib';
import { Ref, computed, onMounted, ref, useTemplateRef } from 'vue';

const search = ref('');
let ownProfile = UserProfile.Outsider;
const { isSmallDisplay } = useWindowSize();

const props = defineProps<{
  workspaceId: WorkspaceID;
  workspaceName: WorkspaceName;
  ownRole: WorkspaceRole | null;
  informationManager: InformationManager;
  eventDistributor: EventDistributor;
}>();

interface UserRole {
  user: UserTuple;
  role: WorkspaceRole | null;
  isSelected: boolean;
}

const clientInfo: Ref<ClientInfo | null> = ref(null);

const showCheckboxes = ref<boolean>(false);
const dropdownRef = useTemplateRef<InstanceType<typeof MsDropdown>>('dropdown');

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
  const roleOrder = [WorkspaceRole.Owner, WorkspaceRole.Manager, WorkspaceRole.Contributor, WorkspaceRole.Reader];
  return filteredUserRoles.value
    .filter((userRole: UserRole) => userRole.role !== null)
    .sort((item1, item2) => {
      const roleCompare = roleOrder.indexOf(item1.role!) - roleOrder.indexOf(item2.role!);

      if (roleCompare === 0) {
        return item1.user.humanHandle.label.localeCompare(item2.user.humanHandle.label);
      }

      return roleCompare;
    });
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
  oldRole: WorkspaceRole | null,
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
    props.eventDistributor.dispatchEvent(Events.WorkspaceRoleUpdate, {
      newRole: newRole,
      oldRole: oldRole,
      workspaceId: props.workspaceId,
      userId: user.id,
    });
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

  dropdownRef.value?.setCurrentKey(0);
  showCheckboxes.value = false;
  await refreshSharingInfo();
}
</script>

<style scoped lang="scss">
.sharing-modal__title {
  position: relative;
  margin-inline: 2rem;
  z-index: 10;
  display: block;
  color: var(--parsec-color-light-secondary-hard-grey);
}

.report-text-content {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  position: relative;
  padding-inline: 2rem;
  margin-top: 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1.5rem;
  }
}

.modal-head-content {
  display: flex;
  align-items: stretch;
  justify-content: space-between;
  gap: 1.5rem;
  margin-top: 1rem;
  overflow: hidden !important;
  flex-shrink: 0;
  padding: 0.25rem 2rem;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1.5rem;
    margin-top: 0.5rem;
  }

  &__search {
    max-height: 2.5rem;
    max-width: 20rem;
    margin: 0;

    @include ms.responsive-breakpoint('sm') {
      max-width: 100%;
    }
  }

  &-right {
    display: flex;
    align-items: stretch;
    flex-shrink: 0;
    gap: 1rem;

    @include ms.responsive-breakpoint('sm') {
      position: absolute;
      left: 0;
      bottom: 0;
      width: 100%;
      padding: 1rem 1.5rem 2rem;
      justify-content: space-between;
      background: var(--parsec-color-light-secondary-white);
      border-radius: var(--parsec-radius-8) var(--parsec-radius-8) 0 0;
      box-shadow: var(--parsec-shadow-strong);
      z-index: 2;
    }

    .dropdown {
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-8);
    }

    #batch-activate-button {
      font-size: 0.875rem;
      padding: 0.625rem 1rem;
      color: var(--parsec-color-light-secondary-text);
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-8);
      cursor: pointer;
      transition: background 0.2s;
      display: flex;
      align-items: center;
      margin-left: auto;
      gap: 0.375rem;

      @include ms.responsive-breakpoint('sm') {
        order: 3;
      }

      .checkbox-icon {
        color: var(--parsec-color-light-secondary-text);
        font-size: 1rem;
      }

      &:hover {
        background: var(--parsec-color-light-secondary-medium);
      }

      &:active {
        box-shadow: none;
      }

      &.done-button {
        background: var(--parsec-color-light-secondary-text);
        color: var(--parsec-color-light-secondary-white);
        border: none;

        &:hover {
          background: var(--parsec-color-light-secondary-contrast);
        }
      }
    }
  }

  .selected-counter {
    color: var(--parsec-color-light-secondary-grey);
    margin-right: auto;
    text-align: center;
    align-self: center;

    @include ms.responsive-breakpoint('sm') {
      margin: auto;
      order: 2;
    }
  }
}

.modal-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding-inline: 2rem;
  position: relative;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1.5rem;

    &::after {
      content: '';
      position: relative;
      width: 100%;
      height: 8rem;
    }
  }
}

.user-list {
  padding: 0;
  margin-top: 1rem;
  overflow-y: auto;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &-title {
    color: var(--parsec-color-light-secondary-grey);
    background: var(--parsec-color-light-secondary-premiere);
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

    .member-checkbox,
    .suggested-checkbox {
      padding-left: 0.75rem;

      @include ms.responsive-breakpoint('sm') {
        padding-left: 0.625rem;
        position: absolute;
        justify-content: center;
      }
    }

    &-item {
      display: flex;
      align-items: center;
      gap: 0.5rem;

      &:last-child .workspace-user-role::after {
        border-bottom: none;
      }
    }

    .workspace-user-role,
    .current-user {
      flex: 1;
    }
  }

  .checkbox-space {
    padding-left: 3rem;

    @include ms.responsive-breakpoint('sm') {
      padding-left: 0.5rem;
    }
  }
}
</style>
