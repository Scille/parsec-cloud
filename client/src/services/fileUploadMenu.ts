// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { ref } from 'vue';

export enum UploadMenuState {
  Open = 'open',
  Collapsed = 'collapsed',
  Floating = 'floating',
}

const _isVisible = ref(false);
const _state = ref<UploadMenuState>(UploadMenuState.Floating);

interface UploadMenuControl {
  show: () => void;
  hide: () => void;
  open: () => void;
  collapse: () => void;
  minimize: () => void;
  isVisible: () => boolean;
  getState: () => UploadMenuState;
}

export default function useUploadMenu(): UploadMenuControl {
  function show(): void {
    _isVisible.value = true;
  }

  function hide(): void {
    _isVisible.value = false;
  }

  function open(): void {
    _state.value = UploadMenuState.Open;
  }

  function collapse(): void {
    _state.value = UploadMenuState.Collapsed;
  }

  function minimize(): void {
    _state.value = UploadMenuState.Floating;
  }

  function isVisible(): boolean {
    return _isVisible.value;
  }

  function getState(): UploadMenuState {
    return _state.value;
  }

  return {
    show,
    hide,
    open,
    collapse,
    minimize,
    isVisible,
    getState,
  };
}
