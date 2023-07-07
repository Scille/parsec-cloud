// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import PasswordInput from '@/components/PasswordInput.vue';
import { VueWrapper, mount } from '@vue/test-utils';
import { IonInput } from '@ionic/vue';

describe('Password Input', () => {
  let wrapper: VueWrapper;
  beforeEach(() => {
    wrapper = mount(PasswordInput, {
      props: {
        label: 'A Label'
      }
    });
  });

  it('should emit a signal when input changes', async () => {
    const ionInput = wrapper.findComponent(IonInput);
    ionInput.vm.$emit('ionInput', {detail: {value: 'P@ssw0rd'}});
    expect(wrapper.emitted('change')?.length).to.equal(1);
    expect(wrapper.emitted('change')?.at(0)).to.have.same.members(['P@ssw0rd']);
  });

  it('should emit enter when Enter key is pressed', async () => {
    // Setting a value
    (wrapper.vm as any).passwordRef = 'P@ssw0rd';
    const ionInput = wrapper.findComponent(IonInput);
    await ionInput.trigger('keyup.enter');
    expect(wrapper.emitted('enter')?.length).to.equal(1);
  });

  it('should not emit enter when input is empty', async () => {
    const ionInput = wrapper.findComponent(IonInput);
    await ionInput.trigger('keyup.enter');
    expect(wrapper.emitted('enter')).to.be.undefined;
  });

  it('should toggle password visibility button icon and password input type on password visibility button click', async () => {
    expect((wrapper.vm as any).passwordVisible).to.be.false;
    await wrapper.find('ion-button').trigger('click');
    expect((wrapper.vm as any).passwordVisible).to.be.true;
    await wrapper.find('ion-button').trigger('click');
    expect((wrapper.vm as any).passwordVisible).to.be.false;
  });
});
