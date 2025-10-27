<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    lines="full"
    class="ion-no-padding file-list-item"
  >
    <div class="list-item-container">
      <div class="file-loading">
        <ms-spinner class="file-loading__spinner" />
      </div>

      <!-- file name -->
      <div class="file-name">
        <ms-image
          v-if="isLargeDisplay"
          :image="getFileIcon(fileName)"
          class="file-icon"
        />

        <ion-text class="label-name cell">
          {{ fileName }}
        </ion-text>
      </div>

      <!-- updated by -->
      <div
        class="file-updated-by"
        v-if="clientInfo && isLargeDisplay"
      >
        <user-avatar-name
          :user-avatar="clientInfo.humanHandle.label"
          :user-name="clientInfo.humanHandle.label"
        />
      </div>

      <!-- last update -->
      <div
        v-if="clientInfo?.currentProfile !== UserProfile.Outsider"
        class="file-last-update"
      >
        <ion-text class="label-last-update cell" />
      </div>

      <!-- creation date -->
      <div class="file-creation-date">
        <ion-text class="label-creation-date cell">
          {{ $msTranslate(getFileOperationLabel()) }}
        </ion-text>
      </div>

      <!-- file size -->
      <div
        class="file-size"
        v-if="data.getDataType() === FileOperationDataType.Import && isLargeDisplay"
      >
        <ion-text class="label-size cell">
          {{ $msTranslate(formatFileSize((data as ImportData).file.size)) }}
        </ion-text>
      </div>

      <!-- options -->
      <div class="file-empty ion-item-child-clickable" />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon } from '@/common/file';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { ClientInfo, EntryName, getClientInfo, Path, UserProfile } from '@/parsec';
import { CopyData, FileOperationData, FileOperationDataType, ImportData } from '@/services/fileOperationManager';
import { IonItem, IonText } from '@ionic/vue';
import { MsImage, MsSpinner, Translatable, useWindowSize } from 'megashark-lib';
import { onMounted, Ref, ref } from 'vue';

const props = defineProps<{
  data: FileOperationData;
  progress: number;
}>();

const clientInfo: Ref<ClientInfo | null> = ref(null);
const fileName: Ref<EntryName> = ref('');
const { isLargeDisplay } = useWindowSize();

onMounted(async () => {
  const result = await getClientInfo();
  if (result.ok) {
    clientInfo.value = result.value;
  }
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
