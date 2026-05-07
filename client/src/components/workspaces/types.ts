// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { WorkspaceID } from '@/parsec';
import { DisplayState, Translatable } from 'megashark-lib';

export const WORKSPACES_PAGE_DATA_KEY = 'WorkspacesPage';

export interface RoleUpdateAuthorization {
  authorized: boolean;
  reason?: Translatable;
}

export interface WorkspacesPageSavedData {
  displayState?: DisplayState;
  favoriteList?: WorkspaceID[];
  hiddenList?: WorkspaceID[];
}

export interface WorkspacesPageFilters {
  owner: boolean;
  manager: boolean;
  contributor: boolean;
  reader: boolean;
}

export const WorkspaceDefaultData: Required<WorkspacesPageSavedData> = {
  displayState: DisplayState.Grid,
  favoriteList: [],
  hiddenList: [],
};
