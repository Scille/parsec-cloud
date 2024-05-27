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
            :image="getFileIcon('')"
            class="file-icon"
          />
        </div>
      </div>
      <div class="element-details">
        <ion-text class="element-details__name body">
          COPY {{ shortenFileName(operationData.srcPath, { maxLength: 40, prefixLength: 8, suffixLength: 32 }) }}
        </ion-text>
        <ion-label class="element-details__size body-sm">
          <span
            class="default-state"
            v-if="workspaceInfo"
          >
            &bull; {{ workspaceInfo.currentName }}
          </span>
          <span
            class="hover-state"
            v-show="[FileOperationState.EntryCopied].includes(state)"
          >
            {{ $msTranslate('FoldersPage.ImportFile.browse') }}
          </span>
        </ion-label>
      </div>

      <!-- waiting -->
      <div class="waiting-info">
        <ion-text
          class="waiting-text body"
          v-if="state === FileOperationState.CopyAdded"
        >
          {{ $msTranslate('FoldersPage.ImportFile.waiting') }}
        </ion-text>
        <ion-button
          fill="clear"
          size="small"
          class="cancel-button"
          v-if="state === FileOperationState.CopyAdded"
          @click="$emit('cancel', operationData.id)"
        >
          <ion-icon
            class="cancel-button__icon"
            slot="icon-only"
            :icon="close"
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
          :icon="close"
        />
      </ion-button>

      <!-- done -->
      <ion-icon
        class="checkmark-icon default-state"
        v-show="state === FileOperationState.EntryCopied"
        :icon="checkmark"
      />
      <ion-icon
        class="arrow-icon hover-state"
        v-show="state === FileOperationState.EntryCopied"
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
          text="FoldersPage.ImportFile.failedDetails"
          class="information-icon"
          slot="end"
        />
      </div>
    </ion-item>
    <ion-progress-bar
      class="element-progress-bar"
      :value="state === FileOperationState.Cancelled ? 100 : progress / 100"
    />
  </div>
</template>

<script setup lang="ts">
import { getFileIcon, shortenFileName } from '@/common/file';
import { MsImage, MsInformationTooltip } from 'megashark-lib';
import { StartedWorkspaceInfo, getWorkspaceInfo } from '@/parsec';
import { CopyData, FileOperationState } from '@/services/fileOperationManager';
import { IonButton, IonIcon, IonItem, IonLabel, IonProgressBar, IonText } from '@ionic/vue';
import { arrowForward, checkmark, close } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';

const props = defineProps<{
  operationData: CopyData;
  progress: number;
  state: FileOperationState;
}>();

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

defineEmits<{
  (event: 'cancel', id: string): void;
  (event: 'click', operationData: CopyData, state: FileOperationState): void;
}>();
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
  .waiting-info,
  .cancel-button,
  .failed-text,
  .cancel-text,
  .failed-content {
    margin-left: auto;
  }
}

// states of the element-container
.waiting,
.progress {
  .cancel-button {
    color: var(--parsec-color-light-secondary-text);
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
  background: var(--parsec-color-light-primary-50);

  .hover-state {
    display: none;
  }

  &:hover {
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
