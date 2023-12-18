// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { claimDeviceLinkValidator, claimLinkValidator, claimUserLinkValidator, organizationValidator } from '@/common/validators';
import { vi } from 'vitest';

const VALID_TOKEN = 'a'.repeat(32);

describe('Validators', () => {
  beforeEach(async () => {
    vi.mock('@/services/translation', () => {
      return {
        getI18n: (): any => {
          return { global: { t: (key: string) => key } };
        },
      };
    });

    vi.mock('@/parsec', () => {
      return {
        isValidOrganizationName: async (_value: string): Promise<boolean> => {
          return false;
        },
        parseBackendAddr: async (_value: string): Promise<any> => {
          return { ok: false, error: 'error' };
        },
      };
    });
  });

  it('Validates organization name', async () => {
    const invalidNameResult = await organizationValidator('Org#');
    expect(invalidNameResult.reason).to.equal('validators.organizationName.forbiddenCharacters');

    const nameTooLongResult = await organizationValidator('a'.repeat(33));
    expect(nameTooLongResult.reason).to.equal('validators.organizationName.tooLong');
  });

  it('Validates claim link', async () => {
    const invalidProtocolResult = await claimLinkValidator(`http://host/org?action=claim_user&token=${VALID_TOKEN}`);
    expect(invalidProtocolResult.reason).to.equal('validators.claimLink.invalidProtocol');

    const missingActionResult = await claimLinkValidator(`parsec://host/org?token=${VALID_TOKEN}`);
    expect(missingActionResult.reason).to.equal('validators.claimLink.missingAction');

    const invalidActionResult = await claimLinkValidator(`parsec://host/org?action=bootstrap_organization&token=${VALID_TOKEN}`);
    expect(invalidActionResult.reason).to.equal('validators.claimLink.invalidAction');

    const missingTokenResult = await claimLinkValidator('parsec://host/org?action=claim_user');
    expect(missingTokenResult.reason).to.equal('validators.claimLink.missingToken');

    const invalidTokenResult = await claimLinkValidator('parsec://host/org?action=claim_user&token=abcdefg');
    expect(invalidTokenResult.reason).to.equal('validators.claimLink.invalidToken');
  });

  it('Validates claim user', async () => {
    const invalidProtocolResult = await claimUserLinkValidator(`http://host/org?action=claim_user&token=${VALID_TOKEN}`);
    expect(invalidProtocolResult.reason).to.equal('validators.claimUserLink.invalidProtocol');

    const missingActionResult = await claimUserLinkValidator(`parsec://host/org?token=${VALID_TOKEN}`);
    expect(missingActionResult.reason).to.equal('validators.claimUserLink.missingAction');

    const invalidActionResult = await claimUserLinkValidator(`parsec://host/org?action=claim_device&token=${VALID_TOKEN}`);
    expect(invalidActionResult.reason).to.equal('validators.claimUserLink.invalidAction');

    const missingTokenResult = await claimUserLinkValidator('parsec://host/org?action=claim_user');
    expect(missingTokenResult.reason).to.equal('validators.claimUserLink.missingToken');

    const invalidTokenResult = await claimUserLinkValidator('parsec://host/org?action=claim_user&token=abcdefg');
    expect(invalidTokenResult.reason).to.equal('validators.claimUserLink.invalidToken');
  });

  it('Validates claim device', async () => {
    const invalidProtocolResult = await claimDeviceLinkValidator(`http://host/org?action=claim_device&token=${VALID_TOKEN}`);
    expect(invalidProtocolResult.reason).to.equal('validators.claimDeviceLink.invalidProtocol');

    const missingActionResult = await claimDeviceLinkValidator(`parsec://host/org?token=${VALID_TOKEN}`);
    expect(missingActionResult.reason).to.equal('validators.claimDeviceLink.missingAction');

    const invalidActionResult = await claimDeviceLinkValidator(`parsec://host/org?action=claim_user&token=${VALID_TOKEN}`);
    expect(invalidActionResult.reason).to.equal('validators.claimDeviceLink.invalidAction');

    const missingTokenResult = await claimDeviceLinkValidator('parsec://host/org?action=claim_device');
    expect(missingTokenResult.reason).to.equal('validators.claimDeviceLink.missingToken');

    const invalidTokenResult = await claimDeviceLinkValidator('parsec://host/org?action=claim_device&token=abcdefg');
    expect(invalidTokenResult.reason).to.equal('validators.claimDeviceLink.invalidToken');
  });
});
