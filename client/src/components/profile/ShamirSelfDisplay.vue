<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
        :image="ShamirIconCircle"
        class="shamir-done-image"
        alt="Shamir Icon"
      />

      <div class="shamir-done-text">
        <ion-text class="shamir-done-text__title title-h3">
          {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.setupDone') }}
          <ion-icon
            :icon="checkmarkCircle"
            class="title-icon"
          />
        </ion-text>
        <ion-text class="shamir-done-text__description body">
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
            class="shamir-done-list-item__text subtitles-sm"
            v-if="recipient.isAnonymous()"
          >
            {{ $msTranslate('UsersPage.anonymous') }}
          </ion-text>
          <ion-text
            class="shamir-done-list-item__text subtitles-sm"
            v-else
          >
            <span class="shamir-done-list-item__text-label subtitles-sm">
              {{ recipient.humanHandle.label }}
            </span>
            <span class="shamir-done-list-item__text-description body">
              {{ recipient.humanHandle.email }}
            </span>
          </ion-text>
          <ion-text
            v-if="recipient.isRevoked()"
            class="shamir-done-list-item__revoked subtitles-sm"
          >
            {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.revoked') }}
          </ion-text>
        </ion-item>
      </ion-list>

      <ion-text class="shamir-done-info button-medium">
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
        class="shamir-done-footer__button shamir-done-footer__button--delete"
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
        class="shamir-done-footer__button shamir-done-footer__button--close"
      >
        {{ $msTranslate('OrganizationRecovery.shamir.modalSelf.close') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import ShamirIconCircle from '@/assets/images/shamir-icon-circle.svg?raw';
import { SelfShamirRecoveryInfo } from '@/parsec';
import { deleteSelfShamirRecovery } from '@/parsec/shamir';
import { SelfShamirRecoveryInfoTag } from '@/plugins/libparsec';
import { IonButton, IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { checkmarkCircle, informationCircle, shieldCheckmark } from 'ionicons/icons';
import { Answer, askQuestion, MsImage, MsReportText, MsReportTheme } from 'megashark-lib';

defineProps<{
  shamirInfo: SelfShamirRecoveryInfo;
}>();

const emits = defineEmits<{
  (e: 'shamirDeleted'): void;
  (e: 'closeModal'): void;
}>();

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
  padding: 2rem;
  gap: 1rem;
  color: var(--parsec-color-light-secondary-text);
  background: var(--parsec-color-light-secondary-background);
  border-radius: var(--parsec-radius-12);
  border: 1px solid var(--parsec-color-light-secondary-medium);

  @include ms.responsive-breakpoint('sm') {
    padding: 1rem;
  }

  &-image {
    width: 4.5rem;
    height: 4.5rem;
    box-shadow: var(--parsec-shadow-card);
    background: var(--parsec-color-light-secondary-white);
    border-radius: var(--parsec-radius-circle);
  }

  &-text {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    text-align: center;

    &__title {
      display: flex;
      align-items: center;
      gap: 0.5rem;

      .title-icon {
        font-size: 1.25rem;
        color: var(--parsec-color-light-primary-500);
        padding: 0.125rem;
        border-radius: var(--parsec-radius-18);
        background: var(--parsec-color-light-secondary-white);
      }
    }

    &__description {
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  &-info {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 1rem;
    color: var(--parsec-color-light-secondary-text);

    &__icon {
      font-size: 1rem;
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  &-footer {
    display: flex;
    gap: 0.5rem;
    margin-top: 1.5rem;
    justify-content: space-between;

    &__button {
      &--delete {
        color: var(--parsec-color-light-danger-500);

        &:hover {
          --background-hover: var(--parsec-color-light-danger-50);
          color: var(--parsec-color-light-danger-700);
        }

        &.required-action {
          --background: var(--parsec-color-light-danger-500) !important;
          color: var(--parsec-color-light-secondary-white) !important;
          width: 100% !important;

          &:hover {
            --background-hover: var(--parsec-color-light-danger-600) !important;
          }
        }
      }
      &--close {
        min-width: 5rem;
        --background: var(--parsec-color-light-secondary-text) !important;
        color: var(--parsec-color-light-secondary-white);

        &:hover {
          --background-hover: var(--parsec-color-light-secondary-contrast) !important;
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
  border-radius: var(--parsec-radius-8);
  background: var(--parsec-color-light-secondary-white);
  overflow-y: auto;
  box-shadow: var(--parsec-shadow-card);

  &-item {
    border-bottom: 1px solid var(--parsec-color-light-secondary-premiere);
    box-shadow: var(--parsec-shadow-card);
    flex-shrink: 0;

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
      }

      &-description {
        color: var(--parsec-color-light-secondary-grey);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }

    &__icon {
      flex-shrink: 0;
      font-size: 1rem;
      color: var(--parsec-color-light-secondary-grey);

      &:first-child {
        color: var(--parsec-color-light-primary-500);
      }

      &:last-child {
        padding: 0.125rem;
        border-radius: var(--parsec-radius-18);
        color: var(--parsec-color-light-secondary-grey);
        cursor: pointer;

        &:hover {
          background: var(--parsec-color-light-danger-100);
          color: var(--parsec-color-light-danger-600);
        }
      }
    }

    &__revoked {
      color: var(--parsec-color-light-danger-500);
      font-style: italic;
    }

    &:has(.shamir-done-list-item__revoked) {
      .shamir-done-list-item__text-label {
        text-decoration: line-through;
      }
    }

    &:hover {
      --background: var(--parsec-color-light-secondary-premiere);
    }
  }
}
</style>
