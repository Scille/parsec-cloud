<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="user-information">
    <ms-input
      label="CreateOrganization.fullname"
      placeholder="CreateOrganization.fullnamePlaceholder"
      name="fullname"
      v-model="fullName"
      ref="firstInputField"
      :disabled="!$props.nameEnabled"
      @change="$emit('fieldUpdate')"
      @on-enter-keyup="$emit('onEnterKeyup', fullName)"
      :validator="userNameValidator"
    />
    <ms-input
      label="CreateOrganization.email"
      placeholder="CreateOrganization.emailPlaceholder"
      v-model="email"
      name="email"
      :disabled="!$props.emailEnabled"
      @change="$emit('fieldUpdate')"
      @on-enter-keyup="$emit('onEnterKeyup', email)"
      :validator="emailValidator"
    />
  </div>
</template>

<script setup lang="ts">
import { emailValidator, userNameValidator } from '@/common/validators';
import { HumanHandle } from '@/parsec';
import { MsInput, Validity } from 'megashark-lib';
import { ref, useTemplateRef } from 'vue';

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
const firstInputFieldRef = useTemplateRef<InstanceType<typeof MsInput>>('firstInputField');

defineEmits<{
  (e: 'fieldUpdate'): void;
  (e: 'onEnterKeyup', value: string): void;
}>();

defineExpose({
  areFieldsCorrect,
  email,
  fullName,
  setFocus,
  getEmail,
  getFullName,
  getHumanHandle,
});

async function setFocus(): Promise<void> {
  await firstInputFieldRef.value?.setFocus();
}

function getEmail(): string {
  return email.value;
}

function getFullName(): string {
  return fullName.value;
}

function getHumanHandle(): HumanHandle {
  return { label: fullName.value, email: email.value };
}

async function areFieldsCorrect(): Promise<boolean> {
  return (
    (await emailValidator(email.value)).validity === Validity.Valid && (await userNameValidator(fullName.value)).validity === Validity.Valid
  );
}
</script>

<style scoped lang="scss">
.user-information {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
</style>
