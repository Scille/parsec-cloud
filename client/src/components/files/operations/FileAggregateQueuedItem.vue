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
        <div class="element-details__action">
          <ms-spinner />
          <ion-icon
            class="cancel-button"
            @click="$emit('cancel')"
            :icon="closeCircle"
          />
        </div>
      </div>
    </ion-item>
  </div>
</template>

<script setup lang="ts">
import { FileOperationDataType } from '@/services/fileOperationManager';
import { IonIcon, IonItem, IonText } from '@ionic/vue';
import { closeCircle } from 'ionicons/icons';
import { Copy, ImportMultipleFiles, Move, MsImage, MsSpinner } from 'megashark-lib';

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
      return 'FoldersPage.operationAmount';
  }
}

function getImageFromType(): string {
  switch (props.type) {
    case FileOperationDataType.Copy:
      return Copy;
    case FileOperationDataType.Move:
      return Move;
    default:
      return ImportMultipleFiles;
  }
}
</script>

<style scoped lang="scss">
.element-container {
  border-top: 1px solid var(--parsec-color-light-secondary-premiere);
  border-bottom: 1px solid var(--parsec-color-light-secondary-premiere);
  transition: background 0.2s ease-in-out;
  position: relative;
  overflow: hidden;
  flex-shrink: 0;
}

.element {
  --background: none;
  padding: 0.75rem 1rem 1rem;
  --inner-padding-end: 0;

  .file-icon {
    width: 2rem;
    height: 2rem;
  }

  &-details {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    margin-left: 0.875rem;

    &__name {
      color: var(--parsec-color-light-primary-700);
      margin-right: auto;
    }

    &__action {
      display: flex;
      gap: 0.5rem;
    }
  }

  .cancel-button {
    font-size: 1.15rem;
    color: var(--parsec-color-light-secondary-grey);
    padding: 0.25rem;
    border-radius: var(--parsec-radius-4);
    cursor: pointer;

    &:hover {
      background: var(--parsec-color-light-secondary-disabled);
    }
  }
}
</style>
