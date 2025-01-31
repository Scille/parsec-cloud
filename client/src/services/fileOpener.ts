// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectFileContentType, FileContentType } from '@/common/fileTypes';
import {
  entryStat,
  EntryStat,
  FsPath,
  getSystemPath,
  getWorkspaceInfo,
  isDesktop,
  WorkspaceHandle,
  WorkspaceHistory,
  WorkspaceHistoryEntryStat,
} from '@/parsec';
import { currentRouteIs, getDocumentPath, navigateTo, Routes } from '@/router';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { DateTime } from 'luxon';
import { Base64, openSpinnerModal } from 'megashark-lib';

interface OpenPathOptions {
  skipViewers?: boolean;
  onlyViewers?: boolean;
  atTime?: DateTime;
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
        message: entry.isFile() ? 'FoldersPage.open.fileFailed' : 'FoldersPage.open.folderFailed',
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

async function openPath(
  workspaceHandle: WorkspaceHandle,
  path: FsPath,
  informationManager: InformationManager,
  options: OpenPathOptions,
): Promise<void> {
  let statsResult;
  if (options.atTime) {
    const infoResult = await getWorkspaceInfo(workspaceHandle);
    if (!infoResult.ok) {
      return;
    }
    const history = new WorkspaceHistory(infoResult.value.id);
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

  const entry = statsResult.value;

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
  if (isDesktop() && options.skipViewers) {
    await openWithSystem(workspaceHandle, entry, informationManager);
    return;
  }

  if (currentRouteIs(Routes.Viewer) && getDocumentPath() === path) {
    return;
  }

  const modal = await openSpinnerModal('fileViewers.openingFile');
  const contentType = await detectFileContentType(workspaceHandle, entry.path, options.atTime);

  try {
    if (!contentType || contentType.type === FileContentType.Unknown || (isDesktop() && !ENABLED_FILE_VIEWERS.includes(contentType.type))) {
      if (!options.onlyViewers) {
        await openWithSystem(workspaceHandle, entry, informationManager);
      } else {
        await modal.dismiss();
        await informationManager.present(
          new Information({
            message: 'FoldersPage.open.unhandledFileType',
            level: InformationLevel.Error,
          }),
          PresentationMode.Modal,
        );
      }
    } else {
      if ((entry as any).size > OPEN_FILE_SIZE_LIMIT) {
        informationManager.present(
          new Information({
            message: 'fileViewers.fileTooBig',
            level: InformationLevel.Warning,
          }),
          PresentationMode.Toast,
        );
        if (!options.onlyViewers) {
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
  } finally {
    await modal.dismiss();
  }
}

export { openPath, showInExplorer };
