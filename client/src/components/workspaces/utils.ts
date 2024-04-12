// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { writeTextToClipboard } from '@/common/clipboard';
import { workspaceNameValidator } from '@/common/validators';
import { getTextInputFromUser } from '@/components/core';
import {
  ClientRenameWorkspaceErrorTag,
  UserProfile,
  WorkspaceInfo,
  WorkspaceName,
  WorkspaceRole,
  getClientProfile,
  getSystemPath,
  getPathLink as parsecGetPathLink,
  renameWorkspace as parsecRenameWorkspace,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { Translatable } from '@/services/translation';
import WorkspaceContextMenu, { WorkspaceAction } from '@/views/workspaces/WorkspaceContextMenu.vue';
import WorkspaceSharingModal from '@/views/workspaces/WorkspaceSharingModal.vue';
import { modalController, popoverController } from '@ionic/vue';

interface RoleUpdateAuthorization {
  authorized: boolean;
  reason?: Translatable;
}

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
  // Cannot change role of an Owner
  if (userRole === WorkspaceRole.Owner) {
    return { authorized: false, reason: 'workspaceRoles.updateRejectedReasons.ownerImmunity' };
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

export async function workspaceShareClick(workspace: WorkspaceInfo, informationManager: InformationManager): Promise<void> {
  const modal = await modalController.create({
    component: WorkspaceSharingModal,
    componentProps: {
      workspaceId: workspace.id,
      workspaceName: workspace.currentName,
      ownRole: workspace.currentSelfRole,
      informationManager: informationManager,
    },
    cssClass: 'workspace-sharing-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
}

export async function openWorkspaceContextMenu(
  event: Event,
  workspace: WorkspaceInfo,
  informationManager: InformationManager,
): Promise<void> {
  const clientProfile = await getClientProfile();
  const popover = await popoverController.create({
    component: WorkspaceContextMenu,
    cssClass: 'workspace-context-menu',
    event: event,
    translucent: true,
    showBackdrop: false,
    dismissOnSelect: true,
    alignment: 'end',
    componentProps: {
      workspaceName: workspace.currentName,
      clientProfile: clientProfile,
      clientRole: workspace.currentSelfRole,
    },
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  if (data !== undefined) {
    switch (data.action) {
      case WorkspaceAction.Share:
        await workspaceShareClick(workspace, informationManager);
        break;
      case WorkspaceAction.CopyLink:
        await copyLinkToClipboard(workspace, informationManager);
        break;
      case WorkspaceAction.OpenInExplorer:
        await openWorkspace(workspace, informationManager);
        break;
      case WorkspaceAction.Rename:
        await openRenameWorkspaceModal(workspace, informationManager);
        break;
      default:
        console.warn('No WorkspaceAction match found');
    }
  }
}

async function openWorkspace(workspace: WorkspaceInfo, informationManager: InformationManager): Promise<void> {
  const result = await getSystemPath(workspace.handle, '/');

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: { key: 'FoldersPage.open.folderFailed', data: { name: workspace.currentName } },
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
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

async function openRenameWorkspaceModal(workspace: WorkspaceInfo, informationManager: InformationManager): Promise<void> {
  const newWorkspaceName = await getTextInputFromUser({
    title: 'WorkspacesPage.RenameWorkspaceModal.pageTitle',
    trim: true,
    validator: workspaceNameValidator,
    inputLabel: 'WorkspacesPage.RenameWorkspaceModal.label',
    placeholder: 'WorkspacesPage.RenameWorkspaceModal.placeholder',
    okButtonText: 'WorkspacesPage.RenameWorkspaceModal.rename',
    defaultValue: workspace.currentName,
    selectionRange: [0, workspace.currentName.length],
  });

  if (newWorkspaceName) {
    await renameWorkspace(workspace, newWorkspaceName, informationManager);
  }
}

async function copyLinkToClipboard(workspace: WorkspaceInfo, informationManager: InformationManager): Promise<void> {
  const result = await parsecGetPathLink(workspace.handle, '/');

  if (result.ok) {
    if (!(await writeTextToClipboard(result.value))) {
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
