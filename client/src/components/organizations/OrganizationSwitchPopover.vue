<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="popover-switch">
    <div class="current-organization">
      <div class="current-organization-content">
        <ion-avatar class="organization-avatar">
          <span v-if="currentOrg && !currentOrg.trial">{{ currentOrg.id.substring(0, 2) }}</span>
          <!-- prettier-ignore -->
          <ms-image
            v-else
            :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
            class="organization-avatar-logo"
          />
        </ion-avatar>
        <ion-text class="organization-name title-h4">
          {{ currentOrg?.id }}
        </ion-text>
      </div>
      <div
        class="section-buttons custom-button custom-button-fill"
        @click="goToLogin"
      >
        <ion-icon
          class="section-buttons-icon"
          :icon="repeat"
        />
        <ion-text
          class="button-medium"
          button
        >
          {{ $msTranslate('OrganizationSwitch.goToLogin') }}
        </ion-text>
      </div>
    </div>
    <div
      class="connected-organization"
      v-if="filteredOrgs.length > 0"
    >
      <ion-title class="connected-organization-title body">{{ $msTranslate('OrganizationSwitch.loggedInOrgs') }}</ion-title>
      <ion-list class="organization-list">
        <ion-item
          class="organization-list__item body"
          button
          lines="none"
          v-for="org in filteredOrgs"
          :key="org.handle"
          @click="onOrganizationClick(org)"
        >
          <div class="organization">
            <ion-avatar
              class="organization-avatar"
              :class="org.isOnline ? 'online' : 'offline'"
            >
              <!-- prettier-ignore -->
              <ms-image
                v-if="org.trial"
                :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
                class="organization-avatar-logo"
              />
              <span v-else>{{ org.id.substring(0, 2) }}</span>
            </ion-avatar>
            <div class="organization-text-content">
              <ion-label class="organization-text-content__name">
                <span class="body">
                  {{ org.id }}
                </span>
              </ion-label>
              <ion-label class="organization-text-content__email body-sm">
                <span v-if="org.trial && org.device">
                  {{ $msTranslate(formatExpirationTime(getDurationBeforeExpiration(org.device.createdOn))) }}
                </span>
                <span v-else>
                  {{ org.userLabel }}
                </span>
              </ion-label>
            </div>
          </div>
        </ion-item>
      </ion-list>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatExpirationTime, getDurationBeforeExpiration, isTrialOrganizationDevice } from '@/common/organization';
import { AvailableDevice, ConnectionHandle, OrganizationID, getLoggedInDevices } from '@/parsec';
import { getConnectionHandle } from '@/router';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonAvatar, IonIcon, IonItem, IonLabel, IonList, IonText, IonTitle, popoverController } from '@ionic/vue';
import { repeat } from 'ionicons/icons';
import { LogoIconGradient, MsImage, MsModalResult } from 'megashark-lib';
import { Ref, onMounted, ref } from 'vue';

interface ConnectedOrganization {
  id: OrganizationID;
  userLabel: string;
  userEmail: string;
  active: boolean;
  handle: ConnectionHandle;
  device: AvailableDevice;
  trial: boolean;
  isOrganizationExpired: boolean;
  isOnline: boolean;
}

const connectedOrgs: Ref<Array<ConnectedOrganization>> = ref([]);
const currentOrg: Ref<ConnectedOrganization | undefined> = ref(undefined);
const filteredOrgs: Ref<Array<ConnectedOrganization>> = ref([]);

onMounted(async () => {
  const result = await getLoggedInDevices();
  connectedOrgs.value = result.map((info) => {
    return {
      id: info.device.organizationId,
      active: getConnectionHandle() === info.handle,
      handle: info.handle,
      userLabel: info.device.humanHandle.label,
      userEmail: info.device.humanHandle.email,
      device: info.device,
      trial: isTrialOrganizationDevice(info.device),
      isOrganizationExpired: info.isOrganizationExpired,
      isOnline: info.isOnline,
    };
  });
  currentOrg.value = connectedOrgs.value.find((org) => org.active);
  filteredOrgs.value = connectedOrgs.value.filter(
    (org) => org.id !== currentOrg.value?.id || org.userEmail !== currentOrg.value?.userEmail,
  );
});

async function onOrganizationClick(org: ConnectedOrganization): Promise<void> {
  if (org.handle !== getConnectionHandle()) {
    await popoverController.dismiss({ handle: org.handle }, MsModalResult.Confirm);
  }
}

async function goToLogin(): Promise<void> {
  await popoverController.dismiss({ handle: null }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.popover-switch {
  display: flex;
  flex-direction: column;
}

.current-organization,
.connected-organization {
  display: flex;
  flex-direction: column;
  padding: 1rem;
  border-bottom: 1px solid var(--parsec-color-light-secondary-premiere);
  gap: 0.75rem;

  .organization-avatar {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: var(--parsec-radius-12);
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--parsec-color-light-primary-600);
    text-align: center;
    flex-shrink: 0;

    &-logo {
      width: 2.5rem;
      padding: 0.375rem;
    }
  }
}

.current-organization {
  background: var(--parsec-color-light-secondary-background);

  &-content {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .organization-avatar {
    background-color: var(--parsec-color-light-secondary-medium);
  }

  .organization-name {
    color: var(--parsec-color-light-secondary-text);
  }
}

.connected-organization {
  gap: 0.5rem;

  &-title {
    color: var(--parsec-color-light-secondary-grey);
    padding: 0;
  }

  .organization-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0;

    &__item {
      --background: none;
      --background-hover: var(--parsec-color-light-primary-50);
      --background-hover-opacity: 1;
      border-radius: var(--parsec-radius-6);
      --inner-padding-end: 0;
      --padding-start: 0;
      position: relative;
      z-index: 2;
      pointer-events: auto;

      &::part(native) {
        padding: 0;
      }

      &:hover {
        .organization-avatar {
          background-color: var(--parsec-color-light-secondary-white);
        }
      }
    }
  }

  .organization {
    padding: 0.5rem;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;

    &-avatar {
      background-color: var(--parsec-color-light-secondary-premiere);
      position: relative;
      z-index: 1;

      &:not(.ellipsis)::after {
        content: '';
        position: absolute;
        bottom: -3px;
        right: -5px;
        height: 0.625rem;
        width: 0.625rem;
        border-radius: var(--parsec-radius-circle);
        border: 3px solid var(--parsec-color-light-secondary-white);
      }

      &.online:not(.ellipsis)::after {
        background-color: var(--parsec-color-light-success-500);
      }

      &.offline:not(.ellipsis)::after {
        background-color: var(--parsec-color-light-secondary-light);
      }
    }

    &-text-content {
      margin: 0;
      display: flex;
      flex-direction: column;
      overflow: hidden;

      &::part(native) {
        padding: 0;
      }

      &__name {
        color: var(--parsec-color-light-secondary-text);

        .body {
          font-size: 0.875rem;
          display: block;
          text-wrap: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
      }

      &__email {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}

.section-buttons {
  margin: 1rem 0 0.5rem 0;

  &:hover {
    background-color: var(--parsec-color-light-secondary-premiere);
  }

  &-icon {
    margin-bottom: 0.05rem;
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.item-disabled {
  opacity: 1;
  pointer-events: none;
}
</style>
