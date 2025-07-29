// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ref } from 'vue';

const isVisible = ref<boolean>(true);

export default function useHeaderControl(): any {
  function showHeader(): void {
    isVisible.value = true;
  }

  function hideHeader(): void {
    isVisible.value = false;
  }

  function toggleHeader(): void {
    isVisible.value = !isVisible.value;
  }

  function isHeaderVisible(): boolean {
    return isVisible.value;
  }

  return {
    showHeader,
    hideHeader,
    toggleHeader,
    isHeaderVisible,
  };
}
