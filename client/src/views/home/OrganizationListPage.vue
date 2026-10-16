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
        <div class="create-organization">
          <div class="create-organization-text">
            <ion-text class="create-organization-text__title">
              {{ $msTranslate('HomePage.noDevices.titleCreateOrga') }}
            </ion-text>
            <ion-text class="create-organization-text__subtitle">{{ $msTranslate('HomePage.noDevices.subtitle') }}</ion-text>
            <div class="buttons-container">
              <ion-button
                @click="$emit('createOrganizationClick')"
                id="create-organization-button"
              >
                <ion-icon
                  :icon="addCircle"
                  class="icon button-icon-left"
                />
                {{ $msTranslate('HomePage.noExistingOrganization.createOrganization') }}
              </ion-button>
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
          <ion-text class="invitation__title">
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
              <ion-text class="webAccess-header__title">
                {{ $msTranslate('HomePage.webAccess.title') }}
              </ion-text>
              <ion-text class="webAccess-header-info">
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
              class="webAccess-header__subtitle"
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
              <ion-text class="webAccess-step-item">
                <ion-icon
                  :icon="caretForward"
                  class="webAccess-step-item__icon"
                />
                <span class="webAccess-step-item__text">
                  <ms-rich-text text="HomePage.webAccess.step.one" />
                </span>
              </ion-text>
              <ion-text class="webAccess-step-item">
                <ion-icon
                  :icon="caretForward"
                  class="webAccess-step-item__icon"
                />
                <span class="webAccess-step-item__text">
                  <ms-rich-text text="HomePage.webAccess.step.two" />
                </span>
              </ion-text>
              <ion-text class="webAccess-step-item">
                <ion-icon
                  :icon="caretForward"
                  class="webAccess-step-item__icon"
                />
                <span class="webAccess-step-item__text">
                  <ms-rich-text text="HomePage.webAccess.step.three" />
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
            <ion-text class="webAccess-info">
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
  MsRichText,
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
    box-shadow: var(--parsec-shadow-strong);
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
  max-width: 50rem;
  margin-bottom: ms.spacing('padding-lg');
  gap: ms.spacing('gap-4xl');
  padding: 0;
  overflow: auto;
  background: transparent;
  box-shadow: none;

  .create-organization,
  .invitation,
  .webAccess {
    background: ms.color('surface-base-default');
    border: ms.border('thin') solid ms.color('border-base-default');
    border-radius: ms.radius('2xl');
    display: flex;
  }

  .invitation,
  .webAccess {
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    padding: ms.spacing('padding-5xl');

    @include ms.responsive-breakpoint('xs') {
      padding: ms.spacing('padding-4xl');
    }

    ion-button {
      min-width: 7rem;

      @include ms.responsive-breakpoint('xs') {
        width: 100%;
      }
    }
  }

  .create-organization {
    align-items: center;
    gap: ms.spacing('gap-4xl');
    padding: ms.spacing('padding-5xl') ms.spacing('padding-5xl');

    @include ms.responsive-breakpoint('xs') {
      padding: ms.spacing('padding-4xl');
    }

    &-text {
      display: flex;
      flex-direction: column;
      max-width: 33rem;

      @include ms.responsive-breakpoint('xs') {
        max-width: 100%;
      }

      &__title {
        @include ms.font('heading-h4');
        color: ms.color('text-brand-default');
        margin-bottom: ms.spacing('padding-3xl');
      }

      &__subtitle {
        @include ms.font('body-lg-regular');
        color: ms.color('text-base-description');
        margin-bottom: ms.spacing('padding-4xl');
      }

      .buttons-container {
        display: flex;
        gap: ms.spacing('gap-xl');

        @include ms.responsive-breakpoint('md') {
          flex-direction: column;
        }
      }

      #create-organization-button {
        width: fit-content;

        @include ms.responsive-breakpoint('md') {
          width: 100%;
        }

        ion-icon {
          margin-inline-end: ms.spacing('gap-lg');
          font-size: 1rem;
        }
      }

      .recovery-no-devices {
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-lg');
        color: ms.color('text-base-description');
        margin-top: ms.spacing('padding-3xl');

        @include ms.responsive-breakpoint('sm') {
          flex-direction: column;
        }

        ion-text {
          @include ms.font('body-md-regular');

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
      @include ms.font('heading-h5');
      color: ms.color('text-neutral-default');
      padding: 0;
    }

    &-link {
      display: flex;
      gap: ms.spacing('gap-3xl');
      align-items: flex-end;

      @include ms.responsive-breakpoint('xs') {
        flex-direction: column;
      }

      &__input {
        width: 100%;
        max-width: 33rem;
      }
    }
  }

  .webAccess {
    position: relative;
    margin-bottom: ms.spacing('padding-5xl');

    &-header {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: ms.spacing('gap-2xl');
      cursor: pointer;

      &__title-container {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: ms.spacing('gap-lg');
      }

      &__title {
        @include ms.font('heading-h5');
        color: ms.color('text-neutral-default');
        padding: 0;
        width: auto;

        &:hover {
          color: ms.color('text-neutral-default');
        }
      }

      &__subtitle {
        @include ms.font('body-lg-regular');
        color: ms.color('text-base-description');
      }

      &-info {
        @include ms.font('label-md-medium');
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-sm');
        color: ms.color('text-base-description');
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
          color: ms.color('text-neutral-default');
        }
      }
    }

    &__content {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-3xl');
    }

    &-step {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-lg');
      color: ms.color('text-base-description');
      padding: ms.spacing('padding-3xl');
      border-radius: ms.radius('lg');
      background: ms.color('surface-base-default-secondary');

      @include ms.responsive-breakpoint('sm') {
        flex-direction: column;
      }

      &-item {
        @include ms.font('body-md-regular');
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-lg');

        &__icon {
          color: ms.color('icon-disabled-default');
          font-size: 1rem;
          flex-shrink: 0;
        }

        &__text {
          color: ms.color('text-neutral-default');
        }
      }
    }

    &-link {
      display: flex;
      gap: ms.spacing('gap-3xl');
      justify-content: flex-start;

      @include ms.responsive-breakpoint('xs') {
        flex-direction: column;
      }

      &__input {
        width: 100%;
        max-width: 33rem;
      }

      #join-organization-button {
        flex-grow: 0;
        height: fit-content;
      }
    }

    &-info {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');

      .more-link {
        color: ms.color('text-brand-default');
        cursor: pointer;
        text-decoration: underline;

        &:hover {
          color: ms.color('text-brand-default-hover');
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
      box-shadow: var(--parsec-shadow-light);
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
