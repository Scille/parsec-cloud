// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Path } from '@/parsec/path';
import {
  FileDescriptor,
  FsPath,
  Result,
  WorkspaceHistoryEntryStat,
  WorkspaceHistoryEntryStatFile,
  WorkspaceHistoryEntryStatFolder,
  WorkspaceHistoryEntryStatTag,
  WorkspaceHistoryFdCloseError,
  WorkspaceHistoryFdCloseErrorTag,
  WorkspaceHistoryFdReadError,
  WorkspaceHistoryFdReadErrorTag,
  WorkspaceHistoryHandle,
  WorkspaceHistoryInternalOnlyError,
  WorkspaceHistoryInternalOnlyErrorTag,
  WorkspaceHistoryOpenFileError,
  WorkspaceHistoryOpenFileErrorTag,
  WorkspaceHistorySetTimestampOfInterestError,
  WorkspaceHistorySetTimestampOfInterestErrorTag,
  WorkspaceHistoryStartError,
  WorkspaceHistoryStatEntryError,
  WorkspaceHistoryStatEntryErrorTag,
  WorkspaceHistoryStatFolderChildrenError,
  WorkspaceHistoryStatFolderChildrenErrorTag,
  WorkspaceID,
} from '@/parsec/types';
import { getUserInfoFromDeviceID } from '@/parsec/user';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { DateTime } from 'luxon';

export class WorkspaceHistory {
  private workspaceId: WorkspaceID;
  private handle?: WorkspaceHistoryHandle;
  private upperBound: DateTime;
  private lowerBound: DateTime;
  private currentTime: DateTime;

  constructor(workspaceId: WorkspaceID) {
    this.workspaceId = workspaceId;
    this.handle = undefined;
    this.upperBound = DateTime.fromMillis(0);
    this.lowerBound = DateTime.fromMillis(0);
    this.currentTime = DateTime.fromMillis(0);
  }

  isStarted(): boolean {
    return this.handle !== undefined;
  }

  getLowerBound(): DateTime {
    return this.lowerBound;
  }

  getUpperBound(): DateTime {
    return this.upperBound;
  }

  getCurrentTime(): DateTime {
    return this.currentTime;
  }

  reset(): void {
    this.handle = undefined;
    this.upperBound = DateTime.fromMillis(0);
    this.lowerBound = DateTime.fromMillis(0);
    this.currentTime = DateTime.fromMillis(0);
  }

  async start(currentTime?: DateTime): Promise<Result<WorkspaceHistoryHandle, WorkspaceHistoryStartError>> {
    if (this.handle !== undefined) {
      return { ok: true, value: this.handle };
    }
    const connectionHandle = getConnectionHandle();
    if (!connectionHandle) {
      return generateNoHandleError<WorkspaceHistoryStartError>();
    }
    const result = await libparsec.clientStartWorkspaceHistory(connectionHandle, this.workspaceId);
    if (result.ok) {
      this.handle = result.value;
      const upperBoundResult = await libparsec.workspaceHistoryGetTimestampHigherBound(result.value);
      if (upperBoundResult.ok) {
        this.upperBound = DateTime.fromSeconds(upperBoundResult.value as any as number);
      }
      const lowerBoundResult = await libparsec.workspaceHistoryGetTimestampLowerBound(result.value);
      if (lowerBoundResult.ok) {
        this.lowerBound = DateTime.fromSeconds(lowerBoundResult.value as any as number);
      }
      if (currentTime) {
        await this.setCurrentTime(currentTime);
      } else {
        const toiResult = await libparsec.workspaceHistoryGetTimestampOfInterest(result.value);
        if (toiResult.ok) {
          this.currentTime = DateTime.fromSeconds(toiResult.value as any as number);
        }
      }
    }
    return result;
  }

  async stop(): Promise<Result<null, WorkspaceHistoryInternalOnlyError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistoryInternalOnlyErrorTag.Internal, error: 'Not started' } };
    }
    const result = await libparsec.workspaceHistoryStop(this.handle);
    this.reset();
    return result;
  }

  async setCurrentTime(time: DateTime): Promise<Result<null, WorkspaceHistorySetTimestampOfInterestError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistorySetTimestampOfInterestErrorTag.Internal, error: 'Not started' } };
    }
    const result = await libparsec.workspaceHistorySetTimestampOfInterest(this.handle, time.toSeconds() as any as DateTime);
    if (result.ok) {
      this.currentTime = time;
    }
    return result;
  }

  async statFolderChildren(path: FsPath): Promise<Result<Array<WorkspaceHistoryEntryStat>, WorkspaceHistoryStatFolderChildrenError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistoryStatFolderChildrenErrorTag.Internal, error: 'Not started' } };
    }
    const result = await libparsec.workspaceHistoryStatFolderChildren(this.handle, path);
    if (result.ok) {
      const cooked: Array<WorkspaceHistoryEntryStat> = [];
      for (const [name, stat] of result.value) {
        const userInfoResult = await getUserInfoFromDeviceID(stat.lastUpdater);
        stat.created = DateTime.fromSeconds(stat.created as any as number);
        stat.updated = DateTime.fromSeconds(stat.updated as any as number);
        if (stat.tag === WorkspaceHistoryEntryStatTag.File) {
          (stat as unknown as WorkspaceHistoryEntryStatFile).size = Number(stat.size);
          (stat as unknown as WorkspaceHistoryEntryStatFile).isFile = (): boolean => true;
          (stat as unknown as WorkspaceHistoryEntryStatFile).name = name;
          (stat as unknown as WorkspaceHistoryEntryStatFile).path = await Path.join(path, name);
          (stat as unknown as WorkspaceHistoryEntryStatFile).lastUpdater = userInfoResult.ok ? userInfoResult.value : undefined;
        } else {
          (stat as unknown as WorkspaceHistoryEntryStatFolder).isFile = (): boolean => false;
          (stat as unknown as WorkspaceHistoryEntryStatFolder).name = name;
          (stat as unknown as WorkspaceHistoryEntryStatFolder).path = await Path.join(path, name);
          (stat as unknown as WorkspaceHistoryEntryStatFolder).lastUpdater = userInfoResult.ok ? userInfoResult.value : undefined;
        }
        cooked.push(stat as unknown as WorkspaceHistoryEntryStat);
      }
      return { ok: true, value: cooked };
    }
    return result;
  }

  async entryStat(path: FsPath): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistoryStatEntryErrorTag.Internal, error: 'Not started' } };
    }

    const fileName = (await Path.filename(path)) || '';
    const result = await libparsec.workspaceHistoryStatEntry(this.handle, path);
    if (result.ok) {
      const userInfoResult = await getUserInfoFromDeviceID(result.value.lastUpdater);
      if (result.value.tag === WorkspaceHistoryEntryStatTag.File) {
        (result.value as unknown as WorkspaceHistoryEntryStatFile).size = Number(result.value.size);
        (result.value as unknown as WorkspaceHistoryEntryStatFile).isFile = (): boolean => true;
        (result.value as unknown as WorkspaceHistoryEntryStatFile).name = fileName;
        (result.value as unknown as WorkspaceHistoryEntryStatFile).path = path;
        (result.value as unknown as WorkspaceHistoryEntryStatFile).lastUpdater = userInfoResult.ok ? userInfoResult.value : undefined;
      } else {
        (result.value as unknown as WorkspaceHistoryEntryStatFolder).isFile = (): boolean => false;
        (result.value as unknown as WorkspaceHistoryEntryStatFolder).name = fileName;
        (result.value as unknown as WorkspaceHistoryEntryStatFolder).path = path;
        (result.value as unknown as WorkspaceHistoryEntryStatFolder).lastUpdater = userInfoResult.ok ? userInfoResult.value : undefined;
      }
      return result as unknown as Result<WorkspaceHistoryEntryStat, WorkspaceHistoryStatEntryError>;
    }
    return result;
  }

  async openFile(path: FsPath): Promise<Result<FileDescriptor, WorkspaceHistoryOpenFileError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistoryOpenFileErrorTag.Internal, error: 'Not started' } };
    }
    return await libparsec.workspaceHistoryOpenFile(this.handle, path);
  }

  async closeFile(fd: FileDescriptor): Promise<Result<null, WorkspaceHistoryFdCloseError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistoryFdCloseErrorTag.Internal, error: 'Not started' } };
    }
    return await libparsec.workspaceHistoryFdClose(this.handle, fd);
  }

  async readFile(fd: FileDescriptor, offset: number, size: number): Promise<Result<Uint8Array, WorkspaceHistoryFdReadError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistoryFdReadErrorTag.Internal, error: 'Not started' } };
    }
    return await libparsec.workspaceHistoryFdRead(this.handle, fd, BigInt(offset), BigInt(size));
  }
}

export interface HistoryEntryTree {
  totalSize: number;
  entries: Array<WorkspaceHistoryEntryStatFile>;
  maxRecursionReached: boolean;
  maxFilesReached: boolean;
}

export async function listTreeAt(history: WorkspaceHistory, path: FsPath, depthLimit = 12, filesLimit = 10000): Promise<HistoryEntryTree> {
  async function _innerListTreeAt(history: WorkspaceHistory, path: FsPath, depth: number): Promise<HistoryEntryTree> {
    const tree: HistoryEntryTree = { totalSize: 0, entries: [], maxRecursionReached: false, maxFilesReached: false };

    if (depth > depthLimit) {
      console.warn('Max depth reached for listTree');
      tree.maxRecursionReached = true;
      return tree;
    }
    const result = await history.statFolderChildren(path);
    if (result.ok) {
      for (const entry of result.value) {
        if (tree.maxRecursionReached || tree.maxFilesReached) {
          break;
        }
        if (!entry.isFile()) {
          const subTree = await _innerListTreeAt(history, entry.path, depth + 1);
          if (subTree.maxRecursionReached) {
            tree.maxRecursionReached = true;
            return tree;
          }
          if (subTree.maxFilesReached) {
            tree.maxFilesReached = true;
            return tree;
          }
          tree.totalSize += subTree.totalSize;
          tree.entries.push(...subTree.entries);
          if (tree.entries.length > filesLimit) {
            tree.maxFilesReached = true;
          }
        } else {
          tree.totalSize += Number((entry as WorkspaceHistoryEntryStatFile).size);
          tree.entries.push(entry as WorkspaceHistoryEntryStatFile);
          if (tree.entries.length > filesLimit) {
            tree.maxFilesReached = true;
          }
        }
      }
    }
    return tree;
  }

  return await _innerListTreeAt(history, path, 0);
}
