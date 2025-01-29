<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="organization-card"
    :class="orgNameOnly ? 'header-only' : ''"
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
        :image="LogoIconGradient"
        class="organization-card-header__logo"
      />
    </div>
    <ion-card-content class="organization-card-content">
      <div class="organization-card-content-text">
        <ion-text class="organization-name title-h4">{{ device.organizationId }}</ion-text>
        <ion-text
          class="organization-connected body-sm"
          v-show="isDeviceLoggedIn(device)"
        >
          {{ $msTranslate('HomePage.organizationList.loggedIn') }}
        </ion-text>
      </div>
      <div class="organization-card-content-login">
        <div
          v-show="!isDeviceLoggedIn(device)"
          v-if="!orgNameOnly"
          class="login-time"
        >
          <ion-icon
            :icon="time"
            class="login-time__icon"
          />
          <ion-text class="login-time__text body-sm">
            {{ getLastLoginText() }}
          </ion-text>
        </div>
        <ion-text class="login-name body">({{ device.humanHandle.label }})</ion-text>
      </div>
      <!-- trial expiration badge -->
      <ion-text
        v-if="expirationDuration"
        class="organization-card-expiration button-small"
        :class="{ expired: isExpired(expirationDuration) }"
      >
        {{ $msTranslate(formatExpirationTime(expirationDuration)) }}
      </ion-text>
    </ion-card-content>
    <img
      src="@/assets/images/arrow-forward-circle.svg"
      class="organization-card-icon"
      v-if="!orgNameOnly"
    />
  </div>
</template>

<script setup lang="ts">
import { AvailableDevice, isDeviceLoggedIn } from '@/parsec';
import { IonText, IonIcon, IonCardContent } from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { time } from 'ionicons/icons';
import { MsImage, LogoIconGradient, formatTimeSince, I18n } from 'megashark-lib';
import { formatExpirationTime, isExpired, isTrialOrganizationDevice, getDurationBeforeExpiration } from '@/common/organization';
import { Duration, DateTime } from 'luxon';

const isTrialOrg = ref(false);
const expirationDuration = ref<Duration>();

const props = defineProps<{
  device: AvailableDevice;
  lastLoginDevice?: DateTime;
  orgNameOnly?: boolean;
}>();

onMounted(async () => {
  isTrialOrg.value = isTrialOrganizationDevice(props.device);
  if (isTrialOrg.value) {
    expirationDuration.value = getDurationBeforeExpiration(props.device.createdOn);
  }
});

function getLastLoginText(): string {
  if (!props.lastLoginDevice) {
    return '--';
  }
  const defaultTranslatable = 'HomePage.organizationList.noLastLogin';
  const timeSince = formatTimeSince(props.lastLoginDevice, defaultTranslatable);
  return I18n.translate(timeSince);
}
</script>

<style lang="scss" scoped>
.organization-card {
  background: var(--parsec-color-light-secondary-premiere);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12);
  width: 100%;
  height: 100%;
  min-height: 5rem;
  padding-right: 1.5rem;
  display: flex;
  gap: 0.75rem;
  user-select: none;
  box-shadow: none;
  overflow: hidden;
  cursor: pointer;
  transition: all 150ms linear;

  // when using the card as a header only
  &.header-only {
    padding: 0;
    box-shadow: none;
    border: none;
    background: var(--parsec-color-light-secondary-background);
    cursor: inherit;
  }

  &:hover:not(.header-only) {
    box-shadow: var(--parsec-shadow-light);
    padding-right: 1rem;
    border: 1px solid var(--parsec-color-light-primary-600);
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
    }

    &__initials {
      color: var(--parsec-color-light-secondary-white);
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
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
    gap: 0.5rem;
    padding: 0;
    gap: 0.25rem;
    width: 100%;
    overflow: hidden;

    &-text {
      display: flex;
      gap: 0.375rem;
      overflow: hidden;

      .organization-name {
        color: var(--parsec-color-light-secondary-text);
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
      }

      .organization-connected {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: var(--parsec-color-light-success-700);
      }
    }

    &-login {
      display: flex;
      gap: 0.5rem;

      .login-name {
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      .login-time {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0;

        &__icon {
          color: var(--parsec-color-light-secondary-light);
          font-size: 1rem;
        }
      }
    }
  }

  &-icon {
    width: 1.5rem;
    margin-left: auto;
    flex-shrink: 0;
  }
}

.organization-card-expiration {
  border-radius: var(--parsec-radius-12);
  padding: 0.1875rem 0.375rem;
  background: var(--parsec-color-light-primary-700);
  color: var(--parsec-color-light-secondary-white);

  &.expired {
    background: var(--parsec-color-light-secondary-text);
    color: var(--parsec-color-light-secondary-white);
  }
}
</style>
