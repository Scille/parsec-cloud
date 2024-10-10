// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { generateEntries } from '@/parsec/mock_generator';
import { Path } from '@/parsec/path';
import {
  FsPath,
  Result,
  WorkspaceHandle,
  WorkspaceHistoryEntryStat,
  WorkspaceHistoryEntryStatFile,
  WorkspaceHistoryEntryStatFolder,
  WorkspaceHistoryEntryStatTag,
  WorkspaceHistoryStatFolderChildrenError,
} from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export async function statFolderChildrenAt(
  handle: WorkspaceHandle,
  path: FsPath,
  at: DateTime,
): Promise<Result<Array<WorkspaceHistoryEntryStat>, WorkspaceHistoryStatFolderChildrenError>> {
  if (!needsMocks()) {
    const result = await libparsec.workspaceHistoryStatFolderChildren(handle, at.toSeconds() as any as DateTime, path);
    if (result.ok) {
      const cooked: Array<WorkspaceHistoryEntryStat> = [];
      for (const [name, stat] of result.value) {
        stat.created = DateTime.fromSeconds(stat.created as any as number);
        stat.updated = DateTime.fromSeconds(stat.updated as any as number);
        if (stat.tag === WorkspaceHistoryEntryStatTag.File) {
          (stat as WorkspaceHistoryEntryStatFile).isFile = (): boolean => true;
          (stat as WorkspaceHistoryEntryStatFile).name = name;
          (stat as WorkspaceHistoryEntryStatFile).path = await Path.join(path, name);
        } else {
          (stat as WorkspaceHistoryEntryStatFolder).isFile = (): boolean => false;
          (stat as WorkspaceHistoryEntryStatFolder).name = name;
          (stat as WorkspaceHistoryEntryStatFolder).path = await Path.join(path, name);
        }
        cooked.push(stat as WorkspaceHistoryEntryStat);
      }
      return { ok: true, value: cooked };
    }
    return result;
  }

  const items = (await generateEntries(path)).map((entry) => {
    return entry as WorkspaceHistoryEntryStat;
  });
  return { ok: true, value: items };
}
