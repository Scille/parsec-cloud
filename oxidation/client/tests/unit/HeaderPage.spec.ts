import { mount, shallowMount } from '@vue/test-utils';
import { createI18n } from 'vue-i18n';
import HeaderPage from '@/views/HeaderPage.vue';
import router from '../../src/router';
import frFR from '../../src/locales/fr-FR.json';
import enUS from '../../src/locales/en-US.json';

describe('HeaderPage.vue', () => {
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

  const mockRouter = {
    push: jest.fn()
  };

  const wrapper = mount(HeaderPage, {
    global: {
      plugins: [i18n, router]
      // mocks: {
      //   $router: mockRouter
      // }
    },
    attachToDocument: true,
    sync: false
  });

  it('renders header vue', () => {
    expect(wrapper.text()).toMatch(new RegExp('Welcome. Please add an organization to start using Parsec.'));
  });
});
