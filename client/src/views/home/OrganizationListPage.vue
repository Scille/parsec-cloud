<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-card class="right-side-container">
    <template v-if="deviceList.length === 0 && !querying">
      <ion-card-content class="organization-container">
        <ion-card-title>
          {{ $t('HomePage.noDevices') }}
        </ion-card-title>
        {{ $t('HomePage.howToAddDevices') }}
      </ion-card-content>
    </template>
    <template v-if="deviceList.length > 0">
      <ion-card-content class="organization-container">
        <ion-card-title class="organization-filter">
          <ms-search-input
            :label="$t('HomePage.organizationList.search')"
            v-model="searchQuery"
            id="ms-search-input"
          />
          <!-- No use in showing the sort/filter options for less than 2 devices -->
          <template v-if="deviceList.length >= 2">
            <ms-sorter
              id="organization-filter-select"
              label="t('HomePage.organizationList.labelSortBy')"
              :options="msSorterOptions"
              default-option="organization"
              :sorter-labels="msSorterLabels"
              @change="onMsSorterChange($event)"
            />
          </template>
        </ion-card-title>
        <ion-grid class="organization-list">
          <ion-row class="organization-list-row">
            <ion-col
              v-for="device in filteredDevices"
              :key="device.slug"
              class="organization-list-row__col"
              size="2"
            >
              <ion-card
                button
                class="organization-card"
                @click="$emit('organizationSelect', device)"
              >
                <ion-card-content class="card-content">
                  <ion-grid>
                    <organization-card
                      :device="device"
                      class="card-content__body"
                    />
                    <ion-row class="card-content__footer">
                      <ion-col size="auto">
                        <p>
                          {{ $t('HomePage.organizationList.lastLogin') }}
                        </p>
                        <p>
                          {{ device.slug in storedDeviceDataDict ? timeSince(storedDeviceDataDict[device.slug].lastLogin, '--') : '--' }}
                        </p>
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
import { Formatters, FormattersKey } from '@/common/injectionKeys';
import { MsOptions, MsSearchInput, MsSorter, MsSorterChangeEvent } from '@/components/core';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import { AvailableDevice, listAvailableDevices } from '@/parsec';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import { IonCard, IonCardContent, IonCardTitle, IonCol, IonGrid, IonRow } from '@ionic/vue';
import { DateTime } from 'luxon';
import { Ref, computed, inject, onMounted, onUpdated, ref } from 'vue';
import { useI18n } from 'vue-i18n';

const emits = defineEmits<{
  (e: 'organizationSelect', device: AvailableDevice): void;
}>();

const { t } = useI18n();
const deviceList: Ref<AvailableDevice[]> = ref([]);
const storedDeviceDataDict = ref<{ [slug: string]: StoredDeviceData }>({});
const storageManager: StorageManager = inject(StorageManagerKey)!;
const { timeSince } = inject(FormattersKey)! as Formatters;
const sortBy = ref('organization');
const sortByAsc = ref(true);
const searchQuery = ref('');
const querying = ref(true);

const msSorterOptions: MsOptions = new MsOptions([
  {
    label: t('HomePage.organizationList.sortByOrganization'),
    key: 'organization',
  },
  { label: t('HomePage.organizationList.sortByUserName'), key: 'user_name' },
  { label: t('HomePage.organizationList.sortByLastLogin'), key: 'last_login' },
]);

const msSorterLabels = {
  asc: t('HomePage.organizationList.sortOrderAsc'),
  desc: t('HomePage.organizationList.sortOrderDesc'),
};

onMounted(async (): Promise<void> => {
  storedDeviceDataDict.value = await storageManager.retrieveDevicesData();
  await refreshDeviceList();
});

onUpdated(async (): Promise<void> => {
  await refreshDeviceList();
});

async function refreshDeviceList(): Promise<void> {
  querying.value = true;
  deviceList.value = await listAvailableDevices();
  if (deviceList.value.length === 1) {
    emits('organizationSelect', deviceList.value[0]);
  }
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
.right-side-container {
  height: 100%;
  box-shadow: none;
  flex-grow: 0;
  flex-shrink: 0;
  margin: 0;
  width: 60vw;
}

.organization-container {
  padding: 2rem 3.5rem 0;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-width: var(--parsec-max-content-width);

  .organization-filter {
    display: flex;
    justify-content: space-between;
    margin: 0;
  }

  .organization-list {
    max-height: 80%;
    margin: 0;
    overflow-y: auto;
    --ion-grid-columns: 6;
  }

  .organization-list-row {
    &__col {
      display: flex;
      align-items: center;
    }
  }

  .organization-card {
    background: var(--parsec-color-light-secondary-background);
    user-select: none;
    transition: box-shadow 150ms linear;
    box-shadow: none;
    border-radius: 0.5em;
    margin-inline: 0;
    margin-top: 0;
    margin-bottom: 0;
    width: 100%;

    &:hover {
      box-shadow: 0px 4px 12px rgba(0, 0, 0, 0.08);
    }

    .card-content {
      padding-top: 0px;
      padding-bottom: 0px;
      padding-inline-end: 0px;
      padding-inline-start: 0px;

      &__footer {
        padding: 0.5em 1em;
        background: var(--parsec-color-light-secondary-medium);
        border-top: 1px solid var(--parsec-color-light-secondary-disabled);
        color: var(--parsec-color-light-secondary-grey);
        height: 4.6em;
      }

      &:hover {
        background: var(--parsec-color-light-primary-50);
        cursor: pointer;

        .card-content__footer {
          background: var(--parsec-color-light-primary-50);
          border-top: 1px solid var(--parsec-color-light-primary-100);
        }
      }
    }
  }
}
</style>
