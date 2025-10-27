<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="organization-card"
    :class="{
      'header-only': orgNameOnly,
      'organization-card-logged-in': loggedIn,
    }"
  >
    <div class="organization-card-header">
      <ion-text
        v-if="!isTrialOrg"
        class="organization-card-header__initials subtitles-normal"
      >
        {{ device.organizationId?.substring(0, 2) }}
      </ion-text>
      <ms-image
        v-else
        :image="ResourcesManager.instance().get(Resources.LogoIcon, LogoIconWhite) as string"
        class="organization-card-header__logo"
      />
    </div>
    <ion-card-content class="organization-card-content">
      <div class="organization-card-content-text">
        <ion-text class="organization-name title-h4">{{ device.organizationId }}</ion-text>
      </div>
      <div class="organization-card-content-login">
        <div
          v-show="!loggedIn"
          v-if="!orgNameOnly"
          class="login-content"
        >
          <ion-icon
            :icon="time"
            class="login-icon"
          />
          <ion-text class="login-text body">
            {{ getLastLoginText() }}
          </ion-text>
        </div>
        <div class="login-content">
          <ion-icon
            :icon="person"
            class="login-icon"
          />
          <ion-text class="login-text login-name body">{{ device.humanHandle.label }}</ion-text>
        </div>
      </div>
    </ion-card-content>
    <!-- trial expiration badge -->
    <ion-text
      v-if="expirationDuration"
      class="organization-card-expiration button-small"
    >
      {{ $msTranslate(formatExpirationTime(expirationDuration)) }}
    </ion-text>
    <ion-text
      class="organization-card-badge-connected button-small"
      v-if="loggedIn"
    >
      <ion-icon
        :icon="checkmarkCircle"
        class="connected-icon"
      />
      <span
        class="connected-text"
        v-if="isLargeDisplay"
      >
        {{ $msTranslate('HomePage.organizationList.loggedIn') }}
      </span>
    </ion-text>
    <ms-image
      v-if="!orgNameOnly && !(isTrialOrg && isSmallDisplay) && !loggedIn"
      class="organization-card-icon"
      :image="ArrowForwardCircleGradient"
    />
  </div>
</template>

<script setup lang="ts">
import ArrowForwardCircleGradient from '@/assets/images/arrow-forward-circle.svg?raw';
import { formatExpirationTime, getDurationBeforeExpiration, isTrialOrganizationDevice } from '@/common/organization';
import { AvailableDevice } from '@/parsec';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonCardContent, IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle, person, time } from 'ionicons/icons';
import { DateTime, Duration } from 'luxon';
import { formatTimeSince, I18n, LogoIconWhite, MsImage, useWindowSize } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const { isSmallDisplay, isLargeDisplay } = useWindowSize();
const isTrialOrg = ref(false);
const expirationDuration = ref<Duration>();

const props = defineProps<{
  device: AvailableDevice;
  lastLoginDevice?: DateTime;
  orgCreationDate?: DateTime;
  orgNameOnly?: boolean;
  loggedIn?: boolean;
}>();

onMounted(async () => {
  isTrialOrg.value = isTrialOrganizationDevice(props.device);
  if (isTrialOrg.value && props.orgCreationDate) {
    expirationDuration.value = getDurationBeforeExpiration(props.orgCreationDate);
  }
});

function getLastLoginText(): string {
  if (!props.lastLoginDevice) {
    return '--';
  }
  const defaultTranslatable = 'HomePage.organizationList.noLastLogin';
  const timeSince = formatTimeSince(props.lastLoginDevice, defaultTranslatable, 'short');
  return I18n.translate(timeSince);
}
</script>

<style lang="scss" scoped>
.organization-card {
  background: var(--parsec-color-light-secondary-premiere);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12);
  width: 100%;
  min-height: 5rem;
  padding-right: 4rem;
  display: flex;
  gap: 0.75rem;
  user-select: none;
  box-shadow: none;
  overflow: hidden;
  cursor: pointer;
  transition:
    box-shadow 150ms linear,
    border 150ms linear;
  position: relative;
  flex-shrink: 0;

  &-logged-in {
    padding-right: 1.5rem;
  }

  @include ms.responsive-breakpoint('sm') {
    padding-right: 1rem;
  }

  // when using the card as a header only
  &.header-only {
    padding: 0 0.5rem 0 0;
    box-shadow: none;
    border: none;
    background: var(--parsec-color-light-secondary-background);
    cursor: inherit;
  }

  &:hover:not(.header-only) {
    box-shadow: var(--parsec-shadow-light);
    border: 1px solid var(--parsec-color-light-primary-600);

    .organization-card-icon {
      right: 1rem;
    }
  }

  &-header {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-shrink: 0;
    gap: 0.5rem;
    width: 4rem;
    z-index: 1;
    background: var(--parsec-color-light-gradient-background);
    position: relative;

    @include ms.responsive-breakpoint('xs') {
      width: 4.5rem;
    }

    &::before {
      content: '';
      position: absolute;
      width: 4rem;
      height: 5rem;
      top: 0;
      right: 0;
      background-image: url('@/assets/images/background/organization-shapes.svg');
      background-size: cover;
      background-repeat: no-repeat;
      background-position: top left;

      @include ms.responsive-breakpoint('xs') {
        width: 4.5rem;
        height: 100%;
      }
    }

    &__initials {
      color: var(--parsec-color-light-secondary-white);
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;

      @include ms.responsive-breakpoint('xs') {
        font-size: 1.15rem;
      }
    }

    &__logo {
      width: 1.5rem;
      flex-shrink: 0;
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding: 1rem 0;
    gap: 0.25rem;
    width: 100%;
    height: 100%;
    overflow: hidden;
    flex-shrink: 1;

    @include ms.responsive-breakpoint('xs') {
      padding: 0.75rem 0;
      overflow: hidden;
      gap: 0.5rem;
    }

    &-text {
      display: flex;
      gap: 0.375rem;
      overflow-x: hidden;

      .organization-name {
        color: var(--parsec-color-light-secondary-text);
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
      }
    }

    &-login {
      display: flex;
      gap: 1rem;

      @include ms.responsive-breakpoint('xs') {
        flex-direction: column;
        gap: 0.25rem;
      }

      .login-content {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0;
        color: var(--parsec-color-light-secondary-hard-grey);
        overflow: hidden;

        .login-icon {
          color: var(--parsec-color-light-secondary-light);
          font-size: 1rem;
          flex-shrink: 0;
        }

        .login-text {
          overflow: hidden;
          white-space: nowrap;
          text-overflow: ellipsis;
        }

        &:nth-child(1) {
          flex-shrink: 0;
        }

        &:nth-child(2) {
          .login-text {
            overflow: hidden;
            white-space: nowrap;
            text-overflow: ellipsis;
          }
        }
      }
    }
  }

  &-icon {
    position: absolute;
    right: 1.5rem;
    top: 50%;
    transform: translateY(-50%);
    width: 1.5rem;
    margin-left: auto;
    flex-shrink: 0;
    transition: right 150ms linear;

    @include ms.responsive-breakpoint('xs') {
      position: relative;
      right: 0.25rem;
    }
  }

  &-badge-connected {
    display: flex;
    margin: auto;
    align-items: center;
    padding: 0.1875rem 0.5rem;
    gap: 0.25rem;
    color: var(--parsec-color-light-success-700);
    background: var(--parsec-color-light-success-50);
    border: 1px solid var(--parsec-color-light-success-700);
    border-radius: var(--parsec-radius-12);
    height: fit-content;
    flex-shrink: 0;

    ion-icon {
      color: var(--parsec-color-light-success-700);
      font-size: 0.8125rem;
    }

    @include ms.responsive-breakpoint('sm') {
      border: none;
      background: transparent;

      ion-icon {
        font-size: 1.375rem;
        padding: 0;
      }
    }
  }

  &-expiration {
    border-radius: var(--parsec-radius-12);
    padding: 0.1875rem 0.5rem;
    width: fit-content;
    align-self: center;
    height: fit-content;
    flex-shrink: 0;
    background: var(--parsec-color-light-secondary-text);
    color: var(--parsec-color-light-secondary-white);

    @include ms.responsive-breakpoint('xs') {
      margin-left: auto;
      flex-shrink: 0;
    }
  }
}
</style>
