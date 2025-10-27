<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="popover-switch">
    <ion-title class="popover-switch-title body">{{ $msTranslate('clientArea.sidebar.switchOrganization.title') }}</ion-title>
    <ion-list class="organization-list">
      <ion-item
        class="organization-list__item body"
        button
        lines="none"
        v-for="org in organizations"
        :key="org.bmsId"
        @click="onOrganizationSelected(org)"
        :class="{ 'current-organization': org.bmsId === currentOrganization.bmsId }"
      >
        <div class="organization">
          <ion-avatar
            class="organization-avatar"
            v-show="!isDefaultOrganization(org)"
          >
            <span>{{ org.name.substring(0, 2) }}</span>
          </ion-avatar>
          <ion-label class="organization-name">
            <span
              class="subtitles-normal"
              v-if="!isDefaultOrganization(org)"
            >
              {{ org.name }}
            </span>
            <span
              class="subtitles-normal all-organization"
              v-else
            >
              {{ $msTranslate('clientArea.sidebar.allOrganization') }}
            </span>
          </ion-label>
          <ion-icon
            class="organization-icon"
            :icon="arrowForward"
          />
          <ion-icon
            class="organization-icon-current"
            v-if="org.bmsId === currentOrganization.bmsId"
            :icon="checkmark"
          />
        </div>
      </ion-item>
    </ion-list>
    <div
      class="organization-divider"
      v-show="false"
    />
    <div
      class="create-organization"
      v-show="false"
    >
      <ion-item
        class="organization-list__item body"
        button
        lines="none"
        @click="openCreateOrganizationModal"
      >
        <div class="organization">
          <div class="organization-icon-container">
            <ion-icon
              :icon="addCircle"
              class="organization-icon-add"
            />
          </div>
          <ion-label class="organization-name">
            <span class="subtitles-normal">
              {{ $msTranslate('clientArea.sidebar.switchOrganization.create') }}
            </span>
          </ion-label>
        </div>
      </ion-item>
    </div>
  </div>
</template>

<script setup lang="ts">
import { BmsOrganization } from '@/services/bms';
import { DefaultBmsOrganization, isDefaultOrganization } from '@/views/client-area/types';
import { IonAvatar, IonIcon, IonItem, IonLabel, IonList, IonTitle, popoverController } from '@ionic/vue';
import { addCircle, arrowForward, checkmark } from 'ionicons/icons';
import { MsModalResult } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  currentOrganization: BmsOrganization;
  organizations: Array<BmsOrganization>;
}>();

const organizations = ref<Array<BmsOrganization>>(Array.from(props.organizations));

onMounted(async () => {
  if (props.organizations.length > 1) {
    organizations.value.push(DefaultBmsOrganization);
  }
});

async function openCreateOrganizationModal(): Promise<void> {
  await popoverController.dismiss(undefined, MsModalResult.Cancel);
}

async function onOrganizationSelected(org: BmsOrganization): Promise<void> {
  if (org && org.bmsId !== props.currentOrganization.bmsId) {
    await popoverController.dismiss({ organization: org }, MsModalResult.Confirm);
  }
}
</script>

<style lang="scss" scoped>
.popover-switch {
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  overflow-y: auto;
  background-color: var(--parsec-color-light-secondary-inversed-contrast);
  max-height: 19rem;

  &-title {
    color: var(--parsec-color-light-secondary-grey);
    padding: 0;
  }
}

.organization-list {
  display: flex;
  flex-direction: column;
  background: transparent;
  gap: 0.25rem;
  padding: 0;

  &__item {
    --background: transparent;
    --background-hover: var(--parsec-color-light-secondary-premiere);
    --background-hover-opacity: 1;
    border-radius: var(--parsec-radius-8);
    position: relative;
    z-index: 2;
    pointer-events: auto;
    overflow: hidden;

    &::part(native) {
      padding: 0;
      --inner-padding-end: 0;
    }

    &:hover {
      .organization-icon {
        right: 0.5rem;
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
  position: relative;

  &:has(.all-organization) {
    min-height: 3rem;
  }

  &-avatar,
  .organization-icon-container {
    background-color: var(--parsec-color-light-primary-100);
    color: var(--parsec-color-light-primary-700);
    width: 2rem;
    height: 2rem;
    margin: 0;
    border-radius: var(--parsec-radius-8);
    text-align: center;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    position: relative;
    z-index: 1;
  }

  &-name {
    flex: 1;
    color: var(--parsec-color-light-secondary-hard-grey);
    text-wrap: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  &-icon {
    color: var(--parsec-color-light-secondary-text);
    font-size: 1.25rem;
    position: absolute;
    right: -1.2rem;
    transition: right 0.2s ease-in-out;
  }

  &-icon-current {
    color: var(--parsec-color-light-secondary-text);
    font-size: 1rem;
  }

  &-divider {
    height: 1px;
    background-color: var(--parsec-color-light-secondary-disabled);
  }

  .organization-icon-container {
    background: var(--parsec-color-light-secondary-premiere);
  }

  &-icon-add {
    color: var(--parsec-color-light-secondary-hard-grey);
    font-size: 1.125rem;
  }
}

.current-organization {
  pointer-events: none;

  .organization-avatar {
    background-color: var(--parsec-color-light-primary-700);
    color: var(--parsec-color-light-secondary-white);
  }

  .organization-name {
    color: var(--parsec-color-light-secondary-text);
  }
}
</style>
