// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ref } from 'vue';

const isHeaderVisible = ref<boolean>(true);

export default function useHeaderControl(): any {
  function showHeader(): void {
    isHeaderVisible.value = true;
  }

  function hideHeader(): void {
    isHeaderVisible.value = false;
  }

  function toggleHeader(): void {
    isHeaderVisible.value = !isHeaderVisible.value;
  }

  function isVisible(): boolean {
    return isHeaderVisible.value;
  }

  return {
    showHeader,
    hideHeader,
    toggleHeader,
    isVisible,
  };
}
