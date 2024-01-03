// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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
    expect(getPasswordStrengthText(PasswordStrength.None)).to.equal('');
    expect(getPasswordStrengthText(PasswordStrength.Low)).to.equal('Low');
    expect(getPasswordStrengthText(PasswordStrength.Medium)).to.equal('Moderate');
    expect(getPasswordStrengthText(PasswordStrength.High)).to.equal('Strong');
  });
});
