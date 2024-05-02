// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ref } from 'vue';

const defaultWidth = 300;
const initialWidth = ref<number>(defaultWidth);
const computedWidth = ref<number>(defaultWidth);

export default function useSidebarMenu(): any {
  function isVisible(): boolean {
    return computedWidth.value > 4;
  }

  function reset(): void {
    initialWidth.value = defaultWidth;
    computedWidth.value = defaultWidth;
  }

  return {
    defaultWidth,
    initialWidth,
    computedWidth,
    isVisible,
    reset,
  };
}
