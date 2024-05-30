<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <!-- close button -->
    <ion-buttons
      slot="end"
      class="closeBtn-container"
    >
      <ion-button
        slot="icon-only"
        @click="cancel()"
        class="closeBtn"
      >
        <ion-icon
          :icon="close"
          size="large"
          class="closeBtn__icon"
        />
      </ion-button>
    </ion-buttons>

    <div class="modal">
      <ion-header class="modal-header">
        <ion-title class="modal-header__title title-h2">
          {{ $msTranslate('UsersPage.assignRoles.title') }}
        </ion-title>
        <ion-text class="modal-header__text body">
          <i18n-t
            keypath="UsersPage.userContextMenu.subtitleAssignRoles"
            scope="global"
          >
            <template #sourceUser>
              <strong> {{ sourceUser.humanHandle.label }} </strong>
            </template>
          </i18n-t>
        </ion-text>
      </ion-header>
      <div
        v-if="currentPage === 1"
      >
        <user-select
          :exclude-users="[sourceUser, currentUser]"
          v-model="targetUser"
        />
      </div>

      <div
        class="modal-content"
        v-show="currentPage === 2 && targetUser"
      >
        <ms-spinner title="UsersPage.assignRoles.processing" />
      </div>

      <div
        class="modal-content"
        v-if="currentPage === 3 && targetUser"
      >
        <div class="chosen-users">
          <user-avatar-name
            class="chosen-users-source"
            :user-avatar="sourceUser.humanHandle.label"
            :user-name="sourceUser.humanHandle.label"
          />
          <ion-icon
            :icon="arrowForward"
            class="arrow-icon"
          />
          <div class="chosen-users-target">
            <user-avatar-name
              :user-avatar="targetUser.humanHandle.label"
              :user-name="targetUser.humanHandle.label"
            />
            <ion-icon
              :icon="pencil"
              @click="currentPage = 1"
            />
          </div>
        </div>

        <div>
          <ion-text
            class="subtitle-small no-new-roles"
            v-show="roleUpdates.length === 0"
          >
            {{ $msTranslate({ key: 'UsersPage.assignRoles.noRoles', data: { user: targetUser.humanHandle.label } }) }}
          </ion-text>

          <ion-list class="workspace-list">
            <ion-item
              class="ion-no-padding workspace-item-container"
              v-for="roleUpdate in roleUpdates"
              :key="roleUpdate.workspace.id"
            >
              <div class="workspace-item">
                <div class="workspace-item__name">
                  <ion-icon
                    :icon="business"
                  />
                  <ion-text class="title-h5">{{ roleUpdate.workspace.currentName }}</ion-text>
                </div>
                <div class="workspace-item__role">
                  <ion-text class="body">{{ $msTranslate(getWorkspaceRoleTranslationKey(roleUpdate.oldRole).label) }}</ion-text>
                  <ion-icon
                    :icon="arrowForward"
                  />
                  <ion-text class="body">{{ $msTranslate(getWorkspaceRoleTranslationKey(roleUpdate.newRole).label) }}</ion-text>
                  <ion-icon
                    class="error-icon"
                    v-show="roleUpdate.reassigned === false"
                    :icon="closeCircle"
                  />

                  <ion-icon
                    class="success-icon"
                    v-show="roleUpdate.reassigned === true"
                    :icon="checkmarkCircle"
                  />
                </div>
              </div>
            </ion-item>
          </ion-list>
        </div>
      </div>

      <ion-footer class="modal-footer">
        <ion-buttons
          slot="primary"
          class="modal-footer-buttons"
        >
          <ion-button
            fill="clear"
            size="default"
            @click="cancel"
          >
            {{ $msTranslate('MyProfilePage.cancelButton') }}
          </ion-button>
          <ion-button
            fill="solid"
            size="default"
            id="next-button"
            @click="nextStep"
            :disabled="!canGoForward()"
          >
            {{ $msTranslate(getNextButtonText()) }}
          </ion-button>
        </ion-buttons>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { UserSelect } from '@/components/users';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import {
  IonPage,
  IonButtons,
  IonButton,
  IonFooter,
  IonIcon,
  IonHeader,
  IonTitle,
  IonList,
  IonItem,
  modalController,
} from '@ionic/vue';
import { getWorkspacesSharedWith, shareWorkspace, UserInfo, UserProfile, WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { ref, Ref } from 'vue';
import { close, checkmarkCircle, closeCircle, business, arrowForward, pencil } from 'ionicons/icons';
import { MsModalResult, MsSpinner } from 'megashark-lib';
import { compareWorkspaceRoles } from '@/components/workspaces/utils';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { wait } from '@/parsec/internals';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';

interface WorkspaceRoleUpdate {
  workspace: WorkspaceInfo;
  oldRole: WorkspaceRole | null;
  newRole: WorkspaceRole;
  reassigned: boolean | null;
}

const targetUser: Ref<UserInfo | undefined> = ref();
const currentPage: Ref<1 | 2 | 3> = ref(1);
const roleUpdates: Ref<WorkspaceRoleUpdate[]> = ref([]);
const finished = ref(false);

const props = defineProps<{
  sourceUser: UserInfo;
  currentUser: UserInfo;
  informationManager: InformationManager;
}>();

function canGoForward(): boolean {
  return targetUser.value !== undefined;
}

async function findWorkspaces(): Promise<void> {
  if (!targetUser.value) {
    return;
  }

  const start = Date.now();

  const sourceUserWorkspacesResult = await getWorkspacesSharedWith(props.sourceUser.id);
  const targetUserWorkspacesResult = await getWorkspacesSharedWith(targetUser.value?.id);

  if (!sourceUserWorkspacesResult.ok || !targetUserWorkspacesResult.ok) {
    // TODO: Handle errors
    return;
  }

  const sourceWorkspaces = sourceUserWorkspacesResult.value;
  const targetWorkspaces = targetUserWorkspacesResult.value;
  roleUpdates.value = [];

  for (const wsInfo of sourceWorkspaces) {
    // Not Manager or Owner, cannot assign roles anyway
    if (wsInfo.workspace.currentSelfRole !== WorkspaceRole.Owner && wsInfo.workspace.currentSelfRole !== WorkspaceRole.Manager) {
      continue;
    }
    const commonWs = targetWorkspaces.find((wi) => wi.workspace.id === wsInfo.workspace.id);
    let update: WorkspaceRoleUpdate;
    if (commonWs) {
      update = { workspace: wsInfo.workspace, oldRole: commonWs.userRole, newRole: wsInfo.userRole, reassigned: null };
    } else {
      update = { workspace: wsInfo.workspace, oldRole: null, newRole: wsInfo.userRole, reassigned: null };
    }
    // Target is an outsider, can't become Owner or Manager
    if (
      targetUser.value.currentProfile === UserProfile.Outsider &&
      (update.newRole === WorkspaceRole.Manager || update.newRole === WorkspaceRole.Owner)
    ) {
      update.newRole = WorkspaceRole.Contributor;
    }
    // Manager can't promote, we downgrade
    if (
      wsInfo.workspace.currentSelfRole === WorkspaceRole.Manager &&
      (update.newRole === WorkspaceRole.Manager || update.newRole === WorkspaceRole.Owner)
    ) {
      update.newRole = WorkspaceRole.Contributor;
    }
    // Make sure that newRole is superior to oldRole
    if (update.oldRole === null || (update.oldRole && compareWorkspaceRoles(update.newRole, update.oldRole) === 1)) {
      roleUpdates.value.push(update);
    }
  }

  // This can probably take a little while for an important number of users,
  // but can be very quick otherwise.
  // To avoid flashing, we force at least 1s of spinner.
  const elapsed = Date.now() - start;
  if (elapsed < 1000) {
    await wait(1000 - elapsed);
  }
  currentPage.value = 3;
}

async function assignNewRoles(): Promise<void> {
  if (!targetUser.value) {
    return;
  }
  let failures = 0;
  for (const update of roleUpdates.value) {
    const result = await shareWorkspace(update.workspace.id, targetUser.value.id, update.newRole);
    if (!result.ok) {
      failures += 1;
      update.reassigned = false;
    } else {
      update.reassigned = true;
    }
  }
  if (failures === 0) {
    props.informationManager.present(
      new Information({
        message: {
          key: 'UsersPage.assignRoles.assignSuccess',
          data: { sourceUser: props.sourceUser.humanHandle.label, targetUser: targetUser.value.humanHandle.label },
        },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  } else if (failures === roleUpdates.value.length) {
    props.informationManager.present(
      new Information({
        message: 'UsersPage.assignRoles.assignSomeFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    props.informationManager.present(
      new Information({
        message: 'UsersPage.assignRoles.assignAllFailed',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
  finished.value = true;
}

async function nextStep(): Promise<void> {
  if (currentPage.value === 1) {
    currentPage.value = 2;
    await findWorkspaces();
  } else if (currentPage.value === 3) {
    if (finished.value) {
      await modalController.dismiss(null, MsModalResult.Confirm);
    } else {
      await assignNewRoles();
    }
  }
}

async function cancel(): Promise<void> {
  await modalController.dismiss(null, MsModalResult.Cancel);
}

function getNextButtonText(): string {
  switch (currentPage.value) {
    case 1:
      return 'UsersPage.assignRoles.select';
    case 2:
      return 'UsersPage.assignRoles.okButton';
    case 3:
      return 'UsersPage.assignRoles.okButton';
    default:
      return '';
  }
}
</script>

<style scoped lang="scss">
.modal-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.chosen-users {
  display: flex;
  align-items: center;
  gap: 1.5rem;

  .chosen-users-source, .chosen-users-target {
    display: flex;
    align-items: center;
    padding: 0.25rem 0.5rem 0.25rem 0.25rem;
    border-radius: var(--parsec-radius-6);
    flex-grow: 1;
  }

  .chosen-users-target {
    justify-content: space-between;
    cursor: pointer;
    transition: background 150ms ease-in-out;

    ion-icon {
      color: var(--parsec-color-light-secondary-soft-grey);
    }

    &:hover {
      background: var(--parsec-color-light-primary-50);

      ion-icon {
        color: var(--parsec-color-light-primary-700);
      }
    }
  }

  .arrow-icon {
    color: var(--parsec-color-light-secondary-soft-grey);
    width: 1rem;
  }
}

.no-new-roles {
  padding: 0.5rem;
}

.workspace-list {
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.workspace-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-radius: var(--parsec-radius-4);
  border-left: 3px solid var(--parsec-color-light-secondary-medium);
  background: var(--parsec-color-light-secondary-background);

  gap: 0.5rem;
  padding: 0.75rem 1rem;
  width: 100%;

  &-container {
    --inner-padding-end: 0;
    --background: none;
  }

  &__name {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    overflow: hidden;

    ion-icon {
      color: var(--parsec-color-light-secondary-soft-text);
      font-size: 1.125rem;
      flex-shrink: 0;
    }

    ion-text {
      color: var(--parsec-color-light-secondary-text);
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
    }
  }

  &__role {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-hard-text);

    ion-text {
      white-space: nowrap;
      color: var(--parsec-color-light-secondary-hard-grey);

      &:last-child {
        color: var(--parsec-color-light-primary-700);
      }
    }

    ion-icon {
      color: var(--parsec-color-light-secondary-soft-grey);
    }
  }

  .success-icon {
    color: var(--parsec-color-light-success-500);
    display: flex;
    flex-shrink: 0;
  }

  .error-icon {
    color: var(--parsec-color-light-danger-500);
    display: flex;
    flex-shrink: 0
  }
}
</style>
