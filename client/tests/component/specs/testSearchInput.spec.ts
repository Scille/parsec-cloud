// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import SearchInput from '@/components/SearchInput.vue';
import { mount, VueWrapper } from '@vue/test-utils';
import { IonInput } from '@ionic/vue';

describe('Search Input', () => {
  let wrapper: VueWrapper;
  beforeEach(() => {
    wrapper = mount(SearchInput, {
      props: {
        label: 'A Label',
      },
    });
  });

  it('should emit a signal when input changes', async () => {
    const ionInput = wrapper.findComponent(IonInput);
    ionInput.vm.$emit('ionInput', {detail: {value: 'Search string'}});
    expect(wrapper.emitted('change')?.length).to.equal(1);
    expect(wrapper.emitted('change')?.at(0)).to.have.same.members(['Search string']);
  });

  it('should emit enter when Enter key is pressed', async () => {
    // Setting a value
    (wrapper.vm as any).searchRef = 'Search string';
    const ionInput = wrapper.findComponent(IonInput);
    await ionInput.trigger('keyup.enter');
    expect(wrapper.emitted('enter')?.length).to.equal(1);
  });

  it('should not emit enter when input is empty', async () => {
    const ionInput = wrapper.findComponent(IonInput);
    await ionInput.trigger('keyup.enter');
    expect(wrapper.emitted('enter')).to.be.undefined;
  });
});
