import { mount } from '@vue/test-utils';
import HomePage from '@/views/HomePage.vue';
import { createI18n } from 'vue-i18n';
import frFR from '../../src/locales/fr-FR.json';
import enUS from '../../src/locales/en-US.json';

describe('HomePage.vue', () => {
  type MessageSchema = typeof frFR;
  const defaultLocale = 'fr-FR';
  const supportedLocales:{[key: string]: string} = {
    fr: 'fr-FR',
    en: 'en-US',
    'fr-FR': 'fr-FR',
    'en-US': 'en-US'
  };
  const i18n = createI18n<[MessageSchema], 'fr-FR' | 'en-US'>({
    legacy: false,
    globalInjection: true,
    locale: supportedLocales[window.navigator.language] || defaultLocale,
    messages: {
      'fr-FR': frFR,
      'en-US': enUS
    }
  });

  const wrapper = mount(HomePage, {
    global: {
      plugins: [i18n]
    }
  });

  it('renders home vue', () => {
    expect(wrapper.text()).toMatch(new RegExp('^Welcome. Please add an organization to start using Parsec.'));
  });

  it('calls openCreateOrganizationModal when click on button', () => {
    const button = wrapper.find('#create-organization-button');
    const openCreateOrganizationModalSpy = jest.spyOn(wrapper.vm, 'openCreateOrganizationModal');
    button.trigger('click');
    expect(openCreateOrganizationModalSpy).toBeCalled();
  });
});
