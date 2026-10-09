<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="UsersPage.CreateUserInvitationModal.pageTitle"
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
    <div class="email-input-container">
      <ms-report-text :theme="MsReportTheme.Info">
        <ms-rich-text text="UsersPage.CreateUserInvitationModal.subtitle" />
      </ms-report-text>
      <ms-input
        @change="onInputChange"
        v-model="textModel"
        ref="emailInput"
        placeholder="UsersPage.CreateUserInvitationModal.placeholder"
        label="UsersPage.CreateUserInvitationModal.label"
        id="email-input"
      />
    </div>
    <div
      class="email-list-container"
      v-if="emails.length > 1"
    >
      <ion-text class="email-list__title">
        {{ $msTranslate('UsersPage.CreateUserInvitationModal.emailsList') }} ({{ emails.length }})
      </ion-text>
      <div class="email-list">
        <ion-text
          v-for="email in emails"
          :key="email"
          class="email-list__item"
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
import { MsInput, MsModal, MsModalResult, MsReportText, MsReportTheme, MsRichText, Validity } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const textModel = ref('');
const emails = ref<Array<string>>([]);
const emailInputRef = useTemplateRef<InstanceType<typeof MsInput>>('emailInput');

onMounted(async () => {
  if (emailInputRef.value) {
    emailInputRef.value.setFocus();
  }
});

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

.email-input-container {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
}

.email-list-container {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
  margin-top: 1rem;

  .email-list {
    display: flex;
    flex-wrap: wrap;
    gap: ms.spacing('gap-lg');
    max-height: 10rem;
    overflow-y: auto;

    &__title {
      @include ms.font('label-md-medium');
      color: ms.color('text-base-description');
    }

    &__item {
      @include ms.font('label-sm-medium');
      background-color: ms.color('surface-base-page-secondary');
      color: ms.color('text-base-body');
      padding: ms.spacing('padding-sm') ms.spacing('padding-lg');
      border-radius: ms.radius('md');
    }
  }
}
</style>
