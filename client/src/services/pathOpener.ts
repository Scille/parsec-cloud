// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DetectedFileType, detectFileContentType, FileContentType } from '@/common/fileTypes';
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
import { currentRouteIs, getCurrentRouteQuery, getDocumentPath, navigateTo, Routes } from '@/router';
import { isCryptpadEnabledForDocumentType } from '@/services/cryptpad';
import { Env } from '@/services/environment';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { FileHandlerMode } from '@/views/files/handler';
import { DateTime } from 'luxon';
import { Base64 } from 'megashark-lib';
import { Ref, ref } from 'vue';

const currentlyOpening = ref(false);
let timeoutId: any = undefined;

interface OpenPathOptions {
  skipViewers?: boolean;
  disallowSystem?: boolean;
  atTime?: DateTime;
  readOnly?: boolean;
}

const ENABLED_FILE_VIEWERS = [FileContentType.Audio, FileContentType.Image, FileContentType.PdfDocument, FileContentType.Video];

const OPEN_FILE_SIZE_LIMIT = 15_000_000;

async function _openWithSystem(
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

async function _getEntryStat(
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

async function _openInEditor(
  entry: EntryStat | WorkspaceHistoryEntryStat,
  workspaceHandle: WorkspaceHandle,
  options: OpenPathOptions,
  contentType: DetectedFileType,
): Promise<void> {
  if (entry.isFile() && !options.atTime) {
    recentDocumentManager.addFile({
      entryId: entry.id,
      path: entry.path,
      workspaceHandle: workspaceHandle,
      name: entry.name,
    });
  }

  await navigateTo(Routes.FileHandler, {
    replace: currentRouteIs(Routes.FileHandler),
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
}

async function _openInViewer(
  entry: EntryStat | WorkspaceHistoryEntryStat,
  workspaceHandle: WorkspaceHandle,
  options: OpenPathOptions,
  contentType: DetectedFileType,
): Promise<void> {
  if (entry.isFile() && !options.atTime) {
    recentDocumentManager.addFile({
      entryId: entry.id,
      path: entry.path,
      workspaceHandle: workspaceHandle,
      name: entry.name,
    });
  }

  await navigateTo(Routes.FileHandler, {
    replace: currentRouteIs(Routes.FileHandler),
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
  options: OpenPathOptions,
): Promise<void> {
  if (currentlyOpening.value) {
    window.electronAPI.log('info', 'openPath() called while a file is already being opened.');
    return;
  }
  currentlyOpening.value = true;
  // Make sure that the state gets reset if we miss something
  timeoutId = window.setTimeout(() => {
    window.electronAPI.log('warn', 'Resolved path opened with timeout, might be worth investigating...');
    pathOpened();
  }, 30000);
  const entry = await _getEntryStat(workspaceHandle, path, informationManager, options);

  if (!entry) {
    pathOpened();
    return;
  }

  // The entry is not a file. If allowed, we open it using the system
  if (!entry.isFile()) {
    if (!options.disallowSystem) {
      await _openWithSystem(workspaceHandle, entry, informationManager);
    } else {
      await informationManager.present(
        new Information({
          message: 'fileViewers.errors.noFolderPreview',
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
    }
    pathOpened();
    return;
  }

  // Check if we're online, and otherwise, check if we have the complete file available
  const [clientInfoResult, available] = await Promise.all([getClientInfo(), isFileContentAvailable(workspaceHandle, entry.path)]);
  if (clientInfoResult.ok && !clientInfoResult.value.isServerOnline && !available) {
    await informationManager.present(
      new Information({
        message: 'FoldersPage.open.fileUnavailable',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    pathOpened();
    return;
  }

  // We're on desktop and not using viewers and we're allowed to use the system
  if (isDesktop() && options.skipViewers && !options.disallowSystem) {
    await _openWithSystem(workspaceHandle, entry, informationManager);
    pathOpened();
    return;
  }

  // The file is already opened
  const query = getCurrentRouteQuery();

  if (currentRouteIs(Routes.FileHandler) && getDocumentPath() === path && Boolean(options.readOnly) === Boolean(query.readOnly)) {
    window.electronAPI.log('debug', 'File is already opened.');
    pathOpened();
    return;
  }

  const contentType = detectFileContentType(entry.name);

  if (contentType.type === FileContentType.Unknown) {
    // Couldn't detect the file type, try with the system if allowed/available, otherwise display a message
    if (isDesktop() && !options.disallowSystem) {
      await _openWithSystem(workspaceHandle, entry, informationManager);
    } else {
      await informationManager.present(
        new Information({
          title: isWeb() ? 'FoldersPage.open.noVisibleOnWebTitle' : undefined,
          message: isWeb() ? 'FoldersPage.open.noVisibleOnWeb' : 'FoldersPage.open.unhandledFileType',
          level: isWeb() ? InformationLevel.Info : InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
    }
    pathOpened();
    return;
  }
  if ((entry as any).size > OPEN_FILE_SIZE_LIMIT) {
    // Too big to open, display try with the system if allowed/available, otherwise display a message
    if (isDesktop() && !options.disallowSystem) {
      await _openWithSystem(workspaceHandle, entry, informationManager);
    } else {
      await informationManager.present(
        new Information({
          title: 'fileViewers.errors.titles.fileTooBig',
          message: 'fileViewers.errors.informationPreviewDownload',
          level: InformationLevel.Warning,
        }),
        PresentationMode.Modal,
      );
    }
    pathOpened();
    return;
  }
  if (Env.isEditicsEnabled() && isCryptpadEnabledForDocumentType(contentType.type)) {
    return await _openInEditor(entry, workspaceHandle, options, contentType);
  } else if (ENABLED_FILE_VIEWERS.includes(contentType.type)) {
    return await _openInViewer(entry, workspaceHandle, options, contentType);
  } else {
    window.electronAPI.log('warn', `No way to open file of type '${contentType.type}' (ext '${contentType.extension}'), should not happen`);
    await informationManager.present(
      new Information({
        title: isWeb() ? 'FoldersPage.open.noVisibleOnWebTitle' : undefined,
        message: isWeb() ? 'FoldersPage.open.noVisibleOnWeb' : 'FoldersPage.open.unhandledFileType',
        level: isWeb() ? InformationLevel.Info : InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    pathOpened();
  }
}

function pathOpened(): void {
  currentlyOpening.value = false;
  if (timeoutId) {
    window.clearTimeout(timeoutId);
    timeoutId = undefined;
  }
}

interface PathOpener {
  openPath: (
    workspaceHandle: WorkspaceHandle,
    path: FsPath,
    informationManager: InformationManager,
    options: OpenPathOptions,
  ) => Promise<void>;
  showInExplorer: (workspaceHandle: WorkspaceHandle, path: FsPath, informationManager: InformationManager) => Promise<void>;
  pathOpened(): void;
  currentlyOpening: Ref<boolean>;
}

export default function useFileOpener(): PathOpener {
  return {
    pathOpened,
    currentlyOpening,
    openPath,
    showInExplorer,
  };
}

export { OpenPathOptions };
