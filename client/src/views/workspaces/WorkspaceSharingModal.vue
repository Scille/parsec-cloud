<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="WorkspaceSharing.title"
      :close-button="{ visible: true }"
    >
      <div class="modal-toolbar">
        <div class="modal-toolbar__actions">
          <div class="profile-filter">
            <ion-button
              class="profile-filter__item button-medium"
              @click="toggleAllProfiles"
              :class="{
                selected: profileFilters.All,
              }"
              fill="outline"
            >
              {{ $msTranslate('WorkspaceSharing.filter.all') }}
            </ion-button>
            <div class="profile-filter__divider" />
            <ion-button
              class="profile-filter__item button-medium"
              @click="toggleProfile(UserProfile.Admin)"
              :class="{
                selected: profileFilters[UserProfile.Admin],
              }"
              fill="outline"
            >
              {{ $msTranslate('WorkspaceSharing.filter.admins') }}
              <ion-icon
                v-show="profileFilters[UserProfile.Admin]"
                :icon="checkmark"
                class="checkbox-icon"
                slot="end"
              />
            </ion-button>
            <ion-button
              class="profile-filter__item button-medium"
              @click="toggleProfile(UserProfile.Standard)"
              :class="{
                selected: profileFilters[UserProfile.Standard],
              }"
              fill="outline"
            >
              {{ $msTranslate('WorkspaceSharing.filter.standards') }}
              <ion-icon
                v-show="profileFilters[UserProfile.Standard]"
                :icon="checkmark"
                class="checkbox-icon"
                slot="end"
              />
            </ion-button>
            <ion-button
              class="profile-filter__item button-medium"
              @click="toggleProfile(UserProfile.Outsider)"
              :class="{
                selected: profileFilters[UserProfile.Outsider],
              }"
              fill="outline"
            >
              {{ $msTranslate('WorkspaceSharing.filter.externals') }}
              <ion-icon
                v-show="profileFilters[UserProfile.Outsider]"
                :icon="checkmark"
                class="checkbox-icon"
                slot="end"
              />
            </ion-button>
          </div>

          <div class="sharing-modal__workspace">
            <ion-icon
              class="sharing-modal__workspace__icon"
              :icon="business"
            />
            <ion-text class="sharing-modal__workspace__title button-large">{{ workspaceName }}</ion-text>
          </div>
        </div>

        <ms-search-input
          class="modal-toolbar__search"
          v-model="search"
          placeholder="WorkspaceSharing.searchPlaceholder"
        />
      </div>
      <!-- We need at least the owner and someone else (2) -->
      <ms-report-text
        :theme="MsReportTheme.Info"
        id="only-user-warning"
        v-if="countAllSharedUsers < 2"
      >
        <i18n-t
          keypath="WorkspaceSharing.onlyUserWarning"
          scope="global"
        >
          <template #link>
            <ion-text
              button
              class="see-more-button button-medium"
              fill="clear"
              @click="openDocumentation"
            >
              {{ $msTranslate('WorkspaceSharing.seeMore') }}
            </ion-text>
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
            <div class="user-list-title title-h5">
              <ms-checkbox
                class="checkbox"
                id="all-members-checkbox"
                :indeterminate="someMembersSelected && !allMembersSelected"
                :checked="allMembersSelected"
                @change="selectAllMembers()"
                v-show="showCheckboxes && selectableFilteredMembers.length > 0"
              />
              <ion-text
                v-if="countFiltersSharedUsers !== countAllSharedUsers"
                class="count-filter"
              >
                {{
                  $msTranslate({
                    key: 'workspaceRoles.haveAccessFiltered',
                    data: { count: countFiltersSharedUsers, total: countAllSharedUsers },
                  })
                }}
              </ion-text>
              <ion-text v-else>
                {{
                  $msTranslate({
                    key: 'workspaceRoles.haveAccess',
                    data: { count: countAllSharedUsers },
                  })
                }}
              </ion-text>
            </div>
            <workspace-user-role
              v-if="clientInfo && currentUserMatchSearch() && (profileFilters.All || profileFilters[clientInfo.currentProfile])"
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
              :class="{
                'user-list-members-item--selectable': showCheckboxes,
                'user-list-members-item--selected': entry.isSelected,
              }"
              @click="showCheckboxes ? (entry.isSelected = !entry.isSelected) : null"
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
              :class="{
                'user-list-suggestions-item--selectable': showCheckboxes,
                'user-list-suggestions-item--selected': entry.isSelected,
              }"
              @click="showCheckboxes ? (entry.isSelected = !entry.isSelected) : null"
            >
              <ms-checkbox
                class="suggested-checkbox"
                :class="isSmallDisplay ? 'checkbox-mobile' : ''"
                v-show="showCheckboxes"
                v-model="entry.isSelected"
              />
              <workspace-user-role
                class="workspace-user-role"
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

      <div class="modal-footer">
        <ion-text
          id="batch-activate-button"
          @click="onBatchSharingActivate()"
          v-show="batchSharingEnabled"
          :fill="showCheckboxes ? 'fill' : 'clear'"
          class="button-medium"
          :class="{ 'done-button': showCheckboxes }"
        >
          {{ $msTranslate(showCheckboxes ? 'WorkspaceSharing.batchSharing.buttonCancel' : 'WorkspaceSharing.batchSharing.buttonSelect') }}
        </ion-text>
        <ion-text
          class="modal-footer__counter button-medium"
          v-show="showCheckboxes && selectedUsers.length > 0"
        >
          {{ $msTranslate({ key: 'WorkspaceSharing.batchSharing.counter', data: { count: selectedUsers.length } }) }}
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
import { Env } from '@/services/environment';
import { EventDistributor, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { IonButton, IonIcon, IonList, IonPage, IonText } from '@ionic/vue';
import { business, checkmark } from 'ionicons/icons';
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
const profileFilters = ref<Record<UserProfile | 'All', boolean>>({
  All: true,
  [UserProfile.Admin]: false,
  [UserProfile.Standard]: false,
  [UserProfile.Outsider]: false,
});

const filteredUserRoles = computed(() => {
  const searchString = search.value.toLocaleLowerCase();
  return userRoles.value.filter((userRole: UserRole) => {
    return (
      (userRole.user.humanHandle.email.toLocaleLowerCase().includes(searchString) ||
        userRole.user.humanHandle.label.toLocaleLowerCase().includes(searchString)) &&
      (profileFilters.value.All || profileFilters.value[userRole.user.profile])
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
const countFiltersSharedUsers = computed(() => {
  const currentUserCounted =
    currentUserMatchSearch() && (profileFilters.value.All || profileFilters.value[clientInfo.value!.currentProfile]) ? 1 : 0;
  return filteredSharedUserRoles.value.length + currentUserCounted;
});
const countAllSharedUsers = computed(
  () => userRoles.value.filter((userRole) => userRole.role !== null).length + (clientInfo.value ? 1 : 0),
);
const selectableFilteredMembers = computed(() => {
  return filteredSharedUserRoles.value.filter((user) => user.role && canSelectUser(user.user.profile, user.role));
});
const someMembersSelected = computed(() => selectableFilteredMembers.value.some((user) => user.isSelected === true));
const allMembersSelected = computed(() => selectableFilteredMembers.value.every((user) => user.isSelected === true));
const batchSharingEnabled = computed(() => {
  return props.ownRole === WorkspaceRole.Owner || props.ownRole === WorkspaceRole.Manager;
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

function toggleProfile(profile: UserProfile): void {
  profileFilters.value.All = false;
  profileFilters.value[profile] = !profileFilters.value[profile];
  if (
    (profileFilters.value[UserProfile.Admin] && profileFilters.value[UserProfile.Outsider] && profileFilters.value[UserProfile.Standard]) ||
    (!profileFilters.value[UserProfile.Admin] && !profileFilters.value[UserProfile.Outsider] && !profileFilters.value[UserProfile.Standard])
  ) {
    toggleAllProfiles();
  }
}

function toggleAllProfiles(): void {
  profileFilters.value.All = true;
  profileFilters.value[UserProfile.Admin] = false;
  profileFilters.value[UserProfile.Standard] = false;
  profileFilters.value[UserProfile.Outsider] = false;
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

async function openDocumentation(): Promise<void> {
  await Env.Links.openDocumentationUserGuideLink('parsec_workspaces', 'share-a-workspace');
}
</script>

<style scoped lang="scss">
#only-user-warning {
  margin-top: 1rem;
  margin-inline: 2rem;

  @include ms.responsive-breakpoint('sm') {
    margin-inline: 1.5rem;
  }

  .see-more-button {
    font-size: 0.875rem;
    text-decoration: underline;
    color: var(--parsec-color-light-primary-500);
    cursor: pointer;
    transition: background 0.2s;

    &:hover {
      color: var(--parsec-color-light-primary-600);
    }
  }
}

.modal-toolbar {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: space-between;
  gap: 1.25rem;
  margin-top: 1rem;
  overflow: hidden !important;
  flex-shrink: 0;
  padding: 0 2rem 1.5rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1.5rem;
    margin-top: 0.5rem;
  }

  &__search {
    max-height: 2.5rem;
    margin: 0;
    background: var(--parsec-color-light-secondary-background);

    @include ms.responsive-breakpoint('sm') {
      max-width: 100%;
    }
  }

  &__actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-shrink: 0;
    gap: 1rem;
  }

  .profile-filter {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    flex-wrap: wrap;
    position: relative;

    @include ms.responsive-breakpoint('sm') {
      padding-inline: 0;
    }

    &__divider {
      width: 1px;
      height: 1rem;
      background: var(--parsec-color-light-secondary-light);
      margin-inline: 0.25rem;
    }

    &__item {
      --padding-top: 0.5rem;
      --padding-bottom: 0.5rem;
      --padding-start: 0.8rem;
      --padding-end: 0.8rem;
      position: relative;

      &::part(native) {
        border-radius: var(--parsec-radius-24);
        border: 1px solid var(--parsec-color-light-secondary-medium);
        color: var(--parsec-color-light-secondary-grey);
        background: var(--parsec-color-light-secondary-white);
      }

      &:hover::part(native) {
        border: 1px solid var(--parsec-color-light-secondary-medium);
        color: var(--parsec-color-light-secondary-text);
        background: var(--parsec-color-light-secondary-background);
      }

      &.selected::part(native) {
        border: 1px solid var(--parsec-color-light-secondary-light);
        color: var(--parsec-color-light-secondary-text);
        background: var(--parsec-color-light-secondary-medium);
      }

      &.selected:first-child::part(native) {
        background: var(--parsec-color-light-secondary-text);
        color: var(--parsec-color-light-secondary-white);
        border: 1px solid var(--parsec-color-light-secondary-text);
      }

      .checkbox-icon {
        color: var(--parsec-color-light-secondary-text);
        font-size: 0.875rem;
        margin-left: 0.25rem;
      }
    }
  }

  .sharing-modal__workspace {
    display: flex;
    width: fit-content;
    align-items: center;
    gap: 0.25rem;
    color: var(--parsec-color-light-primary-500);
    background: var(--parsec-color-light-primary-50);
    padding: 0.5rem;
    border-radius: var(--parsec-radius-8);

    @include ms.responsive-breakpoint('sm') {
      margin-top: 0.5rem;
    }

    &__title {
      font-weight: 600;
    }

    &__icon {
      color: var(--parsec-color-light-primary-500);
      font-size: 1rem;
    }
  }
}

.modal-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding-inline: 1.5rem;
  position: relative;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1rem;

    &::after {
      content: '';
      position: relative;
      width: 100%;
      height: 0.5rem;
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
      height: 1.875rem;
      color: var(--parsec-color-light-secondary-grey);
      background: var(--parsec-color-light-secondary-white);
      text-transform: uppercase;
      font-weight: 700;
      font-size: 0.825rem;
      padding: 0.625rem 0.5rem;
      display: flex;
      position: sticky;
      align-items: center;
      gap: 0.5rem;
      top: 0;
      z-index: 3;

      .checkbox {
        margin-left: 0.25rem;
      }

      .count-filter {
        display: flex;
        align-items: center;
        gap: 0.125rem;
      }
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

      &-item--selectable {
        cursor: pointer;

        &:hover {
          background: var(--parsec-color-light-secondary-background);
        }
      }

      &-item--selected {
        cursor: pointer;
        background: var(--parsec-color-light-primary-30);

        &:hover {
          background: var(--parsec-color-light-primary-50);
        }
      }

      .workspace-user-role {
        flex: 1;
      }
    }

    .checkbox-space {
      padding-left: 3rem;

      &.current-user {
        @include ms.responsive-breakpoint('sm') {
          padding-left: 2.8125rem;
        }
      }

      @include ms.responsive-breakpoint('sm') {
        padding-left: 0.5rem;
      }
    }
  }
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding: 0.75rem 1.5rem 1rem;
  border-top: 1px solid var(--parsec-color-light-secondary-medium);
  box-shadow: var(--parsec-shadow-strong);
  background: var(--parsec-color-light-secondary-premiere);

  #batch-activate-button {
    font-size: 0.875rem;
    padding: 0.625rem 0.75rem;
    color: var(--parsec-color-light-secondary-text);
    border-radius: var(--parsec-radius-8);
    background: var(--parsec-color-light-secondary-white);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    cursor: pointer;
    transition: background 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
    user-select: none;
    gap: 0.375rem;
    font-weight: 600;
    box-shadow: var(--parsec-shadow-card);
    min-width: 5rem;

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
      border: 1px solid var(--parsec-color-light-secondary-text);
      background: var(--parsec-color-light-secondary-text);
      color: var(--parsec-color-light-secondary-white);
      border: none;
      margin-left: 0;

      &:hover {
        background: var(--parsec-color-light-secondary-contrast);
      }
    }
  }

  &__counter {
    color: var(--parsec-color-light-secondary-grey);
    margin-left: auto;
    text-align: center;
    align-self: center;

    @include ms.responsive-breakpoint('sm') {
      order: 2;
      margin: auto;
    }
  }

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1.5rem;
    position: sticky;
    bottom: 0;
    background: var(--parsec-color-light-secondary-white);
    border-radius: 0 0 var(--parsec-radius-8) var(--parsec-radius-8);
    box-shadow: var(--parsec-shadow-strong);
    z-index: 2;
  }
}
</style>
