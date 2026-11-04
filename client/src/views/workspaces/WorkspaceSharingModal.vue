<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
              size="small"
              class="profile-filter__item"
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
              size="small"
              class="profile-filter__item"
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
                class="checkbox-icon button-icon-right"
                slot="end"
              />
            </ion-button>
            <ion-button
              size="small"
              class="profile-filter__item"
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
                class="checkbox-icon button-icon-right"
                slot="end"
              />
            </ion-button>
            <ion-button
              size="small"
              class="profile-filter__item"
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
                class="checkbox-icon button-icon-right"
                slot="end"
              />
            </ion-button>
          </div>

          <div class="sharing-modal__workspace">
            <ion-icon
              class="sharing-modal__workspace__icon"
              :icon="business"
            />
            <ion-text class="sharing-modal__workspace__title">{{ workspaceName }}</ion-text>
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
              class="see-more-button"
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
            class="no-match-result"
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
            <div class="user-list-title">
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
            <ion-text class="user-list-title">
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
        <ion-button
          id="batch-activate-button"
          class="color-secondary"
          @click="onBatchSharingActivate()"
          v-show="batchSharingEnabled"
          :fill="showCheckboxes ? 'clear' : 'outline'"
          :class="{ 'done-button': showCheckboxes }"
        >
          {{ $msTranslate(showCheckboxes ? 'WorkspaceSharing.batchSharing.buttonCancel' : 'WorkspaceSharing.batchSharing.buttonSelect') }}
        </ion-button>
        <ion-text
          class="modal-footer__counter"
          v-show="showCheckboxes && selectedUsers.length > 0 && isLargeDisplay"
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
const { isSmallDisplay, isLargeDisplay } = useWindowSize();

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
    @include ms.font('label-md-medium');

    text-decoration: underline;
    color: ms.color('text-brand-default');
    cursor: pointer;
    transition: background 0.2s;

    &:hover {
      color: ms.color('text-brand-default-hover');
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
  padding: ms.spacing('padding-none') ms.spacing('padding-4xl') ms.spacing('padding-4xl');
  border-bottom: ms.border('thin') solid ms.color('border-base-default');

  @include ms.responsive-breakpoint('sm') {
    padding-inline: ms.spacing('padding-4xl');
    margin-top: 0.5rem;
  }

  &__search {
    max-height: 2.5rem;
    margin: 0;
    background: ms.color('surface-base-page');

    @include ms.responsive-breakpoint('sm') {
      max-width: 100%;
    }
  }

  &__actions {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-shrink: 0;
    gap: ms.spacing('gap-3xl');
  }

  .profile-filter {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-md');
    flex-wrap: wrap;
    position: relative;

    @include ms.responsive-breakpoint('sm') {
      padding-inline: ms.spacing('padding-none');
    }

    &__divider {
      width: 1px;
      height: 1rem;
      background: ms.color('surface-disabled-default');
      margin-inline: 0.25rem;
    }

    &__item {
      --padding-top: #{ms.spacing('padding-lg')};
      --padding-bottom: #{ms.spacing('padding-lg')};
      --padding-start: 0.8rem;
      --padding-end: 0.8rem;
      position: relative;
      --background: #{ms.color('surface-base-default')};
      --background-hover: #{ms.color('surface-neutral-default-subtle-hover')};
      --background-activated: #{ms.color('surface-neutral-default-subtle-pressed')};
      --background-focused: #{ms.color('surface-neutral-default-subtle-pressed')};
      --background-focused-opacity: #{ms.opacity('10')};
      --background-activated-opacity: #{ms.opacity('10')};

      &::part(native) {
        border-radius: ms.radius('5xl');
        border: ms.border('thin') solid ms.color('border-base-default');
        color: ms.color('text-base-description');
      }

      &:hover::part(native) {
        border: ms.border('thin') solid ms.color('border-base-default');
        color: ms.color('text-neutral-default-hover');
      }

      &.selected::part(native) {
        border: ms.border('thin') solid ms.color('border-base-default-hover');
        color: ms.color('text-base-body');
      }

      &.selected:first-child::part(native) {
        --background: #{ms.color('surface-neutral-default')};
        --background-hover: #{ms.color('surface-neutral-default')};
        --background-activated: #{ms.color('surface-neutral-default')};
        --background-focused: #{ms.color('surface-neutral-default')};
        color: ms.color('text-on-color-heading');
        border: ms.border('thin') solid ms.color('border-neutral-default');
      }

      .checkbox-icon {
        color: ms.color('text-base-body');
        font-size: 0.875rem;
      }
    }
  }

  .sharing-modal__workspace {
    display: flex;
    width: fit-content;
    max-width: 35%;
    align-items: center;
    gap: ms.spacing('gap-sm');
    color: ms.color('text-brand-default');
    background: ms.color('surface-brand-default-subtle');
    padding: ms.spacing('padding-lg');
    border-radius: ms.radius('lg');

    @include ms.responsive-breakpoint('sm') {
      margin-top: 0.5rem;
    }

    &__title {
      @include ms.font('label-lg-medium');

      text-overflow: ellipsis;
      overflow: hidden;
      white-space: nowrap;
    }

    &__icon {
      color: ms.color('icon-brand-default');
      font-size: 1rem;
      min-width: 1rem;
    }
  }
}

.modal-container {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding-inline: ms.spacing('padding-4xl');
  position: relative;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: ms.spacing('padding-3xl');

    &::after {
      content: '';
      position: relative;
      width: 100%;
      height: 0.5rem;
    }
  }

  .user-list {
    .no-match-result {
      @include ms.font('body-md-regular');
    }

    padding: ms.spacing('padding-none');
    margin-top: 1rem;
    overflow-y: auto;
    height: 100%;
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');

    &-title {
      height: 1.875rem;
      color: ms.color('text-base-description');
      background: ms.color('surface-base-default');
      @include ms.font('heading-h6');

      text-transform: uppercase;
      padding: ms.spacing('padding-xl') ms.spacing('padding-lg');
      display: flex;
      position: sticky;
      align-items: center;
      gap: ms.spacing('gap-lg');
      top: 0;
      z-index: 3;

      .checkbox {
        margin-left: 0.25rem;
      }

      .count-filter {
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-xs');
      }
    }

    &-members,
    &-suggestions {
      display: flex;
      flex-direction: column;

      .member-checkbox,
      .suggested-checkbox {
        padding-left: ms.spacing('padding-2xl');

        @include ms.responsive-breakpoint('sm') {
          padding-left: ms.spacing('padding-xl');
          justify-content: center;
        }
      }

      &-item {
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-lg');

        &:last-child .workspace-user-role::after {
          border-bottom: none;
        }
      }

      &-item--selectable {
        cursor: pointer;

        &:hover {
          background: ms.color('surface-base-page');
        }
      }

      &-item--selected {
        cursor: pointer;
        background: ms.color('surface-brand-default-subtle');

        &:hover {
          background: ms.color('surface-brand-default-subtle-hover');
        }
      }

      .workspace-user-role {
        flex: 1;
      }
    }

    .checkbox-space {
      padding-left: ms.spacing('padding-7xl');

      &.current-user {
        @include ms.responsive-breakpoint('sm') {
          padding-left: 2.8125rem;
        }
      }

      @include ms.responsive-breakpoint('sm') {
        padding-left: ms.spacing('padding-lg');
      }
    }
  }
}

.modal-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: ms.spacing('gap-3xl');
  padding: ms.spacing('padding-2xl') ms.spacing('padding-4xl') ms.spacing('padding-3xl');
  border-top: ms.border('thin') solid ms.color('border-base-default');
  box-shadow: ms.shadow('strong');
  background: ms.color('surface-base-page');

  #batch-activate-button {
    @include ms.responsive-breakpoint('sm') {
      order: 3;
    }

    .checkbox-icon {
      color: ms.color('text-base-body');
      font-size: 1rem;
    }

    &:active {
      box-shadow: none;
    }

    &.done-button {
      margin-left: 0;
    }
  }

  &__counter {
    @include ms.font('label-md-medium');

    color: ms.color('text-base-description');
    margin-left: auto;
    text-align: center;
    align-self: center;

    @include ms.responsive-breakpoint('sm') {
      order: 2;
      margin: auto;
    }
  }

  @include ms.responsive-breakpoint('sm') {
    padding-inline: ms.spacing('padding-4xl');
    position: sticky;
    bottom: 0;
    border-radius: 0 0 ms.radius('lg') ms.radius('lg');
    box-shadow: ms.shadow('strong');
    z-index: 2;
  }
}
</style>
