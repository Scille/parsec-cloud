<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="element-container"
    :class="{
      waiting: state === FileOperationState.DownloadArchiveAdded,
      progress: state === FileOperationState.OperationProgress,
      done: state === FileOperationState.ArchiveDownloaded,
      cancelled: state === FileOperationState.Cancelled,
      failed: state === FileOperationState.CreateFailed,
    }"
  >
    <ion-item class="element ion-no-padding">
      <div class="element-type">
        <div class="element-type__icon">
          <ms-image
            :image="icon"
            class="file-icon"
          />
        </div>
      </div>
      <div class="element-details">
        <ion-text class="element-details__name body">
          {{ shortenFileName(props.operationData.saveHandle.name, { suffixLength: 4, maxLength: 42 }) }}
        </ion-text>
        <ion-text class="element-details-info body-sm">
          <span class="default-state element-details-info__size">
            {{ $msTranslate(formatFileSize(operationData.totalSize)) }}
          </span>
          <span
            class="default-state element-details-info__workspace"
            v-if="progress.name"
          >
            &bull; {{ progress.name }}
          </span>
          <span
            class="hover-state"
            v-show="state === FileOperationState.ArchiveDownloaded"
          >
            {{ $msTranslate('FoldersPage.ImportFile.browse') }}
          </span>
        </ion-text>
      </div>

      <!-- waiting -->
      <div class="waiting-info">
        <ion-text
          class="waiting-text body"
          v-if="state === FileOperationState.DownloadArchiveAdded"
        >
          {{ $msTranslate('FoldersPage.ImportFile.waiting') }}
        </ion-text>
        <ion-button
          fill="clear"
          size="small"
          class="cancel-button"
          v-if="state === FileOperationState.DownloadArchiveAdded"
          @click="$emit('cancel', operationData.id)"
        >
          <ion-icon
            class="cancel-button__icon"
            slot="icon-only"
            :icon="closeCircle"
          />
        </ion-button>
      </div>

      <!-- in progress -->
      <ion-button
        fill="clear"
        size="small"
        class="cancel-button"
        v-if="state === FileOperationState.OperationProgress"
        @click="$emit('cancel', operationData.id)"
      >
        <ion-icon
          class="cancel-button__icon"
          slot="icon-only"
          :icon="closeCircle"
        />
      </ion-button>

      <!-- done -->
      <ion-icon
        class="checkmark-icon default-state"
        v-show="state === FileOperationState.ArchiveDownloaded"
        :icon="checkmarkCircle"
      />

      <!-- cancel -->
      <ion-text
        class="cancel-text body"
        v-if="state === FileOperationState.Cancelled"
      >
        {{ $msTranslate('FoldersPage.ImportFile.cancelled') }}
      </ion-text>

      <!-- failed -->
      <div
        class="failed-content"
        v-if="state === FileOperationState.DownloadArchiveFailed"
      >
        <ion-text class="failed-text body">
          {{ $msTranslate('FoldersPage.ImportFile.failed') }}
        </ion-text>
        <ms-information-tooltip
          v-show="false"
          text="FoldersPage.ImportFile.failedDetails"
          class="information-icon"
          slot="end"
        />
      </div>
    </ion-item>
    <ms-progress
      class="element-progress-bar"
      :progress="progress.totalProgress"
      :appearance="MsProgressAppearance.Line"
    />
  </div>
</template>

<script setup lang="ts">
import { formatFileSize, shortenFileName } from '@/common/file';
import { DownloadArchiveData, DownloadOperationProgressStateData, FileOperationState, StateData } from '@/services/fileOperationManager';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { checkmarkCircle, closeCircle } from 'ionicons/icons';
import { File, MsImage, MsInformationTooltip, MsProgress, MsProgressAppearance } from 'megashark-lib';
import { computed } from 'vue';

const props = defineProps<{
  operationData: DownloadArchiveData;
  state: FileOperationState;
  stateData?: StateData;
}>();

const icon = File.Zip;
const progress = computed(() => {
  if (props.state === FileOperationState.Cancelled || props.state === FileOperationState.EntryDownloaded) {
    return { name: undefined, size: undefined, totalProgress: 100 };
  } else if (props.state === FileOperationState.OperationProgress && props.stateData) {
    const data = props.stateData as DownloadOperationProgressStateData;
    return { name: data.currentFile, size: data.currentFileSize, totalProgress: data.progress };
  }
  return { name: undefined, size: undefined, totalProgress: 0 };
});

defineExpose({
  props,
});

defineEmits<{
  (event: 'cancel', id: string): void;
}>();
</script>

<style scoped lang="scss">
.element-container {
  background: var(--parsec-color-light-secondary-background);
  transition: background 0.2s ease-in-out;
  border-radius: var(--parsec-radius-8);
  position: relative;
  overflow: hidden;
  flex-shrink: 0;
  margin: 0.5rem 0.5rem 0;

  &:hover {
    background: var(--parsec-color-light-secondary-medium);
  }
}

.element {
  --background: none;
  padding: 0.75rem 1rem;
  --inner-padding-end: 0;

  .file-icon {
    width: 2rem;
    height: 2rem;
  }

  &-details {
    display: flex;
    flex-direction: column;
    position: relative;
    margin-left: 0.875rem;
    max-width: 16rem;

    &__name {
      color: var(--parsec-color-light-primary-800);
    }

    &-info {
      color: var(--parsec-color-light-secondary-grey);
      display: flex;
      gap: 0.25rem;
      overflow: hidden;

      &__workspace {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }
  }

  .checkmark-icon,
  .waiting-info,
  .cancel-button,
  .failed-text,
  .cancel-text,
  .failed-content {
    margin-left: auto;
    flex-shrink: 0;
  }
}

// states of the element-container
.waiting,
.progress {
  .cancel-button {
    color: var(--parsec-color-light-secondary-grey);
    --background-hover: var(--parsec-color-light-secondary-disabled);

    &__icon {
      font-size: 1.15rem;
    }

    &::part(native) {
      padding: 0.25rem;
    }
  }
}

.waiting {
  .waiting-text {
    color: var(--parsec-color-light-primary-700);
  }

  .cancel-button {
    display: none;
  }

  &:hover {
    .waiting-text {
      display: none;
    }

    .cancel-button {
      display: block;
    }
  }
}

.progress {
  .element-progress-bar {
    --progress-background: var(--parsec-color-light-primary-500);
    --background: var(--parsec-color-light-secondary-medium);
    position: absolute;
    bottom: 0;
    width: 100%;
    height: 2px;
  }
}

.done {
  .hover-state {
    display: none;
  }

  &:hover {
    background: var(--parsec-color-light-primary-50);
  }

  .element-progress-bar {
    display: none;
  }

  .checkmark-icon {
    color: var(--parsec-color-light-success-700);
    font-size: 1.25rem;
  }
}

.cancelled {
  --background: var(--parsec-color-light-danger-500);

  .element {
    opacity: 0.8;
    filter: grayscale(100%);
  }

  .cancel-text {
    color: var(--parsec-color-light-secondary-text);
  }

  .element-progress-bar {
    --progress-background: var(--parsec-color-light-secondary-light);
  }
}

.failed {
  &-content {
    color: var(--parsec-color-light-danger-500);
    display: flex;
    align-items: center;
    gap: 0.375rem;

    .information-icon {
      font-size: 1.375rem;

      &:hover {
        color: var(--parsec-color-light-danger-700);
      }
    }
  }
  .element-progress-bar {
    --progress-background: var(--parsec-color-light-danger-500);
  }
}
</style>
