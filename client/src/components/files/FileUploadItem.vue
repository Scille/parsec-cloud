<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="element-container"
    :class="{
      waiting: state === ImportState.FileAdded,
      progress: state === ImportState.FileProgress,
      done: state === ImportState.FileImported,
      cancelled: state === ImportState.Cancelled,
      failed: state === ImportState.CreateFailed,
    }"
  >
    <ion-item
      class="element ion-no-padding"
      @click="onImportClick"
    >
      <div class="element-type">
        <div class="element-type__icon">
          <ms-image
            :image="getFileIcon(fileCache.name)"
            class="file-icon"
          />
        </div>
      </div>
      <div class="element-details">
        <ion-text class="element-details__name body">
          {{ shortenFileName(fileCache.name) }}
        </ion-text>
        <ion-label class="element-details__size body-sm">
          <span class="default-state">{{ formatFileSize(fileCache.size) }}</span>
          <span
            class="hover-state"
            v-show="[ImportState.FileImported].includes(state)"
          >
            {{ $t('FoldersPage.ImportFile.browse') }}
          </span>
        </ion-label>
      </div>

      <!-- waiting -->
      <ion-text
        class="waiting-text body"
        v-if="state === ImportState.FileAdded"
      >
        {{ $t('FoldersPage.ImportFile.waiting') }}
      </ion-text>

      <!-- in progress -->
      <ion-button
        fill="clear"
        size="small"
        class="cancel-button"
        v-if="state === ImportState.FileProgress"
        @click="onCancelClick()"
      >
        <ion-icon
          class="cancel-button__icon"
          slot="icon-only"
          :icon="close"
        />
      </ion-button>

      <!-- done -->
      <ion-icon
        class="checkmark-icon default-state"
        v-show="state === ImportState.FileImported"
        :icon="checkmark"
      />
      <ion-icon
        class="arrow-icon hover-state"
        v-show="state === ImportState.FileImported"
        :icon="arrowForward"
      />

      <!-- cancel -->
      <ion-text
        class="cancel-text body"
        v-if="state === ImportState.Cancelled"
      >
        {{ $t('FoldersPage.ImportFile.cancelled') }}
      </ion-text>

      <!-- failed -->
      <div
        class="failed-content"
        v-if="state === ImportState.CreateFailed"
      >
        <ion-text class="failed-text body">
          {{ $t('FoldersPage.ImportFile.failed') }}
        </ion-text>
        <ms-information-tooltip
          :text="$t('FoldersPage.ImportFile.failedDetails')"
          class="information-icon"
          slot="end"
        />
      </div>
    </ion-item>
    <ion-progress-bar
      class="element-progress-bar"
      :value="state === ImportState.Cancelled ? 100 : progress / 100"
    />
  </div>
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon, shortenFileName } from '@/common/file';
import { MsImage } from '@/components/core/ms-image';
import MsInformationTooltip from '@/components/core/ms-tooltip/MsInformationTooltip.vue';
import { navigateToWorkspace } from '@/router';
import { ImportData, ImportState } from '@/services/importManager';
import { IonButton, IonIcon, IonItem, IonLabel, IonProgressBar, IonText } from '@ionic/vue';
import { arrowForward, checkmark, close } from 'ionicons/icons';

const props = defineProps<{
  importData: ImportData;
  progress: number;
  state: ImportState;
}>();

// Props get refreshed for every event but the file name or size
// will never change, so we cache them.
const fileCache = structuredClone(props.importData.file);

defineExpose({
  props,
});

const emits = defineEmits<{
  (event: 'importCancel', id: string): void;
}>();

async function onImportClick(): Promise<void> {
  if (props.state !== ImportState.FileImported) {
    return;
  }
  await navigateToWorkspace(props.importData.workspaceId, props.importData.path);
}

function onCancelClick(): void {
  emits('importCancel', props.importData.id);
}
</script>

<style scoped lang="scss">
.element-container {
  background: var(--parsec-color-light-secondary-premiere);
  transition: background 0.2s ease-in-out;
  border-radius: var(--parsec-radius-8);
  position: relative;

  &:hover {
    background: var(--parsec-color-light-secondary-medium);
  }
}

.element {
  --background: none;
  padding: 1rem;
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

    &__name {
      color: var(--parsec-color-light-primary-800);
    }

    &__size {
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  &-progress-bar {
    --progress-background: var(--parsec-color-light-primary-500);
    --background: var(--parsec-color-light-secondary-medium);
    border-radius: 0 0 var(--parsec-radius-8) var(--parsec-radius-8);
    position: absolute;
    bottom: 0;
    width: 100%;
  }

  .checkmark-icon,
  .arrow-icon,
  .waiting-text,
  .cancel-button,
  .failed-text,
  .cancel-text,
  .failed-content {
    margin-left: auto;
  }
}

// states of the element-container
.waiting {
  .waiting-text {
    color: var(--parsec-color-light-primary-700);
  }
}

.progress {
  .cancel-button {
    color: var(--parsec-color-light-secondary-text);
    --background-hover: var(--parsec-color-light-secondary-disabled);

    &::part(native) {
      padding: 0.25rem;
    }
  }
  .element-progress-bar {
    --progress-background: var(--parsec-color-light-primary-300);
  }
}

.done {
  cursor: pointer;

  .hover-state {
    display: none;
  }

  &:hover {
    background: var(--parsec-color-light-primary-50);
    .default-state {
      display: none;
    }
    .hover-state {
      display: block;
    }
  }

  .element-progress-bar {
    --progress-background: var(--parsec-color-light-primary-600);
  }

  .arrow-icon,
  .checkmark-icon {
    color: var(--parsec-color-light-primary-600);
  }
}

.cancelled {
  --background: var(--parsec-color-light-danger-500);

  .element {
    opacity: 0.5;
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
