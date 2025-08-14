// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryID, EntryStatFile, EntryStatFolder, WorkspaceHistoryEntryStatFile, WorkspaceHistoryEntryStatFolder } from '@/parsec';
import { FileOperationData } from '@/services/fileOperationManager';
import { Translatable } from 'megashark-lib';

export enum SortProperty {
  Name,
  Size,
  LastUpdate,
  CreationDate,
}

export enum ImportType {
  Files,
  Folder,
}

export enum OpenFallbackChoice {
  Open,
  Download,
  View,
}

export interface FallbackCustomParams {
  title: Translatable;
  subtitle: Translatable;
  viewerOption: boolean;
}
export interface FileOperationProgress {
  data: FileOperationData;
  progress: number;
}

export interface FileModel extends EntryStatFile {
  isSelected: boolean;
}

export interface FolderModel extends EntryStatFolder {
  isSelected: boolean;
}

export type EntryModel = FileModel | FolderModel;

export class EntryCollection<Model extends EntryModel> {
  entries: Array<Model>;

  constructor() {
    this.entries = [];
  }

  hasSelected(): boolean {
    return this.entries.find((entry) => entry.isSelected) !== undefined;
  }

  selectAll(selected: boolean): void {
    for (const entry of this.entries) {
      entry.isSelected = selected;
    }
  }

  getEntries(): Array<Model> {
    return this.entries;
  }

  entriesCount(): number {
    return this.entries.length;
  }

  getSelectedEntries(): Array<Model> {
    return this.entries.filter((entry) => entry.isSelected);
  }

  selectedCount(): number {
    return this.entries.filter((entry) => entry.isSelected).length;
  }

  sort(property: SortProperty, ascending: boolean): void {
    this.entries.sort((item1, item2) => {
      // Because the difference between item1 and item2 will always be -1, 0 or 1, by setting
      // a folder with a score of 3 by default, we're ensuring that it will always be on top
      // of the list.
      const item1Score = item1.isFile() ? 3 : 0;
      const item2Score = item2.isFile() ? 3 : 0;
      let diff = 0;

      if (property === SortProperty.Name) {
        diff = ascending ? item2.name.localeCompare(item1.name) : item1.name.localeCompare(item2.name);
      } else if (property === SortProperty.LastUpdate) {
        diff = ascending ? (item1.updated > item2.updated ? 1 : 0) : item2.updated > item1.updated ? 1 : 0;
      } else if (property === SortProperty.CreationDate) {
        diff = ascending ? (item1.created > item2.created ? 1 : 0) : item2.created > item1.created ? 1 : 0;
      } else if (property === SortProperty.Size) {
        const size1 = item1.isFile() ? (item1 as FileModel).size : 0;
        const size2 = item1.isFile() ? (item2 as FileModel).size : 0;
        diff = ascending ? (size1 < size2 ? 1 : 0) : size2 < size1 ? 1 : 0;
      }
      return item1Score - item2Score - diff;
    });
  }

  clear(): void {
    this.entries = [];
  }

  append(entry: Model): void {
    this.entries.push(entry);
  }

  replace(entries: Model[]): void {
    this.clear();
    this.entries = entries;
  }

  smartUpdate(entries: Model[]): void {
    const toAdd: Model[] = [];
    const updated: EntryID[] = [];
    // First, iter on newly listed entries
    for (const entry of entries) {
      const existing = this.entries.find((e) => e.id === entry.id);
      if (existing) {
        // entry is already listed, updated it
        existing.baseVersion = entry.baseVersion;
        existing.confinementPoint = entry.confinementPoint;
        existing.isConfined = (): boolean => entry.confinementPoint !== null;
        existing.created = entry.created;
        existing.isSelected = entry.isSelected;
        existing.name = entry.name;
        existing.needSync = entry.needSync;
        existing.tag = entry.tag;
        existing.updated = entry.updated;
        existing.path = entry.path;
        if (existing.isFile()) {
          (existing as FileModel).size = (entry as FileModel).size;
        }
        updated.push(existing.id);
      } else {
        // entry is not yet listed, mark it as to be added
        toAdd.push(entry);
      }
    }
    // removing entries that are not newly listed
    for (const entry of this.entries.slice()) {
      const wasUpdated = updated.find((u) => u === entry.id);
      if (!wasUpdated) {
        // entry was not updated, delete it
        const index = this.entries.findIndex((e) => e.id === entry.id);
        if (index !== -1) {
          this.entries.splice(index, 1);
        }
      }
    }
    // add the missing entries
    for (const entry of toAdd) {
      this.entries.push(entry);
    }
  }
}

export interface WorkspaceHistoryFileModel extends WorkspaceHistoryEntryStatFile {
  isSelected: boolean;
}

export interface WorkspaceHistoryFolderModel extends WorkspaceHistoryEntryStatFolder {
  isSelected: boolean;
}

export type WorkspaceHistoryEntryModel = WorkspaceHistoryFileModel | WorkspaceHistoryFolderModel;

export class WorkspaceHistoryEntryCollection<Model extends WorkspaceHistoryEntryModel> {
  entries: Array<Model>;

  constructor() {
    this.entries = [];
  }

  hasSelected(): boolean {
    return this.entries.find((entry) => entry.isSelected) !== undefined;
  }

  selectAll(selected: boolean): void {
    for (const entry of this.entries) {
      entry.isSelected = selected;
    }
  }

  getEntries(): Array<Model> {
    return this.entries;
  }

  entriesCount(): number {
    return this.entries.length;
  }

  getSelectedEntries(): Array<Model> {
    return this.entries.filter((entry) => entry.isSelected);
  }

  selectedCount(): number {
    return this.entries.filter((entry) => entry.isSelected).length;
  }

  sort(property: SortProperty, ascending: boolean): void {
    this.entries.sort((item1, item2) => {
      // Because the difference between item1 and item2 will always be -1, 0 or 1, by setting
      // a folder with a score of 3 by default, we're ensuring that it will always be on top
      // of the list.
      const item1Score = item1.isFile() ? 3 : 0;
      const item2Score = item2.isFile() ? 3 : 0;
      let diff = 0;

      if (property === SortProperty.Name) {
        diff = ascending ? item2.name.localeCompare(item1.name) : item1.name.localeCompare(item2.name);
      } else if (property === SortProperty.LastUpdate) {
        diff = ascending ? (item1.updated > item2.updated ? 1 : 0) : item2.updated > item1.updated ? 1 : 0;
      } else if (property === SortProperty.Size) {
        const size1 = item1.isFile() ? (item1 as WorkspaceHistoryFileModel).size : 0;
        const size2 = item1.isFile() ? (item2 as WorkspaceHistoryFileModel).size : 0;
        diff = ascending ? (size1 < size2 ? 1 : 0) : size2 < size1 ? 1 : 0;
      }
      return item1Score - item2Score - diff;
    });
  }

  clear(): void {
    this.entries = [];
  }

  append(entry: Model): void {
    this.entries.push(entry);
  }

  replace(entries: Model[]): void {
    this.clear();
    this.entries = entries;
  }
}
