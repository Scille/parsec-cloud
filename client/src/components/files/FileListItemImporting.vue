<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    lines="full"
  >
    <div class="file-list-item">
      <div class="file-loading">
        <vue3-lottie
          :animation-data="SpinnerJSON"
          :height="24"
          :width="24"
          :loop="true"
        />
      </div>
      <!-- file name -->
      <div class="file-name">
        <ms-image
          :image="getFileIcon(data.file.name)"
          class="file-icon"
        />
        <ion-label class="file-name__label cell">
          {{ data.file.name }}
        </ion-label>
        <ion-icon
          class="cloud-overlay"
          :icon="cloudOffline"
        />
      </div>

      <!-- updated by -->
      <!-- Can't get the information right now, maybe later -->
      <div
        class="file-updatedBy"
        v-if="clientInfo"
      >
        <user-avatar-name
          :user-avatar="clientInfo.humanHandle.label"
          :user-name="clientInfo.humanHandle.label"
        />
      </div>

      <!-- last update -->
      <div class="file-lastUpdate">
        <ion-label class="label-last-update cell">
          {{ $t('FoldersPage.File.importing') }}
        </ion-label>
      </div>

      <!-- file size -->
      <div class="file-size">
        <ion-label class="label-size cell">
          {{ formatFileSize(data.file.size) }}
        </ion-label>
      </div>

      <!-- options -->
      <div class="file-empty ion-item-child-clickable" />
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import SpinnerJSON from '@/assets/spinner.json';
import { formatFileSize, getFileIcon } from '@/common/file';
import { MsImage } from '@/components/core/ms-image';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { ClientInfo, getClientInfo } from '@/parsec';
import { ImportData } from '@/services/importManager';
import { IonIcon, IonItem, IonLabel } from '@ionic/vue';
import { cloudOffline } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';
import { Vue3Lottie } from 'vue3-lottie';

defineProps<{
  data: ImportData;
  progress: number;
}>();

const clientInfo: Ref<ClientInfo | null> = ref(null);

onMounted(async () => {
  const result = await getClientInfo();
  if (result.ok) {
    clientInfo.value = result.value;
  }
});
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

  .cloud-overlay {
    font-size: 1rem;
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
