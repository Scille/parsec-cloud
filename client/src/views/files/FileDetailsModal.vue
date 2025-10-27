<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="{ key: 'FileDetails.title', data: { name: entry.name } }"
      :close-button="{ visible: true }"
    >
      <div class="file-info-container">
        <!-- Entry type -->
        <div class="file-info">
          <ms-image
            :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
            class="file-info__image"
          />
          <ion-icon
            class="cloud-overlay"
            :class="entry.needSync ? 'cloud-overlay-ko' : 'cloud-overlay-ok'"
            :icon="entry.needSync ? cloudOffline : cloudDone"
            @click="openTooltip($event, getSyncString())"
          />
          <ion-text class="file-info__name title-h4">
            {{ entry.name }}
          </ion-text>
        </div>

        <div class="file-info-details-content">
          <ion-text class="file-info-details-content__title subtitles-sm">{{ $msTranslate('FileDetails.stats.generalInfo') }}</ion-text>
          <div class="file-info-details">
            <!-- Created -->
            <div class="file-info-details-item">
              <ion-label class="file-info-details-item__title subtitles-sm">
                {{ $msTranslate('FileDetails.stats.created') }}
              </ion-label>
              <ion-text class="file-info-details-item__value body">
                {{ $msTranslate(I18n.formatDate(entry.created, 'short')) }}
              </ion-text>
            </div>
            <!-- Size (only for files) -->
            <div
              class="file-info-details-item"
              v-if="entry.isFile()"
            >
              <ion-label class="file-info-details-item__title subtitles-sm">
                {{ $msTranslate('FileDetails.stats.size') }}
              </ion-label>
              <span class="file-info-details-item__value body">{{ $msTranslate(formatFileSize((entry as EntryStatFile).size)) }}</span>
            </div>
            <!-- Version -->
            <div class="file-info-details-item">
              <ion-label class="file-info-details-item__title subtitles-sm">
                {{ $msTranslate('FileDetails.stats.version') }}
              </ion-label>
              <ion-text class="file-info-details-item__value body">
                {{ entry.baseVersion }}
              </ion-text>
            </div>
          </div>
          <technical-id :id="entry.id" />
        </div>

        <div class="file-info-details-content">
          <ion-text class="file-info-details-content__title subtitles-sm">{{ $msTranslate('FileDetails.stats.lastUpdate') }}</ion-text>
          <div class="file-info-details">
            <!-- Updated -->
            <div class="file-info-details-item">
              <ion-label class="file-info-details-item__title subtitles-sm">
                {{ $msTranslate('FileDetails.stats.updated') }}
              </ion-label>
              <ion-text class="file-info-details-item__value body">
                {{ $msTranslate(I18n.formatDate(entry.updated, 'short')) }}
              </ion-text>
            </div>
            <!-- Editor -->
            <div
              class="file-info-details-item"
              v-if="entry.lastUpdater && ownProfile !== UserProfile.Outsider"
            >
              <ion-label class="file-info-details-item__title subtitles-sm">
                {{ $msTranslate('FileDetails.stats.editor') }}
              </ion-label>
              <ion-text class="file-info-details-item__value body">
                {{ entry.lastUpdater.humanHandle.label }}
              </ion-text>
            </div>
          </div>
        </div>

        <!-- Path -->
        <div class="file-info-path">
          <ion-label class="file-info-path__title subtitles-sm">
            {{ $msTranslate('FileDetails.stats.path') }}
            {{ entry.isFile() ? $msTranslate('FileDetails.stats.file') : $msTranslate('FileDetails.stats.folder') }}
          </ion-label>
          <div class="file-info-path-value">
            <ion-text class="file-info-path-value__text body">
              <span>
                {{ showFullPath ? entry.path : shortenFileName(entry.path, { maxLength: 60, prefixLength: 20, suffixLength: 30 }) }}
              </span>
            </ion-text>
            <template v-if="isDesktop()">
              <ion-button
                fill="clear"
                size="small"
                id="copy-link-btn"
                @click="copyPath"
                v-show="copyStatus === CopyStatus.NotCopied"
              >
                <ion-icon
                  class="icon-copy"
                  :icon="copy"
                />
              </ion-button>
              <ion-text
                v-show="copyStatus === CopyStatus.Copied"
                class="file-info-path-value__copied body copied"
              >
                {{ $msTranslate('FileDetails.stats.linkCopied') }}
              </ion-text>
              <ion-text
                v-show="copyStatus === CopyStatus.FailedToCopy"
                class="file-info-path-value__not-copied body"
              >
                {{ $msTranslate('FileDetails.stats.failedToCopy') }}
              </ion-text>
            </template>
          </div>
          <ion-text
            class="file-info-path__full button-small"
            @click="showFullPath = !showFullPath"
            v-if="entry.path.length > 60"
          >
            <span>{{ $msTranslate(showFullPath ? 'FileDetails.stats.hideFullPath' : 'FileDetails.stats.showFullPath') }}</span>
          </ion-text>
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon, shortenFileName } from '@/common/file';
import { TechnicalId } from '@/components/misc';
import { EntryStat, EntryStatFile, getSystemPath, isDesktop, UserProfile, WorkspaceHandle } from '@/parsec';
import { IonButton, IonIcon, IonLabel, IonPage, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, copy } from 'ionicons/icons';
import { Clipboard, Folder, I18n, MsImage, MsModal, openTooltip } from 'megashark-lib';
import { ref } from 'vue';

enum CopyStatus {
  NotCopied,
  Copied,
  FailedToCopy,
}

const props = defineProps<{
  entry: EntryStat;
  workspaceHandle: WorkspaceHandle;
  ownProfile: UserProfile;
}>();

const showFullPath = ref(false);
const copyStatus = ref(CopyStatus.NotCopied);

function getSyncString(): string {
  if (props.entry.isFile()) {
    return props.entry.needSync ? 'FileDetails.fileSyncKo' : 'FileDetails.fileSyncOk';
  } else {
    return props.entry.needSync ? 'FileDetails.folderSyncKo' : 'FileDetails.folderSyncOk';
  }
}

async function copyPath(): Promise<void> {
  const fullPathResult = await getSystemPath(props.workspaceHandle, props.entry.path);

  if (fullPathResult.ok) {
    if (await Clipboard.writeText(fullPathResult.value)) {
      copyStatus.value = CopyStatus.Copied;
      setTimeout(() => {
        copyStatus.value = CopyStatus.NotCopied;
      }, 4000);
      return;
    } else {
      copyStatus.value = CopyStatus.FailedToCopy;
    }
  } else {
    copyStatus.value = CopyStatus.FailedToCopy;
  }
  setTimeout(() => {
    copyStatus.value = CopyStatus.NotCopied;
  }, 4000);
}
</script>

<style scoped lang="scss">
.file-info-container {
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

.file-info {
  display: flex;
  gap: 1rem;
  position: relative;
  align-items: center;

  .cloud-overlay {
    cursor: pointer;
    position: absolute;
    font-size: 1rem;
    background: var(--parsec-color-light-secondary-background);
    padding: 0.25rem;
    border-radius: var(--parsec-radius-12);
    bottom: -0.5rem;
    left: 1.9rem;
    box-shadow: var(--parsec-shadow-strong);

    &-ok {
      color: var(--parsec-color-light-primary-500);
    }

    &-ko {
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  &__image {
    display: flex;
    width: 3rem;
    height: 3rem;
  }

  &__name {
    color: var(--parsec-color-light-secondary-text);
  }

  &-details {
    display: flex;
    border-radius: var(--parsec-radius-6);
    background: var(--parsec-color-light-secondary-background);

    &-content {
      display: flex;
      flex-direction: column;
      gap: 0.75rem;

      &__title {
        color: var(--parsec-color-light-secondary-text);
      }
    }

    &-item {
      display: flex;
      flex-direction: column;
      padding: 0.625rem 1rem;
      gap: 0.25rem;
      width: 100%;

      &:not(:last-child) {
        border-right: 1px solid var(--parsec-color-light-secondary-disabled);
      }

      &__title {
        color: var(--parsec-color-light-secondary-grey);
      }

      &__value {
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }

  &-path {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &__title {
      color: var(--parsec-color-light-secondary-text);
    }

    &__full {
      cursor: pointer;
      width: fit-content;
      color: var(--parsec-color-light-secondary-grey);

      &:hover {
        color: var(--parsec-color-light-secondary-hard-grey);
        text-decoration: underline;
      }
    }

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
        padding: 0.25rem 0;
        text-overflow: ellipsis;
        overflow: hidden;
      }

      &__copied {
        color: var(--parsec-color-light-success-700);
      }

      &__not-copied {
        color: var(--parsec-color-light-danger-500);
      }
    }
  }
}

#copy-link-btn {
  color: var(--parsec-color-light-secondary-hard-grey);
  margin: 0;

  &::part(native) {
    padding: 0.5rem;
    border-radius: var(--parsec-radius-6);
  }

  &:hover {
    color: var(--parsec-color-light-primary-600);
  }
}
</style>
