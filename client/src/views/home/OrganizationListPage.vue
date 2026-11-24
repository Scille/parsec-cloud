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

    <div
      v-if="querying && !hasOrganizations"
      class="organization-loading"
    >
      <ms-spinner />
    </div>
    <template v-else-if="!hasOrganizations">
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
                  size="large"
                >
                  {{ $msTranslate('HomePage.noDevices.link.continue') }}
                </ion-button>
              </div>
              <ion-button
                class="invitation-paste-button color-secondary"
                fill="outline"
                size="small"
                @click="onPasteLinkClick"
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
          <div class="other-needs-list">
            <div
              class="other-needs-item other-needs-item--orange"
              @click="$emit('trialClick')"
            >
              <div class="other-needs-item__icon other-needs-item__icon--orange">
                <ion-icon :icon="flask" />
              </div>
              <div class="other-needs-item-text">
                <ion-text class="other-needs-item-text__title">
                  {{ $msTranslate('HomePage.noDevices.trial.title') }}
                </ion-text>
                <ion-text class="other-needs-item-text__description">
                  <ms-rich-text text="HomePage.noDevices.trial.description" />
                </ion-text>
              </div>
              <ion-button
                id="trial-organization-button"
                class="other-needs-item__button color-secondary"
                fill="clear"
                slot="icon-only"
              >
                <ion-icon :icon="arrowForward" />
              </ion-button>
            </div>
            <div
              class="other-needs-item other-needs-item--purple"
              @click="$emit('createOrganizationClick')"
            >
              <div class="other-needs-item__icon other-needs-item__icon--purple">
                <ion-icon :icon="business" />
              </div>
              <div class="other-needs-item-text">
                <ion-text class="other-needs-item-text__title">
                  {{ $msTranslate('HomePage.noDevices.create.title') }}
                </ion-text>
                <ion-text class="other-needs-item-text__description">
                  {{ $msTranslate('HomePage.noDevices.create.description') }}
                </ion-text>
              </div>
              <ion-button
                id="create-organization-button"
                class="other-needs-item__button color-secondary"
                slot="icon-only"
                fill="clear"
              >
                <ion-icon :icon="arrowForward" />
              </ion-button>
            </div>
          </div>
        </div>
      </div>
      <!-- end of No organization -->
    </template>
    <template v-else>
      <div
        class="organization-content"
        :class="{ 'organization-content--columns': isColumnsLayout }"
      >
        <div class="organization-brand">
          <ms-image
            :image="LogoRowBlue"
            class="organization-brand__logo"
          />
          <ion-text class="organization-brand__subtitle">
            {{ $msTranslate('HomePage.organizationList.welcome') }}
          </ion-text>
        </div>

        <div class="organization-columns">
          <div class="organization-list">
            <template v-if="joinRequestList.length > 0">
              <ion-text class="organization-section-title">
                {{ $msTranslate('HomePage.organizationRequest.title') }}
              </ion-text>
              <organization-join-request
                v-for="joinRequest in joinRequestList"
                :key="joinRequest.enrollment.enrollmentId"
                :request="joinRequest"
                @join-organization="$emit('joinRequestClick', $event)"
                @delete-request="$emit('joinRequestClick', $event)"
              />
            </template>

            <template v-if="sortedDevices.length > 0">
              <ion-text class="organization-section-title">
                {{ $msTranslate('HomePage.organizationList.subtitle') }}
              </ion-text>
              <div class="organization-list-items">
                <organization-list-item
                  v-for="device in sortedDevices"
                  :key="device.deviceId"
                  :device="device"
                  :last-login-device="storedDeviceDataDict[device.deviceId]?.lastLogin"
                  :org-creation-date="storedDeviceDataDict[device.deviceId]?.orgCreationDate"
                  @click="$emit('organizationSelect', device)"
                  :logged-in="loggedInDevices.find((info: LoggedInDeviceInfo) => info.device.deviceId === device.deviceId) !== undefined"
                />
              </div>
            </template>
          </div>

          <div class="other-actions">
            <ion-text class="organization-section-title">
              {{ $msTranslate('HomePage.organizationList.otherActions') }}
            </ion-text>
            <div class="other-actions-card">
              <div class="other-actions-link">
                <ion-text class="other-actions-link__title">
                  {{ $msTranslate('HomePage.noDevices.link.title') }}
                </ion-text>
                <div class="other-actions-link-form">
                  <ms-input
                    placeholder="HomePage.noDevices.link.placeholder"
                    v-model="link"
                    @on-enter-keyup="onLinkClick(link)"
                    :validator="claimAndBootstrapLinkValidator"
                    class="other-actions-link-form__input"
                    ref="linkInput"
                  />
                  <ion-button
                    id="other-actions-join-button"
                    @click="onLinkClick(link)"
                    :disabled="linkInputRef?.validity !== Validity.Valid"
                    :icon-only="isColumnsLayout"
                  >
                    <ion-icon
                      v-if="isColumnsLayout"
                      :icon="arrowForward"
                    />
                    <template v-else>
                      {{ $msTranslate('HomePage.noDevices.link.continue') }}
                    </template>
                  </ion-button>
                </div>
                <i18n-t
                  scope="global"
                  keypath="HomePage.organizationList.linkDescription"
                  tag="ion-text"
                  class="other-actions-link__description"
                >
                  <template #join>
                    <a
                      class="other-actions-link__doc"
                      @click="Env.Links.openDocumentationUserGuideLink('join-organization', 'join-an-organization')"
                    >
                      {{ $msTranslate('HomePage.organizationList.linkDescriptionJoin') }}
                    </a>
                  </template>
                  <template #device>
                    <a
                      class="other-actions-link__doc"
                      @click="Env.Links.openDocumentationUserGuideLink('manage-devices', 'add-a-new-device')"
                    >
                      {{ $msTranslate('HomePage.organizationList.linkDescriptionDevice') }}
                    </a>
                  </template>
                </i18n-t>
              </div>
              <div class="other-actions-create">
                <ion-text class="other-actions-create__text">
                  {{ $msTranslate('HomePage.organizationList.needOrganization') }}
                </ion-text>
                <ion-button
                  id="other-actions-create-button"
                  class="other-actions-create__button link"
                  fill="clear"
                  size="small"
                  @click="$emit('createOrganizationClick')"
                >
                  {{ $msTranslate('HomePage.noDevices.create.title') }}
                </ion-button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import LogoRowBlue from '@/assets/images/logo-row-blue.svg?raw';
import MailUnreadGradient from '@/assets/images/mail-unread-gradient.svg?raw';
import { bootstrapLinkValidator, claimAndBootstrapLinkValidator } from '@/common/validators';
import OrganizationJoinRequest from '@/components/organizations/OrganizationJoinRequest.vue';
import OrganizationListItem from '@/components/organizations/OrganizationListItem.vue';
import { AccountInvitation, AsyncEnrollmentRequest, AvailableDevice, getLoggedInDevices, LoggedInDeviceInfo } from '@/parsec';
import { Env } from '@/services/environment';
import { StorageManager, StorageManagerKey, StoredDeviceData } from '@/services/storageManager';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { arrowForward, business, flask } from 'ionicons/icons';
import { DateTime } from 'luxon';
import { Clipboard, MsImage, MsInput, MsRichText, MsSpinner, Validity } from 'megashark-lib';
import { computed, inject, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const emits = defineEmits<{
  (e: 'organizationSelect', device: AvailableDevice): void;
  (e: 'createOrganizationClick'): void;
  (e: 'joinOrganizationClick'): void;
  (e: 'joinOrganizationWithLinkClick', link: string): void;
  (e: 'bootstrapOrganizationWithLinkClick', link: string): void;
  (e: 'trialClick'): void;
  (e: 'invitationClick', invitation: AccountInvitation): void;
  (e: 'joinRequestClick', request: AsyncEnrollmentRequest): void;
}>();

const props = defineProps<{
  deviceList: AvailableDevice[];
  invitationList: AccountInvitation[];
  joinRequestList: AsyncEnrollmentRequest[];
  querying: boolean;
}>();

async function onPasteLinkClick(): Promise<void> {
  link.value = await Clipboard.readText();
  await linkInputRef.value?.validate(link.value);
}

const storedDeviceDataDict = ref<{ [deviceId: string]: StoredDeviceData }>({});
const storageManager: StorageManager = inject(StorageManagerKey)!;
const linkInputRef = useTemplateRef<InstanceType<typeof MsInput>>('linkInput');
const link = ref('');
const loggedInDevices = ref<Array<LoggedInDeviceInfo>>([]);

const watchCancel = watch(
  () => props.deviceList,
  async () => {
    loggedInDevices.value = await getLoggedInDevices();
  },
);

async function onLinkClick(link: string): Promise<void> {
  if ((await bootstrapLinkValidator(link)).validity === Validity.Valid) {
    emits('bootstrapOrganizationWithLinkClick', link);
  } else {
    emits('joinOrganizationWithLinkClick', link);
  }
}

onMounted(async (): Promise<void> => {
  storedDeviceDataDict.value = await storageManager.retrieveDevicesData();
  loggedInDevices.value = await getLoggedInDevices();
});

onUnmounted(async () => {
  watchCancel();
});

const hasOrganizations = computed(() => props.deviceList.length > 0 || props.joinRequestList.length > 0);

// avoid creating a block that’s too large, we split it in two after four organisations
const isColumnsLayout = computed(() => props.deviceList.length > 3);

const sortedDevices = computed(() => {
  function deviceIsLoggedIn(device: AvailableDevice): boolean {
    return loggedInDevices.value.find((dInfo) => dInfo.device.deviceId === device.deviceId) !== undefined;
  }

  return props.deviceList.toSorted((a, b) => {
    const loggedInWeight = (deviceIsLoggedIn(b) ? 3 : 0) - (deviceIsLoggedIn(a) ? 3 : 0);

    const aLastLogin =
      storedDeviceDataDict.value[a.deviceId]?.lastLogin !== undefined
        ? (storedDeviceDataDict.value[a.deviceId].lastLogin as DateTime)
        : DateTime.fromMillis(0);
    const bLastLogin =
      storedDeviceDataDict.value[b.deviceId]?.lastLogin !== undefined
        ? (storedDeviceDataDict.value[b.deviceId].lastLogin as DateTime)
        : DateTime.fromMillis(0);
    let diff = bLastLogin.diff(aLastLogin).toObject().milliseconds ?? 0;
    if (diff === 0) {
      diff =
        a.organizationId === b.organizationId
          ? a.humanHandle.label.localeCompare(b.humanHandle.label)
          : a.organizationId.localeCompare(b.organizationId);
    }
    diff = Math.min(Math.max(diff, -1), 1);
    return loggedInWeight + diff;
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

.organization-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-grow: 1;
}

.organization-content {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-6xl');
  overflow-y: auto;
  height: 100%;
  width: 100%;
  max-width: 40rem;
  margin-inline: auto;
  padding: ms.spacing('padding-5xl') ms.spacing('padding-sm');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-lg') ms.spacing('padding-2xl') 0;
  }

  &--columns {
    max-width: 67.5rem;
  }
}

.organization-brand {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: ms.spacing('gap-3xl');
  width: 100%;

  &__logo {
    width: 7.375rem;
    flex-shrink: 0;
  }

  &__subtitle {
    @include ms.font('body-xl-medium');
    color: ms.color('text-brand-default-hover');
    opacity: ms.opacity('description');
    text-align: center;
  }
}

.organization-columns {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-5xl');
  width: 100%;

  .organization-content--columns & {
    flex-direction: row;
    align-items: flex-start;

    @include ms.responsive-breakpoint('md') {
      flex-direction: column;
      align-items: stretch;
    }
  }
}

.organization-section-title {
  @include ms.font('body-md-medium');
  color: ms.color('text-base-description');
  margin-left: 1rem;
}

.organization-list {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');
  width: 100%;

  .organization-content--columns & {
    flex: 1 1 40rem;
    min-width: 0;

    @include ms.responsive-breakpoint('md') {
      flex: auto;
    }
  }

  &-items {
    display: flex;
    flex-direction: column;
    background: ms.color('surface-base-default');
    border-radius: ms.radius('3xl');
    overflow: hidden;

    .organization-content--columns & {
      border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    }
  }
}

.other-actions {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: ms.spacing('gap-2xl');

  .organization-content--columns & {
    flex: 0 1 25rem;
    min-width: 0;

    @include ms.responsive-breakpoint('md') {
      flex-basis: auto;
    }
  }

  &-card {
    display: flex;
    flex-direction: column;
    background: ms.color('surface-base-default');
    border-radius: ms.radius('2xl');
    overflow: hidden;
  }

  &-link {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-2xl');
    padding: ms.spacing('padding-3xl');

    &__title {
      @include ms.font('heading-h6');
      color: ms.color('text-neutral-default');
    }

    &__description {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');
    }

    &__doc {
      color: inherit;
      text-decoration: underline;
      cursor: pointer;
      transition: color 150ms linear;

      &:hover {
        color: ms.color('text-brand-default');
      }
    }

    &-form {
      display: flex;
      align-items: flex-start;
      gap: ms.spacing('gap-2xl');

      &__input {
        flex: 1;
        min-width: 0;
      }

      #other-actions-join-button {
        flex-shrink: 0;
        min-width: 5rem;

        .organization-content--columns & {
          min-width: 2.5rem;
          width: 2.5rem;
        }

        ion-icon {
          font-size: 1.125rem;
          flex-shrink: 0;
        }
      }
    }
  }

  &-create {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-3xl');
    border-top: ms.border('thin') solid ms.color('border-neutral-default-subtle');

    &__text {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');
    }

    &__button {
      @include ms.font('label-md-medium');
      --color: #{ms.color('text-neutral-default')};
      --color-hover: #{ms.color('text-brand-default')};
      --background-hover: transparent;
      margin: 0;

      &::part(native) {
        padding-inline: 0;
      }
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
    align-items: center;

    &__title {
      @include ms.font('body-lg-medium');
      color: ms.color('text-base-description');
      text-align: center;
    }

    &-list {
      display: flex;
      flex-direction: column;
      width: 100%;
      max-width: 33rem;
      background: transparent;
      border-radius: ms.radius('2xl');
      overflow: hidden;
    }

    &-item {
      display: flex;
      align-items: flex-start;
      gap: ms.spacing('gap-3xl');
      padding: ms.spacing('padding-2xl');
      cursor: pointer;
      transition: background 150ms linear;

      &:not(:first-child) {
        border-top: ms.border('thin') solid ms.color('border-neutral-default-subtle');
      }

      &:hover {
        background: ms.color('surface-base-default-secondary');
      }

      &--orange:hover .other-needs-item-text__title {
        color: ms.color('text-extra-orange-default');
      }

      &--purple:hover .other-needs-item-text__title {
        color: ms.color('text-extra-purple-default');
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

      &-text {
        display: flex;
        flex-direction: column;
        gap: ms.spacing('gap-md');
        flex: 1;
        min-width: 0;

        &__title {
          @include ms.font('heading-h5');
          color: ms.color('text-base-heading');
          transition: color 150ms linear;
        }

        &__description {
          @include ms.font('body-md-regular');
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
