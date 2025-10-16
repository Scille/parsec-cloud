// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isWeb } from '@/parsec';

function beforeUnload(event: BeforeUnloadEvent): void {
  event.preventDefault();
  event.returnValue = true;
}

let controller: undefined | AbortController = undefined;

export default function useRefreshWarning(): any {
  function warnOnRefresh(): void {
    if (window.isDev() || !isWeb()) {
      return;
    }
    if (!controller) {
      controller = new AbortController();
      window.addEventListener('beforeunload', beforeUnload, { signal: controller.signal });
    }
  }

  function removeWarning(): void {
    if (controller) {
      controller.abort();
    }
    controller = undefined;
  }

  function isWarningActive(): boolean {
    return controller !== undefined;
  }

  return {
    warnOnRefresh,
    isWarningActive,
    removeWarning,
  };
}
