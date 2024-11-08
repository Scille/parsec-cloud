// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { I18n, PasswordValidation } from 'megashark-lib';
import { describe, expect, it } from 'vitest';

describe('Password validation', () => {
  it('Checks password strength', () => {
    expect(PasswordValidation.getStrength('').level).to.equal(PasswordValidation.StrengthLevel.None);
    expect(PasswordValidation.getStrength('Black').level).to.equal(PasswordValidation.StrengthLevel.Low);
    expect(PasswordValidation.getStrength('BlackMesa').level).to.equal(PasswordValidation.StrengthLevel.Medium);
    expect(PasswordValidation.getStrength('BlackMesaIncident').level).to.equal(PasswordValidation.StrengthLevel.High);
    // Long, but same letter
    expect(PasswordValidation.getStrength('aaaaaaaaaaaaaaaaaaaaaaaaaaaa').level).to.equal(PasswordValidation.StrengthLevel.Low);
    // Sequences
    // cspell:disable-next
    expect(PasswordValidation.getStrength('qwertyuiop').level).to.equal(PasswordValidation.StrengthLevel.Low);
    // cspell:disable-next
    expect(PasswordValidation.getStrength('abcdefghijklmnopqrstuvwxyz').level).to.equal(PasswordValidation.StrengthLevel.Low);
  });

  it('Checks password strength texts', () => {
    expect(I18n.translate(PasswordValidation.getStrength('').label)).to.equal('');
    expect(I18n.translate(PasswordValidation.getStrength('Black').label)).to.equal('Low');
    expect(I18n.translate(PasswordValidation.getStrength('BlackMesa').label)).to.equal('Moderate');
    expect(I18n.translate(PasswordValidation.getStrength('BlackMesaIncident').label)).to.equal('Strong');
  });
});
