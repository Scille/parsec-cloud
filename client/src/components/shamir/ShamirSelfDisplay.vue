<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div id="shamir-self-display">
    <!-- eslint-disable vue/html-indent -->
    <div
      v-if="
        shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupAllValid ||
        shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupWithRevokedRecipients
      "
      class="shamir-done"
    >
      <ms-image
        :image="isDarkTheme ? ShamirIconCircleDark : ShamirIconCircleLight"
        class="shamir-done-image"
        alt="Shamir Icon"
      />

      <div class="shamir-done-text">
        <ion-text class="shamir-done-text__title">
          {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.setupDone') }}
          <ion-icon
            :icon="checkmarkCircle"
            class="title-icon"
          />
        </ion-text>
        <ion-text class="shamir-done-text__description">
          {{
            $msTranslate({
              key: 'OrganizationRecovery.shamir.modalSelf.numberPeople',
              data: { count: shamirInfo.recipients.length },
            })
          }}
        </ion-text>
      </div>

      <ms-report-text
        v-show="shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupWithRevokedRecipients"
        :theme="MsReportTheme.Warning"
      >
        {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.warningRevokedRecipients') }}
      </ms-report-text>

      <ion-list class="shamir-done-list">
        <ion-item
          v-for="recipient in shamirInfo.recipients"
          :key="recipient.id"
          class="shamir-done-list-item"
        >
          <ion-icon
            :icon="shieldCheckmark"
            class="shamir-done-list-item__icon"
          />
          <ion-text
            class="shamir-done-list-item__text"
            v-if="recipient.isAnonymous()"
          >
            {{ $msTranslate('UsersPage.anonymous') }}
          </ion-text>
          <ion-text
            class="shamir-done-list-item__text"
            v-else
          >
            <span class="shamir-done-list-item__text-label">
              {{ recipient.humanHandle.label }}
            </span>
            <span class="shamir-done-list-item__text-description">
              {{ recipient.humanHandle.email }}
            </span>
          </ion-text>
          <ion-text
            v-if="recipient.isRevoked()"
            class="shamir-done-list-item__revoked"
          >
            {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.revoked') }}
          </ion-text>
        </ion-item>
      </ion-list>

      <ion-text class="shamir-done-info">
        <ion-icon
          :icon="informationCircle"
          class="shamir-done-info__icon"
        />
        {{
          $msTranslate({
            key: 'OrganizationRecovery.shamir.modalSelf.necessaryPeople',
            data: { threshold: shamirInfo.threshold },
          })
        }}
      </ion-text>
    </div>

    <ms-report-text
      v-if="shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupButUnusable"
      :theme="MsReportTheme.Error"
    >
      {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.errorRecipientsAllRevoked') }}
    </ms-report-text>

    <div class="shamir-done-footer">
      <ion-button
        fill="clear"
        @click="deleteShamir"
        class="shamir-done-footer__button color-danger"
        :class="shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupButUnusable ? 'required-action' : ''"
      >
        {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.delete') }}
      </ion-button>
      <ion-button
        v-if="
          shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupAllValid ||
          shamirInfo.tag === SelfShamirRecoveryInfoTag.SetupWithRevokedRecipients
        "
        @click="emits('closeModal')"
        class="shamir-done-footer__button color-secondary"
      >
        {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.close') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import ShamirIconCircleDark from '@/assets/images/shamir-icon-circle-dark.svg?raw';
import ShamirIconCircleLight from '@/assets/images/shamir-icon-circle-light.svg?raw';
import { SelfShamirRecoveryInfo, SelfShamirRecoveryInfoTag } from '@/parsec';
import { deleteSelfShamirRecovery } from '@/parsec/shamir';
import { IonButton, IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { checkmarkCircle, informationCircle, shieldCheckmark } from 'ionicons/icons';
import { Answer, askQuestion, MsImage, MsReportText, MsReportTheme, useTheme } from 'megashark-lib';

defineProps<{
  shamirInfo: SelfShamirRecoveryInfo;
}>();

const emits = defineEmits<{
  (e: 'shamirDeleted'): void;
  (e: 'closeModal'): void;
}>();

const { isDarkTheme } = useTheme();

async function deleteShamir(): Promise<void> {
  const answer = await askQuestion(
    'OrganizationRecovery.shamir.modalSelf.deleteModal.title',
    'OrganizationRecovery.shamir.modalSelf.deleteModal.message',
    {
      noText: 'OrganizationRecovery.shamir.modalSelf.deleteModal.no',
      yesText: 'OrganizationRecovery.shamir.modalSelf.deleteModal.yes',
      yesIsDangerous: true,
    },
  );
  if (answer === Answer.No) {
    return;
  }
  const result = await deleteSelfShamirRecovery();
  if (result.ok) {
    emits('shamirDeleted');
  }
}
</script>

<style scoped lang="scss">
.shamir-done {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: ms.spacing('padding-5xl');
  gap: ms.spacing('gap-3xl');
  color: ms.color('text-base-body');
  background: ms.color('surface-base-default-secondary');
  border-radius: ms.radius('2xl');
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-3xl');
  }

  &-image {
    width: 4.5rem;
    height: 4.5rem;
    box-shadow: var(--parsec-shadow-card);
    background: ms.color('surface-base-default');
    border-radius: ms.radius('full');
  }

  &-text {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: ms.spacing('gap-lg');
    text-align: center;

    &__title {
      @include ms.font('heading-h4');
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-lg');

      .title-icon {
        font-size: 1.25rem;
        color: ms.color('icon-brand-default');
        padding: ms.spacing('padding-xs');
        border-radius: ms.radius('4xl');
        background: ms.color('surface-base-default');
      }
    }

    &__description {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');
    }
  }

  &-info {
    @include ms.font('label-md-medium');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    margin-top: 1rem;
    color: ms.color('text-base-body');

    &__icon {
      font-size: 1rem;
      color: ms.color('icon-neutral-default');
    }
  }

  &-footer {
    display: flex;
    gap: ms.spacing('gap-lg');
    margin-top: 1.5rem;
    justify-content: space-between;

    &__button {
      .button-delete {
        color: ms.color('text-error-default');

        &.required-action {
          color: ms.color('text-error-on-color') !important;
          width: 100% !important;
        }
      }
    }
  }
}

.shamir-done-list {
  padding: 0;
  width: 100%;
  min-height: 13rem;
  max-height: 13rem;
  display: flex;
  flex-direction: column;
  border-radius: ms.radius('lg');
  background: ms.color('surface-base-default');
  overflow-y: auto;
  box-shadow: var(--parsec-shadow-card);

  &-item {
    border-bottom: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    box-shadow: var(--parsec-shadow-card);
    flex-shrink: 0;

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
      }

      &-description {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }

    &__icon {
      flex-shrink: 0;
      font-size: 1rem;
      color: ms.color('icon-neutral-default');

      &:first-child {
        color: ms.color('icon-brand-default');
      }

      &:last-child {
        padding: ms.spacing('padding-xs');
        border-radius: ms.radius('4xl');
        color: ms.color('icon-neutral-default');
        cursor: pointer;

        &:hover {
          background: ms.color('surface-error-default-subtle-pressed');
          color: ms.color('icon-error-default');
        }
      }
    }

    &__revoked {
      @include ms.font('body-md-medium');
      color: ms.color('text-error-default');
      font-style: italic;
    }

    &:has(.shamir-done-list-item__revoked) {
      .shamir-done-list-item__text-label {
        text-decoration: line-through;
      }
    }

    &:hover {
      --background: #{ms.color('surface-base-page-secondary')};
    }
  }
}
</style>
