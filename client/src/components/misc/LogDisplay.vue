<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="log-display-modal-container">
    <div
      class="logs"
      v-if="!loading"
    >
      <textarea
        v-show="logs.length > 0"
        class="log-area"
        readonly
        :value="logs"
      />
      <ms-report-text
        v-show="logs.length === 0"
        :theme="MsReportTheme.Info"
      >
        {{ $msTranslate('LogDisplayModal.noLog') }}
      </ms-report-text>
    </div>
    <div
      class="loading"
      v-show="loading"
    >
      <ion-skeleton-text
        :animated="true"
        class="skeleton"
      />
      <ion-skeleton-text
        :animated="true"
        class="skeleton"
      />
      <ion-skeleton-text
        :animated="true"
        class="skeleton"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { getLogs } from '@/components/misc/utils';
import { IonSkeletonText } from '@ionic/vue';
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const logs = ref<string>('');
const loading = ref(true);

onMounted(async () => {
  loading.value = true;
  logs.value = await getLogs();
  loading.value = false;
});
</script>

<style scoped lang="scss">
.log-display-modal-container {
  overflow-y: auto;
}

.log-entry {
  border-radius: var(--parsec-radius-8);
  background: none;

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.25em;

    &__message {
      color: var(--parsec-color-light-primary-text);
    }

    &__timestamp {
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  &:nth-child(odd) {
    background-color: var(--parsec-color-light-secondary-background);
  }
}

.skeleton {
  height: 70px;
}
</style>
