<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div id="shamir-others">
    <div
      v-if="shamirInfo.length === 0"
      class="shamir-others-list-empty"
    >
      <ion-icon
        :icon="people"
        class="shamir-others-list-empty__icon"
      />
      <ion-text class="shamir-others-list-empty__title">
        {{ $msTranslate('OrganizationRecovery.shamir.modalOthers.noPeopleTitle') }}
      </ion-text>
      <ion-text class="shamir-others-list-empty__subtitle">
        {{ $msTranslate('OrganizationRecovery.shamir.modalOthers.noPeopleSubtitle') }}
      </ion-text>
    </div>

    <div
      v-else
      class="shamir-others-list-container"
    >
      <ms-report-text class="shamir-others-info">
        {{ $msTranslate('OrganizationRecovery.shamir.modalOthers.info') }}
      </ms-report-text>
      <div class="shamir-others-list">
        <ion-item
          v-for="info in shamirInfo"
          :key="info.userId"
          class="shamir-others-list-item"
        >
          <ion-text class="shamir-others-list-item__text">
            <span class="shamir-others-list-item__text-label">
              {{ info.owner.humanHandle.label }}
            </span>
            <span class="shamir-others-list-item__text-description">
              {{ info.owner.humanHandle.email }}
            </span>
          </ion-text>
          <div class="shamir-others-list-item-buttons">
            <ion-button
              fill="clear"
              size="small"
              class="shamir-others-list-item-buttons__button"
              @click="$emit('copyLink', info)"
            >
              {{ $msTranslate('OrganizationRecovery.shamir.modalOthers.copyLink') }}
            </ion-button>
            <ion-button
              class="shamir-others-list-item-buttons__button"
              @click="$emit('startRecovery', info)"
            >
              {{ $msTranslate('OrganizationRecovery.shamir.modalOthers.startRecovery') }}
            </ion-button>
          </div>
        </ion-item>
      </div>
    </div>

    <div v-if="error">
      <ms-report-text :theme="MsReportTheme.Error">
        {{ $msTranslate(error) }}
      </ms-report-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { OtherShamirRecoveryInfo, getOthersShamirRecovery } from '@/parsec';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { people } from 'ionicons/icons';
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const shamirInfo = ref<Array<OtherShamirRecoveryInfo>>([]);
const error = ref('');

defineEmits<{
  (e: 'copyLink', info: OtherShamirRecoveryInfo): void;
  (e: 'startRecovery', info: OtherShamirRecoveryInfo): void;
}>();

onMounted(async () => {
  await refreshShamirStatus();
});

async function refreshShamirStatus(): Promise<void> {
  const result = await getOthersShamirRecovery();

  if (!result.ok) {
    error.value = 'OrganizationRecovery.shamir.errors.generic';
  } else {
    shamirInfo.value = result.value;
  }
}
</script>

<style scoped lang="scss">
#shamir-others {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
}

.shamir-others-list-container {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
}

.shamir-others-info {
  @include ms.font('body-lg-medium');
}

.shamir-others-list-empty {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: ms.spacing('gap-lg');
  padding: 1.625rem;
  background-color: ms.color('surface-base-default-secondary');
  border-radius: ms.radius('2xl');
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  min-height: 13rem;
  max-height: 13rem;

  &__icon {
    font-size: 1.5rem;
    color: ms.color('icon-neutral-default');
  }

  &__title {
    @include ms.font('body-lg-medium');
    color: ms.color('text-base-body');
  }

  &__subtitle {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-description');
    text-align: center;
  }
}

.shamir-others-list {
  display: flex;
  flex-direction: column;
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  border-radius: ms.radius('lg');
  background: ms.color('surface-base-default-secondary');
  width: 100%;
  min-height: 15rem;
  max-height: 15rem;
  overflow-y: auto;
}

.shamir-others-list-item {
  --background: #{ms.color('surface-base-default')};

  &::part(native) {
    padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
  }

  &::part(container) {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    overflow: hidden;
    justify-content: space-between;
  }

  &__text {
    @include ms.font('body-md-medium');
    margin: 0;
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-md');
    width: 100%;
    overflow: hidden;

    &-label {
      @include ms.font('body-md-medium');
      color: ms.color('text-base-body');
      flex-shrink: 0;
    }

    &-description {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  &:hover {
    background: ms.color('surface-base-default');
  }
}

.shamir-others-list-item-buttons {
  display: flex;
  gap: ms.spacing('gap-lg');
  flex-shrink: 0;
}
</style>
