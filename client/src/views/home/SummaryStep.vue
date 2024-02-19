<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="summary-list">
    <!-- organization name -->
    <ion-item class="summary-item">
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
    </ion-item>

    <!-- fullname -->
    <ion-item class="summary-item">
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
    </ion-item>

    <!-- Email -->
    <ion-item class="summary-item">
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
    </ion-item>

    <!-- serverMode -->
    <ion-item class="summary-item">
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
    </ion-item>
  </ion-list>
</template>

<script lang="ts">
export enum OrgInfo {
  Organization = 'organization',
  UserInfo = 'userInfo',
  ServerMode = 'serverMode',
}
</script>

<script setup lang="ts">
import { ServerMode } from '@/components/organizations/ChooseServer.vue';
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
}>();
</script>

<style scoped lang="scss">
.summary-list {
  padding: 0;
}

.summary-item {
  display: flex;
  position: relative;
  flex-direction: column;

  &::part(native) {
    padding: 0;
  }

  & > *:not(:last-child) {
    margin-right: 0.5rem;
  }

  &::after {
    content: '';
    position: relative;
    width: 100%;
    height: 1px;
    margin: 0.5rem 0;
    background: var(--parsec-color-light-secondary-disabled);
  }

  &__label {
    max-width: 10rem;
    color: var(--parsec-color-light-secondary-grey);
  }

  &__text {
    max-width: 20rem;
    color: var(--parsec-color-light-secondary-text);
  }
  &__button {
    margin-left: auto;
  }
}
</style>
