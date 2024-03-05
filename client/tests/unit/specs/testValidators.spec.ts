// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { mockValidators } from '@tests/component/support/mocks';

mockValidators();

import { claimDeviceLinkValidator, claimLinkValidator, claimUserLinkValidator, organizationValidator } from '@/common/validators';

const VALID_TOKEN = 'a'.repeat(32);

describe('Validators', () => {
  it('Validates organization name', async () => {
    const invalidNameResult = await organizationValidator('Org#');
    expect(invalidNameResult.reason).to.equal('Only letters, digits, underscores and hyphens. No spaces.');

    const nameTooLongResult = await organizationValidator('a'.repeat(33));
    expect(nameTooLongResult.reason).to.equal('Name is too long, limit is 32 characters.');
  });

  it('Validates claim link', async () => {
    const invalidProtocolResult = await claimLinkValidator(`http://host/org?action=claim_user&token=${VALID_TOKEN}`);
    expect(invalidProtocolResult.reason).to.equal("Link should start with 'parsec3://'.");

    const missingActionResult = await claimLinkValidator(`parsec3://host/org?token=${VALID_TOKEN}`);
    expect(missingActionResult.reason).to.equal("Link doesn't include an action.");

    const invalidActionResult = await claimLinkValidator(`parsec3://host/org?action=bootstrap_organization&token=${VALID_TOKEN}`);
    expect(invalidActionResult.reason).to.equal('Link contains an invalid action.');

    const missingTokenResult = await claimLinkValidator('parsec3://host/org?action=claim_user');
    expect(missingTokenResult.reason).to.equal("Link doesn't include a token.");

    const invalidTokenResult = await claimLinkValidator('parsec3://host/org?action=claim_user&token=abcdefg');
    expect(invalidTokenResult.reason).to.equal('Link contains an invalid token.');
  });

  it('Validates claim user', async () => {
    const invalidProtocolResult = await claimUserLinkValidator(`http://host/org?action=claim_user&token=${VALID_TOKEN}`);
    expect(invalidProtocolResult.reason).to.equal("Link should start with 'parsec3://'.");

    const missingActionResult = await claimUserLinkValidator(`parsec3://host/org?token=${VALID_TOKEN}`);
    expect(missingActionResult.reason).to.equal("Link doesn't include an action.");

    const invalidActionResult = await claimUserLinkValidator(`parsec3://host/org?action=claim_device&token=${VALID_TOKEN}`);
    expect(invalidActionResult.reason).to.equal('Link contains an invalid action.');

    const missingTokenResult = await claimUserLinkValidator('parsec3://host/org?action=claim_user');
    expect(missingTokenResult.reason).to.equal("Link doesn't include a token.");

    const invalidTokenResult = await claimUserLinkValidator('parsec3://host/org?action=claim_user&token=abcdefg');
    expect(invalidTokenResult.reason).to.equal('Link contains an invalid token.');
  });

  it('Validates claim device', async () => {
    const invalidProtocolResult = await claimDeviceLinkValidator(`http://host/org?action=claim_device&token=${VALID_TOKEN}`);
    expect(invalidProtocolResult.reason).to.equal("Link should start with 'parsec3://'.");

    const missingActionResult = await claimDeviceLinkValidator(`parsec3://host/org?token=${VALID_TOKEN}`);
    expect(missingActionResult.reason).to.equal("Link doesn't include an action.");

    const invalidActionResult = await claimDeviceLinkValidator(`parsec3://host/org?action=claim_user&token=${VALID_TOKEN}`);
    expect(invalidActionResult.reason).to.equal('Link contains an invalid action.');

    const missingTokenResult = await claimDeviceLinkValidator('parsec3://host/org?action=claim_device');
    expect(missingTokenResult.reason).to.equal("Link doesn't include a token.");

    const invalidTokenResult = await claimDeviceLinkValidator('parsec3://host/org?action=claim_device&token=abcdefg');
    expect(invalidTokenResult.reason).to.equal('Link contains an invalid token.');
  });
});
