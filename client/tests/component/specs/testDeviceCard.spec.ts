// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import DeviceCard from '@/components/devices/DeviceCard.vue';
import { getDefaultProvideConfig, mockI18n } from '@tests/component/support/mocks';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';

mockI18n();

describe('Device Card', () => {
  it('Display current device', () => {
    const wrapper = mount(DeviceCard, {
      // we are forced to attached the component to a domElement for isVisible to work properly
      // more info: https://test-utils.vuejs.org/api/#isVisible
      attachTo: document.body,
      props: {
        label: 'My Device',
        isCurrent: true,
        date: DateTime.now(),
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.device-name').text()).to.equal('My Device');
    expect(wrapper.get('.join-date').text()).to.equal('DevicesPage.joinedOn One minute ago');
    expect(wrapper.get('.badge').isVisible()).to.be.true;
    expect(wrapper.get('.badge').text()).to.equal('DevicesPage.activeDeviceBadge');
  });

  it('Display not current device', () => {
    const wrapper = mount(DeviceCard, {
      // we are forced to attached the component to a domElement for isVisible to work properly
      // more info: https://test-utils.vuejs.org/api/#isVisible
      attachTo: document.body,
      props: {
        label: 'My Other Device',
        isCurrent: false,
        date: DateTime.now(),
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.device-name').text()).to.equal('My Other Device');
    expect(wrapper.get('.join-date').text()).to.equal('DevicesPage.joinedOn One minute ago');
    expect((wrapper.vm as any).isCurrent).to.be.false;
    expect(wrapper.get('.badge').isVisible()).to.be.false;
  });
});
