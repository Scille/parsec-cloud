<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="card"
    :class="isCurrent ? 'device-active' : ''"
  >
    <div class="card-content">
      <ion-icon
        class="icon-device"
        :icon="deviceInfo.icon"
      />
      <div class="card-text">
        <div class="card-text-info">
          <ion-text class="device-name subtitles-sm">
            {{ $msTranslate(deviceInfo.label) }}
          </ion-text>
          <ion-text class="join-date body-sm">
            {{ $msTranslate('DevicesPage.joinedOn') }}
            <span>{{ $msTranslate(formatTimeSince(device.createdOn, '--', 'short', true)) }}</span>
          </ion-text>
        </div>
      </div>

      <ion-text
        class="badge-active button-medium"
        v-show="isCurrent"
        :outline="true"
      >
        {{ $msTranslate('DevicesPage.activeDeviceBadge') }}
      </ion-text>
    </div>

    <technical-id
      v-show="showId"
      :id="device.id"
    />
  </div>
</template>

<script setup lang="ts">
import { DeviceLabel } from '@/common/device';
import { TechnicalId } from '@/components/misc';
import { DeviceInfo } from '@/parsec';
import { IonIcon, IonText } from '@ionic/vue';
import { desktopOutline, logoAndroid, logoApple, logoTux, logoWindows, phoneLandscapeOutline } from 'ionicons/icons';
import { formatTimeSince, Translatable } from 'megashark-lib';
import { computed } from 'vue';

const props = defineProps<{
  device: DeviceInfo;
  isCurrent?: boolean;
  showId?: boolean;
}>();

const DEVICE_PLATFORMS: Map<DeviceLabel, { label: Translatable; icon: string }> = new Map([
  [DeviceLabel.Android, { label: 'common.deviceTypes.android', icon: logoAndroid }],
  [DeviceLabel.Linux, { label: 'common.deviceTypes.linux', icon: logoTux }],
  [DeviceLabel.Windows, { label: 'common.deviceTypes.windows', icon: logoWindows }],
  [DeviceLabel.MacOS, { label: 'common.deviceTypes.macos', icon: logoApple }],
  [DeviceLabel.MobileWeb, { label: 'common.deviceTypes.mobileWeb', icon: phoneLandscapeOutline }],
  [DeviceLabel.Web, { label: 'common.deviceTypes.web', icon: desktopOutline }],
]);

const deviceInfo = computed(() => {
  return (
    DEVICE_PLATFORMS.get(props.device.deviceLabel as DeviceLabel) ?? {
      label: { key: 'common.deviceTypes.unknown', data: { label: props.device.deviceLabel } },
      icon: desktopOutline,
    }
  );
});
</script>

<style scoped lang="scss">
.card {
  background-color: var(--parsec-color-light-secondary-white);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  padding: 1em;
  width: 100%;
  border-radius: var(--parsec-radius-8);
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &-content {
    display: flex;
    align-items: center;
    gap: 1.5rem;
  }

  .icon-device {
    font-size: 1.5rem;
    flex-shrink: 0;
    padding: 0.5rem;
    border-radius: var(--parsec-radius-circle);
    color: var(--parsec-color-light-secondary-grey);
    background-color: var(--parsec-color-light-secondary-premiere);
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    overflow: hidden;

    &-info {
      display: flex;
      flex-direction: column;
      gap: 0.125rem;
    }

    .device-name {
      color: var(--parsec-color-light-primary-700);
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
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

  &.device-active {
    background-color: var(--parsec-color-light-secondary-background);

    .icon-device {
      color: var(--parsec-color-light-secondary-hard-grey);
      background-color: var(--parsec-color-light-secondary-medium);
    }
  }
}

.badge-active {
  position: absolute;
  top: 1rem;
  right: 1rem;
  background: var(--parsec-color-light-primary-50);
  color: var(--parsec-color-light-primary-500);
  flex-shrink: 0;
  padding: 0.25rem 0.5rem;
  border-radius: var(--parsec-radius-12);
  white-space: nowrap;
}
</style>
