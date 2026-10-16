<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="popover-switch">
    <ion-title class="popover-switch-title">{{ $msTranslate('clientArea.sidebar.switchOrganization.title') }}</ion-title>
    <ion-list class="organization-list">
      <ion-item
        class="organization-list__item"
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
            <span v-if="!isDefaultOrganization(org)">
              {{ org.name }}
            </span>
            <span
              class="all-organization"
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
        class="organization-list__item"
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
            <span>
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
  padding: ms.spacing('padding-3xl');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');
  overflow-y: auto;
  background-color: ms.color('surface-base-default');
  max-height: 19rem;

  &-title {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-description');
    padding: ms.spacing('padding-none');
  }
}

.organization-list {
  display: flex;
  flex-direction: column;
  background: transparent;
  gap: ms.spacing('gap-sm');
  padding: ms.spacing('padding-none');

  &__item {
    @include ms.font('body-md-regular');
    --background: transparent;
    --background-hover: #{ms.color('surface-base-page-secondary')};
    --background-hover-opacity: #{ms.opacity('10')};
    border-radius: ms.radius('lg');
    position: relative;
    z-index: 2;
    pointer-events: auto;
    overflow: hidden;

    &::part(native) {
      padding: ms.spacing('padding-none');
      --inner-padding-end: #{ms.spacing('padding-none')};
    }

    &:hover {
      .organization-icon {
        right: 0.5rem;
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
  position: relative;

  &:has(.all-organization) {
    min-height: 3rem;
  }

  &-avatar,
  .organization-icon-container {
    background-color: ms.color('surface-brand-default-subtle');
    color: ms.color('text-brand-default');
    width: 2rem;
    height: 2rem;
    margin: 0;
    border-radius: ms.radius('lg');
    text-align: center;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    position: relative;
    z-index: 1;
  }

  &-name {
    @include ms.font('body-lg-medium');
    flex: 1;
    color: ms.color('text-base-description');
    text-wrap: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  &-icon {
    color: ms.color('icon-neutral-default');
    font-size: 1.25rem;
    position: absolute;
    right: -1.2rem;
    transition: right 0.2s ease-in-out;
  }

  &-icon-current {
    color: ms.color('icon-neutral-default');
    font-size: 1rem;
  }

  &-divider {
    height: 1px;
    background-color: ms.color('border-base-default');
  }

  .organization-icon-container {
    background: ms.color('surface-base-page-secondary');
  }

  &-icon-add {
    color: ms.color('icon-neutral-default');
    font-size: 1.125rem;
  }
}

.current-organization {
  pointer-events: none;

  .organization-avatar {
    background-color: ms.color('surface-brand-default-subtle');
    color: ms.color('text-on-color-heading');
  }

  .organization-name {
    color: ms.color('text-base-body');
  }
}
</style>
