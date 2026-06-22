// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { matchingStringValidator, workspaceNameValidator } from '@/common/validators';
import { RoleUpdateAuthorization } from '@/components/workspaces/types';
import {
  ClientRenameWorkspaceErrorTag,
  UserProfile,
  WorkspaceInfo,
  WorkspaceName,
  WorkspaceRole,
  getClientInfo,
  getClientProfile,
  getSystemPath,
  getWorkspaceInfo,
  isDesktop,
  mountWorkspace,
  archiveWorkspace as parsecArchiveWorkspace,
  getPathLink as parsecGetPathLink,
  renameWorkspace as parsecRenameWorkspace,
  restoreWorkspace as parsecRestoreWorkspace,
  trashWorkspace as parsecTrashWorkspace,
  selfPromoteToWorkspaceOwner,
  unmountWorkspace,
} from '@/parsec';
import { ClientArchiveWorkspaceErrorTag } from '@/plugins/libparsec';
import { Routes, navigateTo } from '@/router';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { WorkspaceAttributes, useWorkspaceAttributes } from '@/services/workspaceAttributes';
import SmallDisplayWorkspaceContextMenu from '@/views/workspaces/SmallDisplayWorkspaceContextMenu.vue';
import { WorkspaceAction } from '@/views/workspaces/types';
import WorkspaceContextMenu from '@/views/workspaces/WorkspaceContextMenu.vue';
import WorkspaceHiddenModal from '@/views/workspaces/WorkspaceHiddenModal.vue';
import WorkspaceSharingModal from '@/views/workspaces/WorkspaceSharingModal.vue';
import { modalController, popoverController } from '@ionic/vue';
import { DateTime } from 'luxon';
import { Answer, Clipboard, I18n, MsModalResult, Translatable, askQuestion, getTextFromUser, useWindowSize } from 'megashark-lib';
import { Ref, inject } from 'vue';

export function canChangeRole(
  clientProfile: UserProfile,
  userProfile: UserProfile,
  clientRole: WorkspaceRole | null,
  userRole: WorkspaceRole | null,
  targetRole: WorkspaceRole | null,
): RoleUpdateAuthorization {
  // Outsiders cannot do anything
  if (clientProfile === UserProfile.Outsider) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.outsiderProfile' };
  }
  // Outsiders cannot be set to Managers or Owners
  if (userProfile === UserProfile.Outsider && (targetRole === WorkspaceRole.Manager || targetRole === WorkspaceRole.Owner)) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.outsiderLimitedRole' };
  }
  // Contributors or Readers cannot update roles
  if (clientRole === null || clientRole === WorkspaceRole.Contributor || clientRole === WorkspaceRole.Reader) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.insufficientRole' };
  }
  // Managers cannot update the role of other Managers
  if (clientRole === WorkspaceRole.Manager && userRole === WorkspaceRole.Manager) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.managerCannotUpdateManagers' };
  }
  // Managers cannot promote to Managers
  if (clientRole === WorkspaceRole.Manager && targetRole === WorkspaceRole.Manager) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.managerCannotPromoteToManager' };
  }
  // Managers cannot promote to Owners
  if (clientRole === WorkspaceRole.Manager && targetRole === WorkspaceRole.Owner) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.managerCannotPromoteToOwner' };
  }

  return { authorized: true };
}

export async function workspaceShareClick(
  workspace: WorkspaceInfo,
  informationManager: InformationManager,
  eventDistributor: EventDistributor,
  isLargeDisplay = true,
): Promise<void> {
  const modal = await modalController.create({
    component: WorkspaceSharingModal,
    showBackdrop: true,
    handle: false,
    breakpoints: isLargeDisplay ? undefined : [0, 1],
    expandToScroll: false,
    initialBreakpoint: isLargeDisplay ? undefined : 1,
    componentProps: {
      workspaceId: workspace.id,
      workspaceName: workspace.name,
      ownRole: workspace.selfRole,
      informationManager: informationManager,
      eventDistributor: eventDistributor,
    },
    cssClass: 'workspace-sharing-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
}

export function useWorkspaceContextMenu(fromSidebar = false) {
  const workspaceAttributes = useWorkspaceAttributes();
  const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
  const storageManager: StorageManager = inject(StorageManagerKey)!;
  const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
  const { isLargeDisplay } = useWindowSize();

  async function openContextMenu(event: Event, workspace: WorkspaceInfo): Promise<void> {
    const clientProfile = await getClientProfile();
    let action: WorkspaceAction | undefined;

    if (isLargeDisplay.value) {
      const popover = await popoverController.create({
        component: WorkspaceContextMenu,
        cssClass: fromSidebar ? 'workspace-context-menu workspace-context-menu-sidebar' : 'workspace-context-menu',
        event: event,
        reference: event.type === 'contextmenu' ? 'event' : 'trigger',
        translucent: true,
        showBackdrop: false,
        dismissOnSelect: true,
        componentProps: {
          workspace: workspace,
          clientProfile: clientProfile,
          isFavorite: workspaceAttributes.isFavorite(workspace.id),
          isHidden: workspaceAttributes.isHidden(workspace.id),
        },
      });

      await popover.present();
      action = (await popover.onDidDismiss()).data?.action;
    } else {
      const modal = await modalController.create({
        component: SmallDisplayWorkspaceContextMenu,
        cssClass: 'workspace-context-sheet-modal',
        showBackdrop: true,
        breakpoints: [0, 0.5, 1],
        expandToScroll: false,
        initialBreakpoint: 0.5,
        componentProps: {
          workspace: workspace,
          clientProfile: clientProfile,
          isFavorite: workspaceAttributes.isFavorite(workspace.id),
          isHidden: workspaceAttributes.isHidden(workspace.id),
        },
      });

      await modal.present();
      action = (await modal.onDidDismiss()).data?.action;
    }

    switch (action) {
      case undefined:
        break;
      case WorkspaceAction.Share:
        await workspaceShareClick(workspace, informationManager.value, eventDistributor.value);
        break;
      case WorkspaceAction.CopyLink:
        await copyLinkToClipboard(workspace, informationManager.value);
        break;
      case WorkspaceAction.OpenInExplorer:
        await seeInExplorer(workspace, informationManager.value, workspaceAttributes, eventDistributor.value);
        break;
      case WorkspaceAction.Rename:
        await openRenameWorkspaceModal(workspace, informationManager.value, isLargeDisplay.value);
        break;
      case WorkspaceAction.Favorite:
        workspaceAttributes.toggleFavorite(workspace.id);
        await workspaceAttributes.save();
        break;
      case WorkspaceAction.ShowHistory:
        await navigateTo(Routes.History, { query: { documentPath: '/', workspaceHandle: workspace.handle } });
        break;
      case WorkspaceAction.Mount:
        await showWorkspace(workspace, workspaceAttributes, informationManager.value, eventDistributor.value);
        break;
      case WorkspaceAction.UnMount:
        if (isDesktop()) {
          const refreshWorkspaces = await getWorkspaceInfo(workspace.handle);
          if (refreshWorkspaces.ok) {
            await unmountWorkspaceConfirmation(
              workspaceAttributes,
              refreshWorkspaces.value,
              informationManager.value,
              eventDistributor.value,
              storageManager,
            );
          }
        } else {
          await hideWorkspace(workspace, workspaceAttributes, informationManager.value, eventDistributor.value);
        }
        break;
      case WorkspaceAction.Archive:
        await archiveWorkspace(workspace, informationManager.value);
        break;
      case WorkspaceAction.Trash:
        await trashWorkspace(workspace, informationManager.value, isLargeDisplay.value);
        break;
      case WorkspaceAction.TakeOwnership:
        await takeOwnershipOfWorkspace(workspace, informationManager.value);
        break;
      default:
        console.warn('No WorkspaceAction match found');
        break;
    }
  }

  async function openArchivedContextMenu(event: Event, workspace: WorkspaceInfo): Promise<void> {
    const clientProfile = await getClientProfile();
    let action: WorkspaceAction | undefined;

    if (isLargeDisplay.value) {
      const popover = await popoverController.create({
        component: WorkspaceContextMenu,
        cssClass: fromSidebar ? 'workspace-context-menu workspace-context-menu-sidebar' : 'workspace-context-menu',
        event: event,
        reference: event.type === 'contextmenu' ? 'event' : 'trigger',
        translucent: true,
        showBackdrop: false,
        dismissOnSelect: true,
        componentProps: {
          workspace: workspace,
          clientProfile: clientProfile,
          isFavorite: false,
          isHidden: false,
        },
      });

      await popover.present();
      action = (await popover.onDidDismiss()).data?.action;
    } else {
      const modal = await modalController.create({
        component: SmallDisplayWorkspaceContextMenu,
        cssClass: 'workspace-context-sheet-modal',
        showBackdrop: true,
        breakpoints: [0, 0.5, 1],
        expandToScroll: false,
        initialBreakpoint: 0.5,
        componentProps: {
          workspace: workspace,
          clientProfile: clientProfile,
          isFavorite: false,
          isHidden: false,
        },
      });

      await modal.present();
      action = (await modal.onDidDismiss()).data?.action;
    }

    switch (action) {
      case undefined:
        break;
      case WorkspaceAction.ShowHistory:
        await navigateTo(Routes.History, { query: { documentPath: '/', workspaceHandle: workspace.handle } });
        break;
      case WorkspaceAction.Restore:
        await restoreWorkspace(workspace, informationManager.value);
        break;
      case WorkspaceAction.Trash:
        await trashWorkspace(workspace, informationManager.value, isLargeDisplay.value);
        break;
      default:
        console.warn('No WorkspaceAction match found');
        break;
    }
  }

  return {
    openContextMenu,
    openArchivedContextMenu,
  };
}

export async function takeOwnershipOfWorkspace(workspace: WorkspaceInfo, informationManager: InformationManager): Promise<void> {
  const answer = await askQuestion(
    'WorkspacesPage.missingOwnership.title',
    { key: 'WorkspacesPage.missingOwnership.subtitle', data: { workspace: workspace.name } },
    {
      yesText: 'WorkspacesPage.missingOwnership.yes',
      noText: 'WorkspacesPage.missingOwnership.no',
    },
  );
  if (answer !== Answer.Yes) {
    return;
  }
  const result = await selfPromoteToWorkspaceOwner(workspace.id);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: {
          key: 'WorkspacesPage.missingOwnership.failed',
        },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: {
          key: 'WorkspacesPage.missingOwnership.success',
          data: {
            workspace: workspace.name,
          },
        },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
}

async function archiveWorkspace(workspace: WorkspaceInfo, informationManager: InformationManager): Promise<void> {
  const answer = await askQuestion(
    'WorkspacesPage.archiveWorkspace.title',
    { key: 'WorkspacesPage.archiveWorkspace.subtitle', data: { workspace: workspace.name } },
    { yesText: 'WorkspacesPage.archiveWorkspace.yes', noText: 'WorkspacesPage.archiveWorkspace.no' },
  );
  if (answer === Answer.No) {
    return;
  }

  const result = await parsecArchiveWorkspace(workspace.id);
  informationManager.present(
    new Information({
      message: {
        key: result.ok ? 'WorkspacesPage.archiveWorkspace.archive.success' : 'WorkspacesPage.archiveWorkspace.archive.fail',
        data: { workspace: workspace.name },
      },
      level: result.ok ? InformationLevel.Success : InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
}

async function restoreWorkspace(workspace: WorkspaceInfo, informationManager: InformationManager): Promise<void> {
  const answer = await askQuestion(
    'WorkspacesPage.restoreWorkspace.title',
    { key: `WorkspacesPage.restoreWorkspace.subtitle${workspace.isTrashed ? 'Trashed' : 'Archived'}`, data: { workspace: workspace.name } },
    { yesText: 'WorkspacesPage.restoreWorkspace.yes', noText: 'WorkspacesPage.restoreWorkspace.no' },
  );
  if (answer === Answer.No) {
    return;
  }

  const result = await parsecRestoreWorkspace(workspace.id);

  informationManager.present(
    new Information({
      message: {
        key: result.ok ? 'WorkspacesPage.restoreWorkspace.restore.success' : 'WorkspacesPage.restoreWorkspace.restore.fail',
        data: { workspace: workspace.name },
      },
      level: result.ok ? InformationLevel.Success : InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
}

async function trashWorkspace(workspace: WorkspaceInfo, informationManager: InformationManager, isLargeDisplay: boolean) {
  let minimumArchivingPeriodInSeconds = 30 * 24 * 3600; // 30 days
  const clientResult = await getClientInfo();

  if (clientResult.ok) {
    minimumArchivingPeriodInSeconds = Number(clientResult.value.serverOrganizationConfig.realmMinimumArchivingPeriodBeforeDeletion);
  }
  // The real deletion date will only be determined once the realm archiving certificate is created.
  // Typically the more the user waits before accepting the confirmation prompt, the more the actual
  // deletion date will differs. In any case we are talking of just a couple of seconds of difference
  // which is no big deal since the archiving period is supposed to be a multiple-days long period.
  const estimatedDeletionDate = DateTime.now().plus({ seconds: minimumArchivingPeriodInSeconds });

  // Warn about the operation
  const answer = await askQuestion(
    'WorkspacesPage.trashWorkspace.question.title',
    {
      key:
        minimumArchivingPeriodInSeconds > 0
          ? 'WorkspacesPage.trashWorkspace.question.subtitleBin'
          : 'WorkspacesPage.trashWorkspace.question.subtitleDelete',
      data: {
        workspace: workspace.name,
        deletionDelay: I18n.translate(formatWorkspaceDeletionDelay(minimumArchivingPeriodInSeconds)),
      },
    },
    {
      yesText: 'WorkspacesPage.trashWorkspace.question.yes',
      noText: 'WorkspacesPage.trashWorkspace.question.no',
    },
  );
  if (answer === Answer.No) {
    return;
  }

  // Ask for confirmation by inputting the workspace name
  const workspaceName = await getTextFromUser(
    {
      title: 'WorkspacesPage.trashWorkspace.title',
      subtitle: {
        key:
          minimumArchivingPeriodInSeconds > 0
            ? 'WorkspacesPage.trashWorkspace.subtitleBin'
            : 'WorkspacesPage.trashWorkspace.subtitleDelete',
        data: { workspace: workspace.name, deletionDate: I18n.translate(I18n.formatDate(estimatedDeletionDate)) },
      },
      trim: true,
      validator: matchingStringValidator(workspace.name),
      inputLabel: {
        key: 'WorkspacesPage.trashWorkspace.label',
        data: { workspace: workspace.name },
      },
      placeholder: I18n.valueAsTranslatable(workspace.name),
      okButtonText:
        minimumArchivingPeriodInSeconds > 0 ? 'WorkspacesPage.trashWorkspace.yesBin' : 'WorkspacesPage.trashWorkspace.yesDelete',
      yesIsDangerous: true,
    },
    isLargeDisplay,
  );

  if (!workspaceName || workspaceName.localeCompare(workspace.name) !== 0) {
    // Shouldn't happen
    return;
  }

  const result = await parsecTrashWorkspace(workspace.id, minimumArchivingPeriodInSeconds);

  let message: Translatable = '';
  let level: InformationLevel;

  if (result.ok) {
    message = {
      key:
        minimumArchivingPeriodInSeconds > 0
          ? 'WorkspacesPage.trashWorkspace.trash.successBin'
          : 'WorkspacesPage.trashWorkspace.trash.successDelete',
      data: { workspace: workspace.name },
    };
    level = InformationLevel.Success;
  } else {
    switch (result.error.tag) {
      case ClientArchiveWorkspaceErrorTag.ArchivingPeriodTooShort:
      //   message = 'WorkspacesPage.trashWorkspace.error.tooShort';
      //   break;
      // TODO: when implementing custom duration, either re-enable or delete locale
      case ClientArchiveWorkspaceErrorTag.Offline:
        message = 'WorkspacesPage.trashWorkspace.error.offline';
        break;
      case ClientArchiveWorkspaceErrorTag.WorkspaceDeleted:
      case ClientArchiveWorkspaceErrorTag.WorkspaceNotFound:
        message = 'WorkspacesPage.trashWorkspace.error.notFound';
        break;
      default:
        message = {
          key:
            minimumArchivingPeriodInSeconds > 0
              ? 'WorkspacesPage.trashWorkspace.error.failBin'
              : 'WorkspacesPage.trashWorkspace.error.failDelete',
          data: { workspace: workspace.name },
        };
    }
    level = InformationLevel.Error;
  }
  informationManager.present(new Information({ message: message, level: level }), PresentationMode.Toast);
}

export async function showWorkspace(
  workspace: WorkspaceInfo,
  workspaceAttributes: WorkspaceAttributes,
  informationManager: InformationManager,
  eventDistributor: EventDistributor,
): Promise<boolean> {
  if (isDesktop()) {
    const result = await mountWorkspace(workspace.handle);

    if (!result.ok) {
      informationManager.present(
        new Information({
          message: {
            key: 'WorkspacesPage.showHideWorkspace.failedShown',
            data: { workspace: workspace.name },
          },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return false;
    } else {
      workspaceAttributes.removeHidden(workspace.id);
      await workspaceAttributes.save();

      await eventDistributor.dispatchEvent(Events.WorkspaceMountpointsSync, {
        workspaceId: workspace.id,
        isMounted: true,
      });
    }
  } else {
    workspaceAttributes.removeHidden(workspace.id);
    await workspaceAttributes.save();
  }

  informationManager.present(
    new Information({
      message: {
        key: isDesktop() ? 'WorkspacesPage.showHideWorkspace.successDesktopShown' : 'WorkspacesPage.showHideWorkspace.successWebShown',
        data: { workspace: workspace.name },
      },
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );

  return true;
}

export async function hideWorkspace(
  workspace: WorkspaceInfo,
  workspaceAttributes: WorkspaceAttributes,
  informationManager: InformationManager,
  eventDistributor: EventDistributor,
): Promise<void> {
  if (isDesktop()) {
    const result = await unmountWorkspace(workspace);

    if (!result.ok) {
      informationManager.present(
        new Information({
          message: {
            key: 'WorkspacesPage.showHideWorkspace.failedHidden',
            data: { workspace: workspace.name },
          },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return;
    } else {
      workspaceAttributes.addHidden(workspace.id);
      await workspaceAttributes.save();

      await eventDistributor.dispatchEvent(Events.WorkspaceMountpointsSync, {
        workspaceId: workspace.id,
        isMounted: false,
      });
    }
  } else {
    workspaceAttributes.addHidden(workspace.id);
    await workspaceAttributes.save();
  }

  informationManager.present(
    new Information({
      message: {
        key: isDesktop() ? 'WorkspacesPage.showHideWorkspace.successDesktopHidden' : 'WorkspacesPage.showHideWorkspace.successWebHidden',
        data: { workspace: workspace.name },
      },
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
}

async function seeInExplorer(
  workspace: WorkspaceInfo,
  informationManager: InformationManager,
  workspaceAttributes: WorkspaceAttributes,
  eventDistributor: EventDistributor,
): Promise<void> {
  if (workspaceAttributes.isHidden(workspace.id)) {
    const answer = await askQuestion(
      'WorkspacesPage.openInExplorerModal.workspace.title',
      'WorkspacesPage.openInExplorerModal.workspace.description',
      {
        yesText: 'WorkspacesPage.openInExplorerModal.actionConfirm',
        noText: 'WorkspacesPage.openInExplorerModal.actionCancel',
      },
    );

    if (answer === Answer.No) {
      return;
    }
    await showWorkspace(workspace, workspaceAttributes, informationManager, eventDistributor);
  }

  const result = await getSystemPath(workspace.handle, '/');
  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: { key: 'FoldersPage.open.folderFailed', data: { name: workspace.name } },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    window.electronAPI.openFile(result.value);
  }
}

async function renameWorkspace(workspace: WorkspaceInfo, newName: WorkspaceName, informationManager: InformationManager): Promise<void> {
  const result = await parsecRenameWorkspace(newName, workspace.id);
  if (result.ok) {
    informationManager.present(
      new Information({
        message: { key: 'WorkspacesPage.RenameWorkspaceModal.success', data: { newName: newName } },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    recentDocumentManager.updateWorkspace(workspace.id, { name: newName });
  } else {
    let message: Translatable = '';
    switch (result.error.tag) {
      case ClientRenameWorkspaceErrorTag.AuthorNotAllowed ||
        ClientRenameWorkspaceErrorTag.InvalidCertificate ||
        ClientRenameWorkspaceErrorTag.InvalidEncryptedRealmName ||
        ClientRenameWorkspaceErrorTag.InvalidKeysBundle:
        message = 'WorkspacesPage.RenameWorkspaceModal.errors.permission';
        break;
      case ClientRenameWorkspaceErrorTag.Offline:
        message = 'WorkspacesPage.RenameWorkspaceModal.errors.offline';
        break;
      default:
        message = { key: 'WorkspacesPage.RenameWorkspaceModal.errors.generic', data: { reason: result.error.tag } };
        console.error(result.error.tag);
        break;
    }
    informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function unmountWorkspaceConfirmation(
  workspaceAttributes: WorkspaceAttributes,
  workspace: WorkspaceInfo,
  informationManager: InformationManager,
  eventDistributor: EventDistributor,
  storageManager: StorageManager,
): Promise<void> {
  const config = await storageManager.retrieveConfig();

  if (config.skipWorkspaceHiddenWarning === true) {
    await hideWorkspace(workspace as WorkspaceInfo, workspaceAttributes, informationManager, eventDistributor);
    return;
  }

  const modal = await modalController.create({
    component: WorkspaceHiddenModal,
    cssClass: 'workspace-hidden-modal',
    componentProps: {
      workspaceName: workspace.name,
    },
  });

  await modal.present();
  const { data, role } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role === MsModalResult.Confirm) {
    if (data?.skipWorkspaceHiddenWarning === true) {
      config.skipWorkspaceHiddenWarning = true;
      await storageManager.storeConfig(config);
    }

    await hideWorkspace(workspace as WorkspaceInfo, workspaceAttributes, informationManager, eventDistributor);
  }
}

async function openRenameWorkspaceModal(
  workspace: WorkspaceInfo,
  informationManager: InformationManager,
  isLargeDisplay: boolean,
): Promise<void> {
  const newWorkspaceName = await getTextFromUser(
    {
      title: 'WorkspacesPage.RenameWorkspaceModal.pageTitle',
      trim: true,
      validator: workspaceNameValidator,
      inputLabel: 'WorkspacesPage.RenameWorkspaceModal.label',
      placeholder: 'WorkspacesPage.RenameWorkspaceModal.placeholder',
      okButtonText: 'WorkspacesPage.RenameWorkspaceModal.rename',
      defaultValue: workspace.name,
      selectionRange: [0, workspace.name.length],
    },
    isLargeDisplay,
  );

  if (newWorkspaceName) {
    await renameWorkspace(workspace, newWorkspaceName, informationManager);
  }
}

async function copyLinkToClipboard(workspace: WorkspaceInfo, informationManager: InformationManager): Promise<void> {
  const result = await parsecGetPathLink(workspace.handle, '/');

  if (result.ok) {
    const [_, invitationAddrAsHttpRedirection] = result.value;
    if (!(await Clipboard.writeText(invitationAddrAsHttpRedirection))) {
      informationManager.present(
        new Information({
          message: 'WorkspacesPage.linkNotCopiedToClipboard',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: 'WorkspacesPage.linkCopiedToClipboard',
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: 'WorkspacesPage.getLinkError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

/**
 *
 * @param role1 A workspace role
 * @param role2 A workspace role
 * @returns -1 if role2 is inferior to role1, 0 if they're equal, 1 if role1 is superior to role2
 */
export function compareWorkspaceRoles(role1: WorkspaceRole, role2: WorkspaceRole): -1 | 0 | 1 {
  const WEIGHTS = new Map<WorkspaceRole, number>([
    [WorkspaceRole.Owner, 4],
    [WorkspaceRole.Manager, 3],
    [WorkspaceRole.Contributor, 2],
    [WorkspaceRole.Reader, 1],
  ]);

  const diff = (WEIGHTS.get(role1) as number) - (WEIGHTS.get(role2) as number);
  if (diff < 0) {
    return -1;
  } else if (diff > 0) {
    return 1;
  }
  return 0;
}

export function formatWorkspaceDeletionDelay(duration: number | undefined): Translatable {
  if (duration === undefined) {
    return 'WorkspacesPage.trashWorkspace.deletionDelay.default';
  } else if (duration < 60) {
    return { key: 'WorkspacesPage.trashWorkspace.deletionDelay.seconds', data: { amount: duration }, count: duration };
  } else if (duration < 3600) {
    // 60 * 60
    duration = ~~(duration / 60);
    return { key: 'WorkspacesPage.trashWorkspace.deletionDelay.minutes', data: { amount: duration }, count: duration };
  } else if (duration < 86400) {
    // 60 * 60 * 24
    duration = ~~(duration / 3600);
    return { key: 'WorkspacesPage.trashWorkspace.deletionDelay.hours', data: { amount: duration }, count: duration };
  } else {
    duration = ~~(duration / 86400);
    return { key: 'WorkspacesPage.trashWorkspace.deletionDelay.days', data: { amount: duration }, count: duration };
  }
}
