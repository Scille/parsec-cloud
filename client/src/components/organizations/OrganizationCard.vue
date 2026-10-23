<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
        class="organization-card-header__initials"
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
        <ion-text class="organization-name">{{ device.organizationId }}</ion-text>
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
          <ion-text class="login-text">
            {{ getLastLoginText() }}
          </ion-text>
        </div>
        <div class="login-content">
          <ion-icon
            :icon="person"
            class="login-icon"
          />
          <ion-text class="login-text login-name">{{ device.humanHandle.label }}</ion-text>
        </div>
      </div>
    </ion-card-content>
    <!-- trial expiration badge -->
    <ion-text
      v-if="expirationDuration"
      class="organization-card-expiration"
    >
      {{ $msTranslate(formatExpirationTime(expirationDuration)) }}
    </ion-text>
    <ion-text
      class="organization-card-badge-connected"
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
    <ion-icon
      v-if="!orgNameOnly && !(isTrialOrg && isSmallDisplay) && !loggedIn"
      class="organization-card-icon"
      :icon="arrowForwardCircle"
    />
  </div>
</template>

<script setup lang="ts">
import { formatExpirationTime, getDurationBeforeExpiration, isTrialOrganizationDevice } from '@/common/organization';
import { AvailableDevice } from '@/parsec';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonCardContent, IonIcon, IonText } from '@ionic/vue';
import { arrowForwardCircle, checkmarkCircle, person, time } from 'ionicons/icons';
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
  isTrialOrg.value = await isTrialOrganizationDevice(props.device);
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
  background: ms.color('surface-elevated-subtle');
  border: ms.border('thin') solid ms.color('border-neutral-base-on-color');
  border-radius: ms.radius('2xl');
  width: 100%;
  min-height: 5rem;
  padding-right: ms.spacing('padding-8xl');
  display: flex;
  gap: ms.spacing('gap-2xl');
  user-select: none;
  box-shadow: ms.shadow('shadow-xs');
  overflow: hidden;
  cursor: pointer;
  transition:
    box-shadow 150ms linear,
    border 150ms linear;
  position: relative;
  flex-shrink: 0;

  &-logged-in {
    padding-right: ms.spacing('padding-4xl');
  }

  @include ms.responsive-breakpoint('sm') {
    padding-right: ms.spacing('padding-3xl');
  }

  // when using the card as a header only
  &.header-only {
    padding: ms.spacing('padding-none') ms.spacing('padding-lg') ms.spacing('padding-none') ms.spacing('padding-none');
    box-shadow: none;
    border: none;
    cursor: inherit;
  }

  &:hover:not(.header-only) {
    background: ms.color('surface-elevated-subtle-hover');
    box-shadow: var(--parsec-shadow-light);
    border-color: ms.color('border-brand-default');

    .organization-card-icon {
      right: 1rem;
    }
  }

  &-header {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-shrink: 0;
    gap: ms.spacing('gap-lg');
    width: 4rem;
    z-index: 1;
    background: linear-gradient(113deg, ms.color('surface-gradient-from') -1.49%, ms.color('surface-gradient-to') 100%);
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
      @include ms.font('body-lg-medium');
      color: ms.color('text-on-color-label');
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
    padding: ms.spacing('padding-3xl') ms.spacing('padding-none');
    gap: ms.spacing('gap-sm');
    width: 100%;
    height: 100%;
    overflow: hidden;
    flex-shrink: 1;

    @include ms.responsive-breakpoint('xs') {
      padding: ms.spacing('padding-2xl') ms.spacing('padding-none');
      overflow: hidden;
      gap: ms.spacing('gap-lg');
    }

    &-text {
      display: flex;
      gap: ms.spacing('gap-md');
      overflow-x: hidden;

      .organization-name {
        @include ms.font('heading-h5');
        color: ms.color('text-base-body');
        overflow: hidden;
        white-space: nowrap;
        text-overflow: ellipsis;
      }
    }

    &-login {
      display: flex;
      gap: ms.spacing('gap-3xl');

      @include ms.responsive-breakpoint('xs') {
        flex-direction: column;
        gap: ms.spacing('gap-sm');
      }

      .login-content {
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-lg');
        padding: ms.spacing('padding-none');
        overflow: hidden;

        .login-icon {
          color: ms.color('icon-disabled-default');
          font-size: 1rem;
          flex-shrink: 0;
        }

        .login-text {
          @include ms.font('body-md-regular');
          color: ms.color('text-base-description');
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
    color: ms.color('icon-brand-default');
    font-size: 1.5rem;

    @include ms.responsive-breakpoint('xs') {
      position: relative;
      right: 0.25rem;
    }
  }

  &-badge-connected {
    @include ms.font('label-sm-medium');
    display: flex;
    margin: auto;
    align-items: center;
    padding: 0.1875rem ms.spacing('padding-lg');
    gap: ms.spacing('gap-sm');
    color: ms.color('text-success-default');
    background: ms.color('surface-success-default-subtle-hover');
    border: ms.border('thin') solid ms.color('border-success-default');
    border-radius: ms.radius('2xl');
    height: fit-content;
    flex-shrink: 0;

    ion-icon {
      color: ms.color('icon-success-default');
      font-size: 0.8125rem;
    }

    @include ms.responsive-breakpoint('sm') {
      border: none;
      background: transparent;

      ion-icon {
        font-size: 1.375rem;
        padding: ms.spacing('padding-none');
      }
    }
  }

  &-expiration {
    @include ms.font('label-sm-medium');
    border-radius: ms.radius('2xl');
    padding: 0.1875rem ms.spacing('padding-lg');
    width: fit-content;
    align-self: center;
    height: fit-content;
    flex-shrink: 0;
    background: ms.color('surface-neutral-default');
    color: ms.color('text-on-color-heading');

    @include ms.responsive-breakpoint('xs') {
      margin-left: auto;
      flex-shrink: 0;
    }
  }
}
</style>
