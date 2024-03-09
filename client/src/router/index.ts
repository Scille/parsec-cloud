// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export * from '@/router/checks';
export * from '@/router/navigation';
export {
  getConnectionHandle,
  getCurrentRouteName,
  getCurrentRouteParams,
  getCurrentRouteQuery,
  getDocumentPath,
  getRoutePath,
  getWorkspaceHandle,
} from '@/router/params';
export * from '@/router/types';
export { watchOrganizationSwitch, watchRoute } from '@/router/watchers';
