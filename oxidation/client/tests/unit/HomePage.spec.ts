// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { mount, VueWrapper } from '@vue/test-utils';
import HomePage from '@/views/HomePage.vue';
import { createI18n, useI18n } from 'vue-i18n';
import frFR from '../../src/locales/fr-FR.json';
import enUS from '../../src/locales/en-US.json';
import { modalController } from '@ionic/vue';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import { Storage } from '@ionic/storage';
import CreateOrganization from '@/components/CreateOrganizationModal.vue';
import { formatTimeSince } from '@/common/date';

describe('HomePage.vue', () => {
  type MessageSchema = typeof frFR;
  const store = new Storage();
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

  // temporary, delete this when true data will exists by bindings
  store.create().then(() => {
    store.set('devicesData', {
      slug1: { lastLogin: new Date('01/11/2023') },
      slug2: { lastLogin: new Date('01/12/2023 12:03:05') },
      slug3: { lastLogin: new Date('01/12/2023 15:12:04') }
    });
  });

  const wrapper = mount(HomePage, {
    global: {
      plugins: [i18n],
      provide: {
        formatters: {
          timeSince: (date: Date | undefined, defaultValue=''): string => {
            const { t, d } = useI18n();
            return formatTimeSince(date, t, d, defaultValue);
          }
        }
      }
    }
  });

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders home vue', () => {
    expect(wrapper.text()).toMatch(new RegExp('List of your organizations'));
  });

  it('should get devices stored data on mount', () => {
    expect(wrapper.vm.deviceStoredDataDict).toEqual({
      slug1: { lastLogin: new Date('01/11/2023') },
      slug2: { lastLogin: new Date('01/12/2023 12:03:05') },
      slug3: { lastLogin: new Date('01/12/2023 15:12:04') }
    });
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
        organizationId: 'Black Mesa',
        humanHandle: 'Dr. Gordon Freeman',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug3',
        ty: {tag: 'Password'}
      });
    });
  });

  describe('Login filter orgs', () => {
    const wrapper = mount(HomePage, {
      global: {
        plugins: [i18n],
        provide: {
          formatters: {
            timeSince: (date: Date | undefined, defaultValue=''): string => {
              const { t, d } = useI18n();
              return formatTimeSince(date, t, d, defaultValue);
            }
          }
        }
      }
    });

    it('should filter orgs', async () => {
      expect(wrapper.vm.deviceList.length).toEqual(7);
      expect(wrapper.vm.orgSearchString).toEqual('');
      expect(wrapper.vm.sortBy).toEqual('organization');
      expect(wrapper.vm.sortByAsc).toEqual(true);
      expect(wrapper.vm.filteredDevices.length).toEqual(wrapper.vm.deviceList.length);

      // Should be ordered by ascending org name by default
      expect(wrapper.vm.filteredDevices[0].organizationId).toEqual('Black Mesa');
      expect(wrapper.vm.filteredDevices[6].organizationId).toEqual('Sanctum Sanctorum');

      expect(wrapper.findComponent('ion-input').exists()).toBeTruthy();
      await wrapper.findComponent('ion-input').setValue('la');

      expect(wrapper.vm.orgSearchString).toEqual('la');
      expect(wrapper.vm.filteredDevices.length).toEqual(3);
      expect(wrapper.vm.filteredDevices).toEqual([{
        organizationId: 'Black Mesa',
        humanHandle: 'Dr. Gordon Freeman',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug3',
        ty: {tag: 'Password'}
      }, {
        organizationId: 'Planet Express Is The Best Comp!',
        humanHandle: 'Dr. John A. Zoidberg',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug1',
        ty: {tag: 'Password'}
      },
      {
        organizationId: 'Riviera M.D.',
        humanHandle: 'Dr. Nicholas "Nick" Riviera',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug7',
        ty: {tag: 'Password'}
      }]);

      // Resetting the search string
      await wrapper.findComponent('ion-input').setValue('');
      expect(wrapper.vm.orgSearchString).toEqual('');
      expect(wrapper.vm.filteredDevices.length).toEqual(7);

      // Inverting the sort order
      wrapper.vm.sortByAsc = false;

      // Should be ordered by descending org name
      expect(wrapper.vm.filteredDevices[0].organizationId).toEqual('Sanctum Sanctorum');
      expect(wrapper.vm.filteredDevices[6].organizationId).toEqual('Black Mesa');

      wrapper.vm.sortBy = 'last_login';
      // Should be order by last login date descending
      expect(wrapper.vm.filteredDevices[0].slug).toEqual('slug4');
      expect(wrapper.vm.filteredDevices[4].slug).toEqual('slug1');
      expect(wrapper.vm.filteredDevices[6].slug).toEqual('slug3');

      // Sort by last login date ascending
      wrapper.vm.sortByAsc = true;
      expect(wrapper.vm.filteredDevices[0].slug).toEqual('slug3');
      expect(wrapper.vm.filteredDevices[2].slug).toEqual('slug1');
      expect(wrapper.vm.filteredDevices[6].slug).toEqual('slug7');

      wrapper.vm.sortBy = 'user_name';
      // Should be order by user name ascending
      expect(wrapper.vm.filteredDevices[0].humanHandle).toEqual('Dr John H. Watson');
      expect(wrapper.vm.filteredDevices[6].humanHandle).toEqual('Dr. Stephen Strange');

      // Sort by user name descending
      wrapper.vm.sortByAsc = false;
      expect(wrapper.vm.filteredDevices[0].humanHandle).toEqual('Dr. Stephen Strange');
      expect(wrapper.vm.filteredDevices[6].humanHandle).toEqual('Dr John H. Watson');
    });

  });

  describe('Login Popup tests', () => {
    let passwordInput: VueWrapper;
    let consoleLogSpyOn: jest.SpyInstance;

    beforeAll(async () => {
      wrapper.vm.selectedDevice = {
        organizationId: 'OsCorp',
        humanHandle: 'Dr. Otto G. Octavius',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug4',
        ty: {tag: 'Password'}
      };
      wrapper.vm.showOrganizationList = false;
      consoleLogSpyOn = jest.spyOn(console, 'log');
      jest.useFakeTimers();
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
      const expectedMessage = 'Log in to OsCorp with password "password"';
      const loginButton = wrapper.findComponent('#login-button') as VueWrapper;
      const now = new Date();
      jest.setSystemTime(now);
      await loginButton.trigger('click');
      await store.create();
      const deviceStoredDataList = await store.get('devicesData');
      expect(deviceStoredDataList.slug4.lastLogin).toEqual(now.toISOString());
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
      const expectedMessage = 'Log in to OsCorp with password "password"';
      const now = new Date();
      jest.setSystemTime(now);
      passwordInput.vm.$emit('enter');
      await store.create();
      const deviceStoredDataList = await store.get('devicesData');
      expect(deviceStoredDataList.slug4.lastLogin).toEqual(now.toISOString());
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
