// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import { AvailableDevice, DeviceFileType } from '@/plugins/libparsec';
import { VueWrapper, mount } from '@vue/test-utils';
import luxon from 'luxon';

describe('Organization Card', () => {
  let wrapper: VueWrapper;

  const DEVICE: AvailableDevice = {
    keyFilePath: '/path',
    serverUrl: 'https://parsec.invalid',
    createdOn: luxon.DateTime.utc(),
    protectedOn: luxon.DateTime.utc(),
    organizationId: 'Black Mesa',
    userId: '5ada1b25e8904e9ba238834227a40abf',
    deviceId: '9700f0fd005e4752a13bcba0042d4703',
    humanHandle: {
      label: 'Gordon Freeman',
      email: 'freeman.gordon@black-mesa.com',
    },
    deviceLabel: 'hev',
    ty: DeviceFileType.Password,
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
    expect(wrapper.find('ion-card-title span:first-child').text()).to.equal('Black Mesa');
    expect(wrapper.find('ion-card-title span:last-child').text()).to.equal('Gordon Freeman');
  });
});
