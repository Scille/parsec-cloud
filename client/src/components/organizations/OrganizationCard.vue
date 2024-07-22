<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-card class="organization-card__body">
    <ion-card-header class="card-content">
      <div class="organization-info">
        <ion-avatar
          class="orga-avatar body-lg"
          v-show="!isTrialOrg"
        >
          <span>{{ device.organizationId?.substring(0, 2) }}</span>
        </ion-avatar>
        <ion-text
          v-if="expirationTime"
          class="orga-expiration button-small"
          :class="{ expired: isExpired() }"
        >
          {{ $msTranslate(formatExpirationTime()) }}
        </ion-text>
        <div class="orga-text">
          <ion-card-title class="card-title">
            <span class="title-h4">{{ device.organizationId }}</span>
            <span class="subtitles-sm">{{ device.humanHandle.label }}</span>
          </ion-card-title>
        </div>
      </div>
    </ion-card-header>
  </ion-card>
</template>

<script setup lang="ts">
import { AvailableDevice } from '@/parsec';
import { getServerTypeFromHost, ServerType, TRIAL_EXPIRATION_DAYS } from '@/services/parsecServers';
import { IonAvatar, IonCard, IonCardHeader, IonCardTitle, IonText } from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { Duration } from 'luxon';
import { Translatable } from 'megashark-lib';

const props = defineProps<{
  device: AvailableDevice;
}>();

const isTrialOrg = ref(false);
const expirationTime = ref<Duration | undefined>(undefined);

onMounted(async () => {
  const url = new URL(props.device.serverUrl);
  const serverType = getServerTypeFromHost(url.hostname, url.port.length > 0 ? parseInt(url.port) : undefined);
  isTrialOrg.value = serverType === ServerType.Trial;
  if (isTrialOrg.value) {
    expirationTime.value = props.device.createdOn.plus({ days: TRIAL_EXPIRATION_DAYS }).diffNow(['days', 'hours']);
  }
});

function isExpired(): boolean {
  if (!expirationTime.value) {
    return false;
  }
  return expirationTime.value.days <= 0 && expirationTime.value.hours <= 0;
}

function formatExpirationTime(): Translatable {
  if (!expirationTime.value) {
    return '';
  }
  if (expirationTime.value.days > 0) {
    return {
      key: 'HomePage.organizationList.expiration.days',
      count: expirationTime.value.days,
      data: { days: expirationTime.value.days },
    };
  } else if (expirationTime.value.hours > 0) {
    return {
      key: 'HomePage.organizationList.expiration.hours',
      count: Math.floor(expirationTime.value.hours),
      data: { hours: Math.floor(expirationTime.value.hours) },
    };
  }
  return { key: 'HomePage.organizationList.expiration.expired' };
}
</script>

<style lang="scss" scoped>
.organization-card__body {
  user-select: none;
  box-shadow: none;
  background: none;
  margin: 0;

  .card-content {
    padding: 0;
  }

  .organization-info {
    display: flex;
    align-items: center;
    flex-direction: row;
    gap: 0.5rem;
    position: relative;
    z-index: 1;

    .orga-avatar {
      background-color: var(--parsec-color-light-secondary-white);
      color: var(--parsec-color-light-primary-600);
      width: 3.25rem;
      height: 3.25rem;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      position: relative;
      z-index: 1;
      border: 1px solid var(--parsec-color-light-secondary-medium);
    }

    .orga-expiration {
      border-radius: var(--parsec-radius-12);
      padding: 0.25rem 0.5rem;
      position: absolute;
      top: 0;
      right: 0;
      background: var(--parsec-color-light-primary-700);
      color: var(--parsec-color-light-secondary-white);

      &.expired {
        background: var(--parsec-color-light-secondary-grey);
        color: var(--parsec-color-light-secondary-white);
      }
    }

    .card-title {
      display: flex;
      flex-direction: column;
      gap: 0.375rem;

      span:first-child {
        color: var(--parsec-color-light-primary-700);
      }

      span:last-child {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}
</style>
