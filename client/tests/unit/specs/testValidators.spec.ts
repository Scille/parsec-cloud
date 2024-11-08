// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { mockLibParsec } from '@tests/component/support/mocks';
import { describe, expect, it } from 'vitest';

mockLibParsec();

import { claimDeviceLinkValidator, claimLinkValidator, claimUserLinkValidator, organizationValidator } from '@/common/validators';
import { I18n } from 'megashark-lib';

// cspell:disable-next-line
const VALID_PAYLOAD = 'xBCqqqqqqqqqqqqqqqqqqqqq';

describe('Validators', () => {
  it('Validates organization name', async () => {
    const invalidNameResult = await organizationValidator('Org#');
    expect(I18n.translate(invalidNameResult.reason)).to.equal('Only letters, digits, underscores and hyphens. No spaces.');

    const nameTooLongResult = await organizationValidator('a'.repeat(33));
    expect(I18n.translate(nameTooLongResult.reason)).to.equal('Name is too long, limit is 32 characters.');
  });

  it.each([
    // cspell:disable-next-line
    [`http://host/org?a=claim_user&p=${VALID_PAYLOAD}`, "Link should start with 'parsec3://'."],
    [`parsec3://host/org?p=${VALID_PAYLOAD}`, 'Link does not include an action.'],
    [`parsec3://host/org?a=bootstrap_organization&p=${VALID_PAYLOAD}`, 'Link contains an invalid action.'],
    ['parsec3://host/org?a=claim_user', 'Link does not include a token.'],
    ['parsec3://host/org?a=claim_user&p=abcdefg', 'Link contains an invalid token.'],
  ])('Validates claim link', async (link: string, expected: string) => {
    const invalidProtocolResult = await claimLinkValidator(link);
    expect(I18n.translate(invalidProtocolResult.reason)).to.equal(expected);
  });

  it.each([
    // cspell:disable-next-line
    [`http://host/org?a=claim_user&p=${VALID_PAYLOAD}`, "Link should start with 'parsec3://'."],
    [`parsec3://host/org?p=${VALID_PAYLOAD}`, 'Link does not include an action.'],
    [`parsec3://host/org?a=claim_device&p=${VALID_PAYLOAD}`, 'Link contains an invalid action.'],
    ['parsec3://host/org?a=claim_user', 'Link does not include a token.'],
    ['parsec3://host/org?a=claim_user&p=abcdefg', 'Link contains an invalid token.'],
  ])('Validates claim user', async (link: string, expected: string) => {
    const invalidProtocolResult = await claimUserLinkValidator(link);
    expect(I18n.translate(invalidProtocolResult.reason)).to.equal(expected);
  });

  it.each([
    [`http://host/org?a=claim_device&p=${VALID_PAYLOAD}`, "Link should start with 'parsec3://'."],
    [`parsec3://host/org?p=${VALID_PAYLOAD}`, 'Link does not include an action.'],
    [`parsec3://host/org?a=claim_user&p=${VALID_PAYLOAD}`, 'Link contains an invalid action.'],
    ['parsec3://host/org?a=claim_device', 'Link does not include a token.'],
    ['parsec3://host/org?a=claim_device&p=abcdefg', 'Link contains an invalid token.'],
  ])('Validates claim device', async (link: string, expected: string) => {
    const invalidProtocolResult = await claimDeviceLinkValidator(link);
    expect(I18n.translate(invalidProtocolResult.reason)).to.equal(expected);
  });
});
