<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
      <ion-text class="shamir-others-list-empty__title subtitles-normal">
        {{ $msTranslate('OrganizationRecovery.shamir.modalOthers.noPeopleTitle') }}
      </ion-text>
      <ion-text class="shamir-others-list-empty__subtitle body">
        {{ $msTranslate('OrganizationRecovery.shamir.modalOthers.noPeopleSubtitle') }}
      </ion-text>
    </div>

    <div
      v-else
      class="shamir-others-list-container"
    >
      <ms-report-text class="shamir-others-info subtitles-normal">
        {{ $msTranslate('OrganizationRecovery.shamir.modalOthers.info') }}
      </ms-report-text>
      <div class="shamir-others-list">
        <ion-item
          v-for="info in shamirInfo"
          :key="info.userId"
          class="shamir-others-list-item"
        >
          <ion-text class="shamir-others-list-item__text subtitles-sm">
            <span class="shamir-others-list-item__text-label subtitles-sm">
              {{ info.owner.humanHandle.label }}
            </span>
            <span class="shamir-others-list-item__text-description body">
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
  gap: 1rem;
}

.shamir-others-list-container {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.shamir-others-list-empty {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  gap: 0.5rem;
  padding: 1.625rem;
  background-color: var(--parsec-color-light-secondary-background);
  border-radius: var(--parsec-radius-12);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  min-height: 13rem;
  max-height: 13rem;

  &__icon {
    font-size: 1.5rem;
    color: var(--parsec-color-light-secondary-grey);
  }

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &__subtitle {
    color: var(--parsec-color-light-secondary-hard-grey);
    text-align: center;
  }
}

.shamir-others-list {
  display: flex;
  flex-direction: column;
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  border-radius: var(--parsec-radius-8);
  background: var(--parsec-color-light-secondary-background);
  width: 100%;
  min-height: 15rem;
  max-height: 15rem;
  overflow-y: auto;
}

.shamir-others-list-item {
  --background: var(--parsec-color-light-secondary-white);

  &::part(native) {
    padding: 0.625rem 1rem;
  }

  &::part(container) {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    overflow: hidden;
    justify-content: space-between;
  }

  &__text {
    margin: 0;
    display: flex;
    align-items: center;
    gap: 0.375rem;
    width: 100%;
    overflow: hidden;

    &-label {
      color: var(--parsec-color-light-secondary-text);
      flex-shrink: 0;
    }

    &-description {
      color: var(--parsec-color-light-secondary-grey);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  &:hover {
    background: var(--parsec-color-light-secondary-inversed-contrast);
  }
}

.shamir-others-list-item-buttons {
  display: flex;
  gap: 0.5rem;
  flex-shrink: 0;
}
</style>
