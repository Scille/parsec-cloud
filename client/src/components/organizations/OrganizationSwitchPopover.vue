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
        <ion-text class="organization-name">
          {{ currentOrg?.id }}
        </ion-text>
      </div>
      <ion-button
        class="section-buttons"
        fill="outline"
        @click="goToLogin"
      >
        <ion-icon
          class="section-buttons-icon button-icon-left"
          :icon="repeat"
        />
        {{ $msTranslate('OrganizationSwitch.goToLogin') }}
      </ion-button>
    </div>
    <div
      class="connected-organization"
      v-if="filteredOrgs.length > 0"
    >
      <ion-title class="connected-organization-title">{{ $msTranslate('OrganizationSwitch.loggedInOrgs') }}</ion-title>
      <ion-list class="organization-list">
        <ion-item
          class="organization-list__item"
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
                <span>
                  {{ org.id }}
                </span>
              </ion-label>
              <ion-label class="organization-text-content__email">
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
import { IonAvatar, IonButton, IonIcon, IonItem, IonLabel, IonList, IonText, IonTitle, popoverController } from '@ionic/vue';
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
  connectedOrgs.value = await Promise.all(
    result.map(async (info) => {
      return {
        id: info.device.organizationId,
        active: getConnectionHandle() === info.handle,
        handle: info.handle,
        userLabel: info.device.humanHandle.label,
        userEmail: info.device.humanHandle.email,
        device: info.device,
        trial: await isTrialOrganizationDevice(info.device),
        isOrganizationExpired: info.isOrganizationExpired,
        isOnline: info.isOnline,
      };
    }),
  );
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
  padding: ms.spacing('padding-3xl');
  border-bottom: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  gap: ms.spacing('gap-2xl');

  .organization-avatar {
    width: 2.5rem;
    height: 2.5rem;
    border-radius: ms.radius('2xl');
    display: flex;
    align-items: center;
    justify-content: center;
    color: ms.color('text-brand-default');
    text-align: center;
    flex-shrink: 0;

    &-logo {
      width: 2.5rem;
      padding: ms.spacing('padding-md');
    }
  }
}

.current-organization {
  background: ms.color('surface-base-default-secondary');

  &-content {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-2xl');
  }

  .organization-avatar {
    background-color: ms.color('surface-neutral-default-subtle');
  }

  .organization-name {
    @include ms.font('heading-h5');
    color: ms.color('text-base-body');
  }
}

.connected-organization {
  max-height: 20%;
  max-height: 30vh;
  overflow-y: auto;
  gap: ms.spacing('gap-lg');

  &-title {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-description');
    padding: ms.spacing('padding-none');
  }

  .organization-list {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');
    padding: ms.spacing('padding-none');

    &__item {
      @include ms.font('body-md-regular');
      --background: none;
      --background-hover: #{ms.color('surface-brand-default-subtle-hover')};
      --background-hover-opacity: #{ms.opacity('10')};
      border-radius: ms.radius('md');
      --inner-padding-end: #{ms.spacing('padding-none')};
      --padding-start: #{ms.spacing('padding-none')};
      position: relative;
      z-index: 2;
      pointer-events: auto;

      &::part(native) {
        padding: ms.spacing('padding-none');
      }

      &:hover {
        .organization-avatar {
          background-color: ms.color('surface-base-default');
        }
      }
    }
  }

  .organization {
    padding: ms.spacing('padding-lg');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-2xl');
    width: 100%;

    &-avatar {
      background-color: ms.color('surface-base-page-secondary');
      position: relative;
      z-index: 1;

      &:not(.ellipsis)::after {
        content: '';
        position: absolute;
        bottom: -3px;
        right: -5px;
        height: 0.625rem;
        width: 0.625rem;
        border-radius: ms.radius('full');
        border: 3px solid ms.color('surface-base-default');
      }

      &.online:not(.ellipsis)::after {
        background-color: ms.color('surface-success-default');
      }

      &.offline:not(.ellipsis)::after {
        background-color: ms.color('surface-disabled-default');
      }
    }

    &-text-content {
      margin: 0;
      display: flex;
      flex-direction: column;
      overflow: hidden;

      &::part(native) {
        padding: ms.spacing('padding-none');
      }

      &__name {
        color: ms.color('text-base-body');

        span {
          @include ms.font('body-md-regular');
          display: block;
          text-wrap: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
      }

      &__email {
        @include ms.font('body-sm-regular');
        text-overflow: ellipsis;
        white-space: nowrap;
        overflow: hidden;
        color: ms.color('text-base-description');
      }
    }
  }
}

.section-buttons {
  margin: 1rem 0 0.5rem 0;

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
