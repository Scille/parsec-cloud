<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="summary-list">
    <!-- organization name -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $t('CreateOrganization.overview.organization') }}
        </ion-label>
        <ion-text class="summary-item__text body">
          {{ organization }}
        </ion-text>
        <ion-button
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.Organization)"
        >
          {{ $t('CreateOrganization.button.modify') }}
        </ion-button>
      </div>
    </ion-item>

    <!-- fullname -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $t('CreateOrganization.overview.fullname') }}
        </ion-label>
        <ion-text class="summary-item__text body">
          {{ fullname }}
        </ion-text>
        <ion-button
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.UserInfo)"
        >
          {{ $t('CreateOrganization.button.modify') }}
        </ion-button>
      </div>
    </ion-item>

    <!-- Email -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $t('CreateOrganization.overview.email') }}
        </ion-label>
        <ion-text class="summary-item__text body">
          {{ email }}
        </ion-text>
        <ion-button
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.UserInfo)"
        >
          {{ $t('CreateOrganization.button.modify') }}
        </ion-button>
      </div>
    </ion-item>

    <!-- serverMode -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $t('CreateOrganization.overview.server') }}
        </ion-label>
        <ion-text class="summary-item__text body">
          {{ serverMode === ServerMode.SaaS ? $t('CreateOrganization.saas') : serverAddr }}
        </ion-text>
        <ion-button
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.ServerMode)"
        >
          {{ $t('CreateOrganization.button.modify') }}
        </ion-button>
      </div>
    </ion-item>

    <!-- authentication mode -->
    <ion-item class="ion-no-padding">
      <div class="summary-item">
        <ion-label class="summary-item__label subtitles-sm">
          {{ $t('CreateOrganization.overview.authentication') }}
        </ion-label>
        <ion-text class="summary-item__text body">
          {{
            authentication === DeviceSaveStrategyTag.Keyring
              ? $t('CreateOrganization.keyringChoice')
              : $t('CreateOrganization.passwordChoice')
          }}
        </ion-text>
        <ion-button
          fill="clear"
          class="summary-item__button"
          @click="$emit('update-request', OrgInfo.AuthenticationMode)"
        >
          {{ $t('CreateOrganization.button.modify') }}
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
import { DeviceSaveStrategyTag } from '@/parsec';
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
  authentication: DeviceSaveStrategyTag;
}>();
</script>

<style scoped lang="scss">
.summary-list {
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.summary-item {
  display: flex;
  align-items: center;
  flex: 1;
  position: relative;
  gap: 1rem;
  width: -webkit-fill-available;

  &::after {
    content: '';
    position: absolute;
    width: 100%;
    height: 1px;
    bottom: -0.5rem;
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
  }
}
</style>
