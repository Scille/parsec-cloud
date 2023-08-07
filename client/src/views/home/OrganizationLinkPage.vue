<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ms-input
    :label="$t('JoinOrganization.linkFormLabel')"
    :placeholder="$t('JoinOrganization.linkFormPlaceholder')"
    name="joinOrganization"
    v-model="joinLink"
    id="link-orga-input"
    type="url"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';
import { claimUserLinkValidator, Validity } from '@/common/validators';

const joinLink = ref('');

defineExpose({
  areFieldsCorrect,
  joinLink,
});

function areFieldsCorrect(): boolean {
  return claimUserLinkValidator(joinLink.value) === Validity.Valid;
}
</script>

<style scoped lang="scss">

</style>
