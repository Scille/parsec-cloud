// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FsPath, ParsecEntryStat, WorkspaceHandle } from '@/parsec/types';
import { EntryStatTag, libparsec } from '@/plugins/libparsec';

export interface SearchResult {
  name: string;
  path: FsPath;
  stats: ParsecEntryStat;
}

export async function fileSearch(
  workspaceHandle: WorkspaceHandle,
  root: FsPath,
  pattern: string,
  results: Array<SearchResult>,
  signal: AbortSignal,
): Promise<void> {
  const stack: Array<FsPath> = [root];
  const lowerPattern = pattern.toLocaleLowerCase();

  while (stack.length) {
    if (signal.aborted) {
      return;
    }
    const path = stack.pop()!;
    const statsResult = await libparsec.workspaceStatFolderChildren(workspaceHandle, path);
    if (!statsResult.ok) {
      continue;
    }
    for (const [name, entryStat] of statsResult.value) {
      if (name.toLocaleLowerCase().includes(lowerPattern)) {
        results.push({ name: name, stats: entryStat, path: path });
      } else if (entryStat.tag === EntryStatTag.Folder) {
        const fullPath = await libparsec.pathJoin(path, name);
        stack.push(fullPath);
      }
      if (signal.aborted) {
        return;
      }
    }
  }
}
