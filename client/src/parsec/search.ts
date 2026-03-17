// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { statFolderChildren } from '@/parsec/file';
import { EntryStat, FsPath, WorkspaceHandle } from '@/parsec/types';

export interface SearchResult {
  stats: EntryStat;
  parent: FsPath;
  titleMatch: boolean;
}

export async function fileSearch(
  workspaceHandle: WorkspaceHandle,
  root: FsPath,
  pattern: string,
  results: Array<SearchResult>,
  signal: AbortSignal,
): Promise<void> {
  const lowerPattern = pattern.toLocaleLowerCase();
  const stack: Array<{ path: FsPath; force: boolean }> = [
    {
      path: root,
      force: root.toLocaleLowerCase().includes(lowerPattern),
    },
  ];

  // Going through the stack of folders
  while (stack.length) {
    if (signal.aborted) {
      return;
    }
    const { path, force } = stack.pop()!;
    const statsResult = await statFolderChildren(workspaceHandle, path, true, true);
    if (!statsResult.ok) {
      continue;
    }
    for (const entry of statsResult.value) {
      let matched = false;
      if (entry.name.toLocaleLowerCase().includes(lowerPattern)) {
        // Name matches, pushing that in the results
        matched = true;
        results.push({ stats: entry, titleMatch: true, parent: path });
      } else if (force) {
        // A parent folder matches, pushing that in the results
        matched = true;
        results.push({ stats: entry, titleMatch: false, parent: path });
      }
      if (!entry.isFile()) {
        // Entry is a folder, adding to the stack
        stack.push({ path: entry.path, force: matched });
      }
      if (signal.aborted) {
        return;
      }
    }
  }
}
