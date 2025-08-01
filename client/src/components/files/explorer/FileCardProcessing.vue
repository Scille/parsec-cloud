<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item class="file-card-item ion-no-padding">
    <div class="card-content">
      <ms-spinner class="card-content__spinner" />
      <ion-avatar class="card-content-icons">
        <ion-icon
          class="icon-item"
          :icon="documentIcon"
        />
      </ion-avatar>

      <ion-title class="card-content__title body">
        {{ fileName }}
      </ion-title>

      <ion-text class="card-content-last-update body-sm">
        <span>{{ $msTranslate(getFileOperationLabel()) }}</span>
      </ion-text>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { EntryName, Path } from '@/parsec';
import { CopyData, FileOperationData, FileOperationDataType, ImportData } from '@/services/fileOperationManager';
import { IonAvatar, IonIcon, IonItem, IonText, IonTitle } from '@ionic/vue';
import { document as documentIcon } from 'ionicons/icons';
import { MsSpinner, Translatable } from 'megashark-lib';
import { onMounted, ref, Ref } from 'vue';

const props = defineProps<{
  data: FileOperationData;
  progress: number;
}>();

const fileName: Ref<EntryName> = ref('');

onMounted(async () => {
  if (props.data.getDataType() === FileOperationDataType.Import) {
    fileName.value = (props.data as ImportData).file.name;
  } else if (props.data.getDataType() === FileOperationDataType.Copy) {
    fileName.value = (await Path.filename((props.data as CopyData).srcPath)) || '';
  } else if (props.data.getDataType() === FileOperationDataType.Move) {
    fileName.value = (await Path.filename((props.data as CopyData).dstPath)) || '';
  }
});

function getFileOperationLabel(): Translatable {
  if (props.data.getDataType() === FileOperationDataType.Copy) {
    return 'FoldersPage.File.copying';
  } else if (props.data.getDataType() === FileOperationDataType.Move) {
    return 'FoldersPage.File.moving';
  }
  return 'FoldersPage.File.importing';
}
</script>

<style lang="scss" scoped>
.file-card-item {
  position: relative;
  cursor: pointer;
  text-align: center;
  --background: var(--parsec-color-light-secondary-background);
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  user-select: none;
  border-radius: var(--parsec-radius-12);
  width: 10.5rem;

  &::part(native) {
    --inner-padding-end: 0px;
  }

  &:hover {
    --background: var(--parsec-color-light-primary-30);
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
  }

  &.selected {
    --background: var(--parsec-color-light-primary-100);
    --background-hover: var(--parsec-color-light-primary-100);
    border: 1px solid var(--parsec-color-light-primary-100);
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 1.5rem 0.5rem;
  width: 100%;
  margin: auto;

  &__spinner {
    position: absolute;
    top: 0.75rem;
    left: 0.75rem;
  }

  &-icons {
    position: relative;
    color: var(--parsec-color-light-primary-600);
    height: fit-content;
    width: fit-content;
    margin: 0 auto 0.875rem;

    .icon-item {
      font-size: 3rem;
    }
  }

  &__title {
    color: var(--parsec-color-light-primary-900);
    text-align: center;
    padding: 0 0 0.25rem;
    text-overflow: ellipsis;
    white-space: nowrap;
    width: inherit;

    ion-text {
      width: 100%;
      overflow: hidden;
    }
  }
}

.card-content-last-update {
  color: var(--parsec-color-light-secondary-grey);
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

/* No idea how to change the color of the ion-item */
.card-content__title::part(native),
.card-content-last-update::part(native) {
  background-color: var(--parsec-color-light-secondary-background);
}
</style>
