<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="popover-switch">
    <ion-title class="popover-switch-title body">{{ $t('OrganizationSwitch.loggedInOrgs') }}</ion-title>
    <ion-list class="orga-list">
      <ion-item
        class="orga-list__item body"
        button
        lines="none"
        v-for="org in connectedOrgs"
        :key="org.handle"
        :disabled="org.active"
        @click="onOrganizationClick(org)"
      >
        <div class="orga">
          <ion-avatar class="orga-avatar">
            <span>{{ org.id.substring(0, 2) }}</span>
          </ion-avatar>
          <div class="orga-text-content">
            <ion-label class="orga-text-content__name">
              <span class="body">
                {{ org.id }}
              </span>
            </ion-label>
            <ion-label class="orga-text-content__email">
              <span class="body-sm">
                {{ org.userLabel }}
              </span>
            </ion-label>
          </div>

          <ion-text
            class="badge caption-caption"
            v-show="org.active"
            :outline="true"
          >
            {{ $t('OrganizationSwitch.active') }}
          </ion-text>
        </div>
      </ion-item>

      <ion-item
        class="orga-list__item body"
        button
        lines="none"
        @click="onAllOrganizationClick"
      >
        <div class="orga">
          <ion-avatar class="orga-avatar ellipsis">
            <ion-icon :icon="ellipsisHorizontal" />
          </ion-avatar>
          <div class="orga-text-content">
            <ion-label class="orga-text-content__name">
              <span class="body">
                {{ $t('OrganizationSwitch.myOrgs') }}
              </span>
            </ion-label>
          </div>
        </div>
      </ion-item>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { ConnectionHandle, OrganizationID, getLoggedInDevices } from '@/parsec';
import { getConnectionHandle } from '@/router';
import { IonAvatar, IonIcon, IonItem, IonLabel, IonList, IonText, IonTitle, popoverController } from '@ionic/vue';
import { ellipsisHorizontal } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';

interface ConnectedOrganization {
  id: OrganizationID;
  userLabel: string;
  active: boolean;
  handle: ConnectionHandle;
}

const connectedOrgs: Ref<Array<ConnectedOrganization>> = ref([]);

onMounted(async () => {
  const result = await getLoggedInDevices();
  connectedOrgs.value = result.map((info) => {
    return {
      id: info.device.organizationId,
      active: getConnectionHandle() === info.handle,
      handle: info.handle,
      userLabel: info.device.humanHandle.label,
    };
  });
});

async function onOrganizationClick(org: ConnectedOrganization): Promise<void> {
  if (org.handle !== getConnectionHandle()) {
    await popoverController.dismiss({
      handle: org.handle,
    });
  }
}

async function onAllOrganizationClick(): Promise<void> {
  await popoverController.dismiss({ handle: null });
}
</script>

<style lang="scss" scoped>
.popover-switch {
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;

  &-title {
    color: var(--parsec-color-light-secondary-grey);
    padding: 0;
  }
}

.orga-list {
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
      .orga-avatar {
        background-color: var(--parsec-color-light-secondary-white);
      }
    }
  }
}

.orga {
  padding: 0.5rem;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;

  &-avatar {
    background-color: var(--parsec-color-light-secondary-premiere);
    color: var(--parsec-color-light-primary-600);
    width: 2.5rem;
    height: 2.5rem;
    margin: 0;
    border-radius: var(--parsec-radius-circle);
    text-align: center;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    position: relative;
    z-index: 1;

    &:not(.ellipsis)::after {
      content: '';
      position: absolute;
      bottom: 0;
      right: -4px;
      height: 0.625rem;
      width: 0.625rem;
      border-radius: 50%;
      border: var(--parsec-color-light-secondary-white) solid 0.25rem;
      background-color: var(--parsec-color-light-success-500);
    }
  }

  &-text-content {
    margin: 0;
    display: flex;
    flex-direction: column;

    &::part(native) {
      padding: 0;
    }

    &__name {
      color: var(--parsec-color-light-secondary-text);

      .body {
        font-size: 0.875rem;
      }
    }

    &__email {
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.item-disabled {
  opacity: 1;
  pointer-events: none;

  .badge {
    color: var(--parsec-color-light-success-500);
    background: var(--parsec-color-light-success-100);
    margin-left: auto;
    border-radius: var(--parsec-radius-12);
    padding: 0.25rem 0.5rem;
  }
}
</style>
