<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <custom-input
    :label="$t('CreateOrganization.fullname')"
    :placeholder="$t('CreateOrganization.fullnamePlaceholder')"
    name="fullname"
    v-model="fullName"
    :disabled="!$props.nameEnabled"
  />
  <custom-input
    :label="$t('CreateOrganization.email')"
    :placeholder="$t('CreateOrganization.emailPlaceholder')"
    v-model="email"
    name="email"
    :disabled="!$props.emailEnabled"
  />
  <custom-input
    :label="$t('CreateOrganization.deviceNameInputLabel')"
    :placeholder="$t('CreateOrganization.deviceNamePlaceholder')"
    v-model="deviceName"
    name="deviceName"
    :disabled="!$props.deviceEnabled"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue';
import CustomInput from '@/components/CustomInput.vue';
import { Validity, userNameValidator, deviceNameValidator, emailValidator } from '@/common/validators';

function getDefaultDeviceName(): string {
  return 'my_device';
}

const props = defineProps({
  defaultEmail: {
    type: String,
    default: '',
  },
  defaultName: {
    type: String,
    default: '',
  },
  emailEnabled: {
    type: Boolean,
    default: true,
  },
  nameEnabled: {
    type: Boolean,
    default: true,
  },
  deviceEnabled: {
    type: Boolean,
    default: true,
  },
});

const deviceName = ref(getDefaultDeviceName());
const email = ref(props.defaultEmail);
const fullName = ref(props.defaultName);

defineExpose({
  areFieldsCorrect,
  deviceName,
  email,
  fullName,
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
