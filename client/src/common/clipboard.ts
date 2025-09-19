// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { Clipboard, Translatable } from 'megashark-lib';

export async function copyToClipboard(
  value: string,
  informationManager: InformationManager,
  successMessage: Translatable,
  errorMessage: Translatable,
): Promise<void> {
  const result = await Clipboard.writeText(value);
  if (result) {
    informationManager.present(
      new Information({
        message: successMessage,
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: errorMessage,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}
