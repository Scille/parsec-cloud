// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ref } from 'vue';

const defaultWidth = 300;
const hiddenWidth = 0;
const computedWidth = ref<number>(defaultWidth);
const storedWidth = ref<number>(defaultWidth);

export default function useSidebarMenu(): any {
  function isVisible(): boolean {
    return computedWidth.value > 4;
  }

  function reset(): void {
    computedWidth.value = storedWidth.value;
  }

  function hide(): void {
    storedWidth.value = computedWidth.value;
    computedWidth.value = hiddenWidth;
  }

  return {
    computedWidth,
    storedWidth,
    isVisible,
    reset,
    hide,
  };
}
