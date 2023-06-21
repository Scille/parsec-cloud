// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import ChoosePassword from '@/components/ChoosePassword.vue';
import { VueWrapper, mount } from '@vue/test-utils';
import { IonInput } from '@ionic/vue';
import { mockI18n } from '../support/mocks';

mockI18n();

describe('Choose password', () => {
  let wrapper: VueWrapper<any>;

  beforeEach(() => {
    wrapper = mount(ChoosePassword, {});
  });

  it('Validate the fields', async () => {
    // Fields are empty, obviously not valid
    expect(wrapper.vm.areFieldsCorrect()).to.be.false;

    const ionInputs = wrapper.findAllComponents(IonInput);
    ionInputs[0].vm.$emit('ionInput', {target: {value: 'P@ssw0rd'}});
    expect(wrapper.vm.password).to.equal('P@ssw0rd');

    // Confirmation is not filled, not valid
    expect(wrapper.vm.areFieldsCorrect()).to.be.false;

    ionInputs[1].vm.$emit('ionInput', {target: {value: 'P@ssw0rd'}});
    expect(wrapper.vm.passwordConfirm).to.equal('P@ssw0rd');

    // P@ssw0rd is not strong enough
    expect(wrapper.vm.areFieldsCorrect()).to.be.false;

    ionInputs[0].vm.$emit('ionInput', {target: {value: 'ABiggerSaferPassword'}});
    expect(wrapper.vm.password).to.equal('ABiggerSaferPassword');

    // Password is strong enough but password and confirmation don't match
    expect(wrapper.vm.areFieldsCorrect()).to.be.false;

    ionInputs[1].vm.$emit('ionInput', {target: {value: 'ABiggerSaferPassword'}});
    expect(wrapper.vm.passwordConfirm).to.equal('ABiggerSaferPassword');
    expect(wrapper.vm.areFieldsCorrect()).to.be.true;
  });
});
