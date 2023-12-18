<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-input
    :label="$t('CreateOrganization.fullname')"
    :placeholder="$t('CreateOrganization.fullnamePlaceholder')"
    name="fullname"
    v-model="fullName"
    :disabled="!$props.nameEnabled"
    @change="$emit('fieldUpdate')"
    @on-enter-keyup="$emit('onEnterKeyup', fullName)"
  />
  <ms-input
    :label="$t('CreateOrganization.email')"
    :placeholder="$t('CreateOrganization.emailPlaceholder')"
    v-model="email"
    name="email"
    :disabled="!$props.emailEnabled"
    @change="$emit('fieldUpdate')"
    @on-enter-keyup="$emit('onEnterKeyup', email)"
  />
  <ms-input
    :label="$t('CreateOrganization.deviceNameInputLabel')"
    :placeholder="$t('CreateOrganization.deviceNamePlaceholder')"
    v-model="deviceName"
    name="deviceName"
    :disabled="!$props.deviceEnabled"
    @change="$emit('fieldUpdate')"
    @on-enter-keyup="$emit('onEnterKeyup', deviceName)"
  />
</template>

<script setup lang="ts">
import { Validity, deviceNameValidator, emailValidator, userNameValidator } from '@/common/validators';
import { MsInput } from '@/components/core';
import { ref } from 'vue';

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
  defaultDevice: {
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

const deviceName = ref(props.defaultDevice || getDefaultDeviceName());
const email = ref(props.defaultEmail);
const fullName = ref(props.defaultName);

defineEmits<{
  (e: 'fieldUpdate'): void;
  (e: 'onEnterKeyup', value: string): void;
}>();

defineExpose({
  areFieldsCorrect,
  deviceName,
  email,
  fullName,
});

async function areFieldsCorrect(): Promise<boolean> {
  return (
    (await emailValidator(email.value)) === Validity.Valid &&
    (await userNameValidator(fullName.value)) === Validity.Valid &&
    (await deviceNameValidator(deviceName.value)) === Validity.Valid
  );
}
</script>

<style scoped lang="scss"></style>
