<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    lines="full"
  >
    <div class="file-list-item">
      <div class="file-loading">
        <ms-spinner />
      </div>
      <!-- file name -->
      <div class="file-name">
        <ms-image
          :image="getFileIcon(fileName)"
          class="file-icon"
        />

        <ion-label class="file-name__label cell">
          {{ fileName }}
        </ion-label>
      </div>

      <!-- updated by -->
      <!-- Can't get the information right now, maybe later -->
      <div
        class="file-updatedBy"
        v-if="clientInfo"
        v-show="false"
      >
        <user-avatar-name
          :user-avatar="clientInfo.humanHandle.label"
          :user-name="clientInfo.humanHandle.label"
        />
      </div>

      <!-- last update -->
      <div class="file-lastUpdate">
        <ion-label class="label-last-update cell">
          {{ $msTranslate(getFileOperationLabel()) }}
        </ion-label>
      </div>

      <!-- file size -->
      <div
        class="file-size"
        v-show="data.getDataType() === FileOperationDataType.Import"
      >
        <ion-label class="label-size cell">
          {{ $msTranslate(formatFileSize((data as ImportData).file.size)) }}
        </ion-label>
      </div>

      <!-- options -->
      <div class="file-empty ion-item-child-clickable" />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { formatFileSize, getFileIcon } from '@/common/file';
import { MsImage, Translatable } from 'megashark-lib';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { ClientInfo, EntryName, getClientInfo, Path } from '@/parsec';
import { CopyData, FileOperationData, FileOperationDataType, ImportData } from '@/services/fileOperationManager';
import { IonItem, IonLabel } from '@ionic/vue';
import { Ref, onMounted, ref } from 'vue';
import { MsSpinner } from 'megashark-lib';

const props = defineProps<{
  data: FileOperationData;
  progress: number;
}>();

const clientInfo: Ref<ClientInfo | null> = ref(null);
const fileName: Ref<EntryName> = ref('');

onMounted(async () => {
  const result = await getClientInfo();
  if (result.ok) {
    clientInfo.value = result.value;
  }
  if (props.data.getDataType() === FileOperationDataType.Import) {
    fileName.value = (props.data as ImportData).file.name;
  } else if (props.data.getDataType() === FileOperationDataType.Copy) {
    fileName.value = (await Path.filename((props.data as CopyData).dstPath)) || '';
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
.file-name {
  .file-icon {
    width: 2rem;
    height: 2rem;
  }

  &__label {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1em;
  }
}
</style>
