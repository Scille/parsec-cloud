// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { SmallDisplayCategoryFileContextMenu, SmallDisplayFileContextMenu } from '@/components/small-display';
import { EntryStat, UserProfile, WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { useFileActions } from '@/services/contextMenu/fileActions';
import { isFileEditable } from '@/services/cryptpad';
import { FileAction, FileContextMenu, FolderGlobalAction, FolderGlobalContextMenu } from '@/views/files';
import { modalController, popoverController } from '@ionic/vue';
import { useWindowSize } from 'megashark-lib';

export function useFileContextMenu() {
  const { isLargeDisplay } = useWindowSize();
  const fileActions = useFileActions();

  async function openGlobalContextMenu(event: Event, isReadOnly: boolean, isEmpty: boolean): Promise<FolderGlobalAction | undefined> {
    let action: FolderGlobalAction | undefined;

    if (isLargeDisplay.value) {
      if (isReadOnly) {
        return;
      }

      const popover = await popoverController.create({
        component: FolderGlobalContextMenu,
        cssClass: 'folder-global-context-menu',
        event: event,
        reference: event.type === 'contextmenu' ? 'event' : 'trigger',
        translucent: true,
        showBackdrop: false,
        dismissOnSelect: true,
        alignment: 'start',
        componentProps: {
          isReadOnly: isReadOnly,
        },
      });
      await popover.present();
      action = (await popover.onDidDismiss()).data?.action;
      await popover.dismiss();
    } else {
      const modal = await modalController.create({
        component: SmallDisplayCategoryFileContextMenu,
        cssClass: 'file-context-sheet-modal',
        canDismiss: true,
        breakpoints: [0, 0.5],
        expandToScroll: true,
        initialBreakpoint: 0.5,
        showBackdrop: true,
        componentProps: {
          disableSelect: isEmpty,
        },
      });

      await modal.present();
      action = (await modal.onWillDismiss()).data?.action;
      await modal.dismiss();
    }
    return action;
  }

  async function openEntryContextMenu(
    event: Event,
    entries: EntryStat[],
    isReadOnly: boolean,
    navigateFromAnywhere?: boolean,
  ): Promise<FileAction | undefined> {
    let action: FileAction | undefined;

    if (isLargeDisplay.value) {
      const popover = await popoverController.create({
        component: FileContextMenu,
        cssClass: 'file-context-menu',
        event: event,
        reference: event.type === 'contextmenu' ? 'event' : 'trigger',
        translucent: true,
        showBackdrop: false,
        dismissOnSelect: true,
        alignment: 'start',
        componentProps: {
          isReadOnly: isReadOnly,
          multipleFiles: entries.length > 1,
          isFile: entries.length === 1 && entries[0].isFile(),
          isEditable: entries.length === 1 && isFileEditable(entries[0].name),
          navigateFromAnywhere: navigateFromAnywhere,
        },
      });

      await popover.present();
      action = (await popover.onDidDismiss()).data?.action;
      await popover.dismiss();
    } else {
      const modal = await modalController.create({
        component: SmallDisplayFileContextMenu,
        cssClass: 'file-context-sheet-modal',
        breakpoints: [0, 0.5, 1],
        initialBreakpoint: 0.5,
        expandToScroll: false,
        showBackdrop: true,
        componentProps: {
          isReadOnly: isReadOnly,
          multipleFiles: entries.length > 1,
          isFile: entries.length === 1 && entries[0].isFile(),
          isEditable: entries.length === 1 && isFileEditable(entries[0].name),
          navigateFromAnywhere: navigateFromAnywhere,
        },
      });

      await modal.present();
      action = (await modal.onDidDismiss()).data?.action;
      await modal.dismiss();
    }

    return action;
  }

  async function dispatchContextMenuAction(
    action: FileAction,
    entries: Array<EntryStat>,
    workspaceInfo: WorkspaceInfo,
    userProfile: UserProfile,
  ): Promise<void> {
    switch (action) {
      case FileAction.Preview: {
        await fileActions.openEntry(entries[0], { disallowSystem: true, skipViewers: false, readOnly: true }, workspaceInfo);
        break;
      }
      case FileAction.Rename: {
        await fileActions.renameEntry(entries[0], workspaceInfo);
        break;
      }
      case FileAction.Edit: {
        await fileActions.openEntry(entries[0], { readOnly: workspaceInfo.selfRole === WorkspaceRole.Reader }, workspaceInfo);
        break;
      }
      case FileAction.MoveTo: {
        await fileActions.moveEntriesTo(entries, workspaceInfo);
        break;
      }
      case FileAction.MakeACopy: {
        await fileActions.copyEntries(entries, workspaceInfo);
        break;
      }
      case FileAction.Open: {
        await fileActions.openEntry(entries[0], { skipViewers: true }, workspaceInfo);
        break;
      }
      case FileAction.ShowHistory: {
        await fileActions.showHistory(entries, workspaceInfo);
        break;
      }
      case FileAction.Download: {
        await fileActions.downloadEntries(entries, workspaceInfo, false);
        break;
      }
      case FileAction.DownloadAsArchive: {
        await fileActions.downloadEntries(entries, workspaceInfo, true);
        break;
      }
      case FileAction.ShowDetails: {
        await fileActions.showDetails(entries[0], workspaceInfo, userProfile);
        break;
      }
      case FileAction.CopyLink: {
        await fileActions.copyLink(entries[0], workspaceInfo);
        break;
      }
      case FileAction.Delete: {
        await fileActions.deleteEntries(entries, workspaceInfo);
        break;
      }
      case FileAction.SeeInExplorer: {
        await fileActions.seeInExplorer(entries[0].path, workspaceInfo);
        break;
      }
      case FileAction.ShowEnclosingFolder:
        await fileActions.showEnclosingFolder(entries[0], workspaceInfo);
        break;
      default:
        window.electronAPI.log('warn', `Unknown file action ${action}`);
        break;
    }
  }

  return {
    openGlobalContextMenu,
    openEntryContextMenu,
    dispatchContextMenuAction,
  };
}
