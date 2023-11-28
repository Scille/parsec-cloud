// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { VueWrapper, mount } from '@vue/test-utils';
import { IonInput } from '@ionic/vue';
import MsPasswordInput from '@/components/core/ms-input/MsPasswordInput.vue';

describe('Password Input', () => {
  let wrapper: VueWrapper;
  beforeEach(() => {
    wrapper = mount(MsPasswordInput, {
      props: {
        label: 'A Label',
        modelValue: '',
      },
    });
  });

  it('should emit a signal when input changes', async () => {
    const ionInput = wrapper.findComponent(IonInput);
    ionInput.vm.$emit('ionInput', { target: { value: 'P@ssw0rd.' } });
    expect(wrapper.emitted('change')?.length).to.equal(1);
    expect(wrapper.emitted('change')?.at(0)).to.have.same.members(['P@ssw0rd.']);
  });

  it('should emit enter when Enter key is pressed', async () => {
    // Setting a value
    await wrapper.setProps({ modelValue: 'P@ssw0rd.' });
    const ionInput = wrapper.findComponent(IonInput);
    await ionInput.trigger('keyup.enter');
    expect(wrapper.emitted('onEnterKeyup')?.length).to.equal(1);
  });

  it('should not emit enter when input is empty', async () => {
    const ionInput = wrapper.findComponent(IonInput);
    await ionInput.trigger('keyup.enter');
    expect(wrapper.emitted('onEnterKeyup')).to.be.undefined;
  });

  it('should toggle password visibility button icon and password input type on password visibility button click', async () => {
    expect((wrapper.vm as any).passwordVisible).to.be.false;
    await wrapper.find('.input-icon').trigger('click');
    expect((wrapper.vm as any).passwordVisible).to.be.true;
    await wrapper.find('.input-icon').trigger('click');
    expect((wrapper.vm as any).passwordVisible).to.be.false;
  });
});
