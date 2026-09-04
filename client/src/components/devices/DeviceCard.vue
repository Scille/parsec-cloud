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
          <ion-text class="device-name">
            {{ $msTranslate(deviceInfo.label) }}
          </ion-text>
          <ion-text class="join-date">
            {{ $msTranslate('DevicesPage.joinedOn') }}
            <span>{{ $msTranslate(formatTimeSince(device.createdOn, '--', 'short', true)) }}</span>
          </ion-text>
        </div>
      </div>

      <ion-text
        class="badge-active"
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
import { desktopOutline, logoAndroid, logoApple, logoTux, logoWindows, phonePortraitOutline } from 'ionicons/icons';
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
  [DeviceLabel.MobileWeb, { label: 'common.deviceTypes.mobileWeb', icon: phonePortraitOutline }],
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
  background-color: ms.color('surface-base-default');
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  padding: ms.spacing('padding-3xl');
  width: 100%;
  border-radius: ms.radius('lg');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');

  &-content {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-4xl');
  }

  .icon-device {
    font-size: 1.5rem;
    flex-shrink: 0;
    padding: ms.spacing('padding-lg');
    border-radius: ms.radius('full');
    color: ms.color('icon-neutral-default');
    background-color: ms.color('surface-base-page-secondary');
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
    overflow: hidden;

    &-info {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-xs');
    }

    .device-name {
      @include ms.font('body-md-medium');
      color: ms.color('text-brand-default');
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }

    .join-date {
      @include ms.font('body-sm-regular');
      color: ms.color('text-neutral-default');

      &:has(.join-date-now) {
        position: relative;
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-xs');

        &::before {
          content: '';
          position: relative;
          display: block;
          width: 0.5em;
          height: 0.5em;
          border-radius: ms.radius('full');
          background-color: ms.color('surface-brand-default');
        }
      }
    }
  }

  &.device-active {
    background-color: ms.color('surface-base-default-secondary');

    .icon-device {
      color: ms.color('icon-neutral-default');
      background-color: ms.color('surface-neutral-default-subtle-pressed');
    }
  }
}

.badge-active {
  @include ms.font('label-md-medium');
  position: absolute;
  top: 1rem;
  right: 1rem;
  background: ms.color('surface-brand-default-subtle-hover');
  color: ms.color('text-brand-default');
  flex-shrink: 0;
  padding: ms.spacing('padding-sm') ms.spacing('padding-lg');
  border-radius: ms.radius('2xl');
  white-space: nowrap;
}
</style>
