// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { mount, VueWrapper } from '@vue/test-utils';
import SearchInput from '@/components/SearchInput.vue';
import { clearEmitted } from './utils';
import {
  searchOutline
} from 'ionicons/icons';

describe('SearchInput.vue', () => {
  const wrapper = mount(SearchInput, {
    props: {
      label: 'SearchLabel'
    }
  });
  let ionInput: VueWrapper;
  let ionIcon: VueWrapper;

  beforeAll(() => {
    ionInput = wrapper.findComponent('ion-input') as VueWrapper;
    ionIcon = wrapper.findComponent('ion-icon') as VueWrapper;
  });

  beforeEach(() => {
    jest.clearAllMocks();
    clearEmitted(ionInput);
    clearEmitted(wrapper);
    ionInput.setValue('');
  });

  it('renders search input', () => {
    expect(wrapper.find('ion-label').text()).toBe('SearchLabel');
    expect(ionIcon.props('icon')).toEqual(searchOutline);
  });

  it('should emit "change" event with expected value on search input change', () => {
    ionInput.vm.$emit('ionChange', {detail: {value: 'expected_value'}});
    expect(wrapper.emitted('change')).toEqual([['expected_value']]);
  });

  it('should not emit "enter" event when search is empty and on search input enter keyup', () => {
    const spyOnEnterPress = jest.spyOn(wrapper.vm, 'onEnterPress');
    ionInput.trigger('keyup.enter');
    expect(spyOnEnterPress).toHaveBeenCalledTimes(1);
    expect(wrapper.emitted('enter')).toBeUndefined();
  });

  it('should emit "enter" event when search is not empty and on search input enter keyup', async () => {
    const spyOnEnterPress = jest.spyOn(wrapper.vm, 'onEnterPress');
    await ionInput.setValue('search');
    ionInput.trigger('keyup.enter');
    expect(spyOnEnterPress).toHaveBeenCalledTimes(1);
    expect(wrapper.emitted('enter')).toHaveLength(1);
  });
});
