// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import OrganizationCard from '@/components/OrganizationCard.vue';
import { AvailableDevice } from '@/plugins/libparsec/definitions';
import { VueWrapper, mount } from '@vue/test-utils';

describe('Organization Card', () => {
  let wrapper: VueWrapper;

  const DEVICE: AvailableDevice = {
    keyFilePath: '/path',
    organizationId: 'Black Mesa',
    deviceId: '5ada1b25e8904e9ba238834227a40abf@9700f0fd005e4752a13bcba0042d4703',
    humanHandle: 'Gordon Freeman',
    deviceLabel: 'hev',
    slug: '1',
    ty: {tag: 'Password'},
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
    expect(wrapper.find('ion-card-subtitle').text()).to.equal('Gordon Freeman');
  });

  it('handles empty human handles', () => {
    DEVICE.humanHandle = null;

    wrapper = mount(OrganizationCard, {
      props: {
        device: DEVICE,
      },
    });

    expect(wrapper.find('ion-avatar').text()).to.equal('Bl');
    expect(wrapper.find('ion-card-title').text()).to.equal('Black Mesa');
    expect(wrapper.find('ion-card-subtitle').text()).to.equal('5ada1b25e8904e9ba238834227a40abf@9700f0fd005e4752a13bcba0042d4703');
  });
});
