// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { vi } from 'vitest';

function mockRouter(): void {
  vi.mock('vue-router', () => {
    return {
      useRoute: (): any => {
        return {params: {handle: 42}};
      },
    };
  });

  vi.mock('@ionic/vue-router', () => {
    return {
      createWebHistory: (_base? :string | undefined): any => {
        return new Object();
      },
      createRouter: (_obj: any): any => {
        return {
          currentRoute: {
            value: {
              params: {
                handle: 42,
              },
            },
          },
        };
      },
    };
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
