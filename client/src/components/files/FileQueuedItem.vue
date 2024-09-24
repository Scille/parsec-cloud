<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="element-container">
    <ion-item class="element ion-no-padding">
      <div class="element-type">
        <div class="element-type__icon">
          <ms-image
            :image="getImageFromType()"
            class="file-icon"
          />
        </div>
      </div>
      <div class="element-details">
        <ion-text class="element-details__name body">
          {{
            $msTranslate({
              key: getTextFromType(),
              data: { amount: amount },
            })
          }}
        </ion-text>
        <ion-button
          class="cancel-button"
          fill="clear"
          size="small"
          @click="$emit('cancel')"
        >
          <ion-icon
            :icon="closeCircle"
            slot="icon-only"
          />
        </ion-button>
      </div>
    </ion-item>
    <ion-progress-bar
      class="progress-bar"
      type="indeterminate"
    />
  </div>
</template>

<script setup lang="ts">
import { MsImage, Copy, FileImport, Move } from 'megashark-lib';
import { IonIcon, IonItem, IonProgressBar, IonText, IonButton } from '@ionic/vue';
import { closeCircle } from 'ionicons/icons';
import { FileOperationDataType } from '@/services/fileOperationManager';

const props = defineProps<{
  amount: number;
  type: FileOperationDataType;
}>();

defineExpose({
  props,
});

defineEmits<{
  (event: 'cancel'): void;
}>();

function getTextFromType(): string {
  switch (props.type) {
    case FileOperationDataType.Import:
      return 'FoldersPage.uploadsLeft';
    case FileOperationDataType.Copy:
      return 'FoldersPage.copiesLeft';
    case FileOperationDataType.Move:
      return 'FoldersPage.movesLeft';
    default:
      return 'FoldersPage.operationsLeft';
  }
}

function getImageFromType(): string {
  switch (props.type) {
    case FileOperationDataType.Copy:
      return Copy;
    case FileOperationDataType.Move:
      return Move;
    default:
      return FileImport;
  }
}
</script>

<style scoped lang="scss">
.element-container {
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  transition: background 0.2s ease-in-out;
  border-radius: var(--parsec-radius-8);
  position: relative;

  &:hover {
    background: var(--parsec-color-light-secondary-premiere);
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
    flex-direction: row;
    position: relative;
    margin-left: 0.875rem;

    &__name {
      color: var(--parsec-color-light-primary-700);
      margin-right: auto;
    }

    .progress-bar {
      --progress-background: var(--parsec-color-light-primary-500);
      --background: var(--parsec-color-light-secondary-medium);
      border-radius: 0 0 var(--parsec-radius-8) var(--parsec-radius-8);
      position: absolute;
      bottom: 0;
      width: 100%;
    }
  }

  .cancel-button {
    color: var(--parsec-color-light-secondary-grey);
    --background-hover: var(--parsec-color-light-secondary-disabled);
  }
}
</style>
