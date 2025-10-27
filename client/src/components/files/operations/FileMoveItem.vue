<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="element-container"
    :class="{
      waiting: state === FileOperationState.MoveAdded,
      progress: state === FileOperationState.OperationProgress,
      done: state === FileOperationState.EntryMoved,
      cancelled: state === FileOperationState.Cancelled,
      failed: state === FileOperationState.MoveFailed,
    }"
  >
    <ion-item class="element ion-no-padding">
      <div class="element-type">
        <div class="element-type__icon">
          <ms-image
            :image="Move"
            class="file-icon"
          />
        </div>
      </div>
      <div class="element-details">
        <ion-text class="element-details__name body">
          {{
            $msTranslate({
              key: 'FoldersPage.MoveFile.move',
              data: { name: shortenFileName(entryName, { maxLength: 32, prefixLength: 26, suffixLength: 5 }) },
            })
          }}
        </ion-text>
        <ion-label
          class="element-details__workspace body-sm"
          v-if="workspaceName"
        >
          {{ workspaceName }}
        </ion-label>
        <div
          class="element-details-progress-container"
          v-show="state === FileOperationState.OperationProgress"
        >
          <ms-progress
            class="progress-bar"
            :progress="getProgress()"
            :appearance="MsProgressAppearance.Line"
          />
          <ion-button
            fill="clear"
            size="small"
          >
            <ion-icon
              class="cancel-icon"
              v-if="state === FileOperationState.MoveAdded"
              @click="$emit('cancel', operationData.id)"
              type="button"
              :icon="closeCircle"
              slot="icon-only"
            />
          </ion-button>
        </div>
      </div>

      <!-- waiting -->
      <div
        class="waiting-info"
        v-if="state === FileOperationState.MoveAdded"
      >
        <ion-text class="waiting-text body">
          {{ $msTranslate('FoldersPage.MoveFile.waiting') }}
        </ion-text>
        <ion-icon
          class="cancel-icon"
          @click="$emit('cancel', operationData.id)"
          type="button"
          :icon="closeCircle"
          slot="icon-only"
        />
      </div>

      <!-- done -->
      <ion-icon
        class="checkmark-icon"
        v-show="state === FileOperationState.EntryMoved"
        :icon="checkmarkCircle"
      />

      <!-- cancel -->
      <ion-text
        class="cancel-text body"
        v-if="state === FileOperationState.Cancelled"
      >
        {{ $msTranslate('FoldersPage.MoveFile.cancelled') }}
      </ion-text>

      <!-- failed -->
      <div
        class="failed-content"
        v-if="state === FileOperationState.MoveFailed"
      >
        <ion-text class="failed-text body">
          {{ $msTranslate('FoldersPage.MoveFile.failed') }}
        </ion-text>
        <ms-information-tooltip
          v-show="false"
          :text="'FoldersPage.MoveFile.failedDetails'"
          class="information-icon"
          slot="end"
        />
      </div>
    </ion-item>
  </div>
</template>

<script setup lang="ts">
import { shortenFileName } from '@/common/file';
import { EntryName, Path, getWorkspaceName } from '@/parsec';
import { FileOperationState, MoveData, OperationProgressStateData, StateData } from '@/services/fileOperationManager';
import { IonButton, IonIcon, IonItem, IonLabel, IonText } from '@ionic/vue';
import { checkmarkCircle, closeCircle } from 'ionicons/icons';
import { Move, MsImage, MsInformationTooltip, MsProgress, MsProgressAppearance } from 'megashark-lib';
import { Ref, onMounted, ref } from 'vue';

const props = defineProps<{
  operationData: MoveData;
  stateData?: StateData;
  state: FileOperationState;
}>();

const workspaceName = ref('');
const entryName: Ref<EntryName> = ref('');

defineExpose({
  props,
});

onMounted(async () => {
  workspaceName.value = await getWorkspaceName(props.operationData.workspaceHandle);
  entryName.value = (await Path.filename(props.operationData.dstPath)) ?? '';
});

defineEmits<{
  (event: 'cancel', id: string): void;
  (event: 'click', operationData: MoveData, state: FileOperationState): void;
}>();

function getProgress(): number {
  if (props.state === FileOperationState.Cancelled || props.state === FileOperationState.EntryMoved) {
    return 100;
  } else if (props.state === FileOperationState.OperationProgress && props.stateData) {
    return (props.stateData as OperationProgressStateData).progress;
  }
  return 0;
}
</script>

<style scoped lang="scss">
.element-container {
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  transition: background 0.2s ease-in-out;
  border-radius: var(--parsec-radius-8);
  position: relative;
  margin: 0.5rem 0.5rem 0;

  &:hover {
    background: var(--parsec-color-light-secondary-premiere);
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
    overflow: hidden;

    &__name {
      color: var(--parsec-color-light-primary-700);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &__workspace {
      color: var(--parsec-color-light-secondary-grey);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &-progress-container {
      display: flex;
      align-items: center;
      margin-top: 0.25rem;
      gap: 0.5rem;

      .progress-bar {
        --progress-background: var(--parsec-color-light-primary-500);
        --background: var(--parsec-color-light-secondary-medium);
        border-radius: var(--parsec-radius-8);
        width: 100%;
      }

      .cancel-icon {
        color: var(--parsec-color-light-secondary-grey);
        font-size: 1.125rem;
        padding: 0.25rem;
        border-radius: var(--parsec-radius-4);

        &:hover {
          color: var(--parsec-color-light-secondary-text);
          background: var(--parsec-color-light-secondary-disabled);
        }
      }
    }
  }

  .checkmark-icon,
  .waiting-info,
  .failed-text,
  .cancel-text,
  .failed-content {
    margin-left: auto;
    padding-left: 0.5rem;
    flex-shrink: 0;
  }
}

// states of the element-container
.waiting,
.progress {
  .cancel-icon {
    color: var(--parsec-color-light-secondary-grey);
    font-size: 1.15rem;
    padding: 0.25rem;
    border-radius: var(--parsec-radius-4);
    cursor: pointer;

    &:hover {
      color: var(--parsec-color-light-secondary-text);
      background: var(--parsec-color-light-secondary-disabled);
    }
  }
}

.waiting {
  .waiting-info {
    display: flex;
    align-items: center;
  }

  .waiting-text {
    color: var(--parsec-color-light-primary-700);
  }

  .cancel-icon {
    display: none;
  }

  &:hover {
    .waiting-text {
      display: none;
    }

    .cancel-icon {
      display: block;
    }
  }
}

.done {
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
    color: var(--parsec-color-light-secondary-hard-grey);
  }
}

.failed {
  &-content {
    color: var(--parsec-color-light-danger-500);
    display: flex;
    align-items: center;
    gap: 0.375rem;

    .information-icon {
      font-size: 1.25rem;

      &:hover {
        color: var(--parsec-color-light-danger-700);
      }
    }
  }
}
</style>
