<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization">
    <div
      class="account-invitations"
      v-if="invitationList.length > 0"
    >
      <ion-text class="title-h4 account-invitations__title">
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
          <ion-text class="account-invitation-text__title title-h4">{{ invitation.organizationId }}</ion-text>
          <ion-text class="account-invitation-text__subtitle body">{{ $msTranslate('HomePage.invitationList.validation') }}</ion-text>
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
        <div class="create-organization">
          <div class="create-organization-text">
            <ion-text class="create-organization-text__title title-h3">
              {{ $msTranslate('HomePage.noDevices.titleCreateOrga') }}
            </ion-text>
            <ion-text class="create-organization-text__subtitle body-lg">{{ $msTranslate('HomePage.noDevices.subtitle') }}</ion-text>
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
          <ion-text class="invitation__title title-h4">
            {{ $msTranslate('HomePage.noDevices.titleInvitation') }}
          </ion-text>
          <div class="invitation-link">
            <ms-input
              label="HomePage.noDevices.invitationLink"
              placeholder="JoinOrganization.linkFormPlaceholder"
              v-model="link"
              @on-enter-keyup="onLinkClick(link)"
              :validator="claimAndBootstrapLinkValidator"
              class="invitation-link__input"
              ref="linkInput"
            />
            <ion-button
              @click="onLinkClick(link)"
              fill="clear"
              id="join-organization-button"
              :disabled="linkInputRef?.validity !== Validity.Valid"
            >
              {{ $msTranslate('HomePage.noDevices.invitationJoin') }}
            </ion-button>
          </div>
        </div>
        <div class="webAccess">
          <div
            @click="toggleWebAccess"
            class="webAccess-header"
          >
            <div class="webAccess-header__title-container">
              <ion-text class="webAccess-header__title title-h4">
                {{ $msTranslate('HomePage.webAccess.title') }}
              </ion-text>
              <ion-text class="webAccess-header-info button-medium">
                <span
                  v-if="!showWebAccess"
                  class="webAccess-header-info__text"
                >
                  {{ $msTranslate('HomePage.webAccess.step.showButton') }}
                </span>
                <span
                  v-else
                  class="webAccess-header-info__text"
                >
                  {{ $msTranslate('HomePage.webAccess.step.hideButton') }}
                </span>
                <ion-icon
                  :icon="chevronDown"
                  class="webAccess-header-info__icon"
                  :class="{ 'webAccess-header-info__icon--open': showWebAccess }"
                />
              </ion-text>
            </div>
            <ion-text
              class="webAccess-header__subtitle body-lg"
              v-show="showWebAccess"
            >
              {{ $msTranslate('HomePage.webAccess.subtitle') }}
            </ion-text>
          </div>
          <div class="webAccess__content">
            <div
              class="webAccess-step"
              v-show="showWebAccess"
            >
              <ion-text class="body webAccess-step-item">
                <ion-icon
                  :icon="caretForward"
                  class="webAccess-step-item__icon"
                />
                <span class="webAccess-step-item__text">
                  <i18n-t
                    keypath="HomePage.webAccess.step.one"
                    scope="global"
                  >
                    <template #application>
                      <strong> {{ $msTranslate('HomePage.webAccess.step.application') }} </strong>
                    </template>
                    <template #login>
                      <strong> {{ $msTranslate('HomePage.webAccess.step.login') }} </strong>
                    </template>
                  </i18n-t>
                </span>
              </ion-text>
              <ion-text class="body webAccess-step-item">
                <ion-icon
                  :icon="caretForward"
                  class="webAccess-step-item__icon"
                />
                <span class="webAccess-step-item__text">
                  <i18n-t
                    keypath="HomePage.webAccess.step.two"
                    scope="global"
                  >
                    <template #myDevices>
                      <strong> {{ $msTranslate('HomePage.webAccess.step.myDevices') }} </strong>
                    </template>
                    <template #addDevice>
                      <strong> {{ $msTranslate('HomePage.webAccess.step.addDevice') }} </strong>
                    </template>
                    <template #copyInvitation>
                      <strong> {{ $msTranslate('HomePage.webAccess.step.copyInvitation') }} </strong>
                    </template>
                  </i18n-t>
                </span>
              </ion-text>
              <ion-text class="body webAccess-step-item">
                <ion-icon
                  :icon="caretForward"
                  class="webAccess-step-item__icon"
                />
                <span class="webAccess-step-item__text">
                  <i18n-t
                    keypath="HomePage.webAccess.step.three"
                    scope="global"
                  >
                    <template #pasteInvitation>
                      <strong> {{ $msTranslate('HomePage.webAccess.step.pasteInvitation') }} </strong>
                    </template>
                  </i18n-t>
                </span>
              </ion-text>
            </div>
            <div class="webAccess-link">
              <ms-input
                placeholder="HomePage.webAccess.pasteLinkPlaceholder"
                v-model="webLink"
                @on-enter-keyup="onLinkClick(webLink)"
                :validator="claimAndBootstrapLinkValidator"
                class="webAccess-link__input"
                ref="webLinkRef"
              />
              <ion-button
                @click="onLinkClick(webLink)"
                fill="clear"
                id="join-organization-button"
                :disabled="!webLinkRef || webLinkRef.validity !== Validity.Valid"
              >
                {{ $msTranslate('HomePage.webAccess.joinButton') }}
              </ion-button>
            </div>
            <ion-text class="webAccess-info body">
              <i18n-t
                keypath="HomePage.webAccess.info"
                scope="global"
              >
                <template #more>
                  <strong
                    class="more-link"
                    @click="openDocumentation"
                  >
                    {{ $msTranslate('HomePage.webAccess.more') }}
                  </strong>
                </template>
              </i18n-t>
            </ion-text>
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
            ref="searchInput"
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
            class="organization-list__title title-h5"
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
            class="no-match-result body"
            v-show="searchQuery.length > 0 && filteredDevices.length === 0 && deviceList.length > 0"
          >
            {{ $msTranslate({ key: 'HomePage.organizationList.noMatch', data: { query: searchQuery } }) }}
          </ion-text>

          <ion-text
            class="organization-list__title title-h5"
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
      <div class="recovery-devices">
        <ion-text class="body">{{ $msTranslate('HomePage.lostDevice') }}</ion-text>
        <ion-button
          @click="$emit('recoverClick')"
          fill="clear"
          class="button-medium"
        >
          {{ $msTranslate('HomePage.recoverDevice') }}
        </ion-button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import MailUnreadGradient from '@/assets/images/mail-unread-gradient.svg?raw';
import { bootstrapLinkValidator, claimAndBootstrapLinkValidator } from '@/common/validators';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import OrganizationJoinRequest from '@/components/organizations/OrganizationJoinRequest.vue';
import { AccountInvitation, AsyncEnrollmentRequest, AvailableDevice, getLoggedInDevices, LoggedInDeviceInfo } from '@/parsec';
import { Routes } from '@/router';
import { Env } from '@/services/environment';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { addCircle, caretForward, chevronDown } from 'ionicons/icons';
import { DateTime } from 'luxon';
import {
  AddIcon,
  MsImage,
  MsInput,
  MsOptions,
  MsSearchInput,
  MsSorter,
  MsSorterChangeEvent,
  NoOrganization,
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
  (e: 'recoverClick'): void;
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

const { isSmallDisplay } = useWindowSize();
const storedDeviceDataDict = ref<{ [deviceId: string]: StoredDeviceData }>({});
const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const sortBy = ref(SortCriteria.Organization);
const sortByAsc = ref(true);
const confLoaded = ref(false);
const searchQuery = ref('');
const linkInputRef = useTemplateRef<InstanceType<typeof MsInput>>('linkInput');
const searchInputRef = ref();
const webLinkRef = ref();
const link = ref('');
const webLink = ref('');
const showWebAccess = ref(false);
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

function toggleWebAccess(): void {
  showWebAccess.value = !showWebAccess.value;
}

async function openDocumentation(): Promise<void> {
  await Env.Links.openDocumentationUserGuideLink('security', 'how-to-enable-web-browser-storage');
}
</script>

<style lang="scss" scoped>
.organization {
  background: none;
  width: 100%;
  max-width: var(--parsec-max-content-width);
  display: flex;
  margin: 0;
  flex-direction: column;
  gap: 2rem;
  flex-grow: 1;
}

.organization-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  overflow: hidden;
  margin-bottom: 4rem;
  height: 100%;
  border-radius: var(--parsec-radius-12) var(--parsec-radius-12) 0 0;
  padding: 2rem 2rem 0 2rem;
  width: 100%;
  max-width: 40rem;
  background: var(--parsec-color-light-secondary-white);

  @include ms.responsive-breakpoint('sm') {
    margin-inline: auto;
  }

  @include ms.responsive-breakpoint('xs') {
    padding: 1.5rem 1.5rem 0 1.5rem;
    margin-inline: 0;
    box-shadow: var(--parsec-shadow-strong);
  }

  .organization-filter {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    gap: 1rem;

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
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    position: relative;
    z-index: 100;

    @include ms.responsive-breakpoint('md') {
      width: 100%;
    }

    &__title {
      margin-top: 0.5rem;
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}

.recovery-devices {
  display: flex;
  align-items: center;
  justify-content: center;
  position: absolute;
  border-top: 1px solid var(--parsec-color-light-secondary-medium);
  width: 100%;
  gap: 0.5rem;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  padding: 0.5rem 0.5rem 1.5rem;
  color: var(--parsec-color-light-secondary-grey);
  bottom: 0;
  left: 0;
  z-index: 10;

  @include ms.responsive-breakpoint('sm') {
    position: fixed;
  }
}

.no-devices {
  max-width: 50rem;
  margin-bottom: 0.5rem;
  gap: 1.5rem;
  padding: 0;
  overflow: auto;
  background: transparent;
  box-shadow: none;

  .create-organization,
  .invitation,
  .webAccess {
    background: var(--parsec-color-light-secondary-white);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-12);
    display: flex;
  }

  .invitation,
  .webAccess {
    flex-direction: column;
    gap: 1rem;
    padding: 2rem;

    @include ms.responsive-breakpoint('xs') {
      padding: 1.5rem;
    }

    ion-button {
      --background: var(--parsec-color-light-secondary-text);
      --background-hover: var(--parsec-color-light-secondary-contrast);
      --color: var(--parsec-color-light-secondary-white);
      --color-hover: var(--parsec-color-light-secondary-white);
      padding-bottom: 0.25rem;
      min-width: 7rem;

      @include ms.responsive-breakpoint('xs') {
        width: 100%;
      }
    }
  }

  .create-organization {
    align-items: center;
    gap: 1.5rem;
    padding: 2rem 2rem;

    @include ms.responsive-breakpoint('xs') {
      padding: 1.5rem;
    }

    &-text {
      display: flex;
      flex-direction: column;
      max-width: 33rem;

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
          font-size: 1rem;
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
      max-width: 9rem;
      margin: auto;

      @include ms.responsive-breakpoint('md') {
        max-width: 6rem;
      }

      @include ms.responsive-breakpoint('xs') {
        display: none;
      }
    }
  }

  .invitation {
    &__title {
      color: var(--parsec-color-light-secondary-text);
      padding: 0;
    }

    &-link {
      display: flex;
      gap: 1rem;
      align-items: flex-start;

      @include ms.responsive-breakpoint('xs') {
        flex-direction: column;
      }

      &__input {
        width: 100%;
        max-width: 33rem;
      }

      #join-organization-button {
        padding-top: 1.75rem;
      }
    }
  }

  .webAccess {
    position: relative;
    margin-bottom: 2rem;

    &-header {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: 0.75rem;
      cursor: pointer;

      &__title-container {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
      }

      &__title {
        color: var(--parsec-color-light-secondary-text);
        padding: 0;
        width: auto;

        &:hover {
          color: var(--parsec-color-light-secondary-text);
        }
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      &-info {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        color: var(--parsec-color-light-secondary-grey);
        position: relative;
        right: 0;
        top: 0;
        flex-shrink: 0;

        &__text {
          font-weight: 600;
          @include ms.responsive-breakpoint('xs') {
            display: none;
          }
        }

        &__icon {
          transition: transform 0.2s ease-in-out;

          &--open {
            transform: rotate(180deg);
          }

          @include ms.responsive-breakpoint('xs') {
            font-size: 1.25rem;
          }
        }

        &:hover {
          color: var(--parsec-color-light-secondary-text);
        }
      }
    }

    &__content {
      display: flex;
      flex-direction: column;
      gap: 1rem;
    }

    &-step {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      color: var(--parsec-color-light-secondary-grey);
      padding: 1rem;
      border-radius: var(--parsec-radius-8);
      background: var(--parsec-color-light-secondary-background);

      @include ms.responsive-breakpoint('sm') {
        flex-direction: column;
      }

      &-item {
        display: flex;
        align-items: center;
        gap: 0.5rem;

        &__icon {
          color: var(--parsec-color-light-secondary-light);
          font-size: 1rem;
          flex-shrink: 0;
        }

        &__text {
          color: var(--parsec-color-light-secondary-text);
        }
      }
    }

    &-link {
      display: flex;
      gap: 1rem;
      justify-content: flex-start;

      @include ms.responsive-breakpoint('xs') {
        flex-direction: column;
      }

      &__input {
        width: 100%;
        max-width: 33rem;
      }

      #join-organization-button {
        margin-top: 0.25rem;
        flex-grow: 0;
        height: fit-content;
      }
    }

    &-info {
      color: var(--parsec-color-light-secondary-hard-grey);

      .more-link {
        color: var(--parsec-color-light-primary-500);
        cursor: pointer;
        text-decoration: underline;

        &:hover {
          color: var(--parsec-color-light-primary-600);
        }
      }
    }
  }
}

.account-invitations {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  border-radius: var(--parsec-radius-12);
  padding: 2rem;
  width: 100%;
  max-width: 40rem;
  background: var(--parsec-color-light-secondary-white);

  &__title {
    color: var(--parsec-color-light-secondary-text);
    margin-left: auto;
  }

  .account-invitation {
    padding: 0.825rem 1.25rem;
    border: 1px solid var(--parsec-color-light-primary-200);
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-radius: var(--parsec-radius-12);
    background: var(--parsec-color-light-secondary-background);
    cursor: pointer;
    color: var(--parsec-color-light-secondary-text);
    transition: all 0.2s ease-in-out;

    &:hover {
      box-shadow: var(--parsec-shadow-light);
      border: 1px solid var(--parsec-color-light-primary-600);
    }

    &-text {
      display: flex;
      flex-direction: column;
      &__title {
        color: var(--parsec-color-light-secondary-text);
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }

    &-image {
      margin: 0.5rem;
      width: 1.75rem;
      height: 1.75rem;
      flex-shrink: 0;
    }
  }
}
</style>
