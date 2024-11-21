// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DetectedFileType } from '@/common/fileTypes';
import { EntryID, EntryName, FsPath, WorkspaceHandle } from '@/parsec';
import { Ref, ref } from 'vue';

interface RecentFile {
  entryId: EntryID;
  workspaceHandle: WorkspaceHandle;
  path: FsPath;
  name: EntryName;
  contentType?: DetectedFileType;
}

const HISTORY_SIZE = 5;

class RecentFileManager {
  files: Ref<Array<RecentFile>> = ref([]);

  constructor() {
    this.files.value = new Array<RecentFile>();
  }

  addFile(file: RecentFile): void {
    const exists = this.files.value.find((item) => item.entryId === file.entryId) !== undefined;
    if (exists) {
      return;
    }
    if (this.files.value.unshift(file) > HISTORY_SIZE) {
      this.files.value.pop();
    }
  }

  removeFile(file: RecentFile): void {
    const index = this.files.value.findIndex((item) => item.entryId === file.entryId);
    if (index !== -1) {
      this.files.value.splice(index, 1);
    }
  }

  resetHistory(): void {
    this.files.value = new Array<RecentFile>();
  }

  getFiles(): Array<RecentFile> {
    return this.files.value;
  }
}

const recentFileManager = new RecentFileManager();

export { RecentFile, recentFileManager };
