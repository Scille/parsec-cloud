<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="WorkspaceDetails.title"
      :close-button="{ visible: true }"
    >
      <div class="workspace-info-container">
        <div class="workspace-header">
          <ion-icon
            class="workspace-header__icon"
            :icon="business"
          />
          <div class="workspace-header-text">
            <ion-text class="workspace-header-text__name title-h4">
              {{ workspaceInfo.currentName }}
            </ion-text>
            <ion-text class="workspace-header-text__subtitle subtitles-sm">
              {{ $msTranslate('WorkspaceDetails.offlineAvailability.notAvailable') }}
            </ion-text>
          </div>
        </div>

        <div class="workspace-infos">
          <ion-text class="workspace-infos__title subtitles-normal">
            {{ $msTranslate('WorkspaceDetails.stats.generalInfo') }}
          </ion-text>
          <div class="workspace-infos-content">
            <div class="workspace-infos-item">
              <ion-label class="workspace-infos-item__title subtitles-sm">
                {{ $msTranslate('WorkspaceDetails.stats.rights') }}
              </ion-label>
              <workspace-role-tag
                :role="workspaceInfo.currentSelfRole"
                class="workspace-role-tag"
              />
            </div>

            <div
              class="workspace-infos-item"
              v-if="workspaceInfo.joinedOn"
            >
              <ion-label class="workspace-infos-item__title subtitles-sm">
                {{ $msTranslate('WorkspaceDetails.stats.joinedOn') }}
              </ion-label>
              <span class="workspace-infos-item__value body">
                {{ $msTranslate(I18n.formatDate(workspaceInfo.joinedOn, 'short')) }}
              </span>
            </div>

            <div
              class="workspace-infos-item"
              v-if="workspaceInfo.created"
            >
              <ion-label class="workspace-infos-item__title subtitles-sm">
                {{ $msTranslate('WorkspaceDetails.stats.joinedOn') }}
              </ion-label>
              <span class="workspace-infos-item__value body">
                {{ $msTranslate(I18n.formatDate(workspaceInfo.created, 'short')) }}
              </span>
            </div>
          </div>
          <ion-text class="workspace-infos__role body">
            <i18n-t
              :keypath="getRoleInfo(workspaceInfo.currentSelfRole)"
              scope="global"
            >
              <template #role>
                <strong>
                  {{ $msTranslate(getWorkspaceRoleTranslationKey(workspaceInfo.currentSelfRole).label) }}
                </strong>
              </template>
            </i18n-t>
          </ion-text>
        </div>

        <div class="workspace-infos">
          <ion-text class="workspace-infos__title subtitles-normal">
            {{ $msTranslate('WorkspaceDetails.offlineAvailability.title') }}
          </ion-text>
          <div class="workspace-infos-content">
            <div class="workspace-infos-item">
              <ion-label class="workspace-infos-item__title subtitles-sm">
                {{ $msTranslate('WorkspaceDetails.offlineAvailability.storage') }}
              </ion-label>
              <ion-text class="workspace-infos-item__value body-lg">
                {{
                  $msTranslate({
                    key: 'WorkspaceDetails.offlineAvailability.remainingToDownload',
                    data: { size: formatFileSize(workspaceInfo.size) },
                  })
                }}
              </ion-text>
              <template v-if="workspaceInfo.downloadState">
                <ion-button
                  id="available-offline-btn"
                  class="button-medium"
                  :class="{
                    'stop-sync': workspaceInfo.downloadState === WorkspaceDownloadState.Available,
                    'enable-sync': workspaceInfo.downloadState === WorkspaceDownloadState.NotAvailable,
                  }"
                  @click="offlineAvailability"
                  v-if="workspaceInfo.downloadState !== WorkspaceDownloadState.InProgress"
                >
                  <span v-if="workspaceInfo.downloadState === WorkspaceDownloadState.NotAvailable">
                    {{ $msTranslate('WorkspaceDetails.offlineAvailability.makeAvailableOffline') }}
                  </span>
                  <span v-if="workspaceInfo.downloadState === WorkspaceDownloadState.Available">
                    {{ $msTranslate('WorkspaceDetails.offlineAvailability.stopSynchronisation') }}
                  </span>
                </ion-button>

                <div
                  class="download"
                  v-if="workspaceInfo.downloadState === WorkspaceDownloadState.InProgress"
                >
                  <ms-progress
                    class="download__progress-bar"
                    :progress="getProgress()"
                    :appearance="MsProgressAppearance.Bar"
                  />
                  <ion-button
                    class="download__cancel button-medium"
                    @click="stopOfflineAvailability"
                  >
                    {{ $msTranslate('WorkspaceDetails.offlineAvailability.cancel') }}
                  </ion-button>
                </div>
              </template>
              <div v-if="error">
                <ion-text class="body-lg button-medium error">
                  {{ $msTranslate('WorkspaceDetails.offlineAvailability.error.enableFailed') }}
                </ion-text>
              </div>
            </div>
          </div>
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import { WorkspaceRoleTag } from '@/components/workspaces';
import { WorkspaceDownloadState, WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { IonIcon, IonLabel, IonPage, IonText } from '@ionic/vue';
import { business } from 'ionicons/icons';
import { I18n, MsModal, MsProgress, MsProgressAppearance } from 'megashark-lib';
import { ref } from 'vue';

const error = ref(true);

const props = defineProps<{
  workspaceInfo: WorkspaceInfo;
  progress: number;
}>();

const getRoleInfo = (role: WorkspaceRole) => {
  switch (role) {
    case WorkspaceRole.Owner:
      return 'WorkspaceDetails.rightsInfos.owner';
    case WorkspaceRole.Manager:
      return 'WorkspaceDetails.rightsInfos.administrator';
    case WorkspaceRole.Contributor:
      return 'WorkspaceDetails.rightsInfos.contributor';
    case WorkspaceRole.Reader:
      return 'WorkspaceDetails.rightsInfos.reader';
    default:
      return 'Unknown';
  }
};

async function offlineAvailability() {
  if (props.workspaceInfo.downloadState === WorkspaceDownloadState.NotAvailable) {
    await props.workspaceInfo.makeAvailableOffline();
  } else {
    await props.workspaceInfo.stopOfflineAvailability();
  }
}

function getProgress(): number {
  if (props.workspaceInfo.downloadState === WorkspaceDownloadState.NotAvailable) {
    return 100;
  } else if (props.workspaceInfo.downloadState === WorkspaceDownloadState.InProgress) {
    return props.progress;
  }
  return 0;
}
</script>

<style scoped lang="scss">
.workspace-info-container {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  --background-color: var(--parsec-color-light-secondary-premiere);
  --color: var(--parsec-color-light-primary-600);
  margin-top: 0.5rem;

  @include ms.responsive-breakpoint('sm') {
    margin-bottom: 2rem;
  }
}

.workspace-header {
  display: flex;
  gap: 1rem;
  position: relative;
  align-items: center;

  &__icon {
    display: flex;
    align-items: center;
    font-size: 1.5rem;
    background: var(--parsec-color-light-primary-50);
    padding: 0.5rem;
    border-radius: var(--parsec-radius-8);
    color: var(--parsec-color-light-primary-800);
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;

    &__name {
      color: var(--parsec-color-light-secondary-text);
    }

    &__subtitle {
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}

.workspace-infos {
  display: flex;
  flex-direction: column;
  border-radius: var(--parsec-radius-6);
  gap: 0.75rem;

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &-content {
    background: var(--parsec-color-light-secondary-background);
    border-radius: var(--parsec-radius-8);
    box-shadow: var(--parsec-shadow-input);
    display: flex;
    gap: 0.75rem;
  }

  &-item {
    display: flex;
    flex-direction: column;
    padding: 0.625rem 1rem;
    gap: 0.5rem;
    width: 100%;

    &:not(:last-child) {
      border-right: 1px solid var(--parsec-color-light-secondary-disabled);
    }

    .workspace-role-tag {
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-18);
      background: var(--parsec-color-light-secondary-white);
      width: fit-content;
    }

    &__title {
      color: var(--parsec-color-light-secondary-grey);
    }

    &__value {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &__role {
    color: var(--parsec-color-light-secondary-hard-grey);
  }
}

#available-offline-btn {
  --background: transparent;
  --background-hover: transparent;
  width: fit-content;
  margin: 0;
  border-bottom: 1px solid transparent;

  &.enable-sync {
    color: var(--parsec-color-light-primary-500);
  }

  &.stop-sync {
    color: var(--parsec-color-light-danger-500);
  }

  &::part(native) {
    padding: 0.375rem 0;
    border-radius: var(--parsec-radius-6);
  }

  &:hover {
    &.enable-sync {
      color: var(--parsec-color-light-primary-600);
      border-bottom: 1px solid var(--parsec-color-light-primary-600);
    }

    &.stop-sync {
      color: var(--parsec-color-light-danger-500);
      border-bottom: 1px solid var(--parsec-color-light-danger-500);
    }
  }
}

.download {
  display: flex;
  align-items: center;
  gap: 1rem;

  &__progress-bar {
    flex-grow: 1;
  }

  &__cancel {
    color: var(--parsec-color-light-primary-500);
    border-bottom: 1px solid transparent;

    &::part(native) {
      --background: transparent;
      --background-hover: var(--parsec-color-light-primary-50);

      padding: 0.5rem 0.75rem;
    }

    &:hover {
      color: var(--parsec-color-light-primary-600);
    }
  }
}

.error {
  color: var(--parsec-color-light-danger-500);
}
</style>
