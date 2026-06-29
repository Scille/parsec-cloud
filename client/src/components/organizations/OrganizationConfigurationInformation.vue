<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-info">
    <div class="card-header">
      <ion-title class="card-header__title title-h3">
        {{ $msTranslate('OrganizationPage.configuration.title') }}
      </ion-title>
    </div>

    <div class="card-content">
      <div class="info-list">
        <!-- Workspace deletion delay -->
        <div class="info-list-item">
          <ion-label class="info-list-item__title body">
            {{ $msTranslate('OrganizationPage.configuration.workspaceDeletionDelay') }}
          </ion-label>
          <div class="info-list-item__value cell-title info">
            {{
              $msTranslate(
                orgInfo.workspaceDeletionDelay === 0
                  ? 'WorkspacesPage.trashWorkspace.deletionDelay.immediate'
                  : formatWorkspaceDeletionDelay(orgInfo.workspaceDeletionDelay),
              )
            }}
          </div>
        </div>
        <!-- Outsider profile -->
        <div class="info-list-item">
          <ion-label class="info-list-item__title body">
            {{ $msTranslate('OrganizationPage.configuration.outsidersAllowed') }}
          </ion-label>
          <div
            class="info-list-item__value cell-title"
            :class="orgInfo.outsidersAllowed ? 'success' : 'warning'"
          >
            {{
              orgInfo.outsidersAllowed
                ? $msTranslate('OrganizationPage.configuration.allowed')
                : $msTranslate('OrganizationPage.configuration.forbidden')
            }}
          </div>
        </div>
        <!-- User limit -->
        <div class="info-list-item">
          <ion-text class="info-list-item__title body">
            {{ $msTranslate('OrganizationPage.configuration.userLimit') }}
          </ion-text>
          <div
            class="info-list-item__value cell-title"
            :class="orgInfo.userLimit !== undefined ? 'warning' : 'success'"
          >
            {{ orgInfo.userLimit !== undefined ? orgInfo.userLimit : $msTranslate('OrganizationPage.configuration.unlimited') }}
          </div>
        </div>
        <!-- Server addr -->
        <div class="info-list-item server-address">
          <ion-text class="info-list-item__title server-address__title body">
            {{ $msTranslate('OrganizationPage.configuration.serverAddr') }}
          </ion-text>
          <div class="server-address-value">
            <ion-text class="server-address-value__text body">
              {{ orgInfo.organizationAddr }}
            </ion-text>
            <ms-feedback-button
              fill="clear"
              size="small"
              id="copy-link-btn"
              :callback="copyAddress"
              :normal-state="{ icon: copy }"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatWorkspaceDeletionDelay } from '@/components/workspaces/utils';
import { OrganizationInfo } from '@/parsec';
import { IonLabel, IonText, IonTitle } from '@ionic/vue';
import { copy } from 'ionicons/icons';
import { Clipboard, MsFeedbackButton } from 'megashark-lib';

const props = defineProps<{
  orgInfo: OrganizationInfo;
}>();

async function copyAddress(): Promise<boolean | undefined> {
  return await Clipboard.writeText(props.orgInfo.organizationAddr);
}
</script>

<style scoped lang="scss">
.organization-info {
  display: flex;
  flex-direction: column;
  background: var(--parsec-color-light-secondary-white);
  align-items: center;
  gap: 1rem;
  width: 100%;
  max-width: 30rem;
  border-radius: var(--parsec-radius-12);
  height: fit-content;
  box-shadow: var(--parsec-shadow-input);
  padding: 1.5rem;

  .card-header {
    display: flex;
    align-items: center;
    width: 100%;

    &__title {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  .card-content {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 1rem;
  }

  .info-list {
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;

    &-item {
      justify-content: space-between;
      display: flex;
      align-items: center;
      gap: 1rem;

      &__title {
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      &__value.cell-title {
        padding: 0.125rem 0.5rem;
        border-radius: var(--parsec-radius-32);
      }

      .success {
        color: var(--parsec-color-light-success-700);
        background: var(--parsec-color-light-success-50);
      }

      .warning {
        color: var(--parsec-color-light-warning-700);
        background: var(--parsec-color-light-warning-50);
      }

      .info {
        color: var(--parsec-color-light-secondary-text);
        background: var(--parsec-color-light-secondary-premiere);
      }
    }
  }

  .server-address {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    align-items: start;

    &-value {
      color: var(--parsec-color-light-secondary-text);
      background-color: var(--parsec-color-light-secondary-background);
      border-radius: var(--parsec-radius-6);
      padding: 0.5rem 1rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 0.5rem;
      overflow: hidden;
      width: 100%;

      &__text {
        white-space: nowrap;
        overflow-x: auto;
        overflow-y: hidden;
      }

      &__copied {
        color: var(--parsec-color-light-success-700);
      }
    }
  }
}
</style>
