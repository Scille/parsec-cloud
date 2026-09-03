<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-modal :close-button="{ visible: true }">
    <div class="shamir-link">
      <div class="shamir-link-header">
        <ms-image
          :image="theme === Theme.Dark ? ShamirIconCircleDark : ShamirIconCircleLight"
          class="shamir-link-header__image"
        />
        <ion-text class="shamir-link-header__title">
          {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.link.title') }}
        </ion-text>
        <ion-text class="shamir-link-header__subtitle">
          {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.link.subtitle') }}
        </ion-text>
      </div>

      <div class="shamir-link-content">
        <div class="shamir-link-form">
          <ms-input
            v-model="link"
            ref="linkInput"
            label="ImportRecoveryDevicePage.modal.shamir.link.inputLabel"
            :validator="shamirRecoveryLinkValidator"
            @on-enter-keyup="startClicked"
          />
          <ion-button
            class="shamir-link-form__button"
            :disabled="!canStart"
            @click="startClicked"
          >
            {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.link.start') }}
          </ion-button>
        </div>

        <div class="shamir-link-reminder">
          <ion-icon
            class="shamir-link-reminder__icon"
            :icon="informationCircle"
          />
          <div class="shamir-link-reminder__text">
            <ion-text class="shamir-link-reminder__title">
              {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.link.reminderTitle') }}
            </ion-text>
            <ion-text class="shamir-link-reminder__description">
              <ms-rich-text text="ImportRecoveryDevicePage.modal.shamir.link.reminderDescription" />
            </ion-text>
          </div>
        </div>
      </div>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import ShamirIconCircleDark from '@/assets/images/shamir-icon-circle-dark.svg?raw';
import ShamirIconCircleLight from '@/assets/images/shamir-icon-circle-light.svg?raw';
import { shamirRecoveryLinkValidator } from '@/common/validators';
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { informationCircle } from 'ionicons/icons';
import { asyncComputed, MsImage, MsInput, MsModal, MsModalResult, MsRichText, Theme, useTheme, Validity } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const { theme } = useTheme();
const link = ref('');
const linkInputRef = useTemplateRef('linkInput');

const canStart = asyncComputed(async () => {
  const result = await shamirRecoveryLinkValidator(link.value);
  return result.validity === Validity.Valid;
});

onMounted(async () => {
  await linkInputRef.value?.setFocus();
});

async function startClicked(): Promise<void> {
  await modalController.dismiss({ link: link.value }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.shamir-link {
  display: flex;
  flex-direction: column;
}

.shamir-link-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: ms.spacing('gap-lg');
  padding: ms.spacing('padding-5xl') ms.spacing('padding-6xl') ms.spacing('padding-3xl');
  background: ms.color('surface-base-default-secondary');
  text-align: center;

  &__image {
    width: 4.625rem;
    height: 4.625rem;
  }

  &__title {
    @include ms.font('heading-h4');
    color: ms.color('text-base-heading');
  }

  &__subtitle {
    @include ms.font('body-md-medium');
    color: ms.color('text-base-description');
  }
}

.shamir-link-content {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
  padding: ms.spacing('padding-4xl');
}

.shamir-link-form {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
}

.shamir-link-reminder {
  display: flex;
  align-items: flex-start;
  gap: ms.spacing('gap-xl');
  padding: ms.spacing('padding-2xl');
  border-radius: ms.radius('2xl');
  background: ms.color('surface-brand-default-subtle-hover');

  &__icon {
    flex-shrink: 0;
    width: 1.125rem;
    height: 1.125rem;
    color: ms.color('icon-brand-default');
  }

  &__text {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');
  }

  &__title {
    @include ms.font('heading-h6');
    color: ms.color('text-brand-default');
  }

  &__description {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-body');
  }
}
</style>
