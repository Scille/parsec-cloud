// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import { mockI18n } from '@tests/component/support/mocks';
import { VueWrapper, mount } from '@vue/test-utils';

mockI18n();

describe('Sas code provide', () => {
  let wrapper: VueWrapper;
  beforeEach(() => {
    wrapper = mount(SasCodeProvide, {
      props: {
        code: 'ABCD',
      },
    });
  });

  it('should display the sas code', async () => {
    expect(wrapper.get('.caption-code').text()).to.equal('ABCD');
  });
});
