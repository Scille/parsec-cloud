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
        <ion-text class="certificate-card-header__name">{{ certificate.getName() }}</ion-text>
      </div>

      <div class="certificate-card-content">
        <div class="card-expire">
          <ion-icon
            :icon="calendar"
            class="card-expire__icon"
          />
          <ion-text class="card-expire__text">
            {{ $msTranslate(I18n.formatDate(primaryDetails.notBefore, 'short')) }}
          </ion-text>
          <ion-icon
            :icon="arrowForward"
            class="card-expire__arrow"
          />
          <ion-text class="card-expire__text">
            {{ $msTranslate(I18n.formatDate(primaryDetails.notAfter, 'short')) }}
          </ion-text>
          <ion-text
            v-if="certificate.isExpired()"
            class="card-expire__text expired"
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
            v-if="primaryDetails.emails.length === 0"
            class="card-email__text"
          >
            {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.certificate.noEmail') }}
          </ion-text>
          <ion-text
            v-if="primaryDetails.emails.length > 0"
            class="card-email__text"
          >
            {{ primaryDetails.emails[0] }}
          </ion-text>
          <ion-text
            v-if="primaryDetails.emails.length > 1"
            class="card-email__text additional-emails"
            @click.stop="openAdditionalEmailPopover"
          >
            + {{ primaryDetails.emails.length - 1 }}
          </ion-text>
        </div>

        <div class="card-id">
          <ion-text class="card-id__text">
            {{
              $msTranslate({
                key: 'HomePage.organizationRequest.asyncEnrollmentModal.certificate.serialNb',
                data: { serial: certificate.getSerial() },
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
import { CertificateWithDetailsValid } from '@/parsec';
import { IonIcon, IonText, popoverController } from '@ionic/vue';
import { arrowForward, calendar, checkmarkCircle, mail } from 'ionicons/icons';
import { I18n, MsImage } from 'megashark-lib';
import { computed } from 'vue';

const props = defineProps<{
  certificate: CertificateWithDetailsValid;
  isSelected: boolean;
}>();

defineEmits<{
  (e: 'clicked', certificate: CertificateWithDetailsValid): void;
}>();

const primaryDetails = computed(() => {
  return (props.certificate.signCert ?? props.certificate.encryptCert)!.details;
});

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
      emails: primaryDetails.value.emails,
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
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  background: ms.color('surface-base-default-secondary');
  border-radius: ms.radius('2xl');
  display: flex;
  padding: ms.spacing('padding-3xl');
  gap: ms.spacing('gap-lg');
  cursor: pointer;
  width: 100%;

  &-container {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-2xl');
    flex-grow: 1;
  }

  &-header {
    display: flex;
    gap: ms.spacing('gap-lg');
    align-items: center;

    &__icon {
      max-width: 1.25rem;
      max-height: 1.25rem;
    }

    &__name {
      font-weight: bold;
      color: ms.color('text-base-body');
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-md');

    .card-expire,
    .card-email,
    .card-id {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-sm');
      color: ms.color('text-neutral-default');

      ion-text {
        @include ms.font('body-md-medium');
      }

      &__icon {
        color: ms.color('icon-disabled-default');
      }

      &__arrow {
        color: ms.color('icon-disabled-default');
      }

      .additional-emails {
        padding: ms.spacing('padding-sm');
        background: ms.color('surface-neutral-default-subtle-pressed');
        border-radius: ms.radius('md');
        margin-left: 0.25rem;
        font-size: 0.75rem;

        &:hover {
          background: ms.color('surface-neutral-default');
          border-color: ms.color('border-base-on-color');
          color: ms.color('text-on-color-heading');
        }
      }
    }
  }

  &:hover {
    background: ms.color('surface-base-default-secondary');
  }
}

.selected-checkmark {
  color: ms.color('icon-brand-default');
  font-size: 1.25rem;
}

.disabled {
  pointer-events: none;

  .certificate-card-container {
    opacity: ms.opacity('5');
  }

  .expired {
    color: ms.color('text-error-default-hover');
    background: ms.color('surface-error-default-subtle-pressed');
    padding: ms.spacing('padding-xs') ms.spacing('padding-sm');
    border-radius: ms.radius('sm');
  }
}

.isSelected {
  background: ms.color('surface-brand-default-subtle-hover');
  box-shadow: var(--parsec-shadow-input);
  border: ms.border('thin') solid ms.color('border-brand-default');

  .certificate-card-header__name {
    color: ms.color('text-brand-default');
  }

  &:hover {
    background: ms.color('surface-brand-default-subtle-hover');
    cursor: default;

    .additional-emails {
      cursor: pointer;
    }
  }
}
</style>
