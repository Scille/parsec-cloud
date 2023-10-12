<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="$t('FileDetails.title', {name: entry.name})"
      :close-button-enabled="true"
    >
      <div class="file-info">
        <!-- Entry type -->
        <ion-item class="body-lg">
          <ion-label class="file-info-key">
            {{ $t('FileDetails.stats.type') }}
          </ion-label>
          <ion-text class="file-info-value">
            {{ entry.isFile() ? $t('FileDetails.stats.file') : $t('FileDetails.stats.folder') }}
          </ion-text>
        </ion-item>
        <!-- Path -->
        <ion-item class="body-lg">
          <ion-label class="file-info-key">
            {{ $t('FileDetails.stats.path') }}
          </ion-label>
          <ion-text class="file-info-value">
            {{ path }}
          </ion-text>
        </ion-item>
        <!-- Size (only for files) -->
        <ion-item
          class="body-lg"
          v-if="entry.isFile()"
        >
          <ion-label class="file-info-key">
            {{ $t('FileDetails.stats.size') }}
          </ion-label>
          <ion-text class="file-info-value">
            {{ fileSize((entry as EntryStatFile).size) }}
          </ion-text>
        </ion-item>
        <!-- Created -->
        <ion-item class="body-lg">
          <ion-label class="file-info-key">
            {{ $t('FileDetails.stats.created') }}
          </ion-label>
          <ion-text class="file-info-value">
            {{ $d(entry.created.toJSDate(), 'long') }}
          </ion-text>
        </ion-item>
        <!-- Last updated -->
        <ion-item class="body-lg">
          <ion-label class="file-info-key">
            {{ $t('FileDetails.stats.updated') }}
          </ion-label>
          <ion-text class="file-info-value">
            {{ $d(entry.updated.toJSDate(), 'long') }}
          </ion-text>
        </ion-item>
        <ion-item class="body-lg">
          <ion-label class="file-info-key">
            {{ $t('FileDetails.stats.version') }}
          </ion-label>
          <ion-text class="file-info-value">
            {{ entry.baseVersion }}
          </ion-text>
        </ion-item>
        <!-- Is synchronized -->
        <ion-item class="body-lg">
          <ion-label class="file-info-key">
            {{ $t('FileDetails.stats.isSynced') }}
          </ion-label>
          <ion-text class="file-info-value">
            {{ entry.needSync ? $t('FileDetails.stats.no') : $t('FileDetails.stats.yes') }}
          </ion-text>
        </ion-item>
        <!-- Id -->
        <ion-item class="body-lg">
          <ion-label class="file-info-key">
            {{ $t('FileDetails.stats.id') }}
          </ion-label>
          <ion-text class="file-info-value">
            {{ entry.id }}
          </ion-text>
        </ion-item>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonPage,
  IonLabel,
  IonItem,
  IonText,
} from '@ionic/vue';
import {
} from 'ionicons/icons';
import { inject } from 'vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { EntryStat, EntryStatFile } from '@/parsec';
import { FormattersKey, Formatters } from '@/common/injectionKeys';

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { fileSize } = inject(FormattersKey)! as Formatters;

defineProps<{
  entry: EntryStat
  path: string
}>();
</script>

<style scoped lang="scss">
.file-info {
  display: flex;
  flex-direction: column;
  padding-bottom: 1rem;
}
.file-info-key {
  max-width: 12rem;
  margin: 0;
  color: var(--parsec-color-light-secondary-grey);
}

.file-info-value {
  color: var(--parsec-color-light-primary-800);
}
</style>
