<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    lines="full"
    class="ion-no-padding file-list-item-processing"
  >
    <div class="list-item-container">
      <div class="file-loading">
        <ms-spinner class="file-loading__spinner" />
      </div>

      <!-- file name -->
      <div class="file-name">
        <ms-image
          v-if="isLargeDisplay"
          :image="getFileIcon(operation.entryName)"
          class="file-icon"
        />

        <ion-text class="label-name cell">
          {{ operation.entryName }}
        </ion-text>
      </div>

      <!-- updated by -->
      <div class="file-updated-by" />

      <!-- last update -->
      <div
        v-if="profile !== UserProfile.Outsider"
        class="file-last-update"
      >
        <ion-text class="label-last-update cell" />
      </div>

      <!-- creation date -->
      <div class="file-creation-date">
        <ion-text class="label-creation-date cell">
          {{ $msTranslate(operationLabel) }}
        </ion-text>
      </div>

      <!-- file size -->
      <div class="file-size" />

      <!-- options -->
      <div class="file-empty ion-item-child-clickable" />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { getFileIcon } from '@/common/file';
import { FileOperationCurrentFolder } from '@/components/files/types';
import { UserProfile } from '@/parsec';
import { FileOperationDataType } from '@/services/fileOperation';
import { IonItem, IonText } from '@ionic/vue';
import { MsImage, MsSpinner, Translatable, useWindowSize } from 'megashark-lib';

const props = defineProps<{
  operation: FileOperationCurrentFolder;
  profile: UserProfile;
}>();

const { isLargeDisplay } = useWindowSize();
const operationLabel: Translatable = (() => {
  if (props.operation.type === FileOperationDataType.Copy) {
    return 'FoldersPage.File.copying';
  } else if (props.operation.type === FileOperationDataType.Move) {
    return 'FoldersPage.File.moving';
  } else if (props.operation.type === FileOperationDataType.Restore) {
    return 'FoldersPage.File.restoring';
  }
  return 'FoldersPage.File.importing';
})();
</script>

<style lang="scss" scoped>
.file-loading {
  @include ms.responsive-breakpoint('sm') {
    max-width: 3rem;
    min-width: 3rem;
  }

  &__spinner {
    width: 1.25rem;
    height: 1.25rem;
  }
}

.file-name {
  position: relative;
  display: flex;
  gap: 1rem;

  .file-icon {
    min-width: 2rem;
    height: 2rem;
  }

  .label-name {
    color: var(--parsec-color-light-secondary-text);
  }
}
</style>
