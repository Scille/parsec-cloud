<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization">
    <ion-text
      class="organization-title title-h3"
      v-if="deviceList.length > 0"
    >
      {{ $msTranslate('HomePage.organizationList.title') }}
    </ion-text>
    <template v-if="deviceList.length === 0 && !querying">
      <!-- No organization -->
      <div class="organization-content no-devices">
        <div class="create-organization">
          <div class="create-organization-text">
            <ion-text class="create-organization-text__title title-h3">
              {{ $msTranslate('HomePage.noDevices.titleCreateOrga') }}
            </ion-text>
            <ion-text class="create-organization-text__subtitle body">{{ $msTranslate('HomePage.noDevices.subtitle') }}</ion-text>
            <ion-button
              @click="$emit('createOrganizationClick')"
              size="default"
              id="create-organization-button"
              class="button-default"
            >
              <ion-icon
                :icon="addCircle"
                class="icon"
              />
              {{ $msTranslate('HomePage.noExistingOrganization.createOrganization') }}
            </ion-button>
            <div class="recovery-no-devices">
              <ion-text class="body">{{ $msTranslate('HomePage.lostDevice') }}</ion-text>
              <ion-button
                @click="$emit('recoverClick')"
                fill="clear"
              >
                {{ $msTranslate('HomePage.recoverDevice') }}
              </ion-button>
            </div>
          </div>
          <ms-image
            :image="NoOrganization"
            class="create-organization-image"
          />
        </div>
        <div class="invitation">
          <ion-title class="invitation__title title-h4">
            {{ $msTranslate('HomePage.noDevices.titleInvitation') }}
          </ion-title>
          <div class="invitation-link">
            <ms-input
              label="HomePage.noDevices.invitationLink"
              placeholder="JoinOrganization.linkFormPlaceholder"
              v-model="link"
              @on-enter-keyup="onLinkClick"
              :validator="claimAndBootstrapLinkValidator"
              class="invitation-link__input"
              ref="linkRef"
            />
            <ion-button
              @click="onLinkClick(link)"
              size="large"
              fill="clear"
              id="join-organization-button"
              :disabled="!linkRef || linkRef.validity !== Validity.Valid"
            >
              {{ $msTranslate('HomePage.noDevices.invitationJoin') }}
            </ion-button>
          </div>
        </div>
      </div>
      <!-- enf of No organization -->
    </template>
    <template v-else>
      <div class="organization-content">
        <div class="organization-filter">
          <!-- No use in showing the sort/filter options for less than one device -->
          <ms-search-input
            :placeholder="'HomePage.organizationList.search'"
            v-model="searchQuery"
            id="search-input-organization"
            ref="searchInputRef"
          />
          <ms-sorter
            v-if="confLoaded"
            v-show="deviceList.length > 1"
            id="organization-filter-select"
            :label="'HomePage.organizationList.labelSortBy'"
            :options="msSorterOptions"
            :default-option="sortBy"
            :sort-by-asc="sortByAsc"
            :sorter-labels="msSorterLabels"
            @change="onMsSorterChange"
          />
          <!-- will be added with Parsec Auth -->
          <ion-button
            class="organization-add-button"
            @click="$emit('createOrJoinOrganizationClick', $event)"
            v-if="isSmallDisplay && false"
          >
            <ms-image
              :image="AddIcon"
              class="add-button-icon"
            />
          </ion-button>
        </div>
        <div class="organization-list">
          <ion-text
            class="no-match-result body"
            v-show="searchQuery.length > 0 && filteredDevices.length === 0 && deviceList.length > 0"
          >
            {{ $msTranslate({ key: 'HomePage.organizationList.noMatch', data: { query: searchQuery } }) }}
          </ion-text>
          <organization-card
            v-for="device in filteredDevices"
            :key="device.deviceId"
            class="organization-list-item"
            :device="device"
            :last-login-device="storedDeviceDataDict[device.deviceId]?.lastLogin"
            @click="$emit('organizationSelect', device)"
            :logged-in="loggedInDevices.find((info) => info.device.deviceId === device.deviceId) !== undefined"
          />
        </div>
      </div>
      <div class="recovery-devices">
        <ion-text class="body">{{ $msTranslate('HomePage.lostDevice') }}</ion-text>
        <ion-button
          @click="$emit('recoverClick')"
          fill="clear"
        >
          {{ $msTranslate('HomePage.recoverDevice') }}
        </ion-button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { claimAndBootstrapLinkValidator, bootstrapLinkValidator } from '@/common/validators';
import {
  MsImage,
  NoOrganization,
  MsSorter,
  MsOptions,
  MsSorterChangeEvent,
  MsSearchInput,
  MsInput,
  Validity,
  AddIcon,
  useWindowSize,
} from 'megashark-lib';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import { AvailableDevice, getLoggedInDevices, LoggedInDeviceInfo } from '@/parsec';
import { Routes } from '@/router';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import { IonButton, IonIcon, IonText, IonTitle } from '@ionic/vue';
import { addCircle } from 'ionicons/icons';
import { DateTime } from 'luxon';
import { computed, inject, onMounted, onUnmounted, ref, watch } from 'vue';

const emits = defineEmits<{
  (e: 'organizationSelect', device: AvailableDevice): void;
  (e: 'createOrganizationClick'): void;
  (e: 'joinOrganizationClick'): void;
  (e: 'joinOrganizationWithLinkClick', link: string): void;
  (e: 'bootstrapOrganizationWithLinkClick', link: string): void;
  (e: 'recoverClick'): void;
  (e: 'createOrJoinOrganizationClick', event: Event): void;
}>();

const props = defineProps<{
  deviceList: AvailableDevice[];
  querying: boolean;
}>();

enum SortCriteria {
  UserName = 'user_name',
  LastLogin = 'last_login',
  Organization = 'organization',
}

const ORGANIZATION_LIST_DATA_KEY = 'OrganizationList';

const { isSmallDisplay } = useWindowSize();
const storedDeviceDataDict = ref<{ [deviceId: string]: StoredDeviceData }>({});
const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const sortBy = ref(SortCriteria.Organization);
const sortByAsc = ref(true);
const confLoaded = ref(false);
const searchQuery = ref('');
const searchInputRef = ref();
const linkRef = ref();
const link = ref('');
const loggedInDevices = ref<Array<LoggedInDeviceInfo>>([]);

interface OrganizationListSavedData {
  sortByAsc?: boolean;
  sortBy?: SortCriteria;
}

watch(
  () => props.deviceList,
  async () => {
    loggedInDevices.value = await getLoggedInDevices();
  },
);

let hotkeys: HotkeyGroup | null = null;

const msSorterOptions: MsOptions = new MsOptions([
  {
    label: 'HomePage.organizationList.sortByOrganization',
    key: SortCriteria.Organization,
  },
  { label: 'HomePage.organizationList.sortByUserName', key: SortCriteria.UserName },
  { label: 'HomePage.organizationList.sortByLastLogin', key: SortCriteria.LastLogin },
]);

const msSorterLabels = {
  asc: 'HomePage.organizationList.sortOrderAsc',
  desc: 'HomePage.organizationList.sortOrderDesc',
};

async function onLinkClick(link: string): Promise<void> {
  if ((await bootstrapLinkValidator(link)).validity === Validity.Valid) {
    emits('bootstrapOrganizationWithLinkClick', link);
  } else {
    emits('joinOrganizationWithLinkClick', link);
  }
}

onMounted(async (): Promise<void> => {
  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add(
    { key: 'f', modifiers: Modifiers.Ctrl, platforms: Platforms.Web | Platforms.Desktop, disableIfModal: true, route: Routes.Home },
    async () => {
      if (searchInputRef.value) {
        searchInputRef.value.setFocus();
      }
    },
  );
  storedDeviceDataDict.value = await storageManager.retrieveDevicesData();
  const storedData = await storageManager.retrieveComponentData<OrganizationListSavedData>(ORGANIZATION_LIST_DATA_KEY, {
    sortBy: SortCriteria.Organization,
    sortByAsc: true,
  });
  sortBy.value = storedData.sortBy;
  sortByAsc.value = storedData.sortByAsc;
  confLoaded.value = true;
  loggedInDevices.value = await getLoggedInDevices();
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
});

async function onMsSorterChange(event: MsSorterChangeEvent): Promise<void> {
  sortBy.value = event.option.key;
  sortByAsc.value = event.sortByAsc;

  await storageManager.storeComponentData<OrganizationListSavedData>(ORGANIZATION_LIST_DATA_KEY, {
    sortBy: event.option.key,
    sortByAsc: event.sortByAsc,
  });
}

const filteredDevices = computed(() => {
  function deviceIsLoggedIn(device: AvailableDevice): boolean {
    return loggedInDevices.value.find((dInfo) => dInfo.device.deviceId === device.deviceId) !== undefined;
  }

  return props.deviceList
    .filter((item) => {
      const lowerSearchString = searchQuery.value.toLocaleLowerCase();
      return (
        item.humanHandle.label.toLocaleLowerCase().includes(lowerSearchString) ||
        item.organizationId.toLocaleLowerCase().includes(lowerSearchString)
      );
    })
    .sort((a, b) => {
      const loggedInWeight = (deviceIsLoggedIn(b) ? 3 : 0) - (deviceIsLoggedIn(a) ? 3 : 0);

      const aLabel = a.humanHandle.label;
      const bLabel = b.humanHandle.label;
      if (sortBy.value === SortCriteria.Organization) {
        if (sortByAsc.value) {
          // If orgs are the same, sort by user name
          if (a.organizationId === b.organizationId) {
            return loggedInWeight + aLabel.localeCompare(bLabel);
          }
          return loggedInWeight + a.organizationId.localeCompare(b.organizationId);
        } else {
          // If orgs are the same, sort by user name
          if (a.organizationId === b.organizationId) {
            return loggedInWeight + aLabel.localeCompare(bLabel);
          }
          return loggedInWeight + b.organizationId.localeCompare(a.organizationId);
        }
      } else if (sortBy.value === SortCriteria.UserName) {
        if (sortByAsc.value) {
          return loggedInWeight + aLabel.localeCompare(bLabel);
        } else {
          return loggedInWeight + bLabel.localeCompare(aLabel);
        }
      } else if (sortBy.value === SortCriteria.LastLogin) {
        const aLastLogin =
          a.deviceId in storedDeviceDataDict.value && storedDeviceDataDict.value[a.deviceId].lastLogin !== undefined
            ? storedDeviceDataDict.value[a.deviceId].lastLogin
            : DateTime.fromMillis(0);
        const bLastLogin =
          b.deviceId in storedDeviceDataDict.value && storedDeviceDataDict.value[b.deviceId].lastLogin !== undefined
            ? storedDeviceDataDict.value[b.deviceId].lastLogin
            : DateTime.fromMillis(0);
        let diff = 0;
        if (sortByAsc.value) {
          diff = bLastLogin.diff(aLastLogin).toObject().milliseconds ?? 0;
        } else {
          diff = aLastLogin.diff(bLastLogin).toObject().milliseconds ?? 0;
        }
        diff = Math.min(Math.max(diff, -1), 1);
        return loggedInWeight + diff;
      }
      return 0;
    });
});
</script>

<style lang="scss" scoped>
.organization {
  background: none;
  width: 100%;
  max-width: var(--parsec-max-content-width);
  box-shadow: none;
  display: flex;
  margin: 0;
  flex-direction: column;
  gap: 2rem;
  flex-grow: 1;

  &-title {
    padding: 0;
    color: var(--parsec-color-light-secondary-text);

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }
}

.organization-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  overflow: hidden;
  margin-bottom: 4.3125rem;
  width: 100%;
  max-width: var(--parsec-max-content-width);

  .organization-filter {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    gap: 1rem;
    max-width: 34.5rem;

    #search-input-organization:focus-within {
      outline: none;
      border: 1px solid var(--parsec-color-light-primary-300);
    }

    #organization-filter-select {
      margin-left: auto;
    }

    .organization-add-button {
      --background: var(--parsec-color-light-secondary-text);
      --background-hover: var(--parsec-color-light-secondary-contrast);
      width: 2.5rem;
      height: 2.5rem;

      &::part(native) {
        padding: 0.625rem;
        border-radius: var(--parsec-radius-8);
      }

      .add-button-icon {
        --fill-color: var(--parsec-color-light-secondary-white);
        width: fit-content;
      }
    }
  }

  .organization-list {
    margin: 0;
    overflow-y: auto;
    max-width: 34.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    position: relative;
    z-index: 100;

    @include ms.responsive-breakpoint('md') {
      margin: auto;
      width: 100%;
    }
  }
}

.recovery-devices {
  display: flex;
  align-items: center;
  position: fixed;
  border-top: 1px solid var(--parsec-color-light-secondary-medium);
  width: 100%;
  gap: 0.5rem;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  padding: 0.5rem 0.5rem 1.5rem;
  color: var(--parsec-color-light-secondary-grey);
  bottom: 0;
  z-index: 100;
}

.no-devices {
  max-width: 45rem;
  margin-bottom: 0.5rem;
  gap: 1.5rem;
  padding: 0;
  overflow: auto;

  .create-organization,
  .invitation {
    background: var(--parsec-color-light-secondary-white);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-12);
    display: flex;
  }

  .create-organization {
    align-items: center;
    gap: 1.5rem;
    padding: 3rem 2rem;

    @include ms.responsive-breakpoint('xs') {
      padding: 1.5rem;
    }

    &-text {
      display: flex;
      flex-direction: column;
      max-width: 24rem;

      @include ms.responsive-breakpoint('xs') {
        max-width: 100%;
      }

      &__title {
        color: var(--parsec-color-light-primary-700);
        margin-bottom: 1rem;
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-hard-grey);
        margin-bottom: 1.5rem;
      }

      #create-organization-button {
        width: fit-content;

        @include ms.responsive-breakpoint('md') {
          width: 100%;
        }

        ion-icon {
          margin-inline-end: 0.5rem;
        }
      }

      .recovery-no-devices {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: var(--parsec-color-light-secondary-grey);
        margin-top: 1rem;

        @include ms.responsive-breakpoint('sm') {
          flex-direction: column;
        }

        ion-text {
          @include ms.responsive-breakpoint('xs') {
            display: none;
          }
        }
      }
    }

    &-image {
      width: 100%;
      max-width: 10rem;
      margin: auto;

      @include ms.responsive-breakpoint('md') {
        max-width: 8rem;
      }

      @include ms.responsive-breakpoint('xs') {
        display: none;
      }
    }
  }

  .invitation {
    flex-direction: column;
    gap: 1.5rem;
    padding: 2rem 2rem 3rem;

    @include ms.responsive-breakpoint('xs') {
      padding: 1.5rem;
    }

    &__title {
      color: var(--parsec-color-light-primary-800);
      padding: 0;
    }

    .invitation-link {
      display: flex;
      gap: 1rem;
      align-items: flex-end;

      @include ms.responsive-breakpoint('xs') {
        flex-direction: column;
      }

      &__input {
        width: 100%;
        max-width: 30rem;
      }

      #join-organization-button {
        padding-bottom: 0.125rem;

        @include ms.responsive-breakpoint('xs') {
          width: 100%;
        }
      }
    }
  }
}
</style>
