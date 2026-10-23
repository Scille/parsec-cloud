<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <!-- close button -->
    <ion-button
      slot="icon-only"
      @click="cancel()"
      class="closeBtn"
    >
      <ion-icon
        :icon="close"
        class="closeBtn__icon"
      />
    </ion-button>

    <div class="modal">
      <ion-header
        class="modal-header"
        v-if="isLargeDisplay"
      >
        <ion-title class="modal-header__title">
          {{ $msTranslate('UsersPage.assignRoles.title') }}
        </ion-title>
        <ms-rich-text
          class="modal-header__text"
          :text="{ key: 'UsersPage.userContextMenu.subtitleAssignRoles', data: { sourceUser: sourceUser.humanHandle.label } }"
        />
      </ion-header>
      <small-display-modal-header
        v-else
        @close-clicked="cancel()"
        title="UsersPage.assignRoles.title"
      />
      <div
        class="modal-content"
        v-if="currentPage === Steps.SelectUser"
      >
        <ms-rich-text
          class="modal-header__text"
          v-if="isSmallDisplay"
          :text="{ key: 'UsersPage.userContextMenu.subtitleAssignRoles', data: { sourceUser: sourceUser.humanHandle.label } }"
        />
        <user-select
          :exclude-users="[sourceUser]"
          :current-user="currentUser"
          v-model="targetUser"
        />
      </div>

      <div
        class="modal-content"
        v-show="currentPage === Steps.Processing && targetUser"
      >
        <ms-spinner title="UsersPage.assignRoles.processing" />
      </div>

      <div
        class="modal-content"
        v-if="[Steps.Summary, Steps.End].includes(currentPage) && targetUser"
      >
        <div class="chosen-users">
          <div class="chosen-users-source">
            <ion-text class="chosen-users-source__text">
              {{ $msTranslate({ key: 'UsersPage.assignRoles.copyFrom', data: { count: roleUpdates.length } }) }}
            </ion-text>
            <div class="chosen-users-source__info">
              <ion-text class="info-name">{{ sourceUser.humanHandle.label }}</ion-text>
            </div>
          </div>
          <div class="chosen-users-target">
            <ion-text class="chosen-users-target__text">{{ $msTranslate('UsersPage.assignRoles.copyTo') }}</ion-text>
            <div
              class="chosen-users-target__info"
              :class="currentPage === Steps.Summary ? 'chosen-users-target-active' : ''"
              @click="currentPage = Steps.SelectUser"
            >
              <ion-text class="info-name">{{ targetUser.humanHandle.label }}</ion-text>
              <ion-text
                v-show="currentPage !== Steps.End"
                class="info-button"
              >
                {{ $msTranslate('UsersPage.assignRoles.update') }}
              </ion-text>
            </div>
          </div>
        </div>

        <div class="workspace-list-container">
          <ms-report-text
            v-show="roleUpdates.length === 0"
            :theme="MsReportTheme.Info"
          >
            {{ $msTranslate({ key: 'UsersPage.assignRoles.noRoles', data: { user: targetUser.humanHandle.label } }) }}
          </ms-report-text>
          <ms-report-text
            v-show="externalLimitationWarning"
            :theme="MsReportTheme.Info"
          >
            {{ $msTranslate({ key: 'UsersPage.assignRoles.externalLimitation', data: { user: targetUser.humanHandle.label } }) }}
          </ms-report-text>
          <ion-text
            v-show="roleUpdates.length > 0"
            class="workspace-title"
          >
            {{ $msTranslate({ key: 'UsersPage.assignRoles.workspacesList', data: { count: roleUpdates.length } }) }}
          </ion-text>
          <ion-list class="workspace-list">
            <ion-item
              class="ion-no-padding workspace-item-container"
              v-for="roleUpdate in roleUpdates"
              :key="roleUpdate.workspace.id"
            >
              <div class="workspace-item">
                <div class="workspace-item__name">
                  <ion-text>{{ roleUpdate.workspace.name }}</ion-text>
                </div>
                <div class="workspace-item__role">
                  <ion-text class="workspace-item__role-old">
                    {{ $msTranslate(getWorkspaceRoleTranslationKey(roleUpdate.oldRole).label) }}
                  </ion-text>
                  <ion-icon :icon="arrowForward" />
                  <ion-text class="workspace-item__role-new">
                    {{ $msTranslate(getWorkspaceRoleTranslationKey(roleUpdate.newRole).label) }}
                  </ion-text>
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
        <div
          v-show="currentPage !== Steps.SelectUser && currentPage !== Steps.Processing"
          class="modal-footer-buttons"
        >
          <ion-button
            fill="clear"
            size="default"
            @click="cancel"
            v-if="currentPage !== Steps.End"
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
        </div>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import SmallDisplayModalHeader from '@/components/header/SmallDisplayModalHeader.vue';
import { UserSelect } from '@/components/users';
import { compareWorkspaceRoles } from '@/components/workspaces/utils';
import { getWorkspacesSharedWith, shareWorkspace, UserInfo, UserProfile, WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { wait } from '@/parsec/internals';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { IonButton, IonFooter, IonHeader, IonIcon, IonItem, IonList, IonPage, IonText, IonTitle, modalController } from '@ionic/vue';
import { arrowForward, checkmarkCircle, close, closeCircle } from 'ionicons/icons';
import { MsModalResult, MsReportText, MsReportTheme, MsRichText, MsSpinner, useWindowSize } from 'megashark-lib';
import { ref, Ref, watch } from 'vue';

enum Steps {
  SelectUser,
  Processing,
  Summary,
  End,
}

interface WorkspaceRoleUpdate {
  workspace: WorkspaceInfo;
  oldRole: WorkspaceRole | null;
  newRole: WorkspaceRole;
  reassigned: boolean | null;
}

const targetUser: Ref<UserInfo | undefined> = ref();
const currentPage: Ref<Steps> = ref(Steps.SelectUser);
const roleUpdates: Ref<WorkspaceRoleUpdate[]> = ref([]);
const finished = ref(false);
const externalLimitationWarning = ref(false);
const { isLargeDisplay, isSmallDisplay } = useWindowSize();

const props = defineProps<{
  sourceUser: UserInfo;
  currentUser: UserInfo;
  informationManager: InformationManager;
}>();

watch(targetUser, async (newUser) => {
  if (newUser && currentPage.value === Steps.SelectUser) {
    // Automatically move to Processing step
    currentPage.value = Steps.Processing;
    await findWorkspaces();
  }
});

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
    if (wsInfo.workspace.selfRole !== WorkspaceRole.Owner && wsInfo.workspace.selfRole !== WorkspaceRole.Manager) {
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
      externalLimitationWarning.value = true;
      update.newRole = WorkspaceRole.Contributor;
    }
    // Manager can't promote, we downgrade
    if (
      wsInfo.workspace.selfRole === WorkspaceRole.Manager &&
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
  currentPage.value = Steps.Summary;
}

async function assignNewRoles(): Promise<void> {
  externalLimitationWarning.value = false;
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
  switch (currentPage.value) {
    case Steps.Summary:
      await assignNewRoles();
      if (roleUpdates.value.length === 0 || finished.value) {
        currentPage.value = Steps.End;
      }
      break;
    case Steps.End:
      await modalController.dismiss(null, MsModalResult.Confirm);
  }
}

async function cancel(): Promise<void> {
  await modalController.dismiss(null, MsModalResult.Cancel);
}

function getNextButtonText(): string {
  switch (currentPage.value) {
    case Steps.SelectUser:
      return 'UsersPage.assignRoles.select';
    case Steps.Summary:
      return 'UsersPage.assignRoles.okButton';
    case Steps.End:
      return 'UsersPage.assignRoles.close';
    default:
      return '';
  }
}
</script>

<style scoped lang="scss">
.modal {
  @include ms.responsive-breakpoint('sm') {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
}

.modal-header__title {
  @include ms.font('heading-h4');
}

.modal-header__text {
  @include ms.font('body-md-regular');
}

.modal-content {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-none') ms.spacing('padding-4xl');
    height: 100%;
  }
}

.chosen-users {
  display: flex;
  gap: ms.spacing('gap-4xl');

  @include ms.responsive-breakpoint('sm') {
    align-items: flex-start;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
  }

  .chosen-users-source,
  .chosen-users-target {
    display: flex;
    border-radius: ms.radius('md');
    width: 100%;
    flex-direction: column;
    gap: ms.spacing('gap-lg');

    &__text {
      @include ms.font('body-md-medium');
      color: ms.color('text-base-description');
    }

    &__info {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-lg');
      padding: ms.spacing('padding-lg') ms.spacing('padding-3xl') ms.spacing('padding-lg') ms.spacing('padding-2xl');
      justify-content: space-between;
      border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
      border-radius: ms.radius('md');

      @include ms.responsive-breakpoint('sm') {
        width: 100%;
        padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl') ms.spacing('padding-2xl') ms.spacing('padding-2xl');
      }

      .info-name {
        @include ms.font('body-md-medium');

        color: ms.color('text-base-body');
      }

      .info-button {
        @include ms.font('label-md-medium');

        color: ms.color('text-brand-default');
      }
    }
  }

  .chosen-users-target {
    &__info {
      background: ms.color('surface-base-page');
    }

    &-active {
      justify-content: space-between;
      cursor: pointer;
      transition: all 150ms ease-in-out;

      &:hover {
        background: ms.color('surface-brand-default-subtle-hover');
        border: ms.border('thin') solid ms.color('border-brand-default-subtle-hover');
      }
    }
  }
}

.workspace-list-container {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
  overflow: hidden;

  .workspace-title {
    @include ms.font('body-md-medium');

    color: ms.color('text-base-description');
  }

  .workspace-list {
    padding: ms.spacing('padding-none');
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
    max-height: 16rem;
    overflow-y: auto;

    @include ms.responsive-breakpoint('sm') {
      max-height: 100%;
    }
  }

  .workspace-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-radius: ms.radius('sm');
    background: ms.color('surface-base-page');
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');
    width: 100%;

    &-container {
      flex-shrink: 0;
      --inner-padding-end: #{ms.spacing('padding-none')};
      --background: none;
    }

    &__name {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-lg');
      overflow: hidden;

      ion-icon {
        color: ms.color('icon-neutral-default');
        font-size: 1.125rem;
        flex-shrink: 0;
      }

      ion-text {
        @include ms.font('heading-h6');

        color: ms.color('text-base-body');
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
      }
    }

    &__role {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-lg');
      color: ms.color('text-base-description');

      ion-text {
        @include ms.font('body-md-regular');

        white-space: nowrap;
      }

      &-old {
        color: ms.color('text-base-description');
      }
      &-new {
        color: ms.color('text-brand-default');
      }

      ion-icon {
        color: ms.color('icon-neutral-default');
      }
    }

    .success-icon {
      color: ms.color('icon-success-default');
      display: flex;
      flex-shrink: 0;
    }

    .error-icon {
      color: ms.color('icon-error-default');
      display: flex;
      flex-shrink: 0;
    }
  }
}
</style>
