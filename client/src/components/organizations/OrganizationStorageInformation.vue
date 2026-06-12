<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-storage">
    <div class="card-header">
      <ion-title class="card-header__title title-h3">
        {{ $msTranslate('OrganizationPage.size.title') }}
      </ion-title>
    </div>
    <div class="card-content">
      <ms-report-text
        :theme="MsReportTheme.Info"
        class="card-content__info"
      >
        {{ $msTranslate('OrganizationPage.size.information') }}
      </ms-report-text>
      <div class="storage-list">
        <div class="storage-list-item">
          <ion-label class="storage-list-item__title body">
            {{ $msTranslate('OrganizationPage.size.total') }}
          </ion-label>
          <ion-text
            class="storage-list-item__value title-h5"
            slot="end"
            v-if="orgInfo.size"
          >
            {{ $msTranslate(formatFileSize(orgInfo.size.data + orgInfo.size.metadata)) }}
          </ion-text>
          <div
            v-else
            class="warning body-sm"
          >
            {{ $msTranslate('OrganizationPage.size.unavailable') }}
          </div>
        </div>
        <!-- Meta data -->
        <div class="storage-list-item">
          <ion-label class="storage-list-item__title body">
            {{ $msTranslate('OrganizationPage.size.metadata') }}
          </ion-label>
          <ion-text
            class="storage-list-item__value title-h5"
            v-if="orgInfo.size"
          >
            {{ $msTranslate(formatFileSize(orgInfo.size.metadata)) }}
          </ion-text>
          <div
            v-else
            class="warning body-sm"
          >
            {{ $msTranslate('OrganizationPage.size.unavailable') }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import { OrganizationInfo } from '@/parsec';
import { IonLabel, IonText, IonTitle } from '@ionic/vue';
import { MsReportText, MsReportTheme } from 'megashark-lib';

defineProps<{
  orgInfo: OrganizationInfo;
}>();
</script>

<style scoped lang="scss">
.organization-storage {
  display: flex;
  flex-direction: column;
  background: var(--parsec-color-light-secondary-white);
  align-items: center;
  gap: 1rem;
  width: 100%;
  max-width: 30rem;
  border-radius: var(--parsec-radius-18);
  height: fit-content;
  box-shadow: var(--parsec-shadow-input);
  padding: 1.5rem;

  .card-header {
    display: flex;
    align-items: center;
    width: 100%;

    &__title {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  .card-content {
    display: flex;
    flex-direction: column;
    border-radius: var(--parsec-radius-8);
    width: 100%;
    gap: 1rem;

    &__info {
      padding: 0.5rem 0.75rem;
    }
  }

  .storage-list {
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;

    &-item {
      justify-content: space-between;
      display: flex;
      align-items: center;
      gap: 1rem;

      &__title {
        color: var(--parsec-color-light-secondary-hard-grey);
        flex-shrink: 0;
      }

      .warning {
        color: var(--parsec-color-light-warning-700);
        padding: 0.125rem 0.5rem;
        border-radius: var(--parsec-radius-32);
        background: var(--parsec-color-light-warning-50);
      }
    }
  }
}
</style>
