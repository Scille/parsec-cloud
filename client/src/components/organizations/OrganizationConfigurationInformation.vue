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
            {{ $msTranslate('OrganizationPage.configuration.serverAddress') }}
          </ion-text>
          <div class="server-address-value">
            <ion-text class="server-address-value__text body">
              {{ orgInfo.organizationAddr }}
            </ion-text>
            <ion-button
              fill="clear"
              size="small"
              id="copy-link-btn"
              @click="copyAddress(orgInfo.organizationAddr)"
              v-if="!addressCopiedToClipboard"
            >
              <ion-icon
                class="icon-copy"
                :icon="copy"
              />
            </ion-button>
            <ion-text
              v-if="addressCopiedToClipboard"
              class="server-address-value__copied body copied"
            >
              {{ $msTranslate('OrganizationPage.configuration.copyPath') }}
            </ion-text>
          </div>
        </div>
        <div class="info-list-item">
          <ion-label class="info-list-item__title body">
            {{ $msTranslate('OrganizationPage.configuration.serverVersion') }}
          </ion-label>
          <ion-text
            class="info-list-item__value title-h5"
            id="server-version"
            v-if="serverConfig && serverConfig.serverVersion"
          >
            {{ serverConfig.serverVersion }}
          </ion-text>
          <div
            v-else
            id="unavailable-server-version"
            class="info-list-item__value cell-title warning"
          >
            {{ $msTranslate('OrganizationPage.size.unavailable') }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatWorkspaceDeletionDelay } from '@/components/workspaces/utils';
import { OrganizationInfo, ServerConfig } from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { IonButton, IonIcon, IonLabel, IonText, IonTitle } from '@ionic/vue';
import { copy } from 'ionicons/icons';
import { Clipboard } from 'megashark-lib';
import { inject, Ref, ref } from 'vue';

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const addressCopiedToClipboard = ref(false);

defineProps<{
  orgInfo: OrganizationInfo;
  serverConfig: ServerConfig | null;
}>();

async function copyAddress(address: string): Promise<void> {
  const result = await Clipboard.writeText(address);
  if (result) {
    addressCopiedToClipboard.value = true;
    setTimeout(() => {
      addressCopiedToClipboard.value = false;
    }, 5000);
  } else {
    informationManager.value.present(
      new Information({
        message: 'OrganizationPage.configuration.copyFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
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

      #server-version {
        color: var(--parsec-color-light-secondary-text);
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
}

.server-address {
  display: flex;
  flex-direction: column;
  gap: 0.5rem !important;
  align-items: start;

  &__title {
    width: 100%;
  }

  &-value {
    color: var(--parsec-color-light-secondary-soft-text);
    background-color: var(--parsec-color-light-secondary-background);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-8);
    padding: 0.5rem 0.5rem 0.5rem 1rem;
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
      scrollbar-gutter: auto;
      scrollbar-width: none;
    }

    &__copied {
      color: var(--parsec-color-light-success-700);
    }
  }

  #copy-link-btn {
    color: var(--parsec-color-light-secondary-text);
    margin: 0;
    height: 2rem;
    width: 2rem;

    &::part(native) {
      padding: 0.375rem;
      border-radius: var(--parsec-radius-6);
    }

    &:hover {
      color: var(--parsec-color-light-primary-600);
    }
  }
}
</style>
