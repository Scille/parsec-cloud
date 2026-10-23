<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="entry.isFile() ? 'FileDetails.fileTitle' : 'FileDetails.folderTitle'"
      :close-button="{ visible: true }"
    >
      <div class="file-details">
        <div class="file-header">
          <ms-image
            :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
            class="file-header__icon"
          />
          <div class="file-header-text">
            <ion-text class="file-header-text__name">
              {{ entry.name }}
            </ion-text>
            <ion-icon
              class="cloud-overlay"
              :class="entry.needSync ? 'cloud-overlay-ko' : 'cloud-overlay-ok'"
              :icon="entry.needSync ? cloudOffline : cloudDone"
              @click="openTooltip($event, getSyncString())"
            />
          </div>
        </div>

        <div class="file-section">
          <ion-text class="file-section__title">{{ $msTranslate('FileDetails.stats.generalInfo') }}</ion-text>
          <div class="file-info">
            <!-- Created -->
            <div class="file-info-item">
              <ion-label class="file-info-item__title">
                {{ $msTranslate('FileDetails.stats.created') }}
              </ion-label>
              <ion-text class="file-info-item__value">
                {{ $msTranslate(I18n.formatDate(entry.created, 'full')) }}
              </ion-text>
            </div>
            <!-- Size (only for files) -->
            <div
              class="file-info-item"
              v-if="entry.isFile()"
            >
              <ion-label class="file-info-item__title">
                {{ $msTranslate('FileDetails.stats.size') }}
              </ion-label>
              <ion-text class="file-info-item__value">
                {{ $msTranslate(formatFileSize((entry as EntryStatFile).size)) }}
              </ion-text>
            </div>
            <!-- Version -->
            <div class="file-info-item">
              <ion-label class="file-info-item__title">
                {{ $msTranslate('FileDetails.stats.version') }}
              </ion-label>
              <ion-text class="file-info-item__value">
                {{ entry.baseVersion }}
              </ion-text>
            </div>
          </div>
          <technical-id :id="entry.id" />
        </div>

        <div class="file-section">
          <ion-text class="file-section__title">{{ $msTranslate('FileDetails.stats.lastUpdate') }}</ion-text>
          <div class="file-info">
            <!-- Updated -->
            <div class="file-info-item">
              <ion-label class="file-info-item__title">
                {{ $msTranslate('FileDetails.stats.updated') }}
              </ion-label>
              <ion-text class="file-info-item__value">
                {{ $msTranslate(I18n.formatDate(entry.updated, 'full')) }}
              </ion-text>
            </div>
            <!-- Editor -->
            <div
              class="file-info-item"
              v-if="entry.lastUpdater && ownProfile !== UserProfile.Outsider"
            >
              <ion-label class="file-info-item__title">
                {{ $msTranslate('FileDetails.stats.editor') }}
              </ion-label>
              <ion-text class="file-info-item__value">
                {{ entry.lastUpdater.humanHandle.label }}
              </ion-text>
            </div>
          </div>
        </div>

        <!-- Path -->
        <div class="file-section">
          <ion-text class="file-section__title">
            {{ $msTranslate('FileDetails.stats.path') }}
            {{ entry.isFile() ? $msTranslate('FileDetails.stats.file') : $msTranslate('FileDetails.stats.folder') }}
          </ion-text>
          <div class="file-path">
            <ion-text class="file-path__text">
              <span>
                {{ showFullPath ? entry.path : shortenFileName(entry.path, { maxLength: 60, prefixLength: 20, suffixLength: 30 }) }}
              </span>
            </ion-text>
            <template v-if="isDesktop()">
              <ion-button
                fill="clear"
                size="small"
                slot="icon-only"
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
                class="file-path__copied copied"
              >
                {{ $msTranslate('FileDetails.stats.linkCopied') }}
              </ion-text>
              <ion-text
                v-show="copyStatus === CopyStatus.FailedToCopy"
                class="file-path__not-copied"
              >
                {{ $msTranslate('FileDetails.stats.failedToCopy') }}
              </ion-text>
            </template>
          </div>
          <ion-text
            class="file-path-toggle"
            @click="showFullPath = !showFullPath"
            v-if="entry.path.length > 60"
          >
            {{ $msTranslate(showFullPath ? 'FileDetails.stats.hideFullPath' : 'FileDetails.stats.showFullPath') }}
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
.file-details {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');

  @include ms.responsive-breakpoint('sm') {
    margin-bottom: 2rem;
  }
}

.file-header {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-2xl');

  &__icon {
    display: flex;
    width: 3rem;
    height: 3rem;
    flex-shrink: 0;
  }

  &-text {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    min-width: 0;

    &__name {
      @include ms.font('heading-h5');

      color: ms.color('text-base-heading');
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  .cloud-overlay {
    cursor: pointer;
    flex-shrink: 0;
    font-size: 1rem;
    padding: ms.spacing('padding-sm');
    border-radius: ms.radius('2xl');
    background: ms.color('surface-base-default-secondary');

    &-ok {
      color: ms.color('icon-brand-default');
    }

    &-ko {
      color: ms.color('text-base-description');
    }
  }
}

.file-section {
  display: flex;
  flex-direction: column;

  &__title {
    @include ms.font('body-md-medium');

    color: ms.color('text-base-body');
    margin-bottom: 0.5rem;
  }
}

.file-info {
  display: flex;
  overflow: hidden;
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('lg');

  &:not(:last-child) {
    margin-bottom: 0.75rem;
  }

  &-item {
    display: flex;
    flex-direction: column;
    flex: 1 0 0;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
    background: ms.color('surface-base-default-secondary');

    &:not(:last-child) {
      border-right: ms.border('thin') solid ms.color('border-base-default');
    }

    &__title {
      @include ms.font('body-md-medium');

      color: ms.color('text-base-description');
    }

    &__value {
      @include ms.font('body-md-regular');

      color: ms.color('text-base-body');
    }
  }
}

.file-path {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: ms.spacing('gap-lg');
  overflow: hidden;
  padding: ms.spacing('padding-sm') ms.spacing('padding-3xl');
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('lg');
  background: ms.color('surface-base-default-secondary');
  color: ms.color('text-base-body');

  &__text {
    @include ms.font('body-md-regular');

    overflow: hidden;
    text-overflow: ellipsis;
  }

  &__copied {
    @include ms.font('body-md-regular');

    color: ms.color('text-success-default');
  }

  &__not-copied {
    @include ms.font('body-md-regular');

    color: ms.color('text-error-default');
  }

  &-toggle {
    @include ms.font('label-sm-medium');

    width: fit-content;
    margin-top: 0.5rem;
    cursor: pointer;
    color: ms.color('text-base-description');

    &:hover {
      color: ms.color('text-base-label');
      text-decoration: underline;
    }
  }
}

#copy-link-btn {
  color: ms.color('text-base-label');
  margin: 0;

  &::part(native) {
    padding: ms.spacing('padding-lg');
    border-radius: ms.radius('md');
  }

  &:hover {
    color: ms.color('text-brand-default-hover');
  }
}
</style>
