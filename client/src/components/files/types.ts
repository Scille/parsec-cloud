// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryStatFile, EntryStatFolder } from '@/parsec';
import { ImportData } from '@/services/importManager';

export enum SortProperty {
  Name,
  Size,
  LastUpdate,
}

export interface FileImportProgress {
  data: ImportData;
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
}
