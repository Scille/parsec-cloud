// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { workspaceNameValidator } from '@/common/validators';
import {
  ClientRenameWorkspaceErrorTag,
  UserProfile,
  WorkspaceID,
  WorkspaceInfo,
  WorkspaceName,
  WorkspaceRole,
  getClientProfile,
  getSystemPath,
  getPathLink as parsecGetPathLink,
  renameWorkspace as parsecRenameWorkspace,
} from '@/parsec';
import { Routes, navigateTo } from '@/router';
import { EventDistributor, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { StorageManager } from '@/services/storageManager';
import WorkspaceContextMenu, { WorkspaceAction } from '@/views/workspaces/WorkspaceContextMenu.vue';
import WorkspaceSharingModal from '@/views/workspaces/WorkspaceSharingModal.vue';
import { modalController, popoverController } from '@ionic/vue';
import { Clipboard, DisplayState, Translatable, getTextFromUser } from 'megashark-lib';

export const WORKSPACES_PAGE_DATA_KEY = 'WorkspacesPage';

interface RoleUpdateAuthorization {
  authorized: boolean;
  reason?: Translatable;
}

export interface WorkspacesPageSavedData {
  displayState?: DisplayState;
  favoriteList?: WorkspaceID[];
}

export const WorkspaceDefaultData: Required<WorkspacesPageSavedData> = {
  displayState: DisplayState.Grid,
  favoriteList: [],
};

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

export async function toggleFavorite(
  workspace: WorkspaceInfo,
  favorites: WorkspaceID[],
  eventDistributor: EventDistributor,
  storageManager: StorageManager,
): Promise<void> {
  if (favorites.includes(workspace.id)) {
    favorites.splice(favorites.indexOf(workspace.id), 1);
  } else {
    favorites.push(workspace.id);
  }
  await storageManager.updateComponentData<WorkspacesPageSavedData>(
    WORKSPACES_PAGE_DATA_KEY,
    { favoriteList: favorites },
    WorkspaceDefaultData,
  );
  eventDistributor.dispatchEvent(Events.WorkspaceFavorite);
}

export async function openWorkspaceContextMenu(
  event: Event,
  workspace: WorkspaceInfo,
  favorites: WorkspaceID[],
  eventDistributor: EventDistributor,
  informationManager: InformationManager,
  storageManager: StorageManager,
  fromSidebar = false,
): Promise<void> {
  const clientProfile = await getClientProfile();
  const popover = await popoverController.create({
    component: WorkspaceContextMenu,
    cssClass: fromSidebar ? 'workspace-context-menu workspace-context-menu-sidebar' : 'workspace-context-menu',
    event: event,
    reference: event.type === 'contextmenu' ? 'event' : 'trigger',
    translucent: true,
    showBackdrop: false,
    dismissOnSelect: true,
    componentProps: {
      workspaceName: workspace.currentName,
      clientProfile: clientProfile,
      clientRole: workspace.currentSelfRole,
      isFavorite: favorites.includes(workspace.id),
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
      case WorkspaceAction.Favorite:
        await toggleFavorite(workspace, favorites, eventDistributor, storageManager);
        break;
      case WorkspaceAction.ShowHistory:
        await navigateTo(Routes.History, { query: { documentPath: '/', workspaceHandle: workspace.handle } });
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
  const newWorkspaceName = await getTextFromUser({
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
    if (!(await Clipboard.writeText(result.value))) {
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
