<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="$t('JoinByLinkModal.pageTitle')"
    :subtitle="$t('JoinByLinkModal.pleaseEnterUrl')"
    :close-button-enabled="true"
    :confirm-button="{
      label: $t('JoinByLinkModal.join'),
      disabled: !canProgress,
      onClick: confirm
    }"
  >
    <ms-input
      :label="$t('JoinOrganization.linkFormLabel')"
      :placeholder="$t('JoinOrganization.linkFormPlaceholder')"
      name="joinOrganization"
      v-model="joinLink"
      id="link-orga-input"
      type="url"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { Validity, claimLinkValidator } from '@/common/validators';
import { modalController } from '@ionic/vue';
import { MsModalResult } from '@/components/core/ms-modal/MsModal.vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';
import { asyncComputed } from '@/common/asyncComputed';

const joinLink = ref('');

const canProgress = asyncComputed(async () => {
  return await claimLinkValidator(joinLink.value) === Validity.Valid;
});

/* by the way pressing Enter won't send the form, you unfortunately have to click the button
see https://github.com/ionic-team/ionic-framework/issues/19368 */
function confirm(): Promise<boolean> {
  return modalController.dismiss(joinLink.value.trim(), MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
</style>
