// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Path } from '@/parsec/path';
import { getParsecHandle } from '@/parsec/routing';
import {
  FileDescriptor,
  FsPath,
  Result,
  WorkspaceHistory2EntryStatTag,
  WorkspaceHistory2FdCloseError,
  WorkspaceHistory2FdCloseErrorTag,
  WorkspaceHistory2FdReadError,
  WorkspaceHistory2FdReadErrorTag,
  WorkspaceHistory2InternalOnlyError,
  WorkspaceHistory2InternalOnlyErrorTag,
  WorkspaceHistory2OpenFileError,
  WorkspaceHistory2OpenFileErrorTag,
  WorkspaceHistory2SetTimestampOfInterestError,
  WorkspaceHistory2SetTimestampOfInterestErrorTag,
  WorkspaceHistory2StartError,
  WorkspaceHistory2StatEntryError,
  WorkspaceHistory2StatEntryErrorTag,
  WorkspaceHistory2StatFolderChildrenError,
  WorkspaceHistory2StatFolderChildrenErrorTag,
  WorkspaceHistoryEntryStat,
  WorkspaceHistoryEntryStatFile,
  WorkspaceHistoryEntryStatFolder,
  WorkspaceHistoryHandle,
  WorkspaceID,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
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

  async start(currentTime?: DateTime): Promise<Result<WorkspaceHistoryHandle, WorkspaceHistory2StartError>> {
    if (this.handle !== undefined) {
      return { ok: true, value: this.handle };
    }
    const connectionHandle = getParsecHandle();
    if (!connectionHandle) {
      return generateNoHandleError<WorkspaceHistory2StartError>();
    }
    const result = await libparsec.clientStartWorkspaceHistory2(connectionHandle, this.workspaceId);
    if (result.ok) {
      this.handle = result.value;
      const upperBoundResult = await libparsec.workspaceHistory2GetTimestampHigherBound(result.value);
      if (upperBoundResult.ok) {
        this.upperBound = DateTime.fromSeconds(upperBoundResult.value as any as number);
      }
      const lowerBoundResult = await libparsec.workspaceHistory2GetTimestampLowerBound(result.value);
      if (lowerBoundResult.ok) {
        this.lowerBound = DateTime.fromSeconds(lowerBoundResult.value as any as number);
      }
      if (currentTime) {
        await this.setCurrentTime(currentTime);
      } else {
        const toiResult = await libparsec.workspaceHistory2GetTimestampOfInterest(result.value);
        if (toiResult.ok) {
          this.currentTime = DateTime.fromSeconds(toiResult.value as any as number);
        }
      }
    }
    return result;
  }

  async stop(): Promise<Result<null, WorkspaceHistory2InternalOnlyError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2InternalOnlyErrorTag.Internal, error: 'Not started' } };
    }
    const result = await libparsec.workspaceHistory2Stop(this.handle);
    this.reset();
    return result;
  }

  async setCurrentTime(time: DateTime): Promise<Result<null, WorkspaceHistory2SetTimestampOfInterestError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2SetTimestampOfInterestErrorTag.Internal, error: 'Not started' } };
    }
    const result = await libparsec.workspaceHistory2SetTimestampOfInterest(this.handle, time.toSeconds() as any as DateTime);
    if (result.ok) {
      this.currentTime = time;
    }
    return result;
  }

  async statFolderChildren(path: FsPath): Promise<Result<Array<WorkspaceHistoryEntryStat>, WorkspaceHistory2StatFolderChildrenError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2StatFolderChildrenErrorTag.Internal, error: 'Not started' } };
    }
    const result = await libparsec.workspaceHistory2StatFolderChildren(this.handle, path);
    if (result.ok) {
      const cooked: Array<WorkspaceHistoryEntryStat> = [];
      for (const [name, stat] of result.value) {
        stat.created = DateTime.fromSeconds(stat.created as any as number);
        stat.updated = DateTime.fromSeconds(stat.updated as any as number);
        if (stat.tag === WorkspaceHistory2EntryStatTag.File) {
          (stat as unknown as WorkspaceHistoryEntryStatFile).size = Number(stat.size);
          (stat as unknown as WorkspaceHistoryEntryStatFile).isFile = (): boolean => true;
          (stat as unknown as WorkspaceHistoryEntryStatFile).name = name;
          (stat as unknown as WorkspaceHistoryEntryStatFile).path = await Path.join(path, name);
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

  async entryStat(path: FsPath): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistory2StatEntryError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2StatEntryErrorTag.Internal, error: 'Not started' } };
    }

    const fileName = (await Path.filename(path)) || '';
    const result = await libparsec.workspaceHistory2StatEntry(this.handle, path);
    if (result.ok) {
      if (result.value.tag === WorkspaceHistory2EntryStatTag.File) {
        (result.value as unknown as WorkspaceHistoryEntryStatFile).size = Number(result.value.size);
        (result.value as unknown as WorkspaceHistoryEntryStatFile).isFile = (): boolean => true;
        (result.value as unknown as WorkspaceHistoryEntryStatFile).name = fileName;
        (result.value as unknown as WorkspaceHistoryEntryStatFile).path = path;
      } else {
        (result.value as WorkspaceHistoryEntryStatFolder).isFile = (): boolean => false;
        (result.value as WorkspaceHistoryEntryStatFolder).name = fileName;
        (result.value as WorkspaceHistoryEntryStatFolder).path = path;
      }
      return result as Result<WorkspaceHistoryEntryStat, WorkspaceHistory2StatEntryError>;
    }
    return result;
  }

  async openFile(path: FsPath): Promise<Result<FileDescriptor, WorkspaceHistory2OpenFileError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2OpenFileErrorTag.Internal, error: 'Not started' } };
    }
    return await libparsec.workspaceHistory2OpenFile(this.handle, path);
  }

  async closeFile(fd: FileDescriptor): Promise<Result<null, WorkspaceHistory2FdCloseError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2FdCloseErrorTag.Internal, error: 'Not started' } };
    }
    return await libparsec.workspaceHistory2FdClose(this.handle, fd);
  }

  async readFile(fd: FileDescriptor, offset: number, size: number): Promise<Result<Uint8Array, WorkspaceHistory2FdReadError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2FdReadErrorTag.Internal, error: 'Not started' } };
    }
    return await libparsec.workspaceHistory2FdRead(this.handle, fd, BigInt(offset), BigInt(size));
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
