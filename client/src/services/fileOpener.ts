// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { detectFileContentType, FileContentType } from '@/common/fileTypes';
import { entryStat, EntryStat, entryStatAt, FsPath, getSystemPath, isDesktop, WorkspaceHandle, WorkspaceHistoryEntryStat } from '@/parsec';
import { currentRouteIs, getDocumentPath, navigateTo, Routes } from '@/router';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { DateTime } from 'luxon';
import { Base64, openSpinnerModal } from 'megashark-lib';

interface OpenPathOptions {
  skipViewers?: boolean;
  atTime?: DateTime;
}

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
    statsResult = await entryStatAt(workspaceHandle, path, options.atTime);
  } else {
    statsResult = await entryStat(workspaceHandle, path);
  }
  if (!statsResult.ok) {
    await informationManager.present(
      new Information({
        message: 'FoldersPage.open.fileFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    return;
  }

  const entry = statsResult.value;

  if (!entry.isFile()) {
    await openWithSystem(workspaceHandle, entry, informationManager);
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
    if (!contentType || contentType.type === FileContentType.Unknown) {
      await openWithSystem(workspaceHandle, entry, informationManager);
    } else {
      if ((entry as any).size > OPEN_FILE_SIZE_LIMIT) {
        informationManager.present(
          new Information({
            message: 'fileViewers.fileTooBig',
            level: InformationLevel.Warning,
          }),
          PresentationMode.Toast,
        );
        await openWithSystem(workspaceHandle, entry, informationManager);
        return;
      }

      recentDocumentManager.addFile({
        entryId: entry.id,
        path: entry.path,
        workspaceHandle: workspaceHandle,
        name: entry.name,
        contentType: contentType,
      });

      await navigateTo(Routes.Viewer, {
        query: { workspaceHandle: workspaceHandle, documentPath: entry.path, fileTypeInfo: Base64.fromObject(contentType) },
      });
    }
  } finally {
    await modal.dismiss();
  }
}

export { openPath, showInExplorer };
