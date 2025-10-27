<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="LongPathsSupportModal.title"
    :close-button="{ visible: false }"
    :confirm-button="{
      label: 'LongPathsSupportModal.closeButton',
      onClick: dismiss,
      disabled: false,
    }"
  >
    <div class="long-paths-modal">
      <ion-text class="long-paths-modal__info body">
        {{ $msTranslate('LongPathsSupportModal.labelInfo') }}
      </ion-text>
      <ion-text class="long-paths-modal__recommendation body-lg">
        <i18n-t
          keypath="LongPathsSupportModal.labelFix"
          scope="global"
        >
          <template #link>
            <a
              :href="$msTranslate('LongPathsSupportModal.fixUrl')"
              target="_blank"
              class="link"
            >
              {{ $msTranslate('LongPathsSupportModal.linkLabel') }}
            </a>
          </template>
        </i18n-t>
      </ion-text>
    </div>
    <ms-checkbox
      class="long-paths-modal__checkbox body"
      v-model="skipLongPathsSupportWarning"
    >
      <ion-text>
        {{ $msTranslate('LongPathsSupportModal.noReminder') }}
      </ion-text>
    </ms-checkbox>
  </ms-modal>
</template>

<script setup lang="ts">
import { IonText, modalController } from '@ionic/vue';
import { MsCheckbox, MsModal, MsModalResult } from 'megashark-lib';
import { ref } from 'vue';

const skipLongPathsSupportWarning = ref(false);

async function dismiss(): Promise<boolean> {
  return await modalController.dismiss({ skipLongPathsSupportWarning: skipLongPathsSupportWarning.value }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.long-paths-modal {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &__info {
    padding: 0.5rem 1rem;
    background-color: var(--parsec-color-light-info-50);
    color: var(--parsec-color-light-info-700);
    border-radius: var(--parsec-radius-8);
  }

  &__recommendation {
    color: var(--parsec-color-light-secondary-text);
    border-radius: var(--parsec-radius-8);

    .link {
      color: var(--parsec-color-light-primary-500);

      &:hover {
        text-decoration: underline;
        color: var(--parsec-color-light-primary-600);
      }
    }
  }

  &__checkbox {
    position: absolute;
    bottom: 2.25rem;
    color: var(--parsec-color-light-secondary-soft-text);
    padding: 0.25rem 0.25rem;
    border-radius: var(--parsec-radius-8);
    transition: background-color 0.2s ease-in-out;

    &:hover {
      color: var(--parsec-color-light-secondary-text);
      background-color: var(--parsec-color-light-secondary-premiere);
    }
  }
}
</style>
