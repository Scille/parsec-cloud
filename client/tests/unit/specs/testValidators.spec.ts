// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  Validity,
  userNameValidator,
  organizationValidator,
  deviceNameValidator,
  emailValidator,
  claimUserLinkValidator,
  backendAddrValidator
} from '@/common/validators';

describe('Validators', () => {
  it('Check user name validator', async () => {
    expect(userNameValidator('')).to.equal(Validity.Intermediate);
    expect(userNameValidator('    ')).to.equal(Validity.Intermediate);
    expect(userNameValidator('a')).to.equal(Validity.Valid);
    expect(userNameValidator('  a  ')).to.equal(Validity.Valid);
    expect(userNameValidator('a'.repeat(129))).to.equal(Validity.Invalid);
    expect(userNameValidator(`  ${'a'.repeat(126)}   `)).to.equal(Validity.Valid);
  });

  it('Check organization validator', async () => {
    expect(organizationValidator('')).to.equal(Validity.Intermediate);
    expect(organizationValidator('    ')).to.equal(Validity.Intermediate);
    expect(organizationValidator('a')).to.equal(Validity.Valid);
    expect(organizationValidator('  a  ')).to.equal(Validity.Valid);
    expect(organizationValidator('a'.repeat(33))).to.equal(Validity.Invalid);
    expect(organizationValidator(`  ${'a'.repeat(31)}   `)).to.equal(Validity.Valid);
    expect(organizationValidator('a0_-1')).to.equal(Validity.Valid);
    expect(organizationValidator('aa*aa')).to.equal(Validity.Invalid);
  });

  it('Check device name validator', async () => {
    expect(deviceNameValidator('')).to.equal(Validity.Intermediate);
    expect(deviceNameValidator('    ')).to.equal(Validity.Intermediate);
    expect(deviceNameValidator('a')).to.equal(Validity.Valid);
    expect(deviceNameValidator('  a  ')).to.equal(Validity.Valid);
    expect(deviceNameValidator('a'.repeat(33))).to.equal(Validity.Invalid);
    expect(deviceNameValidator(`  ${'a'.repeat(31)}   `)).to.equal(Validity.Valid);
    expect(deviceNameValidator('a0_-1')).to.equal(Validity.Valid);
    expect(deviceNameValidator('aa*aa')).to.equal(Validity.Invalid);
  });

  it('Check email validator', async () => {
    expect(emailValidator('')).to.equal(Validity.Intermediate);
    expect(emailValidator('    ')).to.equal(Validity.Intermediate);
    expect(emailValidator('a@b.c')).to.equal(Validity.Valid);
    expect(emailValidator('  a@b.c  ')).to.equal(Validity.Valid);
    expect(emailValidator('aaaaaa')).to.equal(Validity.Intermediate);
    expect(emailValidator('@aa')).to.equal(Validity.Invalid);
    expect(emailValidator('a@a')).to.equal(Validity.Valid);
  });

  it('Check backend addr validator', async () => {
    expect(backendAddrValidator('')).to.equal(Validity.Intermediate);
    expect(backendAddrValidator('    ')).to.equal(Validity.Intermediate);
    expect(backendAddrValidator('parsec://host:1337')).to.equal(Validity.Valid);
    expect(backendAddrValidator('    parsec://host:1337   ')).to.equal(Validity.Valid);
    expect(backendAddrValidator('parsec://host:1337?no_ssl=true')).to.equal(Validity.Valid);
    // Invalid scheme
    expect(backendAddrValidator('http://host:1337?no_ssl=true')).to.equal(Validity.Invalid);
    // Unexpected path
    expect(backendAddrValidator('http://host:1337/path?no_ssl=true')).to.equal(Validity.Invalid);
  });

  it('Check claim user link validator', async () => {
    expect(claimUserLinkValidator('')).to.equal(Validity.Intermediate);
    expect(claimUserLinkValidator('    ')).to.equal(Validity.Intermediate);
    expect(claimUserLinkValidator(`parsec://host:1337/org?action=claim_user&token=${'a'.repeat(32)}`)).to.equal(Validity.Valid);
    expect(claimUserLinkValidator(`    parsec://host:1337/org?action=claim_user&token=${'a'.repeat(32)}   `)).to.equal(Validity.Valid);
    // Invalid org
    expect(claimUserLinkValidator(`parsec://host:1337/invalid/org?action=claim_user&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing org
    expect(claimUserLinkValidator(`parsec://host:1337?action=claim_user&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing action
    expect(claimUserLinkValidator(`parsec://host:1337/org?token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Invalid action
    expect(claimUserLinkValidator(`parsec://host:1337/org?action=something_else&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing token
    expect(claimUserLinkValidator('parsec://host:1337?/org?action=claim_user')).to.equal(Validity.Invalid);
    // Invalid token
    expect(claimUserLinkValidator(`parsec://host:1337/org?action=claim_user&token=${'z'.repeat(33)}`)).to.equal(Validity.Invalid);
    // Invalid scheme
    expect(claimUserLinkValidator(`http://host:1337/org?action=claim_user&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
  });
});
