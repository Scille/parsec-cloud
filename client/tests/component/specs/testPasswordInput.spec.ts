// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import MsPasswordInput from '@/components/core/ms-input/MsPasswordInput.vue';
import { IonInput } from '@ionic/vue';
import { getDefaultProvideConfig } from '@tests/component/support/mocks';
import { VueWrapper, mount } from '@vue/test-utils';

describe('Password Input', () => {
  let wrapper: VueWrapper;
  beforeEach(() => {
    wrapper = mount(MsPasswordInput, {
      props: {
        label: 'A Label',
        modelValue: '',
      },
      global: {
        provide: getDefaultProvideConfig(),
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
