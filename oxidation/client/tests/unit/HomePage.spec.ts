import { mount, VueWrapper } from '@vue/test-utils';
import HomePage from '@/views/HomePage.vue';
import { createI18n } from 'vue-i18n';
import frFR from '../../src/locales/fr-FR.json';
import enUS from '../../src/locales/en-US.json';
import { modalController } from '@ionic/vue';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import CreateOrganization from '@/components/CreateOrganizationModal.vue';
import { getSpyOnLastCallResult } from './utils';

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
    },
    datetimeFormats: {
      'en-US': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric'
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric'
        }
      },
      'fr-FR': {
        short: {
          year: 'numeric',
          month: 'short',
          day: 'numeric'
        },
        long: {
          year: 'numeric',
          month: 'long',
          day: 'numeric',
          weekday: 'long',
          hour: 'numeric',
          minute: 'numeric'
        }
      }
    }
  });

  const wrapper = mount(HomePage, {
    global: {
      plugins: [i18n]
    }
  });

  let modalControllerCreateSpy:jest.SpyInstance;

  beforeAll(() => {
    modalControllerCreateSpy = jest.spyOn(modalController, 'create');
  });

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders home vue', () => {
    expect(wrapper.text()).toMatch(new RegExp('^List of your organizations'));
  });

  it('calls openCreateOrganizationModal when click on create organization button', async () => {
    const createOrganizationButton = wrapper.find('#create-organization-button');
    await createOrganizationButton.trigger('click');
    expect(modalControllerCreateSpy).toHaveBeenCalledTimes(1);
    expect(modalControllerCreateSpy).toHaveBeenCalledWith({
      component: CreateOrganization,
      canDismiss: wrapper.vm.canDismissModal,
      cssClass: 'create-organization-modal'
    });
    const modal = await getSpyOnLastCallResult(modalControllerCreateSpy) as HTMLIonModalElement;
    await modal.present();
    expect(modal.className).not.toContain('overlay-hidden');
  });

  it('calls openJoinByLinkModal when click on join by link button', async () => {
    const joinByLinkButton = wrapper.find('#join-by-link-button');
    await joinByLinkButton.trigger('click');
    expect(modalControllerCreateSpy).toHaveBeenCalledTimes(1);
    expect(modalControllerCreateSpy).toHaveBeenCalledWith({
      component: JoinByLinkModal,
      cssClass: 'join-by-link-modal'
    });
  });
});
