// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { translate } from '@/services/translation';
import { mockValidators } from '@tests/component/support/mocks';
import { it } from 'vitest';

mockValidators();

import { claimDeviceLinkValidator, claimLinkValidator, claimUserLinkValidator, organizationValidator } from '@/common/validators';

// cspell:disable-next-line
const VALID_PAYLOAD = 'xBCqqqqqqqqqqqqqqqqqqqqq';

describe('Validators', () => {
  it('Validates organization name', async () => {
    const invalidNameResult = await organizationValidator('Org#');
    expect(translate(invalidNameResult.reason)).to.equal('Only letters, digits, underscores and hyphens. No spaces.');

    const nameTooLongResult = await organizationValidator('a'.repeat(33));
    expect(translate(nameTooLongResult.reason)).to.equal('Name is too long, limit is 32 characters.');
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
    expect(translate(invalidProtocolResult.reason)).to.equal(expected);
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
    expect(translate(invalidProtocolResult.reason)).to.equal(expected);
  });

  it.each([
    [`http://host/org?a=claim_device&p=${VALID_PAYLOAD}`, "Link should start with 'parsec3://'."],
    [`parsec3://host/org?p=${VALID_PAYLOAD}`, 'Link does not include an action.'],
    [`parsec3://host/org?a=claim_user&p=${VALID_PAYLOAD}`, 'Link contains an invalid action.'],
    ['parsec3://host/org?a=claim_device', 'Link does not include a token.'],
    ['parsec3://host/org?a=claim_device&p=abcdefg', 'Link contains an invalid token.'],
  ])('Validates claim device', async (link: string, expected: string) => {
    const invalidProtocolResult = await claimDeviceLinkValidator(link);
    expect(translate(invalidProtocolResult.reason)).to.equal(expected);
  });
});
