<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <custom-input
    :label="$t('CreateOrganization.fullname')"
    :placeholder="$t('CreateOrganization.fullnamePlaceholder')"
    name="fullname"
    v-model="fullName"
  />
  <custom-input
    :label="$t('CreateOrganization.email')"
    :placeholder="$t('CreateOrganization.emailPlaceholder')"
    v-model="email"
    name="email"
  />
  <custom-input
    :label="$t('CreateOrganization.deviceNameInputLabel')"
    :placeholder="$t('CreateOrganization.deviceNamePlaceholder')"
    v-model="deviceName"
    name="deviceName"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import CustomInput from '@/components/CustomInput.vue';
import { Validity, userNameValidator, deviceNameValidator, emailValidator } from '@/common/validators';

function getDefaultDeviceName(): string {
  return 'my_device';
}

const deviceName = ref(getDefaultDeviceName());
const email = ref('');
const fullName = ref('');

defineExpose({
  areFieldsCorrect,
  deviceName,
  email,
  fullName
});

function areFieldsCorrect(): boolean {
  return (
    deviceNameValidator(deviceName.value) === Validity.Valid
    && emailValidator(email.value) === Validity.Valid
    && userNameValidator(fullName.value) === Validity.Valid
  );
}

</script>

<style scoped lang="scss">

</style>
