<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="organization-item">
    <div class="organization-item-content">
      <div class="organization-item-avatar">
        <ion-text
          v-if="!isTrialOrg"
          class="organization-item-avatar__initials"
        >
          {{ device.organizationId.substring(0, 2) }}
        </ion-text>
        <ms-image
          v-else
          :image="ResourcesManager.instance().get(Resources.LogoIcon, LogoIconWhite) as string"
          class="organization-item-avatar__logo"
        />
      </div>
      <div class="organization-item-text">
        <div class="organization-item-text-header">
          <ion-text class="organization-item-text__name">{{ device.organizationId }}</ion-text>
          <ion-text
            v-if="expirationDuration"
            class="organization-item-expiration"
          >
            {{ $msTranslate(formatExpirationTime(expirationDuration)) }}
          </ion-text>
        </div>
        <ion-text class="organization-item-text__email">{{ device.humanHandle.email }}</ion-text>
      </div>
      <ion-text
        v-if="loggedIn"
        class="organization-item-badge-connected"
      >
        <ion-icon
          :icon="checkmarkCircle"
          class="connected-icon"
        />
        <span class="connected-text">{{ $msTranslate('HomePage.organizationList.loggedIn') }}</span>
      </ion-text>
      <div
        v-else
        class="organization-item-last-login"
      >
        <ion-text class="organization-item-last-login__label">
          {{ $msTranslate('HomePage.organizationList.lastLogin') }}
        </ion-text>
        <ion-text class="organization-item-last-login__date">
          {{ lastLoginText }}
        </ion-text>
      </div>
    </div>
    <ion-icon
      class="organization-item-arrow"
      :icon="arrowForward"
    />
  </div>
</template>

<script setup lang="ts">
import { formatExpirationTime, getDurationBeforeExpiration, isTrialOrganizationDevice } from '@/common/organization';
import { AvailableDevice } from '@/parsec';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonIcon, IonText } from '@ionic/vue';
import { arrowForward, checkmarkCircle } from 'ionicons/icons';
import { DateTime, Duration } from 'luxon';
import { I18n, LogoIconWhite, MsImage } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const isTrialOrg = ref(false);
const expirationDuration = ref<Duration>();

const props = defineProps<{
  device: AvailableDevice;
  lastLoginDevice?: DateTime;
  orgCreationDate?: DateTime;
  loggedIn?: boolean;
}>();

const lastLoginText = computed(() => {
  if (!props.lastLoginDevice) {
    return I18n.translate('HomePage.organizationList.noLastLogin');
  }
  return props.lastLoginDevice.setLocale(I18n.getLocale()).toLocaleString(DateTime.DATE_FULL);
});

onMounted(async () => {
  isTrialOrg.value = await isTrialOrganizationDevice(props.device);
  if (isTrialOrg.value && props.orgCreationDate) {
    expirationDuration.value = getDurationBeforeExpiration(props.orgCreationDate);
  }
});
</script>

<style lang="scss" scoped>
.organization-item {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-3xl');
  padding: ms.spacing('padding-3xl') ms.spacing('padding-4xl') ms.spacing('padding-3xl') ms.spacing('padding-3xl');
  background: ms.color('surface-base-default');
  cursor: pointer;
  user-select: none;
  transition: background 150ms linear;

  &:not(:last-child) {
    border-bottom: ms.border('thin') solid ms.color('border-elevated-default');
  }

  &:hover {
    background: ms.color('surface-brand-default-subtle');

    .organization-item-arrow {
      transform: translateX(0.125rem);
    }
  }

  &-content {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-2xl');
    flex: 1;
    min-width: 0;
  }

  &-avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 2.5rem;
    height: 2.5rem;
    border-radius: ms.radius('2xl');
    background: linear-gradient(113deg, ms.color('surface-gradient-from') 1.49%, ms.color('surface-gradient-to') 100%);

    &__initials {
      @include ms.font('label-md-medium');
      color: ms.color('text-on-color-label');
      text-transform: uppercase;
    }

    &__logo {
      width: 1.5rem;
      flex-shrink: 0;
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');
    flex: 1;
    min-width: 0;

    &-header {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-lg');
      min-width: 0;
    }

    &__name {
      @include ms.font('heading-h5');
      color: ms.color('text-brand-default-hover');
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }

    &__email {
      @include ms.font('body-md-medium');
      color: ms.color('text-base-description');
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }
  }

  &-last-login {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-xs');
    flex-shrink: 0;
    white-space: nowrap;
    min-width: 10rem;

    @include ms.responsive-breakpoint('xs') {
      display: none;
    }

    &__label {
      @include ms.font('body-sm-regular');
      color: ms.color('text-base-description');
    }

    &__date {
      @include ms.font('body-sm-medium');
      color: ms.color('text-base-body');
    }
  }

  &-badge-connected {
    @include ms.font('label-sm-medium');
    display: flex;
    align-items: center;
    flex-shrink: 0;
    gap: ms.spacing('gap-sm');
    padding: 0.1875rem ms.spacing('padding-lg');
    color: ms.color('text-success-default');
    background: ms.color('surface-success-default-subtle-hover');
    border: ms.border('thin') solid ms.color('border-success-default');
    border-radius: ms.radius('2xl');

    ion-icon {
      color: ms.color('icon-success-default');
      font-size: 0.8125rem;
    }
  }

  &-expiration {
    @include ms.font('label-sm-medium');
    flex-shrink: 0;
    padding: 0.1875rem ms.spacing('padding-lg');
    border-radius: ms.radius('2xl');
    background: ms.color('surface-neutral-default');
    color: ms.color('text-on-color-heading');
  }

  &-arrow {
    flex-shrink: 0;
    padding: ms.spacing('padding-sm');
    font-size: 0.875rem;
    color: ms.color('icon-neutral-default');
    transition: transform 150ms linear;
  }
}
</style>
