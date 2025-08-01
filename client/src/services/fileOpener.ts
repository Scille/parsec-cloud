// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DetectedFileType, detectFileContentType, FileContentType } from '@/common/fileTypes';
import { OpenFallbackChoice } from '@/components/files';
import FileOpenFallbackChoice from '@/components/files/FileOpenFallbackChoice.vue';
import {
  entryStat,
  EntryStat,
  FsPath,
  getClientInfo,
  getSystemPath,
  getWorkspaceInfo,
  isDesktop,
  isFileContentAvailable,
  isWeb,
  WorkspaceHandle,
  WorkspaceHistory,
  WorkspaceHistoryEntryStat,
} from '@/parsec';
import { currentRouteIs, getDocumentPath, navigateTo, Routes } from '@/router';
import { isEnabledCryptpadDocumentType } from '@/services/cryptpad';
import { Env } from '@/services/environment';
import { FileOperationManager, FileOperationManagerKey } from '@/services/fileOperationManager';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { downloadEntry } from '@/views/files';
import { modalController } from '@ionic/vue';
import { DateTime } from 'luxon';
import { Base64, openSpinnerModal } from 'megashark-lib';
import { inject } from 'vue';

interface OpenPathOptions {
  skipViewers?: boolean;
  onlyViewers?: boolean;
  atTime?: DateTime;
  useEditor?: boolean;
}

// Uncomment here to enable file viewers on desktop; should be removed when all file viewers are implemented
const ENABLED_FILE_VIEWERS = [
  FileContentType.Audio,
  FileContentType.Image,
  FileContentType.PdfDocument,
  FileContentType.Document,
  FileContentType.Video,
  FileContentType.Spreadsheet,
  FileContentType.Text,
];

const OPEN_FILE_SIZE_LIMIT = 15_000_000;
const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;

async function openWithSystem(
  workspaceHandle: WorkspaceHandle,
  entry: EntryStat | WorkspaceHistoryEntryStat,
  informationManager: InformationManager,
): Promise<void> {
  if (entry.isFile()) {
    recentDocumentManager.addFile({
      entryId: entry.id,
      path: entry.path,
      workspaceHandle: workspaceHandle,
      name: entry.name,
    });
  }

  const result = await getSystemPath(workspaceHandle, entry.path);

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: { key: entry.isFile() ? 'FoldersPage.open.fileFailed' : 'FoldersPage.open.folderFailed', data: { name: entry.name } },
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
  } else {
    window.electronAPI.openFile(result.value);
  }
}

async function showInExplorer(workspaceHandle: WorkspaceHandle, path: FsPath, informationManager: InformationManager): Promise<void> {
  const result = await getSystemPath(workspaceHandle, path);

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: 'FoldersPage.errors.seeInExplorerFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
  } else {
    window.electronAPI.seeInExplorer(result.value);
  }
}

async function getEntryStat(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  informationManager: InformationManager,
  options: OpenPathOptions,
  workspaceId?: string,
): Promise<EntryStat | WorkspaceHistoryEntryStat | void> {
  let statsResult;

  if (options.atTime) {
    if (!workspaceId) {
      return;
    }
    const history = new WorkspaceHistory(workspaceId);
    await history.start(options.atTime);
    statsResult = await history.entryStat(path);
    await history.stop();
  } else {
    statsResult = await entryStat(workspaceHandle, path);
  }
  if (!statsResult.ok) {
    await informationManager.present(
      new Information({
        message: 'FoldersPage.open.fileFailedGeneric',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    return;
  }
  return statsResult.value;
}

async function openFileOpenFallbackModal(
  entry: EntryStat | WorkspaceHistoryEntryStat,
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  informationManager: InformationManager,
  fileOperationManager: FileOperationManager,
  options: OpenPathOptions,
  workspaceId?: string,
): Promise<void> {
  const modal = await modalController.create({
    component: FileOpenFallbackChoice,
    componentProps: {
      viewerOption: !currentRouteIs(Routes.Viewer),
    },
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();

  switch (result.data) {
    case OpenFallbackChoice.View:
      await openPath(workspaceHandle, path, informationManager, {
        skipViewers: options.skipViewers,
        onlyViewers: options.onlyViewers,
        atTime: options.atTime,
        useEditor: false,
      });
      break;
    case OpenFallbackChoice.Download:
      if (!workspaceId) {
        return;
      }
      await downloadEntry({
        name: entry.name,
        workspaceHandle: workspaceHandle,
        workspaceId: workspaceId,
        path: path,
        informationManager: informationManager,
        fileOperationManager: fileOperationManager,
      });
      break;
    case OpenFallbackChoice.Open:
      await openWithSystem(workspaceHandle, entry, informationManager);
      break;
  }
}

async function openInEditor(
  entry: EntryStat | WorkspaceHistoryEntryStat,
  path: FsPath,
  workspaceHandle: WorkspaceHandle,
  options: OpenPathOptions,
  informationManager: InformationManager,
  workspaceId?: string,
  contentType?: DetectedFileType,
): Promise<void> {
  try {
    if (!Env.isEditicsEnabled()) {
      return;
    }
    if (contentType && contentType.type !== FileContentType.Unknown && isEnabledCryptpadDocumentType(contentType.extension)) {
      // Handle Cryptpad supported document types
      if ((entry as any).size <= OPEN_FILE_SIZE_LIMIT) {
        if (!options.atTime) {
          recentDocumentManager.addFile({
            entryId: entry.id,
            path: entry.path,
            workspaceHandle: workspaceHandle,
            name: entry.name,
            contentType: contentType,
          });
        }
        await navigateTo(Routes.Editor, {
          query: {
            workspaceHandle: workspaceHandle,
            documentPath: entry.path,
            timestamp: options.atTime?.toMillis().toString(),
            fileTypeInfo: Base64.fromObject(contentType),
          },
        });
      }
    }
  } catch (e: any) {
    await openFileOpenFallbackModal(entry, workspaceHandle, path, informationManager, fileOperationManager, options, workspaceId);
  }
}

async function openInViewer(
  entry: EntryStat | WorkspaceHistoryEntryStat,
  workspaceHandle: WorkspaceHandle,
  options: OpenPathOptions,
  informationManager: InformationManager,
  contentType: DetectedFileType,
): Promise<void> {
  if ((entry as any).size > OPEN_FILE_SIZE_LIMIT) {
    informationManager.present(
      new Information({
        message: 'fileViewers.fileTooBig',
        level: InformationLevel.Warning,
      }),
      PresentationMode.Toast,
    );
    if (!isWeb() && !options.onlyViewers) {
      await openWithSystem(workspaceHandle, entry, informationManager);
    }
    return;
  }
  if (!options.atTime) {
    recentDocumentManager.addFile({
      entryId: entry.id,
      path: entry.path,
      workspaceHandle: workspaceHandle,
      name: entry.name,
      contentType: contentType,
    });
  }

  await navigateTo(Routes.Viewer, {
    query: {
      workspaceHandle: workspaceHandle,
      documentPath: entry.path,
      timestamp: options.atTime?.toMillis().toString(),
      fileTypeInfo: Base64.fromObject(contentType),
    },
  });
}

async function openPath(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  informationManager: InformationManager,
  options: OpenPathOptions,
): Promise<void> {
  const workspaceInfoResult = await getWorkspaceInfo(workspaceHandle);
  const workspaceId = workspaceInfoResult.ok ? workspaceInfoResult.value.id : undefined;
  const entry = await getEntryStat(workspaceHandle, path, informationManager, options, workspaceId);

  if (!entry) {
    return;
  }

  if (!entry.isFile()) {
    if (!options.onlyViewers) {
      await openWithSystem(workspaceHandle, entry, informationManager);
    } else {
      await informationManager.present(
        new Information({
          message: 'FoldersPage.open.noFolderPreview',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
    }
    return;
  }

  const [clientInfoResult, available] = await Promise.all([getClientInfo(), isFileContentAvailable(workspaceHandle, entry.path)]);

  if (clientInfoResult.ok && !clientInfoResult.value.isServerOnline && !available) {
    await informationManager.present(
      new Information({
        message: 'FoldersPage.open.fileUnavailable',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    return;
  }

  if (isDesktop() && options.skipViewers) {
    await openWithSystem(workspaceHandle, entry, informationManager);
    return;
  }

  if (currentRouteIs(Routes.Viewer) && getDocumentPath() === path && !options.useEditor) {
    return;
  }

  const modal = await openSpinnerModal('fileViewers.openingFile');
  const contentType = await detectFileContentType(entry.name);
  try {
    if (options.useEditor) {
      return await openInEditor(entry, path, workspaceHandle, options, informationManager, workspaceId, contentType);
    }

    // eslint-disable-next-line max-len
    if (!contentType || contentType.type === FileContentType.Unknown || (isDesktop() && !ENABLED_FILE_VIEWERS.includes(contentType.type))) {
      if (!isWeb() && !options.onlyViewers) {
        await openWithSystem(workspaceHandle, entry, informationManager);
      } else {
        await modal.dismiss();
        await informationManager.present(
          new Information({
            title: isWeb() ? 'FoldersPage.open.noVisibleOnWebTitle' : undefined,
            message: isWeb() ? 'FoldersPage.open.noVisibleOnWeb' : 'FoldersPage.open.unhandledFileType',
            level: isWeb() ? InformationLevel.Info : InformationLevel.Error,
          }),
          PresentationMode.Modal,
        );
      }
    } else {
      await openInViewer(entry, workspaceHandle, options, informationManager, contentType);
    }
  } catch (e: any) {
    console.warn(`Error while opening file: ${e}`);
  } finally {
    await modal.dismiss();
  }
}

export { openPath, showInExplorer };
