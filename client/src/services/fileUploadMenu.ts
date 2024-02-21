// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ref } from 'vue';

const _isVisible = ref(false);
const _isMinimized = ref(true);

interface UploadMenuControl {
  show: () => void;
  hide: () => void;
  expand: () => void;
  minimize: () => void;
  isVisible: () => boolean;
  isMinimized: () => boolean;
}

export default function useUploadMenu(): UploadMenuControl {
  function show(): void {
    _isVisible.value = true;
  }

  function hide(): void {
    _isVisible.value = false;
  }

  function expand(): void {
    _isMinimized.value = false;
  }

  function minimize(): void {
    _isMinimized.value = true;
  }

  function isVisible(): boolean {
    return _isVisible.value;
  }

  function isMinimized(): boolean {
    return _isMinimized.value;
  }

  return {
    show,
    hide,
    expand,
    minimize,
    isVisible,
    isMinimized,
  };
}
