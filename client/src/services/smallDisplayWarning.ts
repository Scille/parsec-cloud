// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { useWindowSize } from 'megashark-lib';
import { onMounted, onUnmounted, watch } from 'vue';

// Global, will only reset on page refresh
let warnedAboutDisplay = false;

/** Display warning dialog on small screens
 *
 * The warning is shown only once per refresh and it is also shown if the user
 * reduces the size of the window.
 */
export function useSmallDisplayWarning(infoManager?: InformationManager): any {
  const { isSmallDisplay, windowWidth } = useWindowSize();
  let informationManager: InformationManager | undefined = infoManager;

  const watchCancel = watch(windowWidth, async () => {
    await checkSmallDisplay();
  });

  function setInformationManager(infoManager: InformationManager): void {
    informationManager = infoManager;
  }

  async function checkSmallDisplay(): Promise<void> {
    if (!window.usesTestbed() && isSmallDisplay.value && !warnedAboutDisplay) {
      warnedAboutDisplay = true;
      window.electronAPI.log('warn', 'Small display detected, warning the user...');
      if (informationManager) {
        await informationManager.present(
          new Information({
            message: 'globalErrors.smallDisplayWarning',
            level: InformationLevel.Warning,
          }),
          PresentationMode.Modal,
        );
      }
    }
  }

  onMounted(async () => {
    await checkSmallDisplay();
  });

  onUnmounted(() => {
    watchCancel();
  });

  return { setInformationManager };
}
