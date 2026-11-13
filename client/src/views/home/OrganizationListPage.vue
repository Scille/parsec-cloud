<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="organization">
    <div
      class="account-invitations"
      v-if="invitationList.length > 0"
    >
      <ion-text class="account-invitations__title">
        {{
          $msTranslate({
            key: 'HomePage.invitationList.title',
            data: { count: invitationList.length },
          })
        }}
      </ion-text>
      <div
        class="account-invitation"
        v-for="invitation in invitationList"
        :key="invitation.token"
        @click="$emit('invitationClick', invitation)"
      >
        <div class="account-invitation-text">
          <ion-text class="account-invitation-text__title">{{ invitation.organizationId }}</ion-text>
          <ion-text class="account-invitation-text__subtitle">{{ $msTranslate('HomePage.invitationList.validation') }}</ion-text>
        </div>
        <ms-image
          :image="MailUnreadGradient"
          class="account-invitation-image"
        />
      </div>
    </div>

    <template v-if="deviceList.length === 0 && joinRequestList.length === 0 && !querying">
      <!-- No organization -->
      <div class="organization-content no-devices">
        <div class="invitation-container">
          <ms-image
            :image="LogoRowBlue"
            class="invitation-logo"
          />
          <div class="invitation">
            <div class="invitation-header">
              <ion-text class="invitation-header__title">
                {{ $msTranslate('HomePage.noDevices.link.title') }}
              </ion-text>
              <ion-text class="invitation-header__subtitle">
                {{ $msTranslate('HomePage.noDevices.link.subtitle') }}
              </ion-text>
            </div>
            <div class="invitation-content">
              <div class="invitation-link">
                <ms-input
                  placeholder="HomePage.noDevices.link.placeholder"
                  v-model="link"
                  @on-enter-keyup="onLinkClick(link)"
                  :validator="claimAndBootstrapLinkValidator"
                  class="invitation-link__input"
                  ref="linkInput"
                />
                <ion-button
                  @click="onLinkClick(link)"
                  id="join-organization-button"
                  :disabled="linkInputRef?.validity !== Validity.Valid"
                >
                  {{ $msTranslate('HomePage.noDevices.link.continue') }}
                </ion-button>
              </div>
              <!-- not implemented, add click paste link -->
              <ion-button
                class="invitation-paste-button color-secondary"
                fill="outline"
                size="small"
              >
                {{ $msTranslate('HomePage.noDevices.link.paste') }}
              </ion-button>
            </div>
          </div>
        </div>

        <div class="other-needs">
          <ion-text class="other-needs__title">
            {{ $msTranslate('HomePage.noDevices.otherNeeds') }}
          </ion-text>
          <div class="other-needs-cards">
            <div
              class="other-needs-card"
              :class="{ 'other-needs-card--small': isSmallDisplay }"
              @click="isSmallDisplay && $emit('createOrganizationClick')"
            >
              <div class="other-needs-card-header">
                <div class="other-needs-card-header__title-container">
                  <div class="other-needs-card-header__icon other-needs-card-header__icon--purple">
                    <ion-icon :icon="business" />
                  </div>
                  <ion-text class="other-needs-card-header__title">
                    {{ $msTranslate('HomePage.noDevices.create.title') }}
                  </ion-text>
                  <ion-icon
                    v-if="isSmallDisplay"
                    :icon="arrowForward"
                    class="other-needs-card-header__arrow"
                  />
                </div>
                <ion-text
                  v-if="!isSmallDisplay"
                  class="other-needs-card-header__description"
                >
                  {{ $msTranslate('HomePage.noDevices.create.description') }}
                </ion-text>
              </div>
              <ion-button
                v-if="isLargeDisplay"
                @click="$emit('createOrganizationClick')"
                id="create-organization-button"
                class="other-needs-card__button"
                fill="clear"
              >
                {{ $msTranslate('HomePage.noDevices.create.button') }}
              </ion-button>
            </div>
            <div
              class="other-needs-card"
              :class="{ 'other-needs-card--small': isSmallDisplay }"
              @click="isSmallDisplay && $emit('trialClick')"
            >
              <div class="other-needs-card-header">
                <div class="other-needs-card-header__title-container">
                  <div class="other-needs-card-header__icon other-needs-card-header__icon--orange">
                    <ion-icon :icon="flask" />
                  </div>
                  <ion-text class="other-needs-card-header__title">
                    {{ $msTranslate('HomePage.noDevices.trial.title') }}
                  </ion-text>
                  <ion-icon
                    v-if="isSmallDisplay"
                    :icon="arrowForward"
                    class="other-needs-card-header__arrow"
                  />
                </div>
                <ion-text
                  v-if="!isSmallDisplay"
                  class="other-needs-card-header__description"
                >
                  <ms-rich-text text="HomePage.noDevices.trial.description" />
                </ion-text>
              </div>
              <ion-button
                v-if="!isSmallDisplay"
                @click="$emit('trialClick')"
                id="trial-organization-button"
                class="other-needs-card__button"
                fill="clear"
              >
                {{ $msTranslate('HomePage.noDevices.trial.button') }}
              </ion-button>
            </div>
          </div>
        </div>
      </div>
      <!-- end of No organization -->
    </template>
    <template v-else>
      <div class="organization-content">
        <div class="organization-filter">
          <!-- No use in showing the sort/filter options for less than one device -->
          <ms-search-input
            placeholder="HomePage.organizationList.search"
            v-model="searchQuery"
            id="search-input-organization"
            ref="searchInputRef"
          />
          <ms-sorter
            v-if="confLoaded"
            v-show="deviceList.length > 1"
            id="organization-filter-select"
            label="HomePage.organizationList.labelSortBy"
            :options="msSorterOptions"
            :default-option="sortBy"
            :sort-by-asc="sortByAsc"
            :sorter-labels="msSorterLabels"
            @change="onMsSorterChange"
          />
          <!-- will be added with Parsec Auth -->
          <ion-button
            class="organization-add-button"
            slot="icon-only"
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
            class="organization-list__title"
            v-if="joinRequestList.length > 0"
          >
            {{ $msTranslate('HomePage.organizationRequest.title') }}
          </ion-text>
          <organization-join-request
            v-for="joinRequest in joinRequestList"
            :key="joinRequest.enrollment.enrollmentId"
            :request="joinRequest"
            @join-organization="$emit('joinRequestClick', $event)"
            @delete-request="$emit('joinRequestClick', $event)"
          />

          <ion-text
            class="no-match-result"
            v-show="searchQuery.length > 0 && filteredDevices.length === 0 && deviceList.length > 0"
          >
            {{ $msTranslate({ key: 'HomePage.organizationList.noMatch', data: { query: searchQuery } }) }}
          </ion-text>

          <ion-text
            class="organization-list__title"
            v-if="filteredDevices.length > 0"
          >
            {{ $msTranslate('HomePage.organizationList.subtitle') }}
          </ion-text>
          <organization-card
            v-for="device in filteredDevices"
            :key="device.deviceId"
            class="organization-list-item"
            :device="device"
            :last-login-device="storedDeviceDataDict[device.deviceId]?.lastLogin"
            :org-creation-date="storedDeviceDataDict[device.deviceId]?.orgCreationDate"
            @click="$emit('organizationSelect', device)"
            :logged-in="loggedInDevices.find((info: LoggedInDeviceInfo) => info.device.deviceId === device.deviceId) !== undefined"
          />
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import LogoRowBlue from '@/assets/images/logo-row-blue.svg?raw';
import MailUnreadGradient from '@/assets/images/mail-unread-gradient.svg?raw';
import { bootstrapLinkValidator, claimAndBootstrapLinkValidator } from '@/common/validators';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import OrganizationJoinRequest from '@/components/organizations/OrganizationJoinRequest.vue';
import { AccountInvitation, AsyncEnrollmentRequest, AvailableDevice, getLoggedInDevices, LoggedInDeviceInfo } from '@/parsec';
import { Routes } from '@/router';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { arrowForward, business, flask } from 'ionicons/icons';
import { DateTime } from 'luxon';
import {
  AddIcon,
  MsImage,
  MsInput,
  MsOptions,
  MsRichText,
  MsSearchInput,
  MsSorter,
  MsSorterChangeEvent,
  useWindowSize,
  Validity,
} from 'megashark-lib';
import { computed, inject, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const emits = defineEmits<{
  (e: 'organizationSelect', device: AvailableDevice): void;
  (e: 'createOrganizationClick'): void;
  (e: 'joinOrganizationClick'): void;
  (e: 'joinOrganizationWithLinkClick', link: string): void;
  (e: 'bootstrapOrganizationWithLinkClick', link: string): void;
  (e: 'trialClick'): void;
  (e: 'createOrJoinOrganizationClick', event: Event): void;
  (e: 'invitationClick', invitation: AccountInvitation): void;
  (e: 'joinRequestClick', request: AsyncEnrollmentRequest): void;
}>();

const props = defineProps<{
  deviceList: AvailableDevice[];
  invitationList: AccountInvitation[];
  joinRequestList: AsyncEnrollmentRequest[];
  querying: boolean;
}>();

enum SortCriteria {
  UserName = 'user_name',
  LastLogin = 'last_login',
  Organization = 'organization',
}

const ORGANIZATION_LIST_DATA_KEY = 'OrganizationList';

const { isSmallDisplay, isLargeDisplay } = useWindowSize();
const storedDeviceDataDict = ref<{ [deviceId: string]: StoredDeviceData }>({});
const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const sortBy = ref(SortCriteria.Organization);
const sortByAsc = ref(true);
const confLoaded = ref(false);
const searchQuery = ref('');
const linkInputRef = useTemplateRef<InstanceType<typeof MsInput>>('linkInput');
const searchInputRef = ref();
const link = ref('');
const loggedInDevices = ref<Array<LoggedInDeviceInfo>>([]);

interface OrganizationListSavedData {
  sortByAsc?: boolean;
  sortBy?: SortCriteria;
}

const watchCancel = watch(
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
    async () => await searchInputRef.value?.setFocus(),
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
  watchCancel();
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
          storedDeviceDataDict.value[a.deviceId]?.lastLogin !== undefined
            ? (storedDeviceDataDict.value[a.deviceId].lastLogin as DateTime)
            : DateTime.fromMillis(0);
        const bLastLogin =
          storedDeviceDataDict.value[b.deviceId]?.lastLogin !== undefined
            ? (storedDeviceDataDict.value[b.deviceId].lastLogin as DateTime)
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
  display: flex;
  margin: 0;
  flex-direction: column;
  gap: ms.spacing('gap-5xl');
  flex-grow: 1;
}

.organization-content {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  overflow: hidden;
  height: 100%;
  padding: ms.spacing('padding-sm') ms.spacing('padding-5xl') ms.spacing('padding-sm') ms.spacing('padding-sm');
  width: 100%;
  max-width: 40rem;

  @include ms.responsive-breakpoint('sm') {
    margin-inline: auto;
    padding: ms.spacing('padding-lg') ms.spacing('padding-2xl') 0;
  }

  @include ms.responsive-breakpoint('xs') {
    margin-inline: 0;
    box-shadow: ms.shadow('elevation-md');
  }

  .organization-filter {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    gap: ms.spacing('gap-3xl');

    @include ms.responsive-breakpoint('xs') {
      flex-direction: column;
    }

    #organization-filter-select {
      margin-left: auto;
    }

    .organization-add-button {
      --background: #{ms.color('surface-neutral-default')};
      --background-hover: #{ms.color('surface-neutral-default-hover')};
      width: 2.5rem;
      height: 2.5rem;

      &::part(native) {
        padding: ms.spacing('padding-xl');
        border-radius: ms.radius('lg');
      }

      .add-button-icon {
        --fill-color: #{ms.color('icon-neutral-on-color')};
        width: fit-content;
      }
    }
  }

  .organization-list {
    margin: 0;
    overflow-y: auto;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    position: relative;
    z-index: 100;
    // Reserve space for overlay scrollbar on Firefox
    @supports (-moz-appearance: none) {
      padding-right: ms.spacing('padding-2xl');
    }

    @include ms.responsive-breakpoint('md') {
      width: 100%;
    }

    &__title {
      @include ms.font('heading-h6');
      margin-top: ms.spacing('padding-lg');
      color: ms.color('text-base-description');
    }
  }
}

.no-devices {
  max-width: 52rem;
  justify-content: space-between;
  gap: ms.spacing('gap-5xl');
  padding: 0 ms.spacing('padding-4xl');
  overflow: auto;
  background: transparent;
  box-shadow: none;
  margin: auto;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: ms.spacing('padding-4xl');
  }

  @include ms.responsive-breakpoint('xs') {
    gap: ms.spacing('gap-4xl');
    padding-inline: ms.spacing('padding-2xl');
  }

  .invitation-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: ms.spacing('gap-6xl');
    height: 100%;
    width: 100%;
    align-items: center;

    @include ms.responsive-breakpoint('xs') {
      gap: ms.spacing('gap-4xl');
    }
  }

  .invitation-logo {
    width: 10rem;
    flex-shrink: 0;
  }

  .invitation {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: ms.spacing('gap-5xl');
    padding: ms.spacing('padding-5xl');
    background: ms.color('surface-base-default');
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    border-radius: ms.radius('3xl');
    box-shadow: ms.shadow('elevation-lg');

    @include ms.responsive-breakpoint('xs') {
      gap: ms.spacing('gap-4xl');
      padding: ms.spacing('padding-4xl');
    }

    &-header {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: ms.spacing('gap-md');
      text-align: center;

      &__title {
        @include ms.font('heading-h2');
        background: linear-gradient(165deg, ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));
        background-clip: text;
        -webkit-background-clip: text;
        color: transparent;

        @include ms.responsive-breakpoint('sm') {
          @include ms.font('heading-h3');
        }
      }

      &__subtitle {
        @include ms.font('body-lg-medium');
        color: ms.color('text-neutral-default');
        opacity: ms.opacity('description');
      }
    }

    &-content {
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      gap: ms.spacing('gap-2xl');
    }

    &-link {
      display: flex;
      gap: ms.spacing('gap-2xl');
      align-items: flex-end;
      width: 100%;

      @include ms.responsive-breakpoint('xs') {
        flex-direction: column;
      }

      &__input {
        flex: 1;
        width: 100%;
      }

      #join-organization-button {
        min-width: 5rem;

        @include ms.responsive-breakpoint('xs') {
          width: 100%;
        }
      }
    }
  }

  .other-needs {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-4xl');
    padding-bottom: 5vw;

    &__title {
      @include ms.font('body-lg-regular');
      color: ms.color('text-base-description');
      text-align: center;
    }

    &-cards {
      display: grid;
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: ms.spacing('gap-2xl');

      @include ms.responsive-breakpoint('xs') {
        grid-template-columns: minmax(0, 1fr);
      }
    }

    &-card {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: ms.spacing('gap-3xl');
      padding: ms.spacing('padding-4xl');
      background: ms.color('surface-base-default');
      border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
      border-radius: ms.radius('3xl');

      @include ms.responsive-breakpoint('xs') {
        padding: ms.spacing('padding-3xl');
      }

      &--small {
        padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');
        cursor: pointer;
        transition: all 150ms linear;

        &:hover {
          border-color: ms.color('border-brand-default-hover');
          box-shadow: ms.shadow('elevation-sm');
        }

        &:active {
          scale: 0.99;
        }
      }

      &-header {
        display: flex;
        flex-direction: column;
        gap: ms.spacing('gap-2xl');

        &__title-container {
          display: flex;
          align-items: center;
          gap: ms.spacing('gap-lg');
        }

        &__icon {
          display: flex;
          align-items: center;
          justify-content: center;
          padding: ms.spacing('padding-md');
          border-radius: ms.radius('xl');
          flex-shrink: 0;

          ion-icon {
            font-size: 1.125rem;
          }

          &--purple {
            background: ms.color('surface-extra-purple-default-subtle');
            color: ms.color('icon-extra-purple-default');
          }

          &--orange {
            background: ms.color('surface-extra-orange-default-subtle');
            color: ms.color('icon-extra-orange-default');
          }
        }

        &__title {
          @include ms.font('heading-h5');
          color: ms.color('text-base-heading');
          flex: 1;
        }

        &__arrow {
          color: ms.color('icon-neutral-default');
          font-size: 1.125rem;
          flex-shrink: 0;
        }

        &__description {
          @include ms.font('body-lg-medium');
          color: ms.color('text-base-description');
        }
      }
    }
  }
}

.account-invitations {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  border-radius: ms.radius('2xl');
  padding: ms.spacing('padding-5xl');
  width: 100%;
  max-width: 40rem;
  background: ms.color('surface-base-default');

  &__title {
    @include ms.font('heading-h5');
    color: ms.color('text-neutral-default');
    margin-left: auto;
  }

  .account-invitation {
    padding: 0.825rem 1.25rem;
    border: ms.border('thin') solid ms.color('border-brand-default-subtle');
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-radius: ms.radius('2xl');
    background: ms.color('surface-base-default-secondary');
    cursor: pointer;
    color: ms.color('text-neutral-default');
    transition: all 0.2s ease-in-out;

    &:hover {
      box-shadow: ms.shadow('elevation-sm');
      border: ms.border('thin') solid ms.color('border-brand-default-hover');
    }

    &-text {
      display: flex;
      flex-direction: column;
      &__title {
        @include ms.font('heading-h5');
        color: ms.color('text-neutral-default');
      }

      &__subtitle {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
      }
    }

    &-image {
      margin: ms.spacing('padding-lg');
      width: 1.75rem;
      height: 1.75rem;
      flex-shrink: 0;
    }
  }
}
</style>
