// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { WorkspaceDefaultData, WORKSPACES_PAGE_DATA_KEY, WorkspacesPageSavedData } from '@/components/workspaces';
import { WorkspaceID } from '@/parsec';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { inject, Ref, ref, toRaw } from 'vue';

const favorites = ref<Array<WorkspaceID>>([]);
const hidden = ref<Array<WorkspaceID>>([]);

export interface WorkspaceAttributes {
  getFavorites: () => Ref<Array<WorkspaceID>>;
  getHidden: () => Ref<Array<WorkspaceID>>;
  isFavorite: (id: WorkspaceID) => boolean;
  addFavorite: (id: WorkspaceID) => void;
  removeFavorite: (id: WorkspaceID) => void;
  toggleFavorite: (id: WorkspaceID) => void;
  isHidden: (id: WorkspaceID) => boolean;
  addHidden: (id: WorkspaceID) => void;
  removeHidden: (id: WorkspaceID) => void;
  toggleHidden: (id: WorkspaceID) => void;
  save: () => Promise<void>;
  load: () => Promise<void>;
}

export function useWorkspaceAttributes(): WorkspaceAttributes {
  const storageManager: StorageManager = inject(StorageManagerKey)!;

  function getFavorites(): Ref<Array<WorkspaceID>> {
    return favorites;
  }

  function addFavorite(id: WorkspaceID): void {
    if (!favorites.value.includes(id)) {
      favorites.value.push(id);
    }
  }

  function removeFavorite(id: WorkspaceID): void {
    favorites.value.splice(favorites.value.indexOf(id), 1);
  }

  function toggleFavorite(id: WorkspaceID): void {
    if (isFavorite(id)) {
      removeFavorite(id);
    } else {
      addFavorite(id);
    }
  }

  function isFavorite(id: WorkspaceID): boolean {
    return favorites.value.includes(id);
  }

  function getHidden(): Ref<Array<WorkspaceID>> {
    return hidden;
  }

  function addHidden(id: WorkspaceID): void {
    if (!hidden.value.includes(id)) {
      hidden.value.push(id);
    }
  }

  function removeHidden(id: WorkspaceID): void {
    hidden.value.splice(hidden.value.indexOf(id), 1);
  }

  function toggleHidden(id: WorkspaceID): void {
    if (isHidden(id)) {
      removeHidden(id);
    } else {
      addHidden(id);
    }
  }

  function isHidden(id: WorkspaceID): boolean {
    return hidden.value.includes(id);
  }

  async function save(): Promise<void> {
    await storageManager.updateComponentData<WorkspacesPageSavedData>(
      WORKSPACES_PAGE_DATA_KEY,
      { favoriteList: toRaw(favorites.value), hiddenList: toRaw(hidden.value) },
      WorkspaceDefaultData,
    );
  }

  async function load(): Promise<void> {
    const workspaceData = await storageManager.retrieveComponentData<WorkspacesPageSavedData>(
      WORKSPACES_PAGE_DATA_KEY,
      WorkspaceDefaultData,
    );
    favorites.value = workspaceData.favoriteList;
    hidden.value = workspaceData.hiddenList;
  }

  return {
    getFavorites,
    getHidden,
    isFavorite,
    addFavorite,
    removeFavorite,
    toggleFavorite,
    isHidden,
    addHidden,
    removeHidden,
    toggleHidden,
    save,
    load,
  };
}
