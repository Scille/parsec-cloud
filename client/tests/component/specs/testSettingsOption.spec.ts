// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import SettingsOption from '@/components/settings/SettingsOption.vue';
import { mount, VueWrapper } from '@vue/test-utils';

describe('Setting Option', () => {
  let wrapper: VueWrapper;

  beforeEach(() => {
    wrapper = mount(SettingsOption, {
      props: {
        title: 'A Title',
        description: 'A description, should be a little bit longer than the title.',
      },
    });
  });

  it('display the setting option', () => {
    expect(wrapper.get('.title').text()).to.equal('A Title');
    expect(wrapper.get('.description').text()).to.equal('A description, should be a little bit longer than the title.');
  });
});
