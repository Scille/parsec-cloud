// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { zxcvbn, zxcvbnOptions } from '@zxcvbn-ts/core';
import * as zxcvbnCommonPackage from '@zxcvbn-ts/language-common';
import { ComposerTranslation } from 'vue-i18n';

export enum PasswordStrength {
  None = 0,
  Low = 1,
  Medium = 2,
  High = 3,
}

const OPTIONS = {
  graphs: zxcvbnCommonPackage.adjacencyGraphs,
  dictionary: {
    ...zxcvbnCommonPackage.dictionary,
  },
};

zxcvbnOptions.setOptions(OPTIONS);

export function getPasswordStrength(password: string): PasswordStrength {
  if (password.length <= 0) {
    return PasswordStrength.None;
  }
  const result = zxcvbn(password);

  if (result.score === 0 || result.score === 1) {
    return PasswordStrength.Low;
  } else if (result.score === 2 || result.score === 3) {
    return PasswordStrength.Medium;
  }
  return PasswordStrength.High;
}

export function getPasswordStrengthText(t: ComposerTranslation, strength: PasswordStrength): string {
  if (strength === PasswordStrength.Low) {
    return t('Password.passwordLevelLow');
  } else if (strength === PasswordStrength.Medium) {
    return t('Password.passwordLevelMedium');
  } else if (strength === PasswordStrength.High) {
    return t('Password.passwordLevelHigh');
  }
  return '';
}
