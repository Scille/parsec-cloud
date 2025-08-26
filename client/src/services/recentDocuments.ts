// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DetectedFileType } from '@/common/fileTypes';
import {
  ConnectionHandle,
  EntryID,
  EntryName,
  FsPath,
  getClientInfo,
  listWorkspaces,
  UserID,
  WorkspaceHandle,
  WorkspaceID,
  WorkspaceInfo,
} from '@/parsec';
import { StorageManager } from '@/services/storageManager';
import { Ref, ref } from 'vue';

interface RecentFile {
  entryId: EntryID;
  workspaceHandle: WorkspaceHandle;
  path: FsPath;
  name: EntryName;
  contentType?: DetectedFileType;
}

type RecentWorkspace = WorkspaceInfo;

const FILE_HISTORY_SIZE = 5;
const WORKSPACE_HISTORY_SIZE = 5;

interface RecentDocumentStorageData {
  workspaces: Array<WorkspaceID>;
}

class RecentDocumentManager {
  files: Ref<Array<RecentFile>> = ref([]);
  workspaces: Ref<Array<RecentWorkspace>> = ref([]);

  constructor() {
    this.files.value = new Array<RecentFile>();
    this.workspaces.value = new Array<RecentWorkspace>();
  }

  private _getStorageDataKey(userId: UserID): string {
    return `recentDocuments_${userId}`;
  }

  async loadFromStorage(storage: StorageManager, handle: ConnectionHandle | null = null): Promise<void> {
    const clientInfoResult = await getClientInfo(handle);
    if (!clientInfoResult.ok) {
      window.electronAPI.log('error', `Failed to load recent workspaces: ${JSON.stringify(clientInfoResult.error)}`);
      return;
    }
    const dataKey = this._getStorageDataKey(clientInfoResult.value.userId);
    const storedData = await storage.retrieveComponentData<RecentDocumentStorageData>(dataKey, { workspaces: [] });
    if (!storedData || !storedData.workspaces) {
      return;
    }
    const workspacesResult = await listWorkspaces(handle);
    if (!workspacesResult.ok) {
      window.electronAPI.log('error', `Failed to load recent workspaces: ${JSON.stringify(workspacesResult.error)}`);
      return;
    }

    for (const workspaceId of storedData.workspaces.toReversed()) {
      const wk = workspacesResult.value.find((w) => w.id === workspaceId);
      if (wk) {
        this.addWorkspace(wk);
      }
    }
  }

  async saveToStorage(storage: StorageManager): Promise<void> {
    const clientInfoResult = await getClientInfo();
    if (!clientInfoResult.ok) {
      window.electronAPI.log('error', `Failed to save recent workspaces: ${JSON.stringify(clientInfoResult.error)}`);
      return;
    }
    const dataKey = this._getStorageDataKey(clientInfoResult.value.userId);
    await storage.storeComponentData<RecentDocumentStorageData>(dataKey, {
      workspaces: this.workspaces.value.map((workspace) => workspace.id),
    });
  }

  private _arrayMove(array: Array<any>, from: number, to: number): void {
    array.splice(to, 0, array.splice(from, 1)[0]);
  }

  addFile(file: RecentFile): void {
    const index = this.files.value.findIndex((item) => item.entryId === file.entryId);
    if (index !== -1) {
      this._arrayMove(this.files.value, index, 0);
    } else if (this.files.value.unshift(file) > FILE_HISTORY_SIZE) {
      this.files.value.pop();
    }
  }

  addWorkspace(workspace: RecentWorkspace): void {
    const index = this.workspaces.value.findIndex((item) => item.id === workspace.id);
    if (index !== -1) {
      this._arrayMove(this.workspaces.value, index, 0);
    } else if (this.workspaces.value.unshift(workspace) > WORKSPACE_HISTORY_SIZE) {
      this.workspaces.value.pop();
    }
  }

  removeFile(file: RecentFile): void {
    const index = this.files.value.findIndex((item) => item.entryId === file.entryId);
    if (index !== -1) {
      this.files.value.splice(index, 1);
    }
  }

  removeFileById(entryId: EntryID): void {
    const index = this.files.value.findIndex((item) => item.entryId === entryId);
    if (index !== -1) {
      this.files.value.splice(index, 1);
    }
  }

  updateFile(entryId: EntryID, update: Partial<RecentFile>): void {
    const existingFile = this.files.value.find((item) => item.entryId === entryId);
    if (!existingFile) {
      return;
    }
    Object.assign(existingFile, update);
  }

  updateWorkspace(id: EntryID, update: Partial<RecentWorkspace>): void {
    const existingWorkspace = this.workspaces.value.find((item) => item.id === id);
    if (!existingWorkspace) {
      return;
    }
    Object.assign(existingWorkspace, update);
  }

  removeWorkspace(workspace: RecentWorkspace): void {
    const index = this.workspaces.value.findIndex((item) => item.id === workspace.id);
    if (index !== -1) {
      this.workspaces.value.splice(index, 1);
    }
  }

  resetHistory(): void {
    this.files.value = new Array<RecentFile>();
    this.workspaces.value = new Array<RecentWorkspace>();
  }

  getFiles(): Array<RecentFile> {
    return this.files.value;
  }

  getWorkspaces(): Array<RecentWorkspace> {
    return this.workspaces.value;
  }
}

const recentDocumentManager = new RecentDocumentManager();

export { recentDocumentManager, RecentFile, RecentWorkspace };
