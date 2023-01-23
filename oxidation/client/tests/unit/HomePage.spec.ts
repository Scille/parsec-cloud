import { mount, VueWrapper } from '@vue/test-utils';
import HomePage from '@/views/HomePage.vue';
import { createI18n } from 'vue-i18n';
import frFR from '../../src/locales/fr-FR.json';
import enUS from '../../src/locales/en-US.json';
import { modalController } from '@ionic/vue';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import { Storage } from '@ionic/storage';
import CreateOrganization from '@/components/CreateOrganizationModal.vue';
import { MockDate, mockDateValue } from './utils';
import { login } from '../../../bindings/web/pkg';

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

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders home vue', () => {
    expect(wrapper.text()).toMatch(new RegExp('^List of your organizations'));
  });

  describe('Organization List tests', () => {
    let modalControllerCreateSpy:jest.SpyInstance;

    beforeAll(() => {
      modalControllerCreateSpy = jest.spyOn(modalController, 'create');
    });

    it('should call openCreateOrganizationModal when click on create organization button', async () => {
      const createOrganizationButton = wrapper.find('#create-organization-button');
      await createOrganizationButton.trigger('click');
      expect(modalControllerCreateSpy).toHaveBeenCalledTimes(1);
      expect(modalControllerCreateSpy).toHaveBeenCalledWith({
        component: CreateOrganization,
        canDismiss: wrapper.vm.canDismissModal,
        cssClass: 'create-organization-modal'
      });
    });

    it('should call openJoinByLinkModal when click on join by link button', async () => {
      const joinByLinkButton = wrapper.find('#join-by-link-button');
      await joinByLinkButton.trigger('click');
      expect(modalControllerCreateSpy).toHaveBeenCalledTimes(1);
      expect(modalControllerCreateSpy).toHaveBeenCalledWith({
        component: JoinByLinkModal,
        cssClass: 'join-by-link-modal'
      });
    });

    it('should hide organization list, show login popup and set as selected device on organization card click', async () => {
      const organizationCard = wrapper.findComponent('.organization-card-container') as VueWrapper;
      expect(wrapper.findComponent('#organization-list-container').exists()).toBeTruthy();
      expect(wrapper.findComponent('#login-popup-container').exists()).toBeFalsy();
      expect(wrapper.vm.selectedDevice).toBeUndefined();
      await organizationCard.trigger('click');
      expect(wrapper.findComponent('#organization-list-container').exists()).toBeFalsy();
      expect(wrapper.findComponent('#login-popup-container').exists()).toBeTruthy();
      expect(wrapper.vm.selectedDevice).toEqual({
        organizationId: 'MegaShark',
        humanHandle: 'Maxime Grandcolas',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug1',
        ty: {tag: 'Password'}
      });
    });
  });

  describe('Login Popup tests', () => {
    let passwordInput: VueWrapper;
    let consoleLogSpyOn: jest.SpyInstance;
    const store = new Storage();

    beforeAll(async () => {
      wrapper.vm.selectedDevice = {
        organizationId: 'Eddy',
        humanHandle: 'Maxime Grandcolas',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug4',
        ty: {tag: 'Password'}
      };
      wrapper.vm.showOrganizationList = false;
      global.Date = MockDate as DateConstructor;
      consoleLogSpyOn = jest.spyOn(console, 'log');
    });

    afterAll(() => {
      global.Date = Date;
    });

    it('should update password value on password input change', async () => {
      passwordInput = wrapper.findComponent({name: 'PasswordInput'}) as VueWrapper;
      expect(wrapper.vm.password).toEqual('');
      passwordInput.vm.$emit('change', 'password');
      expect(wrapper.vm.password).toEqual('password');
    });

    it('should log in the console the expected message on forgotten password button click', async () => {
      const expectedMessage = 'forgotten password!';
      const forgottenPasswordButton = wrapper.findComponent('#forgotten-password-button') as VueWrapper;
      forgottenPasswordButton.trigger('click');
      expect(consoleLogSpyOn).toHaveBeenCalledTimes(1);
      expect(consoleLogSpyOn).toHaveBeenCalledWith(expectedMessage);
    });

    it('should log in the console the expected message on login button click', async () => {
      wrapper.vm.password = 'password';
      const expectedMessage = 'Log in to Eddy with password "password"';
      const loginButton = wrapper.findComponent('#login-button') as VueWrapper;
      await loginButton.trigger('click');
      await store.create();
      const deviceStoredDataList = await store.get('devicesData');
      expect(deviceStoredDataList.slug4.lastLogin).toEqual(mockDateValue);
      expect(consoleLogSpyOn).toHaveBeenCalledTimes(1);
      expect(consoleLogSpyOn).toHaveBeenCalledWith(expectedMessage);
    });

    it('should disable the login button on passwordInput change with empty value', async () => {
      passwordInput = wrapper.findComponent({name: 'PasswordInput'}) as VueWrapper;
      wrapper.vm.password = 'password';
      const loginButton = wrapper.findComponent('#login-button') as VueWrapper;
      expect(loginButton.props('disabled')).toBeFalsy();
      await passwordInput.vm.$emit('change', '');
      expect(loginButton.props('disabled')).toBeTruthy();
    });

    it('should log in the console the expected message on password input enter', async () => {
      passwordInput = wrapper.findComponent({name: 'PasswordInput'}) as VueWrapper;
      wrapper.vm.password = 'password';
      const expectedMessage = 'Log in to Eddy with password "password"';
      passwordInput.vm.$emit('enter');
      await store.create();
      const deviceStoredDataList = await store.get('devicesData');
      expect(deviceStoredDataList.slug4.lastLogin).toEqual(mockDateValue);
      expect(consoleLogSpyOn).toHaveBeenCalledTimes(1);
      expect(consoleLogSpyOn).toHaveBeenCalledWith(expectedMessage);
    });

    it('should hide login popup and show organization list on back to list button click', async () => {
      const backToListButton = wrapper.findComponent('#back-to-list-button') as VueWrapper;
      expect(wrapper.findComponent('#organization-list-container').exists()).toBeFalsy();
      expect(wrapper.findComponent('#login-popup-container').exists()).toBeTruthy();
      await backToListButton.trigger('click');
      expect(wrapper.findComponent('#organization-list-container').exists()).toBeTruthy();
      expect(wrapper.findComponent('#login-popup-container').exists()).toBeFalsy();
    });
  });
});
