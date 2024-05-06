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
      </ion-header>
      <div
        class="details"
        v-if="currentPage === 1"
      >
        <user-select
          :exclude-users="[sourceUser, currentUser]"
          v-model="targetUser"
        />
      </div>
      <div
        class="details"
        v-show="currentPage === 2 && targetUser"
      >
        <ms-spinner title="UsersPage.assignRoles.processing" />
      </div>
      <div
        class="details"
        v-if="currentPage === 3 && targetUser"
      >
        <ion-label>
          {{ targetUser.humanHandle.label }}
        </ion-label>
        <ion-button
          fill="clear"
          @click="currentPage = 1"
        >
          {{ $msTranslate('UsersPage.assignRoles.update') }}
        </ion-button>

        <div>
          <ion-label v-show="roleUpdates.length === 0">
            {{ $msTranslate({ key: 'UsersPage.assignRoles.noRoles', data: { user: targetUser.humanHandle.label } }) }}
          </ion-label>

          <ion-list class="role-updates">
            <ion-item
              v-for="roleUpdate in roleUpdates"
              :key="roleUpdate.workspace.id"
            >
              {{ roleUpdate.workspace.currentName }}
              {{ $msTranslate(getWorkspaceRoleTranslationKey(roleUpdate.oldRole).label) }} >
              {{ $msTranslate(getWorkspaceRoleTranslationKey(roleUpdate.newRole).label) }}

              <ion-icon
                v-show="roleUpdate.reassigned === false"
                :icon="closeCircle"
              />

              <ion-icon
                v-show="roleUpdate.reassigned === true"
                :icon="checkmarkCircle"
              />
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
            {{ $msTranslate(finished ? 'UsersPage.assignRoles.close' : 'UsersPage.assignRoles.okButton') }}
          </ion-button>
        </ion-buttons>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { UserSelect } from '@/components/users';
import {
  IonPage,
  IonButtons,
  IonLabel,
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
import { close, checkmarkCircle, closeCircle } from 'ionicons/icons';
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
</script>

<style scoped lang="scss"></style>
