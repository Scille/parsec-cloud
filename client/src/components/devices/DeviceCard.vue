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
      <ion-text class="device-name body">
        {{ label }}
      </ion-text>
      <ion-text class="join-date body-sm">
        {{ $t('DevicesPage.joinedOn') }}
        <span v-if="date">{{ formatTimeSince(date, '--', 'short', true) }}</span>
        <span
          v-if="!date"
          class="join-date-now"
        >
          {{ $t('DevicesPage.now') }}
        </span>
      </ion-text>
    </div>

    <ion-text
      class="badge caption-caption"
      v-show="isCurrent"
      :outline="true"
    >
      {{ $t('DevicesPage.activeDeviceBadge') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import { IonIcon, IonText } from '@ionic/vue';
import { desktopOutline } from 'ionicons/icons';
import { DateTime } from 'luxon';

defineProps<{
  label: string;
  isCurrent: boolean;
  date?: DateTime;
}>();
</script>

<style scoped lang="scss">
.card {
  background-color: var(--parsec-color-light-secondary-background);
  height: 5em;
  padding: 1em;
  width: 100%;
  border-radius: 6px;
  display: flex;
  gap: 1.5rem;

  &.device-active {
    background-color: var(--parsec-color-light-primary-50);
  }

  .icon-device {
    height: 100%;
    width: 3em;
    color: var(--parsec-color-light-primary-800);
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;

    .device-name {
      color: var(--parsec-color-light-secondary-text);
    }

    .join-date {
      color: var(--parsec-color-light-secondary-grey);

      &:has(.join-date-now) {
        position: relative;
        display: flex;
        align-items: center;
        gap: 0.35rem;

        &::before {
          content: '';
          position: relative;
          display: block;
          width: 0.5em;
          height: 0.5em;
          border-radius: 50%;
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
  color: var(--parsec-color-light-success-700);
  border: 1px solid var(--parsec-color-light-success-500);
  background: var(--parsec-color-light-success-100);
}
</style>
