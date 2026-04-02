<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div id="shamir-setup">
    <div
      v-if="step === ShamirSetupStep.Initial"
      class="shamir-information"
      id="shamir-initial-step"
    >
      <div class="shamir-information-header">
        <ms-image
          :image="ShamirIconCircle"
          class="shamir-information-header__image"
          alt="Shamir Icon"
        />
        <ion-text class="shamir-information-header__title title-h4">
          {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.setupInfo.title') }}
        </ion-text>
        <ion-text class="shamir-information-header__subtitle body-lg">
          {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.setupInfo.subtitle') }}
        </ion-text>
      </div>

      <div class="shamir-information-content">
        <div class="shamir-information-content-card">
          <ion-icon
            :icon="search"
            class="shamir-information-content-card-icon"
          />
          <div class="shamir-information-content-card-text">
            <ion-text class="shamir-information-content-card-text__title title-h5">
              {{
                $msTranslate({
                  key: 'OrganizationRecovery.shamir.modalSelf.setupInfo.info1Title',
                  data: { threshold: shamirThreshold },
                })
              }}
            </ion-text>
            <ion-text class="shamir-information-content-card-text__subtitle subtitles-sm">
              {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.setupInfo.info1Subtitle') }}
            </ion-text>
          </div>
        </div>
        <div class="shamir-information-content-card">
          <ion-icon
            :icon="mailOpen"
            class="shamir-information-content-card-icon"
          />
          <div class="shamir-information-content-card-text">
            <ion-text class="shamir-information-content-card-text__title title-h5">
              {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.setupInfo.info2Title') }}
            </ion-text>
            <ion-text class="shamir-information-content-card-text__subtitle subtitles-sm">
              {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.setupInfo.info2Subtitle') }}
            </ion-text>
          </div>
        </div>
      </div>

      <ms-report-text
        v-if="error"
        :theme="MsReportTheme.Error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>

      <ion-button
        @click="startSetup"
        :disabled="allUsers.length < shamirThreshold"
        class="shamir-information-button"
      >
        {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.setupInfo.startSetup') }}
        <ion-icon
          :icon="arrowForward"
          slot="end"
          class="shamir-information-button__icon"
        />
      </ion-button>
    </div>

    <div
      v-if="step === ShamirSetupStep.AddPeople && allUsers.length"
      class="shamir-setup"
      id="shamir-add-people-step"
    >
      <div class="shamir-setup-threshold">
        <div class="shamir-setup-threshold-header">
          <ion-text class="shamir-setup-threshold-header__title title-h4">
            {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.threshold.title') }}
          </ion-text>
          <!-- eslint-disable vue/html-indent -->
          <ion-text class="shamir-setup-threshold-header__number title-h5">
            {{
              selectedUsers.length < shamirThreshold
                ? $msTranslate({
                    key: 'OrganizationRecovery.shamir.modalSelf.threshold.minUsers',
                    data: { count: selectedUsers.length, threshold: shamirThreshold },
                  })
                : $msTranslate({ key: 'OrganizationRecovery.shamir.modalSelf.threshold.selected', data: { count: selectedUsers.length } })
            }}
          </ion-text>
        </div>

        <ion-text class="shamir-setup-threshold__subtitle body">
          {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.threshold.subtitle') }}
        </ion-text>

        <ms-progress
          :progress="progress"
          :appearance="MsProgressAppearance.Bar"
          class="ms-progress"
        />
      </div>

      <div class="shamir-setup-people">
        <div
          ref="searchContainer"
          class="shamir-setup-search"
        >
          <ms-search-input
            class="shamir-setup-search__input"
            v-model="searchInputValue"
            @click="openSearchDropdown"
            placeholder="OrganizationRecovery.shamir.modalSelf.trustedPeople.input"
          />

          <shamir-setup-search-dropdown
            v-if="isSearchOpen"
            class="shamir-setup-search__dropdown"
            :users="searchUsers"
            :selected-users="selectedUsers"
            @user-selected="onSearchUserSelected"
            @deselect-user="onSearchUserDeselected"
          />
        </div>

        <div class="shamir-setup-list-container">
          <div
            v-if="selectedUsers.length === 0"
            class="shamir-setup-list-empty"
          >
            <ion-icon
              :icon="search"
              class="shamir-setup-list-empty__icon"
            />
            <ion-text class="shamir-setup-list-empty__title subtitles-normal">
              {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.trustedPeople.noPeopleAddedTitle') }}
            </ion-text>
            <ion-text class="shamir-setup-list-empty__subtitle body">
              {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.trustedPeople.noPeopleAddedSubtitle') }}
            </ion-text>
          </div>

          <ion-list
            class="shamir-setup-list"
            v-else
          >
            <ion-item
              v-for="user in selectedUsers"
              :key="user.id"
              class="shamir-setup-list-item"
            >
              <ion-icon
                :icon="shieldCheckmark"
                class="shamir-setup-list-item__icon"
              />
              <ion-text class="shamir-setup-list-item__text subtitles-sm">
                <span class="shamir-setup-list-item__text-label subtitles-sm">
                  {{ user.humanHandle.label }}
                </span>
                <span class="shamir-setup-list-item__text-description body">
                  {{ user.humanHandle.email }}
                </span>
              </ion-text>
              <ion-icon
                :icon="close"
                class="shamir-setup-list-item__icon"
                @click="removeUser(user)"
              />
            </ion-item>
          </ion-list>
        </div>
      </div>

      <ms-checkbox
        class="shamir-setup__checkbox"
        v-model="confirmSetupCheckbox"
        v-if="selectedUsers.length >= shamirThreshold"
      >
        {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.trustedPeople.checkbox') }}
      </ms-checkbox>

      <ms-report-text
        v-if="error"
        :theme="MsReportTheme.Error"
      >
        {{ $msTranslate(error) }}
      </ms-report-text>

      <ion-button
        @click="setupShamir"
        :disabled="selectedUsers.length < shamirThreshold || !confirmSetupCheckbox"
      >
        {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.trustedPeople.confirmSetup') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import ShamirIconCircle from '@/assets/images/shamir-icon-circle.svg?raw';
import ShamirSetupSearchDropdown from '@/components/profile/ShamirSetupSearchDropdown.vue';
import { getClientInfo, listUsers, UserInfo, UserProfile } from '@/parsec';
import { getRequiredShamirThreshold, setupShamirRecovery } from '@/parsec/shamir';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonButton, IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { arrowForward, close, mailOpen, search, shieldCheckmark } from 'ionicons/icons';
import {
  MsCheckbox,
  MsImage,
  MsProgress,
  MsProgressAppearance,
  MsReportText,
  MsReportTheme,
  MsSearchInput,
  Translatable,
} from 'megashark-lib';
import { computed, onBeforeUnmount, onMounted, ref, useTemplateRef } from 'vue';

enum ShamirSetupStep {
  Initial = 'shamir-setup-step-initial',
  AddPeople = 'shamir-setup-step-add-people',
}

const step = ref<ShamirSetupStep>(ShamirSetupStep.Initial);
const allUsers = ref<Array<UserInfo>>([]);
const selectedUsers = ref<Array<UserInfo>>([]);
const error = ref<Translatable | undefined>(undefined);
const shamirThreshold = ref(3);
const searchInputValue = ref('');
const searchContainer = useTemplateRef<HTMLElement | null>('searchContainer');
const isSearchOpen = ref(false);
const confirmSetupCheckbox = ref(false);
const currentProfile = ref<UserProfile>(UserProfile.Outsider);

const progress = computed(() => {
  if (selectedUsers.value.length === 0) {
    return 2;
  } else if (selectedUsers.value.length >= shamirThreshold.value) {
    return 100;
  } else {
    return (selectedUsers.value.length / shamirThreshold.value) * 100;
  }
});

const searchUsers = computed(() => {
  const searchLower = searchInputValue.value.trim().toLocaleLowerCase();
  return allUsers.value.filter((user) => {
    return (
      user.humanHandle.label.toLocaleLowerCase().includes(searchLower) || user.humanHandle.email.toLocaleLowerCase().includes(searchLower)
    );
  });
});

const emits = defineEmits<{
  (e: 'shamirSetup'): void;
}>();

const props = defineProps<{
  informationManager: InformationManager;
}>();

onMounted(async () => {
  document.addEventListener('click', onDocumentClick, true);
  const [usersResult, clientResult, threshold] = await Promise.all([listUsers(true), getClientInfo(), getRequiredShamirThreshold()]);
  if (usersResult.ok && clientResult.ok) {
    shamirThreshold.value = threshold;
    currentProfile.value = clientResult.value.currentProfile;
    if (clientResult.value.currentProfile === UserProfile.Outsider) {
      allUsers.value = usersResult.value.filter((user) => user.currentProfile === UserProfile.Admin);
      if (allUsers.value.length < shamirThreshold.value) {
        error.value = { key: 'OrganizationRecovery.shamir.errors.notEnoughAdmins', data: { needed: shamirThreshold.value } };
      }
    } else {
      allUsers.value = usersResult.value.filter(
        (user) => user.id !== clientResult.value.userId && user.currentProfile !== UserProfile.Outsider,
      );
      if (allUsers.value.length < shamirThreshold.value) {
        error.value = { key: 'OrganizationRecovery.shamir.errors.notEnoughPeople', data: { needed: shamirThreshold.value } };
      }
    }
  }
});

onBeforeUnmount(() => {
  document.removeEventListener('click', onDocumentClick, true);
});

function addUser(user: UserInfo): void {
  if (selectedUsers.value.find((selectedUser) => selectedUser.id === user.id)) {
    return;
  }
  selectedUsers.value.push(user);
}

function removeUser(user: UserInfo): void {
  const index = selectedUsers.value.findIndex((selectedUser) => selectedUser.id === user.id);
  if (index !== -1) {
    selectedUsers.value.splice(index, 1);
  }
}

function onSearchUserSelected(user: UserInfo): void {
  addUser(user);
  if (searchInputValue.value.trim().length > 0) {
    isSearchOpen.value = false;
  }
  searchInputValue.value = '';
}

function onSearchUserDeselected(user: UserInfo): void {
  removeUser(user);
}

function openSearchDropdown(): void {
  isSearchOpen.value = true;
}

function onDocumentClick(event: MouseEvent): void {
  if (!searchContainer.value) {
    return;
  }

  const target = event.target as Node | null;
  if (target && !searchContainer.value.contains(target)) {
    isSearchOpen.value = false;
  }
}

async function setupShamir(): Promise<void> {
  if (selectedUsers.value.length < shamirThreshold.value) {
    return;
  }
  const result = await setupShamirRecovery(selectedUsers.value, shamirThreshold.value, 1);
  if (result.ok) {
    props.informationManager.present(
      new Information({
        message: 'OrganizationRecovery.shamir.modalSelf.toasts.setupSuccess',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    emits('shamirSetup');
  } else {
    error.value = 'OrganizationRecovery.shamir.modalSelf.errors.setupFailed';
    props.informationManager.present(
      new Information({
        message: 'OrganizationRecovery.shamir.modalSelf.toasts.setupFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function startSetup(): Promise<void> {
  if (currentProfile.value === UserProfile.Outsider) {
    selectedUsers.value = allUsers.value;
    await setupShamir();
  } else {
    step.value = ShamirSetupStep.AddPeople;
  }
}
</script>

<style scoped lang="scss">
.shamir-information {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.5rem;

  &-header {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;

    &__image {
      width: 4.75rem;
      height: 4.75rem;
    }

    &__title {
      font-size: 1.25rem;
      margin-top: 0.5rem;
      color: var(--parsec-color-light-secondary-text);
    }

    &__subtitle {
      color: var(--parsec-color-light-secondary-hard-grey);
      text-align: center;
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;

    &-card {
      display: flex;
      gap: 1rem;
      padding: 1rem;
      border-radius: var(--parsec-radius-12);
      background: var(--parsec-color-light-secondary-premiere);

      &-icon {
        flex-shrink: 0;
        font-size: 1rem;
        background: var(--parsec-color-light-primary-500);
        color: var(--parsec-color-light-secondary-white);
        padding: 0.375rem;
        border-radius: var(--parsec-radius-circle);
      }

      &-text {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;

        &__title {
          color: var(--parsec-color-light-secondary-text);
        }

        &__subtitle {
          color: var(--parsec-color-light-secondary-hard-grey);
        }
      }
    }
  }

  &-button {
    width: 100%;

    &::part(native) {
      justify-content: center;
    }

    &__icon {
      font-size: 1rem;
      margin-left: 0.5rem;
      color: var(--parsec-color-light-secondary-white);
    }
  }
}

.shamir-setup {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  &-search {
    position: relative;

    &__input {
      width: 100%;
    }

    &__dropdown {
      position: absolute;
      top: 100%;
      left: 0;
      width: 100%;
      z-index: 12;
      box-shadow: 1px 4px 22px 3px rgba(0, 0, 0, 0.2);
    }
  }

  &-threshold {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    background: var(--parsec-color-light-secondary-inversed-contrast);
    padding: 1rem;
    border-radius: var(--parsec-radius-12);
    box-shadow: var(--parsec-shadow-card);

    &-header {
      display: flex;
      justify-content: space-between;
      align-items: center;

      &__title {
        color: var(--parsec-color-light-primary-600);
      }

      &__number {
        color: var(--parsec-color-light-secondary-hard-text);
      }
    }

    &__subtitle {
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    .ms-progress {
      margin-top: 0.5rem;
    }
  }

  &-people {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  &-list-container {
    display: flex;
    width: 100%;
    border: 1px solid var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-8);
    overflow: hidden;
    background: var(--parsec-color-light-secondary-background);
  }

  &-list {
    padding: 0;
    width: 100%;
    min-height: 13rem;
    max-height: 13rem;
    overflow-y: auto;

    &-item {
      border-bottom: 1px solid var(--parsec-color-light-secondary-premiere);
      box-shadow: var(--parsec-shadow-card);

      &::part(native) {
        padding: 0.625rem 1rem;
      }

      &::part(container) {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        overflow: hidden;
        justify-content: space-between;
      }

      &__text {
        margin: 0;
        display: flex;
        align-items: center;
        gap: 0.375rem;
        width: 100%;
        overflow: hidden;

        &-label {
          color: var(--parsec-color-light-secondary-text);
        }

        &-description {
          color: var(--parsec-color-light-secondary-grey);
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }
      }

      &__icon {
        flex-shrink: 0;
        font-size: 1rem;
        color: var(--parsec-color-light-secondary-grey);

        &:first-child {
          color: var(--parsec-color-light-primary-500);
        }

        &:last-child {
          padding: 0.125rem;
          border-radius: var(--parsec-radius-18);
          color: var(--parsec-color-light-secondary-grey);
          cursor: pointer;

          &:hover {
            background: var(--parsec-color-light-danger-100);
            color: var(--parsec-color-light-danger-600);
          }
        }
      }

      &:hover {
        --background: var(--parsec-color-light-secondary-premiere);
      }
    }

    &-empty {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 0.5rem;
      padding: 1rem;
      width: 100%;
      height: 100%;
      min-height: 13rem;

      &__icon {
        font-size: 1.5rem;
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      &__title {
        color: var(--parsec-color-light-secondary-text);
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }
  }
}
</style>
