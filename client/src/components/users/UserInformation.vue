<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-input
    :label="$t('CreateOrganization.fullname')"
    :placeholder="$t('CreateOrganization.fullnamePlaceholder')"
    name="fullname"
    v-model="fullName"
    ref="firstInputFieldRef"
    :disabled="!$props.nameEnabled"
    @change="$emit('fieldUpdate')"
    @on-enter-keyup="$emit('onEnterKeyup', fullName)"
    :validator="userNameValidator"
  />
  <ms-input
    :label="$t('CreateOrganization.email')"
    :placeholder="$t('CreateOrganization.emailPlaceholder')"
    v-model="email"
    name="email"
    :disabled="!$props.emailEnabled"
    @change="$emit('fieldUpdate')"
    @on-enter-keyup="$emit('onEnterKeyup', email)"
    :validator="emailValidator"
  />
</template>

<script setup lang="ts">
import { Validity, emailValidator, userNameValidator } from '@/common/validators';
import { MsInput } from '@/components/core';
import { ref } from 'vue';

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
});

const email = ref(props.defaultEmail);
const fullName = ref(props.defaultName);
const firstInputFieldRef = ref();

defineEmits<{
  (e: 'fieldUpdate'): void;
  (e: 'onEnterKeyup', value: string): void;
}>();

defineExpose({
  areFieldsCorrect,
  email,
  fullName,
  setFocus,
});

async function setFocus(): Promise<void> {
  await firstInputFieldRef.value.setFocus();
}

async function areFieldsCorrect(): Promise<boolean> {
  return (
    (await emailValidator(email.value)).validity === Validity.Valid && (await userNameValidator(fullName.value)).validity === Validity.Valid
  );
}
</script>

<style scoped lang="scss"></style>
