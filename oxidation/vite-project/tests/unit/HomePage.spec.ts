// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { describe, it, beforeEach, beforeAll, vi, expect } from 'vitest';
import { mount, VueWrapper } from '@vue/test-utils';
import HomePage from '@/views/HomePage.vue';
import { createI18n, useI18n } from 'vue-i18n';
import frFR from '../../src/locales/fr-FR.json';
import enUS from '../../src/locales/en-US.json';
import { modalController } from '@ionic/vue';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import CreateOrganization from '@/components/CreateOrganizationModal.vue';
import { formatTimeSince } from '@/common/date';
import { StorageManager } from '@/services/storageManager';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

const storageManager = new StorageManager();
storageManager.create();

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

  vi.useFakeTimers();
  vi.setSystemTime(DateTime.fromISO('1999-04-28T18:00:00').toJSDate());

  const wrapper = mount(HomePage, {
    global: {
      plugins: [i18n],
      provide: {
        formatters: {
          timeSince: (date: DateTime | undefined, defaultValue=''): string => {
            const { t, d } = useI18n();
            return formatTimeSince(date, t, d, defaultValue);
          }
        },
        storageManager: storageManager
      }
    }
  });

  beforeEach(async () => {
    vi.clearAllMocks();
    await libparsec.testNewTestbed('coolorg', 'parsec://127.0.0.1:6777?no_ssl=true');
  });

  it('renders home vue', async () => {
    expect(wrapper.text()).toMatch(new RegExp('List of your organizations'));
  });

  it('should get devices stored data on mount', () => {

    console.log(window.localStorage.getItem('devices'));

    console.log(wrapper.vm.deviceList);

    expect(wrapper.vm.storedDeviceDataDict.slug1.lastLogin.c).toEqual(
      {year: 1999, month: 4, day: 28, hour: 18, minute: 0, second: 0, millisecond: 0}
    );
    expect(wrapper.vm.storedDeviceDataDict.slug2.lastLogin.c).toEqual(
      {year: 1999, month: 4, day: 28, hour: 17, minute: 59, second: 50, millisecond: 0}
    );
    expect(wrapper.vm.storedDeviceDataDict.slug3.lastLogin.c).toEqual(
      {year: 1999, month: 4, day: 28, hour: 17, minute: 50, second: 0, millisecond: 0}
    );
    expect(wrapper.vm.storedDeviceDataDict.slug5.lastLogin.c).toEqual(
      {year: 1999, month: 4, day: 28, hour: 13, minute: 0, second: 0, millisecond: 0}
    );
    expect(wrapper.vm.storedDeviceDataDict.slug6.lastLogin.c).toEqual(
      {year: 1999, month: 4, day: 26, hour: 18, minute: 0, second: 0, millisecond: 0}
    );
    expect(wrapper.vm.storedDeviceDataDict.slug7.lastLogin.c).toEqual(
      {year: 1999, month: 4, day: 18, hour: 18, minute: 0, second: 0, millisecond: 0}
    );
  });

  /*
  describe('Organization List tests', () => {
    let modalControllerCreateSpy: vi.SpyInstance;

    beforeAll(() => {
      modalControllerCreateSpy = vi.spyOn(modalController, 'create');
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
            timeSince: (date: DateTime | undefined, defaultValue=''): string => {
              const { t, d } = useI18n();
              return formatTimeSince(date, t, d, defaultValue);
            }
          },
          storageManager: storageManager
        }
      }
    });

    let searchInput: VueWrapper;

    it('should filter orgs', async () => {
      searchInput = wrapper.findComponent({name: 'SearchInput'}) as VueWrapper;
      expect(wrapper.vm.deviceList.length).toEqual(7);
      expect(wrapper.vm.orgSearchString).toEqual('');
      expect(wrapper.vm.sortBy).toEqual('organization');
      expect(wrapper.vm.sortByAsc).toEqual(true);
      expect(wrapper.vm.filteredDevices.length).toEqual(wrapper.vm.deviceList.length);

      // Should be ordered by ascending org name by default
      expect(wrapper.vm.filteredDevices[0].organizationId).toEqual('Black Mesa');
      expect(wrapper.vm.filteredDevices[6].organizationId).toEqual('Sanctum Sanctorum');

      expect(wrapper.findComponent('ion-input').exists()).toBeTruthy();
      await searchInput.vm.$emit('change', 'la');

      expect(wrapper.vm.orgSearchString).toEqual('la');
      expect(wrapper.vm.filteredDevices.length).toEqual(4);
      expect(wrapper.vm.filteredDevices).toEqual([{
        organizationId: 'Black Mesa',
        humanHandle: 'Dr. Gordon Freeman',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug3',
        ty: {tag: 'Password'}
      }, {
        organizationId: 'Planet Express',
        humanHandle: 'Dr. John A. Zoidberg',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug1',
        ty: {tag: 'Password'}
      },
      {
        organizationId: 'Princeton–Plainsboro Hospital',
        humanHandle: 'Dr. Gregory House',
        deviceLabel: 'device_label',
        keyFilePath: 'key_file_path',
        deviceId: 'device_id',
        slug: 'slug2',
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
      await searchInput.vm.$emit('change', '');
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
      expect(wrapper.vm.filteredDevices[4].slug).toEqual('slug3');
      expect(wrapper.vm.filteredDevices[6].slug).toEqual('slug1');

      // Sort by last login date ascending
      wrapper.vm.sortByAsc = true;
      expect(wrapper.vm.filteredDevices[0].slug).toEqual('slug1');
      expect(wrapper.vm.filteredDevices[2].slug).toEqual('slug3');
      expect(wrapper.vm.filteredDevices[6].slug).toEqual('slug4');

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
    let consoleLogSpyOn: vi.SpyInstance;

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
      consoleLogSpyOn = vi.spyOn(console, 'log');
      vi.useFakeTimers();
    });

    it('should update password value on password input change', async () => {
      passwordInput = wrapper.findComponent({name: 'PasswordInput'}) as VueWrapper;
      expect(wrapper.vm.password).toEqual('');
      await passwordInput.vm.$emit('change', 'password');
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
      const now = DateTime.now();
      vi.setSystemTime(now.toJSDate());
      await loginButton.trigger('click');
      const deviceStoredDataList = await storageManager.retrieveDevicesData();
      expect(deviceStoredDataList.slug4.lastLogin.toSeconds()).toBe(now.toSeconds());
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
      const now = DateTime.now();
      vi.setSystemTime(now.toJSDate());
      await passwordInput.vm.$emit('enter');
      const deviceStoredDataList = await storageManager.retrieveDevicesData();
      expect(deviceStoredDataList.slug4.lastLogin.toSeconds()).toBe(now.toSeconds());
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
  */
});
