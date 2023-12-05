// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { mockI18n, mockRouter, getDefaultProvideConfig, getRoutesCalled, resetRoutesCalled } from '@tests/component/support/mocks';

// Before importing anything else
mockI18n();
mockRouter();

import HeaderBreadcrumbs from '@/components/header/HeaderBreadcrumbs.vue';
import { mount } from '@vue/test-utils';

describe('Header breadcrumbs', () => {
  beforeEach(() => {
    resetRoutesCalled();
  });

  it('Display current device', () => {
    const NODES = [
      {
        id: 1,
        display: 'First Node',
        name: 'route1',
        params: { param1: 1 },
        query: { query1: 1 },
      },
      {
        id: 2,
        display: 'Second Node',
        name: 'route2',
        params: { param2: 2 },
        query: { query2: 2 },
      },
      {
        id: 3,
        display: 'Third Node',
        name: 'route3',
        params: { param3: 3 },
        query: { query3: 3 },
      },
    ];
    const wrapper = mount(HeaderBreadcrumbs, {
      props: {
        pathNodes: NODES,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.findAllComponents('ion-breadcrumb').length).to.equal(3);
    expect(wrapper.findAllComponents('ion-breadcrumb').at(0)?.text()).to.equal('First Node');
    expect(wrapper.findAllComponents('ion-breadcrumb').at(1)?.text()).to.equal('Second Node');
    expect(wrapper.findAllComponents('ion-breadcrumb').at(2)?.text()).to.equal('Third Node');

    wrapper.findAllComponents('ion-breadcrumb').at(1)?.trigger('click');
    expect(wrapper.emitted('change')?.length).to.equal(1);
    expect(wrapper.emitted('change')?.at(0)?.at(0)).to.deep.equal(NODES[1]);
  });
});
