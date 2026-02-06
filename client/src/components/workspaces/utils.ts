// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { workspaceNameValidator } from '@/common/validators';
import {
  ClientRenameWorkspaceErrorTag,
  StartedWorkspaceInfo,
  UserProfile,
  WorkspaceID,
  WorkspaceInfo,
  WorkspaceName,
  WorkspaceRole,
  getClientProfile,
  getSystemPath,
  getWorkspaceInfo,
  isDesktop,
  mountWorkspace,
  getPathLink as parsecGetPathLink,
  renameWorkspace as parsecRenameWorkspace,
  unmountWorkspace,
} from '@/parsec';
import { Routes, navigateTo } from '@/router';
import { EventDistributor, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { StorageManager } from '@/services/storageManager';
import { WorkspaceAttributes } from '@/services/workspaceAttributes';
import SmallDisplayWorkspaceContextMenu from '@/views/workspaces/SmallDisplayWorkspaceContextMenu.vue';
import { WorkspaceAction } from '@/views/workspaces/types';
import WorkspaceContextMenu from '@/views/workspaces/WorkspaceContextMenu.vue';
import WorkspaceHiddenModal from '@/views/workspaces/WorkspaceHiddenModal.vue';
import WorkspaceSharingModal from '@/views/workspaces/WorkspaceSharingModal.vue';
import { modalController, popoverController } from '@ionic/vue';
import { Answer, Clipboard, DisplayState, MsModalResult, Translatable, askQuestion, getTextFromUser } from 'megashark-lib';

export const WORKSPACES_PAGE_DATA_KEY = 'WorkspacesPage';

interface RoleUpdateAuthorization {
  authorized: boolean;
  reason?: Translatable;
}

export interface WorkspacesPageSavedData {
  displayState?: DisplayState;
  favoriteList?: WorkspaceID[];
  hiddenList?: WorkspaceID[];
}

export interface WorkspacesPageFilters {
  owner: boolean;
  manager: boolean;
  contributor: boolean;
  reader: boolean;
}

export const WorkspaceDefaultData: Required<WorkspacesPageSavedData> = {
  displayState: DisplayState.Grid,
  favoriteList: [],
  hiddenList: [],
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
      workspaceName: workspace.currentName,
      ownRole: workspace.currentSelfRole,
      informationManager: informationManager,
      eventDistributor: eventDistributor,
    },
    cssClass: 'workspace-sharing-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
}

export async function openWorkspaceContextMenu(
  event: Event,
  workspace: WorkspaceInfo,
  workspaceAttributes: WorkspaceAttributes,
  eventDistributor: EventDistributor,
  informationManager: InformationManager,
  storageManager: StorageManager,
  fromSidebar = false,
  isLargeDisplay = true,
): Promise<void> {
  const clientProfile = await getClientProfile();
  let data: { action: WorkspaceAction } | undefined;

  if (isLargeDisplay) {
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
        isFavorite: workspaceAttributes.isFavorite(workspace.id),
        isHidden: workspaceAttributes.isHidden(workspace.id),
      },
    });

    await popover.present();
    data = (await popover.onDidDismiss()).data;
  } else {
    const modal = await modalController.create({
      component: SmallDisplayWorkspaceContextMenu,
      cssClass: 'workspace-context-sheet-modal',
      showBackdrop: true,
      breakpoints: [0, 0.5, 1],
      expandToScroll: false,
      initialBreakpoint: 0.5,
      componentProps: {
        workspaceName: workspace.currentName,
        clientProfile: clientProfile,
        clientRole: workspace.currentSelfRole,
        isFavorite: workspaceAttributes.isFavorite(workspace.id),
        isHidden: workspaceAttributes.isHidden(workspace.id),
      },
    });

    await modal.present();
    data = (await modal.onDidDismiss()).data;
  }

  if (data !== undefined) {
    switch (data.action) {
      case WorkspaceAction.Share:
        await workspaceShareClick(workspace, informationManager, eventDistributor);
        break;
      case WorkspaceAction.CopyLink:
        await copyLinkToClipboard(workspace, informationManager);
        break;
      case WorkspaceAction.OpenInExplorer:
        await seeInExplorer(workspace, informationManager, workspaceAttributes, eventDistributor);
        break;
      case WorkspaceAction.Rename:
        await openRenameWorkspaceModal(workspace, informationManager, isLargeDisplay);
        break;
      case WorkspaceAction.Favorite:
        workspaceAttributes.toggleFavorite(workspace.id);
        await workspaceAttributes.save();
        break;
      case WorkspaceAction.ShowHistory:
        await navigateTo(Routes.History, { query: { documentPath: '/', workspaceHandle: workspace.handle } });
        break;
      case WorkspaceAction.Mount:
        await showWorkspace(workspace, workspaceAttributes, informationManager, eventDistributor);
        break;
      case WorkspaceAction.UnMount:
        if (isDesktop()) {
          const refreshWorkspaces = await getWorkspaceInfo(workspace.handle);
          if (refreshWorkspaces.ok) {
            await unmountWorkspaceConfirmation(
              workspaceAttributes,
              refreshWorkspaces.value,
              informationManager,
              eventDistributor,
              storageManager,
            );
          }
        } else {
          await hideWorkspace(workspace, workspaceAttributes, informationManager, eventDistributor);
        }
        break;
      default:
        console.warn('No WorkspaceAction match found');
    }
  }
}

export async function showWorkspace(
  workspace: WorkspaceInfo | StartedWorkspaceInfo,
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
            data: { workspace: workspace.currentName },
          },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      window.electronAPI.log('error', `Error while unmounting workspace: ${result.error}`);
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
        data: { workspace: workspace.currentName },
      },
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );

  return true;
}

export async function hideWorkspace(
  workspace: WorkspaceInfo | StartedWorkspaceInfo,
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
            data: { workspace: workspace.currentName },
          },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      window.electronAPI.log('error', `Error while unmounting workspace: ${result.error}`);
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
        data: { workspace: workspace.currentName },
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
        message: { key: 'FoldersPage.open.folderFailed', data: { name: workspace.currentName } },
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
    recentDocumentManager.updateWorkspace(workspace.id, { currentName: newName });
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
  workspace: WorkspaceInfo | StartedWorkspaceInfo,
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
      workspaceName: workspace.currentName,
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
      defaultValue: workspace.currentName,
      selectionRange: [0, workspace.currentName.length],
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
