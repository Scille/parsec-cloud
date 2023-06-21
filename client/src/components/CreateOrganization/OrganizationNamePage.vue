<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <custom-input
    :label="$t('CreateOrganization.organizationName')"
    :placeholder="$t('CreateOrganization.organizationNamePlaceholder')"
    name="organization"
    v-model="orgName"
    id="org-name-input"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import CustomInput from '@/components/CustomInput.vue';
import { organizationValidator, Validity } from '@/common/validators';

const orgName = ref('');

defineExpose({
  areFieldsCorrect,
  orgName
});

function areFieldsCorrect(): boolean {
  return organizationValidator(orgName.value) === Validity.Valid;
}
</script>

<style scoped lang="scss">

</style>
