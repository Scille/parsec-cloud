// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { WorkspaceMenu } from '@/views/workspaces/types';
import { ref } from 'vue';

export const workspaceMenuState = ref<WorkspaceMenu>(WorkspaceMenu.All);
