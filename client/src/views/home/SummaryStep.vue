<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="summary-list">
    <!-- organization name -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $msTranslate('CreateOrganization.overview.organization') }}
        </ion-label>
        <ion-text class="summary-item__text body">
          {{ organization }}
        </ion-text>
        <ion-button
          v-show="!bootstrapOnly"
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.Organization)"
        >
          {{ $msTranslate('CreateOrganization.button.modify') }}
        </ion-button>
      </div>
    </ion-item>

    <!-- fullname -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $msTranslate('CreateOrganization.overview.fullname') }}
        </ion-label>
        <ion-text class="summary-item__text body">
          {{ fullname }}
        </ion-text>
        <ion-button
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.UserInfo)"
        >
          {{ $msTranslate('CreateOrganization.button.modify') }}
        </ion-button>
      </div>
    </ion-item>

    <!-- Email -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $msTranslate('CreateOrganization.overview.email') }}
        </ion-label>
        <ion-text class="summary-item__text body">
          {{ email }}
        </ion-text>
        <ion-button
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.UserInfo)"
        >
          {{ $msTranslate('CreateOrganization.button.modify') }}
        </ion-button>
      </div>
    </ion-item>

    <!-- serverMode -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $msTranslate('CreateOrganization.overview.server') }}
        </ion-label>
        <ion-text class="summary-item__text body">
          {{ serverMode === ServerMode.SaaS ? $msTranslate('CreateOrganization.saas') : serverAddr }}
        </ion-text>
        <ion-button
          v-show="!bootstrapOnly"
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.ServerMode)"
        >
          {{ $msTranslate('CreateOrganization.button.modify') }}
        </ion-button>
      </div>
    </ion-item>

    <!-- authentication mode -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $msTranslate('CreateOrganization.overview.authentication') }}
        </ion-label>
        <ion-text
          class="summary-item__text body"
          v-if="authentication === DevicePrimaryProtectionStrategyTag.Keyring"
        >
          {{ $msTranslate('CreateOrganization.keyringChoice') }}
        </ion-text>
        <ion-text
          class="summary-item__text body"
          v-if="authentication === DevicePrimaryProtectionStrategyTag.Password"
        >
          {{ $msTranslate('CreateOrganization.passwordChoice') }}
        </ion-text>
        <ion-text
          class="summary-item__text body"
          v-if="authentication === DevicePrimaryProtectionStrategyTag.OpenBao"
        >
          {{ $msTranslate('CreateOrganization.openBaoChoice') }}
        </ion-text>

        <ion-button
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.AuthenticationMode)"
        >
          {{ $msTranslate('CreateOrganization.button.modify') }}
        </ion-button>
      </div>
    </ion-item>
  </ion-list>
</template>

<script lang="ts">
export enum OrgInfo {
  Organization = 'organization',
  UserInfo = 'userInfo',
  ServerMode = 'serverMode',
  AuthenticationMode = 'authenticationMode',
}
</script>

<script setup lang="ts">
import { ServerMode } from '@/components/organizations/ChooseServer.vue';
import { DevicePrimaryProtectionStrategyTag } from '@/parsec';
import { IonButton, IonItem, IonLabel, IonList, IonText } from '@ionic/vue';

defineEmits<{
  (e: 'update-request', info: OrgInfo): void;
}>();

defineExpose({
  OrgInfo,
});

defineProps<{
  organization: string;
  fullname: string;
  email: string;
  serverMode: ServerMode;
  serverAddr: string;
  authentication: DevicePrimaryProtectionStrategyTag;
  bootstrapOnly?: boolean;
}>();
</script>

<style scoped lang="scss">
.summary-list {
  padding: 0;
  display: flex;
  flex-direction: column;
}

.summary-item {
  padding: 0.75rem 0;
  display: flex;
  align-items: center;
  flex: 1;
  position: relative;
  gap: 1rem;
  // multiple lines for cross-browser compatibility
  width: 100%;
  width: -webkit-fill-available;
  width: -moz-available;
  width: stretch;

  &::after {
    content: '';
    position: absolute;
    width: 100%;
    height: 1px;
    bottom: 0;
    left: 7.5rem;
    background: var(--parsec-color-light-secondary-disabled);
    z-index: 2;
  }

  &__label {
    min-width: 8rem;
    color: var(--parsec-color-light-secondary-grey);
  }

  &__text {
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--parsec-color-light-secondary-text);
  }
  &__button {
    margin-left: auto;

    &::part(native) {
      padding: 0.5rem;
    }
  }
}
</style>
