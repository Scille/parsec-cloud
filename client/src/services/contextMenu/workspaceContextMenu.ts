// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { WorkspaceInfo, getClientProfile, getWorkspaceInfo, isDesktop } from '@/parsec';
import { Routes, navigateTo } from '@/router';
import { useWorkspaceActions } from '@/services/contextMenu/workspaceActions';
import { useWorkspaceAttributes } from '@/services/workspaceAttributes';
import SmallDisplayWorkspaceContextMenu from '@/views/workspaces/SmallDisplayWorkspaceContextMenu.vue';
import SmallDisplayWorkspaceGlobalContextMenu from '@/views/workspaces/SmallDisplayWorkspaceGlobalContextMenu.vue';
import { WorkspaceAction } from '@/views/workspaces/types';
import WorkspaceContextMenu from '@/views/workspaces/WorkspaceContextMenu.vue';
import WorkspaceGlobalContextMenu from '@/views/workspaces/WorkspaceGlobalContextMenu.vue';
import { modalController, popoverController } from '@ionic/vue';
import { useWindowSize } from 'megashark-lib';

export function useWorkspaceContextMenu(fromSidebar = false) {
  const workspaceAttributes = useWorkspaceAttributes();
  const { isLargeDisplay } = useWindowSize();
  const actions = useWorkspaceActions();

  async function openContextMenu(event: Event, workspace: WorkspaceInfo): Promise<WorkspaceAction | undefined> {
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
        await actions.workspaceShareClick(workspace);
        break;
      case WorkspaceAction.CopyLink:
        await actions.copyLinkToClipboard(workspace);
        break;
      case WorkspaceAction.OpenInExplorer:
        await actions.seeInExplorer(workspace);
        break;
      case WorkspaceAction.Rename:
        await actions.openRenameWorkspaceModal(workspace);
        break;
      case WorkspaceAction.Favorite:
        workspaceAttributes.toggleFavorite(workspace.id);
        await workspaceAttributes.save();
        break;
      case WorkspaceAction.ShowHistory:
        await navigateTo(Routes.History, { query: { documentPath: '/', workspaceHandle: workspace.handle } });
        break;
      case WorkspaceAction.Mount:
        await actions.showWorkspace(workspace);
        break;
      case WorkspaceAction.UnMount:
        if (isDesktop()) {
          const refreshWorkspaces = await getWorkspaceInfo(workspace.handle);
          if (refreshWorkspaces.ok) {
            await actions.unmountWorkspaceWithConfirmation(workspace);
          }
        } else {
          await actions.hideWorkspace(workspace);
        }
        break;
      case WorkspaceAction.Archive:
        await actions.archiveWorkspace(workspace);
        break;
      case WorkspaceAction.Trash:
        await actions.trashWorkspace(workspace);
        break;
      case WorkspaceAction.TakeOwnership:
        await actions.takeOwnershipOfWorkspace(workspace);
        break;
      default:
        console.warn('No WorkspaceAction match found');
        break;
    }
    return action;
  }

  async function openGlobalContextMenu(event: Event): Promise<WorkspaceAction.CreateWorkspace | undefined> {
    let action: WorkspaceAction.CreateWorkspace | undefined;

    if (isLargeDisplay.value) {
      const popover = await popoverController.create({
        component: WorkspaceGlobalContextMenu,
        cssClass: fromSidebar ? 'workspace-global-context-menu workspace-global-context-menu-sidebar' : 'workspace-global-context-menu',
        event: event,
        reference: event.type === 'contextmenu' ? 'event' : 'trigger',
        translucent: true,
        showBackdrop: false,
        dismissOnSelect: true,
      });

      await popover.present();
      action = (await popover.onDidDismiss()).data?.action;
    } else {
      const modal = await modalController.create({
        component: SmallDisplayWorkspaceGlobalContextMenu,
        cssClass: 'workspace-context-sheet-modal',
        showBackdrop: true,
        breakpoints: [0, 0.5, 1],
        expandToScroll: false,
        initialBreakpoint: 0.5,
      });

      await modal.present();
      action = (await modal.onDidDismiss()).data?.action;
    }

    switch (action) {
      case undefined:
        break;
      case WorkspaceAction.CreateWorkspace:
        await actions.openCreateWorkspaceModal();
        break;
      default:
        console.warn('No WorkspaceAction match found');
        break;
    }
    return action;
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
        await actions.restoreWorkspace(workspace);
        break;
      case WorkspaceAction.Trash:
        await actions.trashWorkspace(workspace);
        break;
      default:
        console.warn('No WorkspaceAction match found');
        break;
    }
  }

  return {
    openContextMenu,
    openArchivedContextMenu,
    openGlobalContextMenu,
  };
}
