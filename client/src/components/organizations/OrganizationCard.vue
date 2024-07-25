<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="organization-card"
    :class="orgNameOnly ? 'header-only' : ''"
  >
    <div class="organization-card-header">
      <ion-avatar class="organization-card-header__avatar body-lg">
        <span v-if="!isTrialOrg">{{ device.organizationId?.substring(0, 2) }}</span>
        <ms-image
          v-else
          :image="LogoIconGradient"
          class="organization-avatar-logo"
        />
      </ion-avatar>
      <ion-card-title class="organization-card-header__title title-h4">{{ device.organizationId }}</ion-card-title>
    </div>
    <ion-card-content
      v-if="!orgNameOnly"
      class="organization-card-content"
    >
      <div class="organization-card-login">
        <ion-text class="organization-card-login__name body">{{ device.humanHandle.label }}</ion-text>
        <div
          v-show="!isDeviceLoggedIn(device)"
          class="organization-card-login-time"
        >
          <ion-icon
            :icon="time"
            class="organization-card-login-time__icon"
          />
          <ion-text
            class="organization-card-login-time__text body-sm"
            v-if="lastLoginDevice"
          >
            {{ device.deviceId in lastLoginDevice ? $msTranslate(formatTimeSince(lastLoginDevice, '--')) : '--' }}
          </ion-text>
        </div>
        <div
          v-show="isDeviceLoggedIn(device)"
          class="organization-card-login-time connected"
        >
          <ion-icon
            :icon="ellipse"
            class="success"
          />
          <ion-text class="body-sm">{{ $msTranslate('HomePage.organizationList.loggedIn') }}</ion-text>
        </div>
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
  </div>
</template>

<script setup lang="ts">
import { AvailableDevice, isDeviceLoggedIn } from '@/parsec';
import { IonAvatar, IonCardTitle, IonText, IonIcon, IonCardContent } from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { ellipse, time } from 'ionicons/icons';
import { MsImage, LogoIconGradient, formatTimeSince } from 'megashark-lib';
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
</script>

<style lang="scss" scoped>
.organization-card {
  background: var(--parsec-color-light-secondary-background);
  transition: box-shadow 150ms linear;
  border: 1px solid var(--parsec-color-light-secondary-medium);
  box-shadow: none;
  border-radius: var(--parsec-radius-12);
  padding: 0.75rem;
  width: 100%;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  user-select: none;
  box-shadow: none;
  cursor: pointer;

  // when using the card as a header only
  &.header-only {
    padding: 0;
    box-shadow: none;
    border: none;
    background: none;
    cursor: inherit;
  }

  &:hover:not(.header-only) {
    box-shadow: var(--parsec-shadow-light);
  }

  &-header {
    display: flex;
    align-items: center;
    flex-direction: row;
    gap: 0.5rem;
    position: relative;
    z-index: 1;

    &__avatar {
      background-color: var(--parsec-color-light-secondary-white);
      color: var(--parsec-color-light-primary-600);
      width: 2.5rem;
      height: 2.5rem;
      border-radius: var(--parsec-radius-12);
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      position: relative;
      z-index: 1;
      border: 1px solid var(--parsec-color-light-secondary-medium);

      .organization-avatar-logo {
        width: 1.5rem;
      }
    }

    &__title {
      display: flex;
      flex-direction: column;
      gap: 0.375rem;
      color: var(--parsec-color-light-primary-700);
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 0;
  }
}

.organization-card-login {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &__name {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &-time {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0;

    &__icon {
      color: var(--parsec-color-light-secondary-light);
      font-size: 1.125rem;
    }

    &.connected {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      color: var(--parsec-color-light-success-700);

      .success {
        color: var(--parsec-color-light-success-700);
        font-size: 0.675rem;
      }
    }
  }
}

.organization-card-expiration {
  border-radius: var(--parsec-radius-12);
  padding: 0.25rem 0.5rem;
  position: absolute;
  bottom: 0;
  right: 0;
  background: var(--parsec-color-light-primary-700);
  color: var(--parsec-color-light-secondary-white);

  &.expired {
    background: var(--parsec-color-light-secondary-grey);
    color: var(--parsec-color-light-secondary-white);
  }
}
</style>
