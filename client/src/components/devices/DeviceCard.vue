<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="card"
    :class="isCurrent ? 'device-active' : ''"
  >
    <ion-icon
      class="icon-device"
      :icon="desktopOutline"
    />
    <div class="card-text">
      <div class="card-text-info">
        <ion-text class="device-name body">
          {{ device.deviceLabel }}
        </ion-text>
        <ion-text class="join-date body-sm">
          {{ $msTranslate('DevicesPage.joinedOn') }}
          <span>{{ $msTranslate(formatTimeSince(device.createdOn, '--', 'short', true)) }}</span>
        </ion-text>
      </div>
      <technical-id
        v-show="showId"
        :id="device.id"
      />
    </div>

    <ion-text
      class="badge button-medium"
      v-show="isCurrent"
      :outline="true"
    >
      {{ $msTranslate('DevicesPage.activeDeviceBadge') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { DeviceInfo } from '@/parsec';
import { formatTimeSince } from 'megashark-lib';
import { IonIcon, IonText } from '@ionic/vue';
import { desktopOutline } from 'ionicons/icons';
import { TechnicalId } from '@/components/misc';

defineProps<{
  device: DeviceInfo;
  isCurrent?: boolean;
  showId?: boolean;
}>();
</script>

<style scoped lang="scss">
.card {
  background-color: var(--parsec-color-light-secondary-white);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  padding: 1em;
  width: 100%;
  border-radius: var(--parsec-radius-8);
  display: flex;
  align-items: center;
  gap: 1.5rem;

  &.device-active {
    background-color: var(--parsec-color-light-secondary-background);
  }

  .icon-device {
    font-size: 1.5rem;
    padding: 0.5rem;
    border-radius: var(--parsec-radius-circle);
    color: var(--parsec-color-light-secondary-grey);
    background-color: var(--parsec-color-light-secondary-premiere);
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &-info {
      display: flex;
      flex-direction: column;
      gap: 0.125rem;
    }

    .device-name {
      color: var(--parsec-color-light-secondary-text);
    }

    .join-date {
      color: var(--parsec-color-light-secondary-soft-text);

      &:has(.join-date-now) {
        position: relative;
        display: flex;
        align-items: center;
        gap: 0.125rem;

        &::before {
          content: '';
          position: relative;
          display: block;
          width: 0.5em;
          height: 0.5em;
          border-radius: var(--parsec-radius-circle);
          background-color: var(--parsec-color-light-primary-500);
        }
      }
    }
  }
}

.badge {
  margin: auto 0 auto auto;
  border-radius: var(--parsec-radius-32);
  padding: 0.25em 0.5em;
  color: var(--parsec-color-light-primary-600);
  background: var(--parsec-color-light-primary-100);
}
</style>
