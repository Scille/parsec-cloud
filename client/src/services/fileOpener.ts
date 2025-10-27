// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DetectedFileType, detectFileContentType, FileContentType } from '@/common/fileTypes';
import { FallbackCustomParams, OpenFallbackChoice } from '@/components/files';
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
import { FileOperationManager } from '@/services/fileOperationManager';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { downloadEntry } from '@/views/files';
import { FileHandlerMode } from '@/views/files/handler';
import { modalController } from '@ionic/vue';
import { DateTime } from 'luxon';
import { Base64, openSpinnerModal } from 'megashark-lib';

interface OpenPathOptions {
  skipViewers?: boolean;
  onlyViewers?: boolean;
  atTime?: DateTime;
  useEditor?: boolean;
  readOnly?: boolean;
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
): Promise<EntryStat | WorkspaceHistoryEntryStat | void> {
  let statsResult;

  if (options.atTime) {
    const workspaceInfoResult = await getWorkspaceInfo(workspaceHandle);
    if (!workspaceInfoResult.ok) {
      return;
    }
    const workspaceId = workspaceInfoResult.value.id;
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
  fallbackCustomParams?: FallbackCustomParams,
): Promise<void> {
  const modal = await modalController.create({
    component: FileOpenFallbackChoice,
    cssClass: 'file-open-fallback-choice',
    componentProps: {
      viewerOption: !currentRouteIs(Routes.FileHandler) && fallbackCustomParams?.viewerOption !== false,
      title: fallbackCustomParams?.title,
      subtitle: fallbackCustomParams?.subtitle,
    },
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();

  switch (result.data) {
    case OpenFallbackChoice.View:
      await openPath(workspaceHandle, path, informationManager, fileOperationManager, {
        skipViewers: options.skipViewers,
        onlyViewers: options.onlyViewers,
        atTime: options.atTime,
        useEditor: false,
      });
      break;
    case OpenFallbackChoice.Download: {
      const workspaceInfoResult = await getWorkspaceInfo(workspaceHandle);
      if (!workspaceInfoResult.ok) {
        return;
      }
      const workspaceId = workspaceInfoResult.value.id;
      await downloadEntry({
        name: entry.name,
        workspaceHandle: workspaceHandle,
        workspaceId: workspaceId,
        path: path,
        informationManager: informationManager,
        fileOperationManager: fileOperationManager,
      });
      break;
    }
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
  fileOperationManager: FileOperationManager,
  contentType?: DetectedFileType,
): Promise<void> {
  try {
    if (!Env.isEditicsEnabled()) {
      window.electronAPI.log('warn', 'FileOpener: Editics is not enabled, skipping editor opening');
      openFileOpenFallbackModal(entry, workspaceHandle, path, informationManager, fileOperationManager, options);
    } else if (contentType && contentType.type !== FileContentType.Unknown && isEnabledCryptpadDocumentType(contentType.type)) {
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

        await navigateTo(Routes.FileHandler, {
          query: {
            workspaceHandle: workspaceHandle,
            documentPath: entry.path,
            timestamp: options.atTime?.toMillis().toString(),
            fileTypeInfo: Base64.fromObject(contentType),
            readOnly: options.readOnly,
          },
          params: {
            mode: FileHandlerMode.Edit,
          },
        });
      } else {
        await openFileOpenFallbackModal(entry, workspaceHandle, path, informationManager, fileOperationManager, options, {
          title: 'fileViewers.errors.titles.fileTooBig',
          subtitle: 'fileViewers.errors.informationEditDownload',
          viewerOption: false,
        });
        window.electronAPI.log(
          'warn',
          `FileOpener: File too large for editor (${(entry as any).size} bytes > ${OPEN_FILE_SIZE_LIMIT}): ${entry.name}`,
        );
      }
    } else {
      if (!contentType) {
        window.electronAPI.log('warn', `FileOpener: No content type detected for file: ${entry.name}`);
        await informationManager.present(
          new Information({
            title: 'fileViewers.errors.titles.impossibleToOpen',
            message: 'fileViewers.errors.noContentFileType',
            level: InformationLevel.Warning,
          }),
          PresentationMode.Modal,
        );
      } else if (contentType.type === FileContentType.Unknown) {
        window.electronAPI.log('warn', `FileOpener: Unknown file type for editor: ${entry.name} (${contentType.extension})`);
        await openFileOpenFallbackModal(entry, workspaceHandle, path, informationManager, fileOperationManager, options, {
          title: 'fileViewers.errors.titles.unSupportedFileType',
          subtitle: 'fileViewers.errors.informationEditDownload',
          viewerOption: false,
        });
      } else if (!isEnabledCryptpadDocumentType(contentType.type)) {
        window.electronAPI.log(
          'warn',
          `FileOpener: File type not supported by editor: ${entry.name} (${contentType.type}, ${contentType.extension})`,
        );
        await informationManager.present(
          new Information({
            title: 'fileViewers.errors.titles.unSupportedFileType',
            message: 'fileViewers.errors.unknownFileExtension',
            level: InformationLevel.Warning,
          }),
          PresentationMode.Modal,
        );
      }
    }
  } catch {
    await openFileOpenFallbackModal(entry, workspaceHandle, path, informationManager, fileOperationManager, options);
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
        title: 'fileViewers.errors.titles.fileTooBig',
        message: 'fileViewers.errors.informationPreviewDownload',
        level: InformationLevel.Warning,
      }),
      PresentationMode.Modal,
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

  await navigateTo(Routes.FileHandler, {
    query: {
      workspaceHandle: workspaceHandle,
      documentPath: entry.path,
      timestamp: options.atTime?.toMillis().toString(),
      fileTypeInfo: Base64.fromObject(contentType),
    },
    params: {
      mode: FileHandlerMode.View,
    },
  });
}

async function openPath(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  informationManager: InformationManager,
  fileOperationManager: FileOperationManager,
  options: OpenPathOptions,
): Promise<void> {
  const entry = await getEntryStat(workspaceHandle, path, informationManager, options);

  if (!entry) {
    return;
  }

  if (!entry.isFile()) {
    if (!options.onlyViewers) {
      await openWithSystem(workspaceHandle, entry, informationManager);
    } else {
      await informationManager.present(
        new Information({
          message: 'fileViewers.errors.noFolderPreview',
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

  if (currentRouteIs(Routes.FileHandler) && getDocumentPath() === path && !options.useEditor) {
    return;
  }

  const modal = await openSpinnerModal('fileViewers.openingFile');
  const contentType = detectFileContentType(entry.name);
  try {
    if (options.useEditor) {
      return await openInEditor(entry, path, workspaceHandle, options, informationManager, fileOperationManager, contentType);
    }

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

export { openPath, OpenPathOptions, showInExplorer };
