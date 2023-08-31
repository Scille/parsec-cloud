// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { vi } from 'vitest';

function mockRouter(): void {
  vi.mock('vue-router', () => {
    return { useRoute: (): any => {
      return {params: {deviceId: 'a@b'}};
    } };
  });
}

mockRouter();

import { isAdmin, isOutsider } from '@/common/permissions';

describe('Permissions', () => {
  it('test isAdmin', async () => {
    expect(isAdmin()).to.be.true;
  });

  it('test isOutsider', async () => {
    expect(isOutsider()).to.be.false;
  });
});
