<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-card class="organization">
    <ion-text
      class="organization-title title-h1"
      v-if="deviceList.length > 0"
    >
      {{ $t('HomePage.organizationList.title') }}
    </ion-text>
    <template v-if="deviceList.length === 0 && !querying">
      <!-- No organization -->
      <ion-card-content class="organization-content no-devices">
        <div class="create-orga">
          <div class="create-orga-text">
            <ion-card-title class="create-orga-text__title title-h3">
              {{ $t('HomePage.noDevices.titleCreateOrga') }}
            </ion-card-title>
            <ion-text class="create-orga-text__subtitle body">{{ $t('HomePage.noDevices.subtitle') }}</ion-text>
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
              {{ $t('HomePage.noExistingOrganization.createOrganization') }}
            </ion-button>
          </div>
          <ms-image
            :image="NoOrganization"
            class="create-orga-image"
          />
        </div>
        <div class="invitation">
          <ion-title class="invitation__title title-h4">
            {{ $t('HomePage.noDevices.titleInvitation') }}
          </ion-title>
          <div class="invitation-link">
            <ms-input
              :label="$t('HomePage.noDevices.invitationLink')"
              :placeholder="$t('JoinOrganization.linkFormPlaceholder')"
              v-model="link"
              @on-enter-keyup="$emit('joinOrganizationWithLinkClick', link)"
              :validator="claimLinkValidator"
              class="invitation-link__input"
              ref="linkRef"
            />
            <ion-button
              @click="$emit('joinOrganizationWithLinkClick', link)"
              size="large"
              fill="default"
              id="join-organization-button"
              :disabled="!linkRef || linkRef.validity !== Validity.Valid"
            >
              {{ $t('HomePage.noDevices.invitationJoin') }}
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
            :placeholder="$t('HomePage.organizationList.search')"
            v-model="searchQuery"
            id="ms-search-input"
            ref="searchInputRef"
          />
          <ms-sorter
            v-show="deviceList.length > 1"
            id="organization-filter-select"
            label="t('HomePage.organizationList.labelSortBy')"
            :options="msSorterOptions"
            default-option="organization"
            :sorter-labels="msSorterLabels"
            @change="onMsSorterChange($event)"
          />
          <ion-button
            @click="togglePopover"
            size="default"
            id="create-organization-button"
            class="button-default"
          >
            {{ $t('HomePage.noExistingOrganization.createOrJoin') }}
          </ion-button>
        </ion-card-title>
        <ion-grid class="organization-list">
          <ion-row class="organization-list-row">
            <ion-col
              v-for="device in filteredDevices"
              :key="device.slug"
              class="organization-list-row__col"
              size="3"
            >
              <ion-card
                button
                class="organization-card"
                @click="$emit('organizationSelect', device)"
              >
                <ion-card-content class="card-content">
                  <ion-grid class="card-content-grid">
                    <organization-card
                      :device="device"
                      class="card-content-body"
                    />
                    <ion-row class="card-content-footer">
                      <ion-col
                        size="auto"
                        v-show="!isDeviceLoggedIn(device)"
                        class="card-content-footer-login"
                      >
                        <ion-icon
                          :icon="time"
                          class="time"
                        />
                        <ion-text class="body-sm">
                          {{
                            device.slug in storedDeviceDataDict ? formatTimeSince(storedDeviceDataDict[device.slug].lastLogin, '--') : '--'
                          }}
                        </ion-text>
                      </ion-col>
                      <ion-col
                        v-show="isDeviceLoggedIn(device)"
                        class="body connected"
                      >
                        <ion-icon
                          :icon="ellipse"
                          class="success"
                        />
                        {{ $t('HomePage.organizationList.loggedIn') }}
                      </ion-col>
                    </ion-row>
                  </ion-grid>
                </ion-card-content>
              </ion-card>
            </ion-col>
          </ion-row>
        </ion-grid>
      </ion-card-content>
    </template>
  </ion-card>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import { Validity, claimLinkValidator } from '@/common/validators';
import { MsModalResult, MsOptions, MsSearchInput, MsSorter, MsSorterChangeEvent } from '@/components/core';
import { MsImage, NoOrganization } from '@/components/core/ms-image';
import { MsInput } from '@/components/core/ms-input';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import { AvailableDevice, isDeviceLoggedIn, listAvailableDevices } from '@/parsec';
import { Groups, HotkeyManager, HotkeyManagerKey, Hotkeys, Modifiers, Platforms } from '@/services/hotkeyManager';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import { translate } from '@/services/translation';
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
import { addCircle, ellipse, time } from 'ionicons/icons';
import { DateTime } from 'luxon';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';

const emits = defineEmits<{
  (e: 'organizationSelect', device: AvailableDevice): void;
  (e: 'createOrganizationClick'): void;
  (e: 'joinOrganizationClick'): void;
  (e: 'joinOrganizationWithLinkClick', link: string): void;
}>();

const deviceList: Ref<AvailableDevice[]> = ref([]);
const storedDeviceDataDict = ref<{ [slug: string]: StoredDeviceData }>({});
const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const sortBy = ref('organization');
const sortByAsc = ref(true);
const searchQuery = ref('');
const querying = ref(true);
const searchInputRef = ref();
const linkRef = ref();
const link = ref('');

let hotkeys: Hotkeys | null = null;

const msSorterOptions: MsOptions = new MsOptions([
  {
    label: translate('HomePage.organizationList.sortByOrganization'),
    key: 'organization',
  },
  { label: translate('HomePage.organizationList.sortByUserName'), key: 'user_name' },
  { label: translate('HomePage.organizationList.sortByLastLogin'), key: 'last_login' },
]);

const msSorterLabels = {
  asc: translate('HomePage.organizationList.sortOrderAsc'),
  desc: translate('HomePage.organizationList.sortOrderDesc'),
};

const isPopoverOpen = ref(false);

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
  hotkeys = hotkeyManager.newHotkeys(Groups.Home);
  hotkeys.add('f', Modifiers.Ctrl, Platforms.Web | Platforms.Desktop, searchInputRef.value.setFocus);
  storedDeviceDataDict.value = await storageManager.retrieveDevicesData();
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

function onMsSorterChange(event: MsSorterChangeEvent): void {
  sortBy.value = event.option.key;
  sortByAsc.value = event.sortByAsc;
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
      const aLabel = a.humanHandle.label;
      const bLabel = b.humanHandle.label;
      if (sortBy.value === 'organization') {
        if (sortByAsc.value) {
          return a.organizationId.localeCompare(b.organizationId);
        } else {
          return b.organizationId.localeCompare(a.organizationId);
        }
      } else if (sortBy.value === 'user_name' && aLabel && bLabel) {
        if (sortByAsc.value) {
          return aLabel?.localeCompare(bLabel ?? '');
        } else {
          return bLabel?.localeCompare(aLabel ?? '');
        }
      } else if (sortBy.value === 'last_login') {
        const aLastLogin =
          a.slug in storedDeviceDataDict.value && storedDeviceDataDict.value[a.slug].lastLogin !== undefined
            ? storedDeviceDataDict.value[a.slug].lastLogin
            : DateTime.fromMillis(0);
        const bLastLogin =
          b.slug in storedDeviceDataDict.value && storedDeviceDataDict.value[b.slug].lastLogin !== undefined
            ? storedDeviceDataDict.value[b.slug].lastLogin
            : DateTime.fromMillis(0);
        if (sortByAsc.value) {
          return bLastLogin.diff(aLastLogin).toObject().milliseconds!;
        } else {
          return aLastLogin.diff(bLastLogin).toObject().milliseconds!;
        }
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
    min-height: 50vh;
  }

  .organization-list-row {
    &__col {
      display: flex;
      align-items: center;
      padding: 0.5rem;
    }
  }

  .organization-card {
    background: var(--parsec-color-light-secondary-background);
    user-select: none;
    transition: box-shadow 150ms linear;
    border: 1px solid var(--parsec-color-light-secondary-medium);
    box-shadow: none;
    border-radius: 0.5em;
    margin-inline: 0;
    margin-top: 0;
    margin-bottom: 0;
    width: 100%;
    cursor: pointer;

    &:hover {
      box-shadow: var(--parsec-shadow-light);
    }

    .card-content {
      padding: 1rem;

      &-body {
        padding-bottom: 0.75rem;
      }

      &-footer {
        color: var(--parsec-color-light-secondary-grey);

        &-login {
          display: flex;
          align-items: center;
          gap: 0.5rem;
          padding: 0;

          .time {
            color: var(--parsec-color-light-secondary-light);
            font-size: 1.125rem;
          }
        }

        .connected {
          display: flex;
          align-items: center;
          gap: 0.5rem;

          .success {
            color: var(--parsec-color-light-success-700);
            font-size: 0.675rem;
          }
        }
      }

      &:hover {
        background: var(--parsec-color-light-primary-50);

        .card-content-footer {
          background: var(--parsec-color-light-primary-50);
        }
      }
    }
  }
}

.no-devices {
  min-height: 50vh;
  max-width: 45rem;
  gap: 0;
  padding: 0;
  overflow: hidden;

  .create-orga {
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
