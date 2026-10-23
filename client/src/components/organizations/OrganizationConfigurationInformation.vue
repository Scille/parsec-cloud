<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="organization-info">
    <div class="card-header">
      <ion-title class="card-header__title">
        {{ $msTranslate('OrganizationPage.configuration.title') }}
      </ion-title>
    </div>

    <div class="card-content">
      <div class="info-list">
        <!-- Workspace deletion delay -->
        <div class="info-list-item">
          <ion-label class="info-list-item__title">
            {{ $msTranslate('OrganizationPage.configuration.workspaceDeletionDelay') }}
          </ion-label>
          <div class="info-list-item__value info-list-item__badge info">
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
          <ion-label class="info-list-item__title">
            {{ $msTranslate('OrganizationPage.configuration.outsidersAllowed') }}
          </ion-label>
          <div
            class="info-list-item__value info-list-item__badge"
            :class="orgInfo.clientInfo.serverOrganizationConfig.userProfileOutsiderAllowed ? 'success' : 'warning'"
          >
            {{
              orgInfo.clientInfo.serverOrganizationConfig.userProfileOutsiderAllowed
                ? $msTranslate('OrganizationPage.configuration.allowed')
                : $msTranslate('OrganizationPage.configuration.forbidden')
            }}
          </div>
        </div>
        <!-- User limit -->
        <div class="info-list-item">
          <ion-text class="info-list-item__title">
            {{ $msTranslate('OrganizationPage.configuration.userLimit') }}
          </ion-text>
          <div
            class="info-list-item__value info-list-item__badge"
            :class="orgInfo.userLimit !== undefined ? 'warning' : 'success'"
          >
            {{ orgInfo.userLimit !== undefined ? orgInfo.userLimit : $msTranslate('OrganizationPage.configuration.unlimited') }}
          </div>
        </div>
        <!-- Server addr -->
        <div class="info-list-item server-address">
          <ion-text class="info-list-item__title server-address__title">
            {{ $msTranslate('OrganizationPage.configuration.serverAddress') }}
          </ion-text>
          <div class="server-address-value">
            <ion-text class="server-address-value__text">
              {{ orgInfo.clientInfo.organizationAddr }}
            </ion-text>
            <ion-button
              fill="clear"
              size="small"
              id="copy-link-btn"
              slot="icon-only"
              @click="copyAddress(orgInfo.clientInfo.organizationAddr)"
              v-if="!addressCopiedToClipboard"
            >
              <ion-icon
                class="icon-copy"
                :icon="copy"
              />
            </ion-button>
            <ion-text
              v-if="addressCopiedToClipboard"
              class="server-address-value__copied copied"
            >
              {{ $msTranslate('OrganizationPage.configuration.copyPath') }}
            </ion-text>
          </div>
        </div>
        <div class="info-list-item">
          <ion-label class="info-list-item__title">
            {{ $msTranslate('OrganizationPage.configuration.serverVersion') }}
          </ion-label>
          <ion-text
            class="info-list-item__value"
            id="server-version"
            v-if="serverConfig && serverConfig.serverVersion"
          >
            {{ serverConfig.serverVersion }}
          </ion-text>
          <div
            v-else
            id="unavailable-server-version"
            class="info-list-item__value info-list-item__badge warning"
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
  background: ms.color('surface-base-default');
  align-items: center;
  gap: ms.spacing('gap-3xl');
  width: 100%;
  max-width: 30rem;
  border-radius: ms.radius('2xl');
  height: fit-content;
  box-shadow: var(--parsec-shadow-input);
  padding: ms.spacing('padding-4xl');

  .card-header {
    display: flex;
    align-items: center;
    width: 100%;

    &__title {
      @include ms.font('heading-h4');
      color: ms.color('text-base-body');
    }
  }

  .card-content {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: ms.spacing('gap-3xl');
  }

  .info-list {
    padding: ms.spacing('padding-none');
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');

    &-item {
      justify-content: space-between;
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-3xl');

      &__title {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
      }

      &__badge {
        @include ms.font('label-xs-emphasis');
        padding: ms.spacing('padding-xs') ms.spacing('padding-lg');
        border-radius: ms.radius('6xl');
      }

      #server-version {
        @include ms.font('heading-h6');
        color: ms.color('text-base-body');
      }

      .success {
        color: ms.color('text-success-default');
        background: ms.color('surface-success-default-subtle-hover');
      }

      .warning {
        color: ms.color('text-warning-default');
        background: ms.color('surface-warning-default-subtle-hover');
      }

      .info {
        color: ms.color('text-base-body');
        background: ms.color('surface-base-page-secondary');
      }
    }
  }
}

.server-address {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg') !important;
  align-items: start;

  &__title {
    width: 100%;
  }

  &-value {
    color: ms.color('text-neutral-default');
    background-color: ms.color('surface-base-default-secondary');
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    border-radius: ms.radius('lg');
    padding: ms.spacing('padding-lg') ms.spacing('padding-lg') ms.spacing('padding-lg') ms.spacing('padding-3xl');
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: ms.spacing('gap-lg');
    overflow: hidden;
    width: 100%;

    &__text {
      @include ms.font('body-md-regular');
      white-space: nowrap;
      overflow-x: auto;
      overflow-y: hidden;
      scrollbar-gutter: auto;
      scrollbar-width: none;
    }

    &__copied {
      @include ms.font('body-md-regular');
      color: ms.color('text-success-default');
    }
  }

  #copy-link-btn {
    color: ms.color('icon-neutral-default');
    margin: 0;
    height: 2rem;
    width: 2rem;

    &::part(native) {
      padding: ms.spacing('padding-md');
      border-radius: ms.radius('md');
    }

    &:hover {
      color: ms.color('icon-brand-default');
    }
  }
}
</style>
