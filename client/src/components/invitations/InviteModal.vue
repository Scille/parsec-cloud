<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="UsersPage.CreateUserInvitationModal.pageTitle"
    subtitle="UsersPage.CreateUserInvitationModal.subtitle"
    :close-button="{ visible: true }"
    :cancel-button="{
      disabled: false,
      label: 'UsersPage.CreateUserInvitationModal.cancel',
    }"
    :confirm-button="{
      label: { key: 'UsersPage.CreateUserInvitationModal.create', count: emails.length, data: { count: emails.length } },
      disabled: emails.length < 1,
      theme: MsReportTheme.Success,
      onClick: onConfirmClicked,
    }"
    :class="emails.length > 5 ? 'has-multiple-emails' : ''"
  >
    <ms-input
      @change="onInputChange"
      v-model="textModel"
      placeholder="UsersPage.CreateUserInvitationModal.placeholder"
      label="UsersPage.CreateUserInvitationModal.label"
      id="email-input"
    />
    <div
      class="email-list-container"
      v-if="emails.length > 1"
    >
      <ion-text class="email-list__title subtitles-sm">
        {{ $msTranslate('UsersPage.CreateUserInvitationModal.emailsList') }} ({{ emails.length }})
      </ion-text>
      <div class="email-list">
        <ion-text
          v-for="email in emails"
          :key="email"
          class="email-list__item button-small"
        >
          {{ email }}
        </ion-text>
      </div>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { emailValidator } from '@/common/validators';
import { IonText, modalController } from '@ionic/vue';
import { MsInput, MsModal, MsModalResult, MsReportTheme, Validity } from 'megashark-lib';
import { ref } from 'vue';

const textModel = ref('');
const emails = ref<Array<string>>([]);

async function onInputChange(): Promise<void> {
  const tmp: Array<string> = [];
  for (const part of textModel.value.split(';')) {
    const email = part.trim();
    if (
      !tmp.find((e) => e.toLocaleLowerCase() === email.toLocaleLowerCase()) &&
      (await emailValidator(email)).validity === Validity.Valid
    ) {
      tmp.push(email);
    }
  }
  emails.value = tmp;
}

async function onConfirmClicked(): Promise<boolean> {
  if (emails.value.length < 1) {
    return false;
  }
  return await modalController.dismiss({ emails: emails.value }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
#email-input {
  @include ms.responsive-breakpoint('sm') {
    margin-top: 1rem;
  }
}

.email-list-container {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-top: 1rem;

  .email-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    max-height: 10rem;
    overflow-y: auto;

    &__title {
      color: var(--parsec-color-light-secondary-grey);
    }

    &__item {
      background-color: var(--parsec-color-light-secondary-premiere);
      color: var(--parsec-color-light-secondary-text);
      padding: 0.25rem 0.5rem;
      border-radius: var(--parsec-radius-6);
    }
  }
}
</style>
