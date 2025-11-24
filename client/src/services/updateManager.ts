// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ref, Ref } from 'vue';

const updatePromptSuppressed: Ref<boolean> = ref(false);

export function useUpdateManager() {
  function isUpdatePromptAllowed(): boolean {
    return !updatePromptSuppressed.value;
  }

  function suppressUpdatePrompt(): void {
    updatePromptSuppressed.value = true;
  }

  return {
    isUpdatePromptAllowed,
    suppressUpdatePrompt,
  };
}
