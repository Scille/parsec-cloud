// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  Validity,
  userNameValidator,
  organizationValidator,
  deviceNameValidator,
  emailValidator,
  claimLinkValidator,
  claimUserLinkValidator,
  claimDeviceLinkValidator,
  backendAddrValidator,
} from '@/common/validators';
import { it } from 'vitest';

describe('Validators', () => {
  it('Check user name validator', async () => {
    expect(await userNameValidator('')).to.equal(Validity.Intermediate);
    expect(await userNameValidator('    ')).to.equal(Validity.Intermediate);
    expect(await userNameValidator('a')).to.equal(Validity.Valid);
    expect(await userNameValidator('  a  ')).to.equal(Validity.Valid);
    expect(await userNameValidator('a'.repeat(129))).to.equal(Validity.Invalid);
    expect(await userNameValidator(`  ${'a'.repeat(126)}   `)).to.equal(Validity.Valid);
  });

  it('Check organization validator', async () => {
    expect(await organizationValidator('')).to.equal(Validity.Intermediate);
    expect(await organizationValidator('    ')).to.equal(Validity.Intermediate);
    expect(await organizationValidator('a')).to.equal(Validity.Valid);
    expect(await organizationValidator('  a  ')).to.equal(Validity.Valid);
    expect(await organizationValidator('a'.repeat(33))).to.equal(Validity.Invalid);
    expect(await organizationValidator(`  ${'a'.repeat(31)}   `)).to.equal(Validity.Valid);
    expect(await organizationValidator('a0_-1')).to.equal(Validity.Valid);
    expect(await organizationValidator('aa*aa')).to.equal(Validity.Invalid);
  });

  it('Check device name validator', async () => {
    expect(await deviceNameValidator('')).to.equal(Validity.Intermediate);
    expect(await deviceNameValidator('    ')).to.equal(Validity.Intermediate);
    expect(await deviceNameValidator('a')).to.equal(Validity.Valid);
    expect(await deviceNameValidator('  a  ')).to.equal(Validity.Valid);
    expect(await deviceNameValidator('a'.repeat(33))).to.equal(Validity.Invalid);
    expect(await deviceNameValidator(`  ${'a'.repeat(31)}   `)).to.equal(Validity.Valid);
    expect(await deviceNameValidator('a0_-1')).to.equal(Validity.Valid);
    expect(await deviceNameValidator('aa*aa')).to.equal(Validity.Invalid);
  });

  it('Check email validator', async () => {
    expect(await emailValidator('')).to.equal(Validity.Intermediate);
    expect(await emailValidator('    ')).to.equal(Validity.Intermediate);
    expect(await emailValidator('a@b.c')).to.equal(Validity.Valid);
    expect(await emailValidator('  a@b.c  ')).to.equal(Validity.Valid);
    expect(await emailValidator('aaaaaa')).to.equal(Validity.Intermediate);
    expect(await emailValidator('@aa')).to.equal(Validity.Invalid);
    expect(await emailValidator('a@a')).to.equal(Validity.Valid);
  });

  it('Check backend addr validator', async () => {
    expect(await backendAddrValidator('')).to.equal(Validity.Intermediate);
    expect(await backendAddrValidator('    ')).to.equal(Validity.Intermediate);
    expect(await backendAddrValidator('parsec://host:1337')).to.equal(Validity.Valid);
    expect(await backendAddrValidator('    parsec://host:1337   ')).to.equal(Validity.Valid);
    expect(await backendAddrValidator('parsec://host:1337?no_ssl=true')).to.equal(Validity.Valid);
    // Invalid scheme
    expect(await backendAddrValidator('http://host:1337?no_ssl=true')).to.equal(Validity.Invalid);
    // Unexpected path
    expect(await backendAddrValidator('http://host:1337/path?no_ssl=true')).to.equal(Validity.Invalid);
  });

  it.each([['claim_user'], ['claim_device']])('Check claim link validator', async (action) => {
    expect(await claimLinkValidator('')).to.equal(Validity.Intermediate);
    expect(await claimLinkValidator('    ')).to.equal(Validity.Intermediate);
    expect(await claimLinkValidator(`parsec://host:1337/org?action=${action}&token=${'a'.repeat(32)}`)).to.equal(Validity.Valid);
    expect(await claimLinkValidator(`    parsec://host:1337/org?action=${action}&token=${'a'.repeat(32)}   `)).to.equal(Validity.Valid);
    // Invalid org
    expect(await claimLinkValidator(`parsec://host:1337/invalid/org?action=${action}&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing org
    expect(await claimLinkValidator(`parsec://host:1337?action=${action}&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing action
    expect(await claimLinkValidator(`parsec://host:1337/org?token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Invalid action
    expect(await claimLinkValidator(`parsec://host:1337/org?action=something_else&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing token
    expect(await claimLinkValidator(`parsec://host:1337?/org?action=${action}`)).to.equal(Validity.Invalid);
    // Invalid token
    expect(await claimLinkValidator(`parsec://host:1337/org?action=${action}&token=${'z'.repeat(33)}`)).to.equal(Validity.Invalid);
    // Invalid scheme
    expect(await claimLinkValidator(`http://host:1337/org?action=${action}&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
  });

  it('Check claim user link validator', async () => {
    expect(await claimUserLinkValidator('')).to.equal(Validity.Intermediate);
    expect(await claimUserLinkValidator('    ')).to.equal(Validity.Intermediate);
    expect(await claimUserLinkValidator(`parsec://host:1337/org?action=claim_user&token=${'a'.repeat(32)}`)).to.equal(Validity.Valid);
    expect(
      await claimUserLinkValidator(`    parsec://host:1337/org?action=claim_user&token=${'a'.repeat(32)}   `),
    ).to.equal(Validity.Valid);
    // Invalid org
    expect(
      await claimUserLinkValidator(`parsec://host:1337/invalid/org?action=claim_user&token=${'a'.repeat(32)}`),
    ).to.equal(Validity.Invalid);
    // Missing org
    expect(await claimUserLinkValidator(`parsec://host:1337?action=claim_user&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing action
    expect(await claimUserLinkValidator(`parsec://host:1337/org?token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Invalid action
    expect(await claimUserLinkValidator(`parsec://host:1337/org?action=something_else&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing token
    expect(await claimUserLinkValidator('parsec://host:1337?/org?action=claim_user')).to.equal(Validity.Invalid);
    // Invalid token
    expect(await claimUserLinkValidator(`parsec://host:1337/org?action=claim_user&token=${'z'.repeat(33)}`)).to.equal(Validity.Invalid);
    // Invalid scheme
    expect(await claimUserLinkValidator(`http://host:1337/org?action=claim_user&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Claim device instead
    expect(await claimUserLinkValidator(`parsec://host:1337/org?action=claim_device&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
  });

  it('Check claim device link validator', async () => {
    expect(await claimDeviceLinkValidator('')).to.equal(Validity.Intermediate);
    expect(await claimDeviceLinkValidator('    ')).to.equal(Validity.Intermediate);
    expect(await claimDeviceLinkValidator(`parsec://host:1337/org?action=claim_device&token=${'a'.repeat(32)}`)).to.equal(Validity.Valid);
    expect(claimDeviceLinkValidator(`    parsec://host:1337/org?action=claim_device&token=${'a'.repeat(32)}   `)).to.equal(Validity.Valid);
    // Invalid org
    // eslint-disable-next-line max-len
    expect(await claimDeviceLinkValidator(`parsec://host:1337/invalid/org?action=claim_device&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing org
    expect(await claimDeviceLinkValidator(`parsec://host:1337?action=claim_device&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Missing action
    expect(await claimDeviceLinkValidator(`parsec://host:1337/org?token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Invalid action
    expect(
      await claimDeviceLinkValidator(`parsec://host:1337/org?action=something_else&token=${'a'.repeat(32)}`),
    ).to.equal(Validity.Invalid);
    // Missing token
    expect(await claimDeviceLinkValidator('parsec://host:1337?/org?action=claim_device')).to.equal(Validity.Invalid);
    // Invalid token
    expect(await claimDeviceLinkValidator(`parsec://host:1337/org?action=claim_device&token=${'z'.repeat(33)}`)).to.equal(Validity.Invalid);
    // Invalid scheme
    expect(await claimDeviceLinkValidator(`http://host:1337/org?action=claim_device&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
    // Claim user instead
    expect(await claimDeviceLinkValidator(`parsec://host:1337/org?action=claim_user&token=${'a'.repeat(32)}`)).to.equal(Validity.Invalid);
  });
});
