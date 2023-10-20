// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import { DeviceFileType,  AvailableDevice } from '@/plugins/libparsec';
import { VueWrapper, mount } from '@vue/test-utils';

describe('Organization Card', () => {
  let wrapper: VueWrapper;

  const DEVICE: AvailableDevice = {
    keyFilePath: '/path',
    organizationId: 'Black Mesa',
    deviceId: '5ada1b25e8904e9ba238834227a40abf@9700f0fd005e4752a13bcba0042d4703',
    humanHandle: {
      label: 'Gordon Freeman',
      email: 'freeman.gordon@black-mesa.com',
    },
    deviceLabel: 'hev',
    slug: '1',
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
    expect(wrapper.find('ion-card-title').text()).to.equal('Black Mesa');
    expect(wrapper.find('ion-card-content').text()).to.equal('Gordon Freeman');
  });
});
