<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="certificate-card"
    :class="{
      selectable: !certificate.isExpired(),
      disabled: certificate.isExpired(),
      isSelected: isSelected,
    }"
    @click="$emit('clicked', certificate)"
  >
    <div class="certificate-card-container">
      <div class="certificate-card-header">
        <ms-image
          :image="CertificateIcon"
          alt="Certificate Icon"
          class="certificate-card-header__icon"
        />
        <ion-text class="certificate-card-header__name">{{ certificate.name }}</ion-text>
      </div>

      <div class="certificate-card-content">
        <div class="card-expire">
          <ion-icon
            :icon="calendar"
            class="card-expire__icon"
          />
          <ion-text class="subtitles-sm card-expire__text">{{ $msTranslate(I18n.formatDate(certificate.createdOn, 'short')) }}</ion-text>
          <ion-icon
            :icon="arrowForward"
            class="card-expire__arrow"
          />
          <ion-text class="subtitles-sm card-expire__text">{{ $msTranslate(I18n.formatDate(certificate.expireOn, 'short')) }}</ion-text>
          <ion-text
            v-if="certificate.isExpired()"
            class="subtitles-sm card-expire__text expired"
          >
            {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.certificate.expired') }}
          </ion-text>
        </div>

        <div class="card-email">
          <ion-icon
            :icon="mail"
            class="card-email__icon"
          />
          <ion-text
            v-if="certificate.emails.length === 0"
            class="subtitles-sm"
          >
            {{ $msTranslate('No email') }}
          </ion-text>
          <ion-text
            v-if="certificate.emails.length > 0"
            class="subtitles-sm card-email__text"
          >
            {{ certificate.emails[0] }}
          </ion-text>
          <ion-text
            v-if="certificate.emails.length > 1"
            class="subtitles-sm card-email__text additional-emails"
            @click.stop="openAdditionalEmailPopover"
          >
            + {{ certificate.emails.length - 1 }}
          </ion-text>
        </div>

        <div class="card-id">
          <ion-text class="subtitles-sm card-id__text">
            {{
              $msTranslate({
                key: 'HomePage.organizationRequest.asyncEnrollmentModal.certificate.serialNb',
                data: { serial: certificate.id },
              })
            }}
          </ion-text>
        </div>
      </div>
    </div>

    <div
      v-if="isSelected"
      class="selected-checkmark"
    >
      <ion-icon :icon="checkmarkCircle" />
    </div>
  </div>
</template>

<script setup lang="ts">
import CertificateIcon from '@/assets/images/certificate-icon.svg?raw';
import AdditionalEmailsPopover from '@/components/misc/AdditionalEmailsPopover.vue';
import { Certificate } from '@/parsec';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { arrowForward, calendar, checkmarkCircle, mail } from 'ionicons/icons';
import { I18n, MsImage } from 'megashark-lib';

const props = defineProps<{
  certificate: Certificate;
  isSelected: boolean;
}>();

defineEmits<{
  (e: 'clicked', certificate: Certificate): void;
}>();

async function openAdditionalEmailPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const popover = await popoverController.create({
    component: AdditionalEmailsPopover,
    alignment: 'center',
    event: event,
    cssClass: 'additional-emails-popover',
    showBackdrop: false,
    backdropDismiss: true,
    componentProps: {
      emails: props.certificate.emails,
    },
  });
  await popover.present();
  const { role } = await popover.onDidDismiss();
  if (role !== 'backdrop') {
    await popover.dismiss();
  }
}
</script>

<style scoped lang="scss">
.certificate-card {
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12);
  display: flex;
  padding: 1rem;
  gap: 0.5rem;
  cursor: pointer;

  &-container {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    flex-grow: 1;
  }

  &-header {
    display: flex;
    gap: 0.5rem;
    align-items: center;

    &__icon {
      max-width: 1.25rem;
      max-height: 1.25rem;
    }

    &__name {
      font-weight: bold;
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;

    .card-expire,
    .card-email,
    .card-id {
      display: flex;
      align-items: center;
      gap: 0.25rem;
      color: var(--parsec-color-light-secondary-text);

      &__icon {
        color: var(--parsec-color-light-secondary-light);
      }

      &__arrow {
        color: var(--parsec-color-light-secondary-grey);
      }

      .additional-emails {
        padding: 0.25rem;
        background: var(--parsec-color-light-secondary-medium);
        border-radius: var(--parsec-radius-6);
        margin-left: 0.25rem;
        font-size: 0.75rem;

        &:hover {
          background: var(--parsec-color-light-secondary-text);
          border-color: var(--parsec-color-light-secondary-white);
          color: var(--parsec-color-light-secondary-white);
        }
      }
    }
  }

  &:hover {
    background: var(--parsec-color-light-secondary-premiere);
  }
}

.isSelected {
  border: 1px solid var(--parsec-color-light-secondary-medium);
}

.selected-checkmark {
  color: var(--parsec-color-light-primary-600);
  font-size: 1.25rem;
}

.disabled {
  pointer-events: none;

  .certificate-card-container {
    opacity: 0.5;
  }

  .expired {
    color: var(--parsec-color-light-danger-700);
    background: var(--parsec-color-light-danger-100);
    padding: 0.125rem 0.25rem;
    border-radius: var(--parsec-radius-4);
  }
}

.isSelected {
  background: var(--parsec-color-light-secondary-premiere);
  box-shadow: var(--parsec-shadow-input);
  border: 1px solid var(--parsec-color-light-primary-600);

  &:hover {
    background: var(--parsec-color-light-secondary-premiere);
    cursor: default;

    .additional-emails {
      cursor: pointer;
    }
  }
}
</style>
