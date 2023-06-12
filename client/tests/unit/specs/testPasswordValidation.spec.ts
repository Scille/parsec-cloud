// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { getPasswordStrength, getPasswordStrengthText, PasswordStrength } from '@/common/passwordValidation';

describe('Password validation', () => {
  it('Checks password strength', () => {
    expect(getPasswordStrength('')).to.equal(PasswordStrength.None);
    expect(getPasswordStrength('Black')).to.equal(PasswordStrength.Low);
    expect(getPasswordStrength('BlackMesa')).to.equal(PasswordStrength.Medium);
    expect(getPasswordStrength('BlackMesaIncident')).to.equal(PasswordStrength.High);
    // Long, but same letter
    expect(getPasswordStrength('aaaaaaaaaaaaaaaaaaaaaaaaaaaa')).to.equal(PasswordStrength.Low);
    // Sequences
    // cspell:disable-next
    expect(getPasswordStrength('qwertyuiop')).to.equal(PasswordStrength.Low);
    // cspell:disable-next
    expect(getPasswordStrength('abcdefghijklmnopqrstuvwxyz')).to.equal(PasswordStrength.Low);
  });

  it('Checks password strength texts', () => {
    function t(s: string): string {
      return s;
    }

    expect(getPasswordStrengthText(t, PasswordStrength.None)).to.equal('');
    expect(getPasswordStrengthText(t, PasswordStrength.Low)).to.equal('Password.passwordLevelLow');
    expect(getPasswordStrengthText(t, PasswordStrength.Medium)).to.equal('Password.passwordLevelMedium');
    expect(getPasswordStrengthText(t, PasswordStrength.High)).to.equal('Password.passwordLevelHigh');
  });
});
