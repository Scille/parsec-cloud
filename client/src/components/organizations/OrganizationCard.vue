<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-card class="organization-card__body">
    <ion-card-header class="card-content">
      <div class="organization-info">
        <ion-avatar class="orga-avatar body-lg">
          <span v-if="isTrialOrg">{{ device.organizationId?.substring(0, 2) }}</span>
          <ms-image
            v-else
            :image="LogoIconGradient"
            class="orga-avatar-logo"
          />
        </ion-avatar>
        <ion-text
          v-if="expirationDuration"
          class="orga-expiration button-small"
          :class="{ expired: isExpired(expirationDuration) }"
        >
          {{ $msTranslate(formatExpirationTime(expirationDuration)) }}
        </ion-text>
        <div class="orga-text">
          <ion-card-title class="card-title">
            <span class="title-h4">{{ device.organizationId }}</span>
            <span class="subtitles-sm">{{ device.humanHandle.label }}</span>
          </ion-card-title>
        </div>
      </div>
    </ion-card-header>
    <ion-text class="orga-email body">{{ device.humanHandle.label }}</ion-text>
  </ion-card>
</template>

<script setup lang="ts">
import { AvailableDevice } from '@/parsec';
import { getServerTypeFromHost, ServerType } from '@/services/parsecServers';
import { IonAvatar, IonCard, IonCardHeader, IonCardTitle, IonText } from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { Duration } from 'luxon';
import { LogoIconGradient } from 'megashark-lib';
import { getDurationBeforeExpiration, formatExpirationTime, isExpired } from '@/common/organization';

const props = defineProps<{
  device: AvailableDevice;
}>();

const isTrialOrg = ref(false);
const expirationDuration = ref<Duration | undefined>(undefined);

onMounted(async () => {
  const url = new URL(props.device.serverUrl);
  const serverType = getServerTypeFromHost(url.hostname, url.port.length > 0 ? parseInt(url.port) : undefined);
  isTrialOrg.value = serverType === ServerType.Trial;
  if (isTrialOrg.value) {
    expirationDuration.value = getDurationBeforeExpiration(props.device.createdOn);
  }
});
</script>

<style lang="scss" scoped>
.organization-card__body {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
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

      &-logo {
        width: 1.5rem;
      }
    }

    .card-title {
      display: flex;
      flex-direction: column;
      gap: 0.375rem;
      color: var(--parsec-color-light-primary-700);
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
  }

  .orga-email {
    color: var(--parsec-color-light-secondary-hard-grey);
  }
}
</style>
