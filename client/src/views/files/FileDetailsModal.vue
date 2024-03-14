<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="$t('FileDetails.title', { name: entry.name })"
      :close-button="{ visible: true }"
    >
      <div class="file-info-container">
        <!-- Entry type -->
        <div class="file-info">
          <ms-image
            :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
            class="file-info-image"
          />
          <ion-icon
            class="cloud-overlay"
            :class="entry.needSync ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
            :icon="entry.needSync ? cloudDone : cloudOffline"
          />
          <div class="file-info-basic">
            <ion-text class="file-info-basic__name title-h4">
              {{ entry.name }}
            </ion-text>
            <ion-label class="file-info-basic__edit body">
              <span>{{ $t('FileDetails.stats.updated') }}</span>
              <span>{{ $d(entry.updated.toJSDate(), 'short') }}</span>
            </ion-label>
          </div>
        </div>

        <div class="file-info-details">
          <!-- Created -->
          <div class="file-info-details-item">
            <ion-label class="file-info-details-item__title caption-caption">
              {{ $t('FileDetails.stats.created') }}
            </ion-label>
            <ion-text class="file-info-details-item__value body">
              {{ $d(entry.created.toJSDate(), 'short') }}
            </ion-text>
          </div>
          <!-- Size (only for files) -->
          <div
            class="file-info-details-item"
            v-if="entry.isFile()"
          >
            <ion-label class="file-info-details-item__title caption-caption">
              {{ $t('FileDetails.stats.size') }}
            </ion-label>
            <span class="file-info-details-item__value body">{{ formatFileSize((entry as EntryStatFile).size) }}</span>
          </div>
          <!-- Version -->
          <div class="file-info-details-item">
            <ion-label class="file-info-details-item__title caption-caption">
              {{ $t('FileDetails.stats.version') }}
            </ion-label>
            <ion-text class="file-info-details-item__value body">
              {{ entry.baseVersion }}
            </ion-text>
          </div>
        </div>

        <!-- Path -->
        <div class="file-info-path">
          <ion-label class="file-info-path__title caption-caption">
            {{ $t('FileDetails.stats.path') }} {{ entry.isFile() ? $t('FileDetails.stats.file') : $t('FileDetails.stats.folder') }}
          </ion-label>
          <div class="file-info-path-value">
            <ion-text class="file-info-path-value__text body">
              {{ shortenFileName(path, { maxLength: 60, prefixLength: 20, suffixLength: 30 }) }}
            </ion-text>
            <ion-button
              fill="clear"
              size="small"
              id="copy-link-btn"
              @click="copyPath"
              v-if="!pathCopiedToClipboard"
            >
              <ion-icon
                class="icon-copy"
                :icon="copy"
              />
            </ion-button>
            <ion-text
              v-if="pathCopiedToClipboard"
              class="file-info-path-value__copied body copied"
            >
              {{ $t('FileDetails.stats.linkCopied') }}
            </ion-text>
          </div>
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { writeTextToClipboard } from '@/common/clipboard';
import { formatFileSize, getFileIcon, shortenFileName } from '@/common/file';
import { MsModal } from '@/components/core';
import { Folder, MsImage } from '@/components/core/ms-image';
import { EntryStat, EntryStatFile } from '@/parsec';
import { IonButton, IonIcon, IonLabel, IonPage, IonText } from '@ionic/vue';
import { cloudDone, cloudOffline, copy } from 'ionicons/icons';
import { defineProps, ref } from 'vue';

const props = defineProps<{
  entry: EntryStat;
  path: string;
}>();

const pathCopiedToClipboard = ref(false);

async function copyPath(): Promise<void> {
  await writeTextToClipboard(props.path);
  pathCopiedToClipboard.value = true;
  setTimeout(() => {
    pathCopiedToClipboard.value = false;
  }, 5000);
}
</script>

<style scoped lang="scss">
.file-info-container {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  --background-color: var(--parsec-color-light-secondary-premiere);
  --color: var(--parsec-color-light-primary-600);
}

.file-info {
  display: flex;
  gap: 1rem;
  position: relative;

  .cloud-overlay {
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

  &-image {
    display: flex;
    width: 3rem;
    height: 3rem;
  }

  &-basic {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;

    &__name {
      color: var(--parsec-color-light-secondary-text);
    }

    &__edit {
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  &-details {
    display: flex;
    border-radius: var(--parsec-radius-6);
    background: var(--parsec-color-light-secondary-background);

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
      color: var(--parsec-color-light-secondary-grey);
    }

    &-value {
      color: var(--parsec-color-light-secondary-text);
      background-color: var(--parsec-color-light-secondary-background);
      border-radius: var(--parsec-radius-6);
      padding: 0.5rem 1rem;
      display: flex;
      align-items: center;
      gap: 0.5rem;
      overflow: hidden;
      width: 100%;

      &__text {
        padding: 0.25rem 0;
        text-overflow: ellipsis;
        white-space: nowrap;
        overflow: hidden;
      }

      &__copied {
        color: var(--parsec-color-light-success-700);
      }
    }
  }
}

#copy-link-btn {
  color: var(--parsec-color-light-secondary-text);
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
