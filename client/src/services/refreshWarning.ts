// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { isWeb } from '@/parsec';
import { ref } from 'vue';

const isActive = ref(false);

export default function useRefreshWarning(): any {
  function warnOnRefresh(): void {
    if (window.isDev() || !isWeb()) {
      return;
    }
    if (!isActive.value) {
      window.addEventListener('beforeunload', (event: BeforeUnloadEvent) => {
        event.preventDefault();
        event.returnValue = true;
      });
      isActive.value = true;
    }
  }

  function isWarningActive(): boolean {
    return isActive.value;
  }

  return {
    warnOnRefresh,
    isWarningActive,
  };
}
