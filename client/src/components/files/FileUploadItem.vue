<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="element-container"
    :class="{
      waiting: state === FileOperationState.FileAdded,
      progress: state === FileOperationState.OperationProgress,
      done: state === FileOperationState.FileImported,
      cancelled: state === FileOperationState.Cancelled,
      failed: state === FileOperationState.CreateFailed,
    }"
  >
    <ion-item
      class="element ion-no-padding"
      @click="$emit('click', operationData, state)"
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
        <ion-label class="element-details-info body-sm">
          <span class="default-state element-details-info__size">{{ $msTranslate(formatFileSize(fileCache.size)) }}</span>
          <span
            class="default-state element-details-info__workspace"
            v-if="workspaceInfo"
          >
            &bull; {{ workspaceInfo.currentName }}
          </span>
          <span
            class="hover-state"
            v-show="[FileOperationState.FileImported].includes(state)"
          >
            {{ $msTranslate('FoldersPage.ImportFile.browse') }}
          </span>
        </ion-label>
      </div>

      <!-- waiting -->
      <div class="waiting-info">
        <ion-text
          class="waiting-text body"
          v-if="state === FileOperationState.FileAdded"
        >
          {{ $msTranslate('FoldersPage.ImportFile.waiting') }}
        </ion-text>
        <ion-button
          fill="clear"
          size="small"
          class="cancel-button"
          v-if="state === FileOperationState.FileAdded"
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
        v-show="state === FileOperationState.FileImported"
        :icon="checkmarkCircle"
      />
      <ion-icon
        class="arrow-icon hover-state"
        v-show="state === FileOperationState.FileImported"
        :icon="arrowForward"
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
        v-if="state === FileOperationState.CreateFailed"
      >
        <ion-text class="failed-text body">
          {{ $msTranslate('FoldersPage.ImportFile.failed') }}
        </ion-text>
        <ms-information-tooltip
          v-show="false"
          :text="'FoldersPage.ImportFile.failedDetails'"
          class="information-icon"
          slot="end"
        />
      </div>
    </ion-item>
    <ion-progress-bar
      class="element-progress-bar"
      :value="getProgress() / 100"
    />
  </div>
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon, shortenFileName } from '@/common/file';
import { MsImage, MsInformationTooltip } from 'megashark-lib';
import { StartedWorkspaceInfo, getWorkspaceInfo } from '@/parsec';
import { ImportData, FileOperationState, StateData, OperationProgressStateData } from '@/services/fileOperationManager';
import { IonButton, IonIcon, IonItem, IonLabel, IonProgressBar, IonText } from '@ionic/vue';
import { arrowForward, checkmarkCircle, closeCircle } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';

const props = defineProps<{
  operationData: ImportData;
  state: FileOperationState;
  stateData?: StateData;
}>();

// Props get refreshed for every event but the file name or size
// will never change, so we cache them.
const fileCache = structuredClone(props.operationData.file);

const workspaceInfo: Ref<StartedWorkspaceInfo | null> = ref(null);

defineExpose({
  props,
});

onMounted(async () => {
  const result = await getWorkspaceInfo(props.operationData.workspaceHandle);
  if (result.ok) {
    workspaceInfo.value = result.value;
  }
});

function getProgress(): number {
  if (props.state === FileOperationState.Cancelled || props.state === FileOperationState.FileImported) {
    return 100;
  } else if (props.state === FileOperationState.OperationProgress && props.stateData) {
    return (props.stateData as OperationProgressStateData).progress;
  }
  return 0;
}

defineEmits<{
  (event: 'cancel', id: string): void;
  (event: 'click', operationData: ImportData, state: FileOperationState): void;
}>();
</script>

<style scoped lang="scss">
.element-container {
  background: var(--parsec-color-light-secondary-background);
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
    max-width: 16rem;

    &__name {
      color: var(--parsec-color-light-primary-800);
    }

    &-info {
      color: var(--parsec-color-light-secondary-grey);
      display: flex;
      gap: 0.25rem;
      overflow: hidden;

      &__size {
        flex-shrink: 0;
      }

      &__workspace {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
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

  .arrow-icon {
    color: var(--parsec-color-light-primary-600);
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
