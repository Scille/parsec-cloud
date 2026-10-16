<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-storage">
    <div class="card-header">
      <ion-title class="card-header__title">
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
          <ion-label class="storage-list-item__title">
            {{ $msTranslate('OrganizationPage.size.total') }}
          </ion-label>
          <ion-text
            class="storage-list-item__value"
            slot="end"
            v-if="orgInfo.size"
          >
            {{ $msTranslate(formatFileSize(orgInfo.size.data + orgInfo.size.metadata)) }}
          </ion-text>
          <div
            v-else
            class="warning"
          >
            {{ $msTranslate('OrganizationPage.size.unavailable') }}
          </div>
        </div>
        <!-- Meta data -->
        <div class="storage-list-item">
          <ion-label class="storage-list-item__title">
            {{ $msTranslate('OrganizationPage.size.metadata') }}
          </ion-label>
          <ion-text
            class="storage-list-item__value"
            v-if="orgInfo.size"
          >
            {{ $msTranslate(formatFileSize(orgInfo.size.metadata)) }}
          </ion-text>
          <div
            v-else
            class="warning"
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
  background: ms.color('surface-base-default');
  align-items: center;
  gap: ms.spacing('gap-3xl');
  width: 100%;
  max-width: 30rem;
  border-radius: ms.radius('2xl');
  height: fit-content;
  box-shadow: var(--parsec-shadow-input);
  padding: ms.spacing('padding-4xl');

  .card-header {
    display: flex;
    align-items: center;
    width: 100%;

    &__title {
      @include ms.font('heading-h4');
      color: ms.color('text-base-body');
    }
  }

  .card-content {
    display: flex;
    flex-direction: column;
    border-radius: ms.radius('lg');
    width: 100%;
    gap: ms.spacing('gap-3xl');

    &__info {
      padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
    }
  }

  .storage-list {
    padding: ms.spacing('padding-none');
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');

    &-item {
      justify-content: space-between;
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-3xl');

      &__title {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
        flex-shrink: 0;
      }

      &__value {
        @include ms.font('heading-h6');
      }

      .warning {
        @include ms.font('body-sm-regular');
        color: ms.color('text-warning-default');
        padding: ms.spacing('padding-xs') ms.spacing('padding-lg');
        border-radius: ms.radius('6xl');
        background: ms.color('surface-warning-default-subtle-hover');
      }
    }
  }
}
</style>
