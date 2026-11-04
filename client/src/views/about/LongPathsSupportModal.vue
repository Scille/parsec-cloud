<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      <ion-text class="long-paths-modal__info">
        {{ $msTranslate('LongPathsSupportModal.labelInfo') }}
      </ion-text>
      <ion-text class="long-paths-modal__recommendation">
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
      class="long-paths-modal__checkbox"
      v-model="skipLongPathsSupportWarning"
    >
      {{ $msTranslate('LongPathsSupportModal.noReminder') }}
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
  gap: ms.spacing('gap-3xl');

  &__info {
    @include ms.font('body-md-regular');
    padding: ms.spacing('padding-lg') ms.spacing('padding-3xl');
    background-color: ms.color('surface-information-default-subtle');
    color: ms.color('text-information-default');
    border-radius: ms.radius('lg');
  }

  &__recommendation {
    @include ms.font('body-lg-regular');
    color: ms.color('text-base-body');
    border-radius: ms.radius('lg');

    .link {
      color: ms.color('text-brand-default');

      &:hover {
        text-decoration: underline;
        color: ms.color('text-brand-default-hover');
      }
    }
  }

  &__checkbox {
    position: absolute;
    z-index: 10;
    bottom: 2.25rem;
    color: ms.color('text-base-description');
    padding: ms.spacing('padding-sm');
    border-radius: ms.radius('lg');
    transition: background-color 0.2s ease-in-out;

    &:hover {
      color: ms.color('text-base-body');
      background-color: ms.color('surface-neutral-default-subtle-hover');
    }
  }
}
</style>
