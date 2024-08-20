<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="client-contract-page">
    <span
      class="form-error body"
      v-show="error"
    >
      {{ $msTranslate(error) }}
    </span>

    <template v-if="!isDefaultOrganization(organization)">
      <div
        class="contract"
        v-if="contractDetails && organizationStats && !error"
      >
        <div class="contract-header">
          <div class="contract-header-title">
            <ion-text class="contract-header-title__text title-h3">
              {{ $msTranslate('clientArea.invoicesCustomOrder.contract') }}{{ contractDetails.id }}
            </ion-text>
            <ion-text class="contract-header-title__badge button-small">
              {{ $msTranslate(getContractStatus()) }}
            </ion-text>
          </div>
          <div class="contract-header-invoice">
            <ms-image
              class="contract-header-invoice__image"
              alt="pdf"
              :image="File.Pdf"
            />
            <ion-text class="contract-header-invoice__text title-h4">
              <span class="body">{{ $msTranslate('clientArea.invoicesCustomOrder.number') }}</span>
              {{ contractDetails.number }}
            </ion-text>
            <div class="contract-header-invoice__button">
              <a
                :href="contractDetails.link"
                class="custom-button custom-button-fill button-medium"
                download
              >
                {{ $msTranslate('clientArea.invoicesCustomOrder.download') }}
              </a>
            </div>
          </div>
        </div>

        <div class="contract-main">
          <!-- info -->
          <div class="contract-main-item">
            <div class="item-header">
              <ion-icon
                class="item-header__icon"
                :icon="informationCircle"
              />
              <ion-title class="item-header__title title-h4">
                {{ $msTranslate('clientArea.invoicesCustomOrder.information') }}
              </ion-title>
            </div>
            <div class="item-content">
              <div class="item-content-column">
                <div class="item-content-date">
                  <ion-text class="item-content-date__label body">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.startingDate') }}
                  </ion-text>
                  <ion-text
                    class="item-content-date__date title-h4"
                    v-if="contractDetails.licenseStart"
                  >
                    {{ $msTranslate(I18n.formatDate(contractDetails.licenseStart, 'short')) }}
                  </ion-text>
                  <ion-text v-else>
                    {{ $msTranslate('clientArea.contracts.errors.unavailable') }}
                  </ion-text>
                </div>
                <div class="item-content-date">
                  <ion-text class="item-content-date__label body">{{ $msTranslate('clientArea.invoicesCustomOrder.endingDate') }}</ion-text>
                  <ion-text
                    class="item-content-date__date title-h4"
                    v-if="contractDetails.licenseEnd"
                  >
                    {{ $msTranslate(I18n.formatDate(contractDetails.licenseEnd, 'short')) }}
                  </ion-text>
                  <ion-text v-else>
                    {{ $msTranslate('clientArea.contracts.errors.unavailable') }}
                  </ion-text>
                </div>
              </div>
            </div>
          </div>

          <!-- user -->
          <div class="contract-main-item">
            <div class="item-header">
              <ion-icon
                class="item-header__icon"
                :icon="person"
              />
              <ion-title class="item-header__title title-h4">{{ $msTranslate('clientArea.invoicesCustomOrder.members.title') }}</ion-title>
            </div>
            <div class="item-content">
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title subtitles-normal">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.members.admin') }}
                  </ion-text>
                  <ion-text class="data-text__description body">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.members.adminDescription') }}
                  </ion-text>
                </div>
                <ion-text class="data-number title-h2">{{ contractDetails.administrators.quantityOrdered }}</ion-text>
              </div>
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title subtitles-normal">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.members.standard') }}
                  </ion-text>
                  <ion-text class="data-text__description body">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.members.standardDescription') }}
                  </ion-text>
                </div>
                <ion-text class="data-number title-h2">{{ contractDetails.standards.quantityOrdered }}</ion-text>
              </div>
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title subtitles-normal">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.members.external') }}
                  </ion-text>
                  <ion-text class="data-text__description body">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.members.externalDescription') }}
                  </ion-text>
                </div>
                <ion-text class="data-number title-h2">{{ contractDetails.outsiders.quantityOrdered }}</ion-text>
              </div>
            </div>
          </div>

          <!-- storage -->
          <div class="contract-main-item">
            <div class="item-header">
              <ion-icon
                class="item-header__icon"
                :icon="pieChart"
              />
              <ion-title class="item-header__title title-h4">
                {{ $msTranslate('clientArea.invoicesCustomOrder.storage.title') }}
              </ion-title>
            </div>
            <div class="item-content">
              <div class="item-content-row">
                <div class="data-text">
                  <ion-text class="data-text__title subtitles-normal">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.storage.storage') }}
                  </ion-text>
                  <ion-text class="data-text__description body">
                    {{ $msTranslate('clientArea.invoicesCustomOrder.storage.stack') }}
                  </ion-text>
                </div>
                <ion-text class="data-number title-h2">{{ $msTranslate(formatFileSize(getStorageSize())) }}</ion-text>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div
        class="organization-content"
        v-if="organizationStats && contractDetails"
      >
        <ion-title class="organization-content__title title-h3">{{ $msTranslate('clientArea.contracts.previewOrganization') }}</ion-title>
        <div
          class="organization-users"
          v-if="organizationStats.adminUsersDetail || organizationStats.standardUsersDetail || organizationStats.outsiderUsersDetail"
        >
          <!-- admins-->
          <div
            class="organization-users-item"
            v-if="organizationStats.adminUsersDetail"
          >
            <div class="item-licence">
              <ion-icon
                :icon="person"
                class="item-licence__icon"
              />
              <div class="item-licence-text">
                <ion-text class="item-licence-text__title title-h4">{{ $msTranslate('clientArea.contracts.user.admin') }}</ion-text>
                <ion-text class="item-licence-text__subtitle subtitles-sm">
                  {{ $msTranslate('clientArea.contracts.user.fullAccess') }}
                </ion-text>
              </div>
            </div>
            <div class="item-active">
              <ion-text class="item-active__number title-h1-xl">
                {{ organizationStats.adminUsersDetail.active }}
                <span class="item-active__label subtitles-normal">
                  {{
                    $msTranslate({
                      key: 'clientArea.contracts.user.onTotal',
                      data: { count: contractDetails.administrators.quantityOrdered },
                    })
                  }}
                </span>
              </ion-text>
              <div class="progress">
                <div class="progress-bar">
                  <div class="progress-bar-used" />
                  <div class="progress-bar-unused" />
                </div>
                <ion-text class="progress-text subtitles-sm">
                  {{ Math.max(contractDetails.administrators.quantityOrdered - organizationStats.adminUsersDetail.active, 0) }}
                  {{ $msTranslate('clientArea.contracts.user.remaining') }}
                </ion-text>
              </div>
            </div>
          </div>
          <!-- standards-->
          <div
            class="organization-users-item"
            v-if="organizationStats.standardUsersDetail"
          >
            <div class="item-licence">
              <ion-icon
                :icon="person"
                class="item-licence__icon"
              />
              <div class="item-licence-text">
                <ion-text class="item-licence-text__title title-h4">{{ $msTranslate('clientArea.contracts.user.standard') }}</ion-text>
                <ion-text class="item-licence-text__subtitle subtitles-sm">
                  {{ $msTranslate('clientArea.contracts.user.allFeaturesAccess') }}
                </ion-text>
              </div>
            </div>
            <div class="item-active">
              <ion-text class="item-active__number title-h1-xl">
                {{ organizationStats.standardUsersDetail.active }}
                <span class="item-active__label subtitles-normal">
                  {{
                    $msTranslate({ key: 'clientArea.contracts.user.onTotal', data: { count: contractDetails.standards.quantityOrdered } })
                  }}
                </span>
              </ion-text>
              <div class="progress">
                <div class="progress-bar">
                  <div class="progress-bar-used" />
                  <div class="progress-bar-unused" />
                </div>
                <ion-text class="progress-text subtitles-sm">
                  {{ Math.max(contractDetails.standards.quantityOrdered - organizationStats.standardUsersDetail.active, 0) }}
                  {{ $msTranslate('clientArea.contracts.user.remaining') }}
                </ion-text>
              </div>
            </div>
          </div>
          <!-- externals-->
          <div
            class="organization-users-item"
            v-if="organizationStats.outsiderUsersDetail"
          >
            <div class="item-licence">
              <ion-icon
                :icon="person"
                class="item-licence__icon"
              />
              <div class="item-licence-text">
                <ion-text class="item-licence-text__title title-h4">{{ $msTranslate('clientArea.contracts.user.external') }}</ion-text>
                <ion-text class="item-licence-text__subtitle subtitles-sm">
                  {{ $msTranslate('clientArea.contracts.user.rightLimited') }}
                </ion-text>
              </div>
            </div>
            <div class="item-active">
              <ion-text class="item-active__number title-h1-xl">
                {{ organizationStats.outsiderUsersDetail.active }}
                <span class="item-active__label subtitles-normal">
                  {{
                    $msTranslate({ key: 'clientArea.contracts.user.onTotal', data: { count: contractDetails.outsiders.quantityOrdered } })
                  }}
                </span>
              </ion-text>
              <div class="progress">
                <div class="progress-bar">
                  <div class="progress-bar-used" />
                  <div class="progress-bar-unused" />
                </div>
                <ion-text class="progress-text subtitles-sm">
                  {{ Math.max(contractDetails.outsiders.quantityOrdered - organizationStats.outsiderUsersDetail.active, 0) }}
                  {{ $msTranslate('clientArea.contracts.user.remaining') }}
                </ion-text>
              </div>
            </div>
          </div>
        </div>

        <div class="organization-storage">
          <!-- storage-->
          <div class="organization-storage-item">
            <div class="item-licence">
              <ion-icon
                :icon="pieChart"
                class="item-licence__icon"
              />
              <div class="item-licence-text">
                <ion-text class="item-licence-text__title title-h4">{{ $msTranslate('clientArea.contracts.storage.title') }}</ion-text>
                <ion-text class="item-licence-text__subtitle subtitles-sm">
                  {{ $msTranslate('clientArea.contracts.storage.stack') }}
                </ion-text>
              </div>
            </div>
            <div class="item-active">
              <ion-text class="item-active__number title-h1-xl">
                {{ $msTranslate(formatFileSize(organizationStats.metadataSize + organizationStats.dataSize)) }}
                <span class="item-active__label subtitles-normal">
                  {{
                    $msTranslate({
                      key: 'clientArea.contracts.storage.onTotal',
                      data: { count: I18n.translate(formatFileSize(getStorageSize())) },
                    })
                  }}
                </span>
              </ion-text>
              <div class="progress">
                <div class="progress-bar">
                  <div class="progress-bar-used" />
                  <div class="progress-bar-unused" />
                </div>
                <ion-text class="progress-text subtitles-sm">
                  {{ $msTranslate({ key: 'clientArea.contracts.storage.used', data: { percentage: getStoragePercentage() } }) }}
                </ion-text>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
    <template v-else>
      {{ $msTranslate('clientArea.contracts.selectAnOrganization') }}
      <span
        v-for="org in organizations"
        :key="org.bmsId"
        @click="$emit('organizationSelected', org)"
      >
        {{ org.parsecId }}
      </span>
    </template>
  </div>
</template>

<script setup lang="ts">
import { IonText, IonIcon, IonTitle } from '@ionic/vue';
import { formatFileSize } from '@/common/file';
// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { MsImage, File, I18n, Translatable } from 'megashark-lib';
import { informationCircle, person, pieChart } from 'ionicons/icons';
import {
  BmsAccessInstance,
  BmsOrganization,
  CustomOrderStatus,
  CustomOrderDetailsResultData,
  DataType,
  OrganizationStatsResultData,
} from '@/services/bms';
import { onMounted, ref } from 'vue';
import { isDefaultOrganization } from '@/views/client-area/types';

const props = defineProps<{
  organization: BmsOrganization;
}>();

defineEmits<{
  (e: 'organizationSelected', organization: BmsOrganization): void;
}>();

const contractStatus = ref(CustomOrderStatus.Unknown);
const contractDetails = ref<CustomOrderDetailsResultData | undefined>(undefined);
const querying = ref(true);
const organizations = ref<Array<BmsOrganization>>([]);
const error = ref('');
const organizationStats = ref<OrganizationStatsResultData | undefined>(undefined);

onMounted(async () => {
  querying.value = true;
  if (isDefaultOrganization(props.organization)) {
    const orgRep = await BmsAccessInstance.get().listOrganizations();
    if (!orgRep.isError && orgRep.data && orgRep.data.type === DataType.ListOrganizations) {
      organizations.value = orgRep.data.organizations;
    } else {
      error.value = 'clientArea.contracts.errors.noOrganizations';
    }
  } else {
    const orgRep = await BmsAccessInstance.get().getOrganizationStats(props.organization.bmsId);
    if (!orgRep.isError && orgRep.data && orgRep.data.type === DataType.OrganizationStats) {
      organizationStats.value = orgRep.data;
    } else {
      error.value = 'clientArea.contracts.errors.noInfo';
    }
    const statusRep = await BmsAccessInstance.get().getCustomOrderStatus(props.organization);
    if (!statusRep.isError && statusRep.data && statusRep.data.type === DataType.CustomOrderStatus) {
      contractStatus.value = statusRep.data.status;
    }
    const detailsRep = await BmsAccessInstance.get().getCustomOrderDetails(props.organization);
    if (!detailsRep.isError && detailsRep.data && detailsRep.data.type === DataType.CustomOrderDetails) {
      contractDetails.value = detailsRep.data;
    } else {
      error.value = 'clientArea.contracts.errors.noInfo';
    }
  }
  querying.value = false;
});

function getStorageSize(): number {
  if (!contractDetails.value) {
    return 0;
  }
  // Slice of 100Go
  return contractDetails.value.storage.quantityOrdered * 100 * 1024 * 1024 * 1024;
}

function getStoragePercentage(): number {
  if (!organizationStats.value) {
    return 0;
  }
  const ordered = getStorageSize();
  const current = organizationStats.value.dataSize + organizationStats.value.metadataSize;

  return Math.round((current / ordered) * 100);
}

function getContractStatus(): Translatable {
  switch (contractStatus.value) {
    case CustomOrderStatus.Unknown:
      return 'clientArea.invoicesCustomOrder.contractStatus.unknown';
    case CustomOrderStatus.ContractEnded:
      return 'clientArea.invoicesCustomOrder.contractStatus.contractEnded';
    case CustomOrderStatus.EstimateLinked:
      return 'clientArea.invoicesCustomOrder.contractStatus.estimateLinked';
    case CustomOrderStatus.InvoicePaid:
      return 'clientArea.invoicesCustomOrder.contractStatus.invoicePaid';
    case CustomOrderStatus.InvoiceToBePaid:
      return 'clientArea.invoicesCustomOrder.contractStatus.invoiceToBePaid';
    case CustomOrderStatus.NothingLinked:
      return 'clientArea.invoicesCustomOrder.contractStatus.nothingLinked';
  }
}
</script>

<style scoped lang="scss">
.client-contract-page {
  display: flex;
  flex-direction: column;
  gap: 2.5rem;
  padding-bottom: 3rem;
}

.contract {
  width: 100%;
  max-width: 70rem;
  height: fit-content;
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12);
  overflow: hidden;
}

.contract-header {
  display: flex;
  justify-content: space-between;
  width: 100%;
  background: var(--parsec-color-light-secondary-background);
  padding: 0.75rem 1rem;

  &-title {
    display: flex;
    gap: 1rem;
    align-items: center;

    &__text {
      color: var(--parsec-color-light-secondary-text);
    }

    &__badge {
      color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-primary-100);
      padding: 0.25rem 0.5rem;
      border-radius: var(--parsec-radius-32);
    }
  }

  &-invoice {
    display: flex;
    gap: 1rem;
    max-width: 20rem;
    width: 100%;
    align-items: center;

    &__image {
      width: 2rem;
      height: 2rem;
    }

    &__text {
      color: var(--parsec-color-light-secondary-text);
      display: flex;
      flex-direction: column;

      span {
        color: var(--parsec-color-light-secondary-grey);
      }
    }

    &__button {
      margin-left: auto;

      .custom-button {
        background: var(--parsec-color-light-secondary-contrast);
        color: var(--parsec-color-light-secondary-white);

        &:hover {
          background: var(--parsec-color-light-secondary-text);
        }
      }
    }
  }
}

.contract-main {
  display: flex;
  gap: 2rem;
  padding: 1.5rem 2rem 2rem;

  &-item {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    width: 100%;

    &:has(.item-content-date) {
      width: fit-content;
      min-width: 12rem;
    }
  }

  .item-header {
    display: flex;
    gap: 0.75rem;
    align-items: center;

    &__icon {
      color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-primary-30);
      font-size: 1.5rem;
      padding: 0.25rem;
      border-radius: var(--parsec-radius-4);
    }

    &__title {
      color: var(--parsec-color-light-primary-900);
    }
  }

  .item-content {
    display: flex;
    flex-direction: column;

    &-row {
      display: flex;
      gap: 2rem;
      justify-content: space-between;
      min-height: 2.5rem;
      padding-left: 2.5rem;
    }

    &-column {
      display: flex;
      flex-direction: column;
      padding-left: 2.5rem;
      gap: 1rem;
    }

    &-date {
      display: flex;
      flex-direction: column;
      gap: 0.75rem;

      &__label {
        color: var(--parsec-color-light-secondary-grey);
      }

      &__date {
        color: var(--parsec-color-light-secondary-text);
      }
    }

    .data-text {
      display: flex;
      flex-direction: column;
      padding: 0.5rem 0;

      &__title {
        color: var(--parsec-color-light-secondary-soft-text);
      }

      &__description {
        color: var(--parsec-color-light-secondary-grey);
      }
    }

    .data-number {
      padding: 0.25rem 0;
      color: var(--parsec-color-light-primary-600);
    }
  }
}

.organization-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  &__title {
    color: var(--parsec-color-light-primary-700);
  }
}

.organization-users,
.organization-storage {
  display: flex;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  gap: 3rem;
  padding: 1.5rem;
  border-radius: var(--parsec-radius-12);
  width: fit-content;

  &-item {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    width: 275px;
  }

  .item-licence {
    display: flex;
    gap: 1.25rem;

    &__icon {
      color: var(--parsec-color-light-primary-600);
      font-size: 1.5rem;
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
      margin-top: 0.125rem;
      flex: 1;

      &__title {
        color: var(--parsec-color-light-secondary-text);
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }

  .item-active {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    border-radius: var(--parsec-radius-8);
    flex: 1;

    &__number {
      display: flex;
      gap: 0.25rem;
      align-items: first baseline;
      color: var(--parsec-color-light-primary-600);
    }

    &__label {
      color: var(--parsec-color-light-secondary-grey);
      line-height: 0;
    }

    .progress {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &-bar {
        display: flex;
        gap: 0.25rem;
        height: 0.5rem;
        background: var(--parsec-color-light-secondary-background);
        border-radius: var(--parsec-radius-8);
        height: 0.25rem;

        &-used {
          width: 70%;
          background: var(--parsec-color-light-primary-500);
          border-radius: var(--parsec-radius-8);
        }

        &-unused {
          width: 30%;
          background: var(--parsec-color-light-secondary-light);
          border-radius: var(--parsec-radius-8);
        }
      }

      &-text {
        color: var(--parsec-color-light-secondary-hard-grey);
        align-self: flex-end;
      }
    }
  }
}
</style>
