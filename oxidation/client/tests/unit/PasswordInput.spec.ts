import { mount } from '@vue/test-utils';
import PasswordInput from '@/components/PasswordInput.vue';
import { createI18n } from 'vue-i18n';
import frFR from '../../src/locales/fr-FR.json';
import enUS from '../../src/locales/en-US.json';

describe('PasswordInput.vue', () => {
  const wrapper = mount(PasswordInput, {
    props: {
        label: 'TestLabel'
    }
  });

  it('renders password input', () => {
    expect(wrapper.find('ion-label').text()).toBe('TestLabel');
  });

  it('enter pressed', () => {
    wrapper.find('ion-input').setValue('password')
    expect(wrapper.emitted('change')).toBeCalledWith('password');
    wrapper.find('ion-input').trigger('keyup.enter');
    expect(wrapper.emitted()).toHaveProperty('enter');
  })
});
