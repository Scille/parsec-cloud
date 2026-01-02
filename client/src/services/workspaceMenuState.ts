// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { WorkspaceMenu } from '@/views/workspaces/types';
import { ref } from 'vue';

const workspaceMenuState = ref<WorkspaceMenu>(WorkspaceMenu.All);

export function setWorkspaceMenu(menu: WorkspaceMenu): void {
  workspaceMenuState.value = menu;
}

export function getWorkspaceMenu(): WorkspaceMenu {
  return workspaceMenuState.value;
}
