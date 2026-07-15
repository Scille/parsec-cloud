// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { entryNameValidator } from '@/common/validators';
import { copyPathLinkToClipboard, EntryModel, selectFolder } from '@/components/files';
import { showWorkspace } from '@/components/workspaces/utils';
import {
  deleteFile,
  deleteFolder,
  EntryName,
  entryStat,
  EntryStatFile,
  FsPath,
  isDesktop,
  Path,
  rename,
  UserProfile,
  WorkspaceInfo,
  WorkspaceMoveEntryErrorTag,
} from '@/parsec';
import { navigateTo, Routes } from '@/router';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { DuplicatePolicy, FileOperationManager, FileOperationManagerKey } from '@/services/fileOperation';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import usePathOpener, { OpenPathOptions } from '@/services/pathOpener';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { useWorkspaceAttributes } from '@/services/workspaceAttributes';
import { downloadFiles, FileDetailsModal, getDuplicatePolicy, openDownloadConfirmationModal } from '@/views/files';
import { modalController } from '@ionic/vue';
import { Answer, askQuestion, getTextFromUser, MsModalResult, Translatable, useWindowSize } from 'megashark-lib';
import { inject, Ref } from 'vue';

export function useFileActions() {
  const { isLargeDisplay } = useWindowSize();
  const workspaceAttributes = useWorkspaceAttributes();
  const pathOpener = usePathOpener();
  const fileOperationManager: Ref<FileOperationManager> = inject(FileOperationManagerKey)!;
  const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
  const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
  const storageManager: StorageManager = inject(StorageManagerKey)!;

  async function deleteEntries(entries: EntryModel[], workspaceInfo: WorkspaceInfo): Promise<void> {
    if (entries.length === 0) {
      return;
    }
    if (entries.length === 1) {
      const entry = entries[0];
      const title = entry.isFile() ? 'FoldersPage.deleteOneFileQuestionTitle' : 'FoldersPage.deleteOneFolderQuestionTitle';
      const subtitle = entry.isFile()
        ? { key: 'FoldersPage.deleteOneFileQuestionSubtitle', data: { name: entry.name } }
        : { key: 'FoldersPage.deleteOneFolderQuestionSubtitle', data: { name: entry.name } };
      const answer = await askQuestion(title, subtitle, {
        yesIsDangerous: true,
        yesText: entry.isFile() ? 'FoldersPage.deleteOneFileYes' : 'FoldersPage.deleteOneFolderYes',
        noText: entry.isFile() ? 'FoldersPage.deleteOneFileNo' : 'FoldersPage.deleteOneFolderNo',
      });

      if (answer === Answer.No) {
        return;
      }
      const result = entry.isFile()
        ? await deleteFile(workspaceInfo.handle, entry.path)
        : await deleteFolder(workspaceInfo.handle, entry.path);
      if (!result.ok) {
        informationManager.value.present(
          new Information({
            message: { key: 'FoldersPage.errors.deleteFailed', data: { name: entry.name } },
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
      } else {
        await eventDistributor.value.dispatchEvent(Events.EntryDeleted, {
          workspaceHandle: workspaceInfo.handle,
          entryId: entry.id,
          path: entry.path,
        });
      }
    } else {
      const answer = await askQuestion(
        'FoldersPage.deleteMultipleQuestionTitle',
        {
          key: 'FoldersPage.deleteMultipleQuestionSubtitle',
          data: {
            count: entries.length,
          },
        },
        {
          yesIsDangerous: true,
          yesText: { key: 'FoldersPage.deleteMultipleYes', data: { count: entries.length } },
          noText: { key: 'FoldersPage.deleteMultipleNo', data: { count: entries.length } },
        },
      );
      if (answer === Answer.No) {
        return;
      }
      let errorsEncountered = 0;
      for (const entry of entries) {
        const result = entry.isFile()
          ? await deleteFile(workspaceInfo.handle, entry.path)
          : await deleteFolder(workspaceInfo.handle, entry.path);
        if (!result.ok) {
          errorsEncountered += 1;
        } else {
          await eventDistributor.value.dispatchEvent(Events.EntryDeleted, {
            workspaceHandle: workspaceInfo.handle,
            entryId: entry.id,
            path: entry.path,
          });
        }
      }
      if (errorsEncountered > 0) {
        informationManager.value.present(
          new Information({
            message:
              errorsEncountered === entries.length
                ? 'FoldersPage.errors.deleteMultipleAllFailed'
                : 'FoldersPage.errors.deleteMultipleSomeFailed',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
      }
    }
  }

  async function renameEntry(entry: EntryModel, workspaceInfo: WorkspaceInfo): Promise<void> {
    const ext = Path.getFileExtension(entry.name);
    const newName = await getTextFromUser(
      {
        title: entry.isFile() ? 'FoldersPage.RenameModal.fileTitle' : 'FoldersPage.RenameModal.folderTitle',
        trim: true,
        validator: entryNameValidator(entry.isFile(), {
          checkExists: {
            workspaceHandle: workspaceInfo.handle,
            path: entry.parent,
          },
          checkConfined: true,
        }),
        inputLabel: entry.isFile() ? 'FoldersPage.RenameModal.fileLabel' : 'FoldersPage.RenameModal.folderLabel',
        placeholder: entry.isFile() ? 'FoldersPage.RenameModal.filePlaceholder' : 'FoldersPage.RenameModal.folderPlaceholder',
        okButtonText: 'FoldersPage.RenameModal.rename',
        defaultValue: entry.name,
        selectionRange: [0, entry.name.length - (ext.length > 0 ? ext.length + 1 : 0)],
      },
      isLargeDisplay.value,
    );

    if (!newName) {
      return;
    }
    const result = await rename(workspaceInfo.handle, entry.path, newName);
    if (!result.ok) {
      let message: Translatable = '';
      switch (result.error.tag) {
        case WorkspaceMoveEntryErrorTag.DestinationExists:
          message = 'FoldersPage.errors.renameFailedAlreadyExists';
          break;
        default:
          message = { key: 'FoldersPage.errors.renameFailed', data: { name: entry.name } };
      }
      informationManager.value.present(
        new Information({
          message: message,
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      await eventDistributor.value.dispatchEvent(Events.EntryRenamed, {
        workspaceHandle: workspaceInfo.handle,
        entryId: entry.id,
        oldPath: entry.path,
        newPath: result.value,
        oldName: entry.name,
        newName: newName,
      });

      entry.name = newName;
      entry.path = result.value;
    }
  }

  async function copyLink(entry: EntryModel, workspaceInfo: WorkspaceInfo): Promise<void> {
    copyPathLinkToClipboard(entry.path, workspaceInfo.handle, informationManager.value);
  }

  async function moveEntriesTo(entries: EntryModel[], workspaceInfo: WorkspaceInfo, currentPath: string): Promise<void> {
    if (entries.length === 0) {
      return;
    }
    const excludePaths: Array<FsPath> = [];
    for (const entry of entries) {
      if (!entry.isFile()) {
        excludePaths.push(entry.path);
      }
    }
    const folder = await selectFolder({
      title: { key: 'FoldersPage.moveSelectFolderTitle', data: { count: entries.length }, count: entries.length },
      startingPath: currentPath,
      workspaceHandle: workspaceInfo.handle,
      excludePaths: excludePaths,
    });
    if (!folder) {
      return;
    }

    const existing: Array<EntryModel> = [];

    for (const entry of entries) {
      const destPath = await Path.join(folder, entry.name);
      const result = await entryStat(workspaceInfo.handle, destPath);
      if (result.ok) {
        existing.push(entry);
      }
    }
    let dupPolicy: DuplicatePolicy | undefined = undefined;
    if (existing.length > 0) {
      dupPolicy = await getDuplicatePolicy(existing);
      if (!dupPolicy) {
        return;
      }

      if (dupPolicy === DuplicatePolicy.Ignore) {
        entries = entries.filter((entry1) => existing.find((entry2) => entry1.path === entry2.path) === undefined);
      }
    }

    if (entries.length === 0) {
      informationManager.value.present(
        new Information({
          message: 'FoldersPage.noFilesToMove',
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
      return;
    }

    await fileOperationManager.value.move(workspaceInfo.handle, entries, folder, dupPolicy);
  }

  async function showDetails(entry: EntryModel, workspaceInfo: WorkspaceInfo, userProfile: UserProfile): Promise<void> {
    const modal = await modalController.create({
      component: FileDetailsModal,
      cssClass: 'file-details-modal',
      componentProps: {
        entry: entry,
        ownProfile: userProfile,
        workspaceHandle: workspaceInfo.handle,
      },
    });
    await modal.present();
    await modal.onWillDismiss();
  }

  async function copyEntries(entries: EntryModel[], workspaceInfo: WorkspaceInfo, currentPath: string): Promise<void> {
    if (entries.length === 0) {
      return;
    }

    const excludePaths: Array<FsPath> = [];
    for (const entry of entries) {
      if (!entry.isFile()) {
        excludePaths.push(entry.path);
      }
    }
    const folder = await selectFolder({
      title: { key: 'FoldersPage.copySelectFolderTitle', data: { count: entries.length }, count: entries.length },
      startingPath: currentPath,
      workspaceHandle: workspaceInfo.handle,
      excludePaths: excludePaths,
      allowStartingPath: true,
      okButtonLabel: 'FoldersPage.copyHere',
    });
    if (!folder) {
      return;
    }

    const existing: Array<EntryModel> = [];

    for (const entry of entries) {
      const destPath = await Path.join(folder, entry.name);
      const result = await entryStat(workspaceInfo.handle, destPath);
      if (result.ok) {
        existing.push(entry);
      }
    }
    let dupPolicy: DuplicatePolicy | undefined = undefined;
    if (existing.length > 0) {
      dupPolicy = await getDuplicatePolicy(existing);
      if (!dupPolicy) {
        return;
      }

      if (dupPolicy === DuplicatePolicy.Ignore) {
        entries = entries.filter((entry1) => existing.find((entry2) => entry1.path === entry2.path) === undefined);
      }
    }
    if (entries.length === 0) {
      informationManager.value.present(
        new Information({
          message: 'FoldersPage.noFilesToCopy',
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
      return;
    }

    await fileOperationManager.value.copy(workspaceInfo.handle, entries, folder, dupPolicy);
  }

  async function downloadEntries(
    entries: EntryModel[],
    workspaceInfo: WorkspaceInfo,
    currentFolder: string,
    currentPath: string | undefined,
    asArchive?: boolean,
  ): Promise<void> {
    if (entries.length < 1) {
      return;
    }

    const result = await openDownloadConfirmationModal(storageManager, entries.length === 1 && entries.at(0)?.isFile() ? false : true);
    if (result === MsModalResult.Cancel) {
      return;
    }

    function _getArchiveName(): EntryName {
      if (!workspaceInfo) {
        return 'archive.zip';
      }
      if (entries.length === 1) {
        return `${entries[0].name}.zip`;
      }
      if (currentFolder === '/') {
        return `${workspaceInfo.name}.zip`;
      }
      return `${workspaceInfo.name}_${currentFolder}.zip`;
    }

    let archiveOpts: any = undefined;
    if (asArchive) {
      archiveOpts = {
        archiveName: _getArchiveName(),
        relativePath: currentPath ?? '/',
      };
    }

    await downloadFiles({
      entries: entries,
      workspaceHandle: workspaceInfo.handle,
      workspaceId: workspaceInfo.id,
      informationManager: informationManager.value,
      fileOperationManager: fileOperationManager.value,
      asArchive: archiveOpts,
    });
  }

  async function showHistory(entries: EntryModel[], workspaceInfo: WorkspaceInfo): Promise<void> {
    if (entries.length !== 1) {
      return;
    }

    await navigateTo(Routes.History, {
      query: {
        documentPath: entries[0].path,
        workspaceHandle: workspaceInfo.handle,
        selectFile: entries[0].isFile() ? entries[0].name : undefined,
      },
    });
  }

  async function openEntry(entryToOpen: EntryModel, options: OpenPathOptions, workspaceInfo: WorkspaceInfo): Promise<void> {
    if (!entryToOpen.isFile()) {
      window.electronAPI.log('warn', 'Trying to open an entry that is not a file.');
      return;
    }

    const entry = entryToOpen as EntryStatFile;
    const workspaceHandle = workspaceInfo.handle;

    if (workspaceAttributes.isHidden(workspaceInfo.id) && isDesktop() && options.skipViewers) {
      const answer = await askQuestion('WorkspacesPage.openFile.title', 'WorkspacesPage.openFile.description', {
        yesText: 'WorkspacesPage.openFile.actionConfirm',
        noText: 'WorkspacesPage.openFile.actionCancel',
      });
      if (answer === Answer.No) {
        return;
      }

      const result = await showWorkspace(workspaceInfo, workspaceAttributes, informationManager.value, eventDistributor.value);

      if (!result) {
        return;
      }
    }

    await pathOpener.openPath(workspaceHandle, entry.path, options);
  }

  async function seeInExplorer(path: string, workspaceInfo: WorkspaceInfo): Promise<void> {
    if (!isDesktop()) {
      return;
    }
    if (workspaceAttributes.isHidden(workspaceInfo.id)) {
      const answer = await askQuestion(
        'WorkspacesPage.openInExplorerModal.file.title',
        'WorkspacesPage.openInExplorerModal.file.description',
        {
          yesText: 'WorkspacesPage.openInExplorerModal.actionConfirm',
          noText: 'WorkspacesPage.openInExplorerModal.actionCancel',
        },
      );

      if (answer === Answer.No) {
        return;
      }

      const result = await showWorkspace(workspaceInfo, workspaceAttributes, informationManager.value, eventDistributor.value);

      if (!result) {
        return;
      }
    }

    await pathOpener.showInExplorer(workspaceInfo.handle, path);
  }

  async function showEnclosingFolder(entry: EntryModel, workspaceInfo: WorkspaceInfo): Promise<void> {
    const parent = await Path.parent(entry.path);
    await navigateTo(Routes.Documents, {
      query: { documentPath: parent, workspaceHandle: workspaceInfo.handle, selectFile: entry.name },
    });
  }

  return {
    deleteEntries,
    copyEntries,
    renameEntry,
    copyLink,
    moveEntriesTo,
    showDetails,
    downloadEntries,
    seeInExplorer,
    showEnclosingFolder,
    showHistory,
    openEntry,
  };
}
