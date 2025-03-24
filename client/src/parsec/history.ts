// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { wait } from '@/parsec/internals';
import { MockFileType, getMockFileContent } from '@/parsec/mock_files';
import { generateEntriesForEachFileType, generateFile, generateFolder } from '@/parsec/mock_generator';
import { Path } from '@/parsec/path';
import { getParsecHandle } from '@/parsec/routing';
import {
  EntryName,
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
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

const MOCK_OPENED_FILES = new Map<FileDescriptor, FsPath>();
let MOCK_CURRENT_FD = 1;

const MOCK_OPENED_HISTORY = new Map<WorkspaceHistoryHandle, WorkspaceID>();
let MOCK_OPEN_HISTORY_FD = 1;

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
    if (connectionHandle && !needsMocks()) {
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
    } else {
      if (Array.from(MOCK_OPENED_HISTORY.values()).find((v) => v === this.workspaceId) !== undefined) {
        console.log('This workspace already has an opened history');
      }
      const fd = MOCK_OPEN_HISTORY_FD;
      this.handle = fd;
      MOCK_OPEN_HISTORY_FD += 1;
      MOCK_OPENED_HISTORY.set(fd, this.workspaceId);
      this.upperBound = DateTime.now();
      this.lowerBound = DateTime.now().minus({ months: 4 });
      if (currentTime) {
        await this.setCurrentTime(currentTime);
      } else {
        this.currentTime = this.upperBound;
      }
      return { ok: true, value: fd };
    }
  }

  async stop(): Promise<Result<null, WorkspaceHistory2InternalOnlyError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2InternalOnlyErrorTag.Internal, error: 'Not started' } };
    }
    if (!needsMocks()) {
      const result = await libparsec.workspaceHistory2Stop(this.handle);
      this.reset();
      return result;
    } else {
      if (!MOCK_OPENED_HISTORY.has(this.handle)) {
        return { ok: false, error: { tag: WorkspaceHistory2InternalOnlyErrorTag.Internal, error: 'Not started' } };
      }
      MOCK_OPENED_HISTORY.delete(this.handle);
      this.handle = undefined;
    }
    return { ok: true, value: null };
  }

  async setCurrentTime(time: DateTime): Promise<Result<null, WorkspaceHistory2SetTimestampOfInterestError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2SetTimestampOfInterestErrorTag.Internal, error: 'Not started' } };
    }
    if (!needsMocks()) {
      const result = await libparsec.workspaceHistory2SetTimestampOfInterest(this.handle, time.toSeconds() as any as DateTime);
      if (result.ok) {
        this.currentTime = time;
      }
      return result;
    } else {
      this.currentTime = time;
      return { ok: true, value: null };
    }
  }

  async statFolderChildren(path: FsPath): Promise<Result<Array<WorkspaceHistoryEntryStat>, WorkspaceHistory2StatFolderChildrenError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2StatFolderChildrenErrorTag.Internal, error: 'Not started' } };
    }
    if (!needsMocks()) {
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
    // Take some time to load
    await wait(1500);
    const items = (await generateEntriesForEachFileType(path)).map((entry) => {
      return entry as any as WorkspaceHistoryEntryStat;
    });
    return { ok: true, value: items };
  }

  async entryStat(path: FsPath): Promise<Result<WorkspaceHistoryEntryStat, WorkspaceHistory2StatEntryError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2StatEntryErrorTag.Internal, error: 'Not started' } };
    }

    const fileName = (await Path.filename(path)) || '';

    if (!needsMocks()) {
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
    const entry = fileName.startsWith('File_') ? await generateFile(path, { fileName: fileName }) : await generateFolder(path);
    return { ok: true, value: entry as any as WorkspaceHistoryEntryStat };
  }

  async openFile(path: FsPath): Promise<Result<FileDescriptor, WorkspaceHistory2OpenFileError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2OpenFileErrorTag.Internal, error: 'Not started' } };
    }
    if (!needsMocks()) {
      const result = await libparsec.workspaceHistory2OpenFile(this.handle, path);
      return result;
    } else {
      const fd = MOCK_CURRENT_FD;
      MOCK_CURRENT_FD += 1;
      MOCK_OPENED_FILES.set(fd, path);
      return { ok: true, value: fd };
    }
  }

  async closeFile(fd: FileDescriptor): Promise<Result<null, WorkspaceHistory2FdCloseError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2FdCloseErrorTag.Internal, error: 'Not started' } };
    }
    if (!needsMocks()) {
      return await libparsec.workspaceHistory2FdClose(this.handle, fd);
    } else {
      if (!MOCK_OPENED_FILES.has(fd)) {
        return { ok: false, error: { tag: WorkspaceHistory2FdCloseErrorTag.BadFileDescriptor, error: 'Invalid file descriptor' } };
      }
      MOCK_OPENED_FILES.delete(fd);
      return { ok: true, value: null };
    }
  }

  async readFile(fd: FileDescriptor, offset: number, size: number): Promise<Result<Uint8Array, WorkspaceHistory2FdReadError>> {
    if (this.handle === undefined) {
      return { ok: false, error: { tag: WorkspaceHistory2FdReadErrorTag.Internal, error: 'Not started' } };
    }
    if (!needsMocks()) {
      return await libparsec.workspaceHistory2FdRead(this.handle, fd, BigInt(offset), BigInt(size));
    } else {
      if (!MOCK_OPENED_FILES.has(fd)) {
        return { ok: false, error: { tag: WorkspaceHistory2FdReadErrorTag.BadFileDescriptor, error: 'Invalid file descriptor' } };
      }
      await wait(100);
      const path = MOCK_OPENED_FILES.get(fd) as string;
      const fileName = (await Path.filename(path)) as EntryName;
      const ext = Path.getFileExtension(fileName);

      switch (ext) {
        case 'xlsx':
          offset === 0 && console.log('Using XLSX content');
          return { ok: true, value: (await getMockFileContent(MockFileType.XLSX)).slice(offset, offset + size) };
        case 'png':
          offset === 0 && console.log('Using PNG content');
          return { ok: true, value: (await getMockFileContent(MockFileType.PNG)).slice(offset, offset + size) };
        case 'docx':
          offset === 0 && console.log('Using DOCX content');
          return { ok: true, value: (await getMockFileContent(MockFileType.DOCX)).slice(offset, offset + size) };
        case 'txt':
          offset === 0 && console.log('Using TXT content');
          return { ok: true, value: (await getMockFileContent(MockFileType.TXT)).slice(offset, offset + size) };
        case 'py':
          offset === 0 && console.log('Using PY content');
          return { ok: true, value: (await getMockFileContent(MockFileType.PY)).slice(offset, offset + size) };
        case 'pdf':
          offset === 0 && console.log('Using PDF content');
          return { ok: true, value: (await getMockFileContent(MockFileType.PDF)).slice(offset, offset + size) };
        case 'mp3':
          offset === 0 && console.log('Using MP3 content');
          return { ok: true, value: (await getMockFileContent(MockFileType.MP3)).slice(offset, offset + size) };
        case 'mp4':
          offset === 0 && console.log('Using MP4 content');
          return { ok: true, value: (await getMockFileContent(MockFileType.MP4)).slice(offset, offset + size) };
      }
      offset === 0 && console.log('Using default file content');
      return {
        ok: true,
        value: new Uint8Array([
          137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 5, 0, 0, 0, 5, 8, 6, 0, 0, 0, 141, 111, 38, 229, 0, 0, 0,
          28, 73, 68, 65, 84, 8, 215, 99, 248, 255, 255, 63, 195, 127, 6, 32, 5, 195, 32, 18, 132, 208, 49, 241, 130, 88, 205, 4, 0, 14,
          245, 53, 203, 209, 142, 14, 31, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
        ]),
      };
    }
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
          tree.totalSize += (entry as WorkspaceHistoryEntryStatFile).size;
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
