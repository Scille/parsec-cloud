// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import { AvailableDevice, AvailableDeviceTypeTag } from '@/plugins/libparsec';
import { VueWrapper, mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { beforeAll, beforeEach, describe, expect, it } from 'vitest';

describe('Organization Card', () => {
  beforeAll(() => {
    window.isDev = (): boolean => false;
  });

  let wrapper: VueWrapper;

  const DEVICE: AvailableDevice = {
    keyFilePath: '/path',
    serverAddr: 'parsec3://parsec.invalid',
    createdOn: DateTime.utc(),
    protectedOn: DateTime.utc(),
    organizationId: 'Black Mesa',
    userId: '5ada1b25e8904e9ba238834227a40abf',
    deviceId: '9700f0fd005e4752a13bcba0042d4703',
    humanHandle: {
      label: 'Gordon Freeman',
      email: 'freeman.gordon@black-mesa.com',
    },
    deviceLabel: 'hev',
    ty: { tag: AvailableDeviceTypeTag.Password },
  };

  beforeEach(() => {
    wrapper = mount(OrganizationCard, {
      props: {
        device: DEVICE,
      },
    });
  });

  it('display the organization', async () => {
    expect(wrapper.find('ion-avatar').text()).to.equal('Bl');
    expect(wrapper.find('.organization-card-header__title').text()).to.equal('Black Mesa');
    expect(wrapper.find('.organization-card-login__name').text()).to.equal('Gordon Freeman');
  });
});
