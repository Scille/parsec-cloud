// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/*
Format the specified number of bytes with the corresponding system.

More specifically:
- `0 <= bytes < 10`:          1 significant figures:     `X B`
- `10 <= bytes < 100`:        2 significant figures:    `XY B`
- `100 <= bytes < 1000`:      3 significant figures:   `XYZ B`
- `1000 <= bytes < 1024`:     2 significant figures: `0.9X KB`
- `1 <= kilobytes < 10`:      3 significant figures: `X.YZ KB`
- `10 <= kilobytes < 100`:    3 significant figures: `XY.Z KB`
- `100 <= kilobytes < 1000`:  3 significant figures:  `XYZ KB`
- `1000 <= kilobytes < 1024`: 2 significant figures: `0.9X MB`
- And so on for MB, GB and TB
*/

import { ComposerTranslation } from 'vue-i18n';

function size(bytes: number, system: [number, string][]): string {

  if (bytes < 0) {
    throw Error('Bytes must be >= 0');
  }

  // Iterate over factors, expecting them to be in increasing order
  let formattedAmount = '';
  let factor = 1;
  let suffix = '';
  for (const item of system) {
    // Stop when the right factor is reached
    factor = item[0];
    suffix = item[1];
    if (bytes / factor < 999.5) {
      break;
    }
  }
  // Convert to the right unit
  let amount = bytes / factor;
  // Truncate to two decimals in order to avoid misleading rounding to 1.00
  amount = Math.trunc(amount * 100) / 100;
  // Factor is one, the amount is an integer
  if (factor === 1) {
    formattedAmount = amount.toFixed();
  } else if (amount < 10.0) {
    formattedAmount = amount.toFixed(2);
  } else if (amount < 99.95) {
    formattedAmount = amount.toFixed(1);
  } else {
    formattedAmount = amount.toFixed();
  }
  return `${formattedAmount} ${suffix}`;
}

export function formatFileSize(bytesize: number, t: ComposerTranslation): string {
  const SYSTEM: [number, string][] = [
    [Math.pow(1024, 0), t('common.filesize.bytes')],
    [Math.pow(1024, 1), t('common.filesize.kilobytes')],
    [Math.pow(1024, 2), t('common.filesize.megabytes')],
    [Math.pow(1024, 3), t('common.filesize.gigabytes')],
    [Math.pow(1024, 4), t('common.filesize.terabytes')]
  ];
  return size(bytesize, SYSTEM);
}
