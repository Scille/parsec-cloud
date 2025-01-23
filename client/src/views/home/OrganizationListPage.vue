<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-card class="organization">
    <ion-text
      class="organization-title title-h1"
      v-if="deviceList.length > 0"
    >
      {{ $msTranslate('HomePage.organizationList.title') }}
    </ion-text>
    <template v-if="deviceList.length === 0 && !querying">
      <!-- No organization -->
      <ion-card-content class="organization-content no-devices">
        <div class="create-organization">
          <div class="create-organization-text">
            <ion-card-title class="create-organization-text__title title-h3">
              {{ $msTranslate('HomePage.noDevices.titleCreateOrga') }}
            </ion-card-title>
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
          </div>
          <ms-image
            :image="NoOrganization"
            class="create-organization-image"
          />
          <div class="recovery-no-devices">
            {{ $msTranslate('HomePage.lostDevice') }}
            <ion-button
              @click="$emit('recoverClick')"
              fill="clear"
            >
              {{ $msTranslate('HomePage.recoverDevice') }}
            </ion-button>
          </div>
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
              fill="default"
              id="join-organization-button"
              :disabled="!linkRef || linkRef.validity !== Validity.Valid"
            >
              {{ $msTranslate('HomePage.noDevices.invitationJoin') }}
            </ion-button>
          </div>
        </div>
      </ion-card-content>
      <!-- enf of No organization -->
    </template>
    <template v-else>
      <ion-card-content class="organization-content">
        <ion-card-title class="organization-filter">
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
          <ion-button
            @click="togglePopover"
            size="default"
            id="create-organization-button"
            class="button-default"
          >
            {{ $msTranslate('HomePage.noExistingOrganization.createOrJoin') }}
          </ion-button>
        </ion-card-title>
        <ion-grid class="organization-list">
          <ion-row class="organization-list-row">
            <ion-text
              class="no-match-result body"
              v-show="searchQuery.length > 0 && filteredDevices.length === 0 && deviceList.length > 0"
            >
              {{ $msTranslate({ key: 'HomePage.organizationList.noMatch', data: { query: searchQuery } }) }}
            </ion-text>
            <ion-col
              v-for="device in filteredDevices"
              :key="device.deviceId"
              class="organization-list-row__col"
              size="3"
            >
              <organization-card
                :device="device"
                :last-login-device="storedDeviceDataDict[device.deviceId]?.lastLogin"
                @click="$emit('organizationSelect', device)"
              />
            </ion-col>
          </ion-row>
        </ion-grid>
        <div class="recovery-devices">
          {{ $msTranslate('HomePage.lostDevice') }}
          <ion-button
            @click="$emit('recoverClick')"
            fill="clear"
          >
            {{ $msTranslate('HomePage.recoverDevice') }}
          </ion-button>
        </div>
      </ion-card-content>
    </template>
  </ion-card>
</template>

<script setup lang="ts">
import { claimAndBootstrapLinkValidator, bootstrapLinkValidator } from '@/common/validators';
import {
  MsImage,
  NoOrganization,
  MsSorter,
  MsModalResult,
  MsOptions,
  MsSorterChangeEvent,
  MsSearchInput,
  MsInput,
  Validity,
} from 'megashark-lib';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import { AvailableDevice, isDeviceLoggedIn, listAvailableDevices } from '@/parsec';
import { Routes } from '@/router';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import HomePageButtons, { HomePageAction } from '@/views/home/HomePageButtons.vue';
import {
  IonButton,
  IonCard,
  IonCardContent,
  IonCardTitle,
  IonCol,
  IonGrid,
  IonIcon,
  IonRow,
  IonText,
  IonTitle,
  popoverController,
} from '@ionic/vue';
import { addCircle } from 'ionicons/icons';
import { DateTime } from 'luxon';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';

const emits = defineEmits<{
  (e: 'organizationSelect', device: AvailableDevice): void;
  (e: 'createOrganizationClick'): void;
  (e: 'joinOrganizationClick'): void;
  (e: 'joinOrganizationWithLinkClick', link: string): void;
  (e: 'bootstrapOrganizationWithLinkClick', link: string): void;
  (e: 'recoverClick'): void;
}>();

defineExpose({
  refreshDeviceList,
});

enum SortCriteria {
  UserName = 'user_name',
  LastLogin = 'last_login',
  Organization = 'organization',
}

const ORGANIZATION_LIST_DATA_KEY = 'OrganizationList';

const deviceList: Ref<AvailableDevice[]> = ref([]);
const storedDeviceDataDict = ref<{ [deviceId: string]: StoredDeviceData }>({});
const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const sortBy = ref(SortCriteria.Organization);
const sortByAsc = ref(true);
const confLoaded = ref(false);
const searchQuery = ref('');
const querying = ref(true);
const searchInputRef = ref();
const linkRef = ref();
const link = ref('');

interface OrganizationListSavedData {
  sortByAsc?: boolean;
  sortBy?: SortCriteria;
}

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

const isPopoverOpen = ref(false);

async function onLinkClick(link: string): Promise<void> {
  if ((await bootstrapLinkValidator(link)).validity === Validity.Valid) {
    emits('bootstrapOrganizationWithLinkClick', link);
  } else {
    emits('joinOrganizationWithLinkClick', link);
  }
}

async function togglePopover(event: Event): Promise<void> {
  isPopoverOpen.value = !isPopoverOpen.value;
  openPopover(event);
}

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: HomePageButtons,
    cssClass: 'homepage-popover',
    event: event,
    showBackdrop: false,
    alignment: 'end',
    componentProps: {
      replaceEmit: dismissPopover,
    },
  });
  await popover.present();
  const result = await popover.onWillDismiss();
  await popover.dismiss();
  if (result.role !== MsModalResult.Confirm) {
    return;
  }
  onAction(result.data.action);
}

async function dismissPopover(action: HomePageAction): Promise<void> {
  await popoverController.dismiss({ action: action }, MsModalResult.Confirm);
}

async function onAction(action: HomePageAction): Promise<void> {
  if (action === HomePageAction.CreateOrganization) {
    emits('createOrganizationClick');
  } else if (action === HomePageAction.JoinOrganization) {
    emits('joinOrganizationClick');
  }
}

onMounted(async (): Promise<void> => {
  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add(
    { key: 'f', modifiers: Modifiers.Ctrl, platforms: Platforms.Web | Platforms.Desktop, disableIfModal: true, route: Routes.Home },
    searchInputRef.value.setFocus,
  );
  storedDeviceDataDict.value = await storageManager.retrieveDevicesData();
  const storedData = await storageManager.retrieveComponentData<OrganizationListSavedData>(ORGANIZATION_LIST_DATA_KEY, {
    sortBy: SortCriteria.Organization,
    sortByAsc: true,
  });
  sortBy.value = storedData.sortBy;
  sortByAsc.value = storedData.sortByAsc;
  confLoaded.value = true;
  await refreshDeviceList();
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
});

async function refreshDeviceList(): Promise<void> {
  querying.value = true;
  deviceList.value = await listAvailableDevices();
  querying.value = false;
}

async function onMsSorterChange(event: MsSorterChangeEvent): Promise<void> {
  sortBy.value = event.option.key;
  sortByAsc.value = event.sortByAsc;

  await storageManager.storeComponentData<OrganizationListSavedData>(ORGANIZATION_LIST_DATA_KEY, {
    sortBy: event.option.key,
    sortByAsc: event.sortByAsc,
  });
}

const filteredDevices = computed(() => {
  return deviceList.value
    .filter((item) => {
      const lowerSearchString = searchQuery.value.toLocaleLowerCase();
      return (
        item.humanHandle.label.toLocaleLowerCase().includes(lowerSearchString) ||
        item.organizationId.toLocaleLowerCase().includes(lowerSearchString)
      );
    })
    .sort((a, b) => {
      const loggedInWeight = (isDeviceLoggedIn(b) ? 3 : 0) - (isDeviceLoggedIn(a) ? 3 : 0);

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
  height: auto;
  width: 60vw;
  max-width: var(--parsec-max-organization-width);
  margin: auto;
  box-shadow: none;
  display: flex;
  align-items: center;
  flex-direction: column;
  gap: 2rem;

  &-title {
    text-align: center;
    padding: 0;
    display: flex;
    justify-content: center;
    color: var(--parsec-color-light-secondary-white);
  }
}

.organization-content {
  display: flex;
  padding: 2rem 2.5rem 0.5rem;
  flex-direction: column;
  gap: 1.5rem;
  max-width: var(--parsec-max-content-width);
  background: var(--parsec-color-light-secondary-white);
  border-radius: var(--parsec-radius-12);
  width: 100%;

  .organization-filter {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-inline: 0.5rem;

    #organization-filter-select {
      margin-left: auto;
      margin-right: 1rem;
    }
  }

  .organization-list {
    margin: 0;
    overflow-y: auto;
    --ion-grid-columns: 6;
    max-height: 50vh;
    min-height: 45vh;
  }

  .organization-list-row {
    &__col {
      display: flex;
      align-items: center;
      padding: 0.5rem;
    }
  }
}

.no-devices {
  max-width: 45rem;
  gap: 0;
  padding: 0;
  overflow: hidden;
  margin: auto;

  .create-organization {
    display: flex;
    align-items: center;
    padding: 3rem 2rem;

    &-text {
      display: flex;
      flex-direction: column;
      max-width: 24rem;

      &__title {
        color: var(--parsec-color-light-primary-700);
        margin-bottom: 1rem;
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-grey);
        margin-bottom: 1.5rem;
      }

      #create-organization-button {
        width: fit-content;

        ion-icon {
          margin-inline-end: 0.5rem;
        }
      }
    }

    &-image {
      width: 100%;
      max-width: 10rem;
      margin: auto;
    }
  }

  .invitation {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    padding: 2rem 2rem 3rem;
    background: var(--parsec-color-light-secondary-background);

    &__title {
      color: var(--parsec-color-light-primary-800);
      padding: 0;
    }

    .invitation-link {
      display: flex;
      gap: 1rem;
      align-items: flex-end;

      &__input {
        width: 100%;
        max-width: 30rem;
      }

      #join-organization-button {
        padding-bottom: 0.125rem;
        color: var(--parsec-color-light-secondary-white);
      }
    }
  }
}
</style>
