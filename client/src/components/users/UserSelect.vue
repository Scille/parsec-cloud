<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="user-select-container">
    <ms-input
      id="select-user-input"
      ref="input"
      :label="label || 'UserSelect.defaultLabel'"
      :placeholder="placeholder || 'UserSelect.defaultPlaceholder'"
      @change="onInput"
      :model-value="selectedUser ? `${selectedUser.humanHandle.label} (${selectedUser.humanHandle.email})` : undefined"
      :disabled="querying"
      :debounce="800"
    />
    <ion-text
      class="body form-helperText"
      v-if="errorMessage"
    >
      {{ $msTranslate(errorMessage) }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { popoverController, IonText } from '@ionic/vue';
import { onMounted, Ref, ref, useTemplateRef } from 'vue';
import { MsInput, MsModalResult, Translatable } from 'megashark-lib';
import { listUsers, UserInfo } from '@/parsec';
import UserSelectDropdown from '@/components/users/UserSelectDropdown.vue';

const props = defineProps<{
  label?: Translatable;
  placeholder?: Translatable;
  excludeUsers?: UserInfo[];
  modelValue?: UserInfo;
}>();

const emits = defineEmits<{
  (e: 'update:modelValue', value: UserInfo | undefined): void;
}>();

const querying = ref(false);
const inputRef = useTemplateRef<InstanceType<typeof MsInput>>('input');
const selectedUser: Ref<UserInfo | undefined> = ref();
let queryCount = 0;
const errorMessage = ref('');

async function queryMatchingUsers(search: string): Promise<void> {
  if (querying.value) {
    return;
  }
  querying.value = true;

  const result = await listUsers(true, search);

  if (result.ok && props.excludeUsers) {
    result.value = result.value.filter((user) => props.excludeUsers?.find((item) => item.id === user.id) === undefined);
  }

  if (!result.ok || result.value.length === 0) {
    querying.value = false;
    if (!result.ok) {
      errorMessage.value = 'UserSelect.queryFailed';
    } else if (result.value.length === 0) {
      errorMessage.value = 'UserSelect.noMatch';
    }
    return;
  }

  const popover = await popoverController.create({
    component: UserSelectDropdown,
    cssClass: 'user-select-dropdown-popover',
    componentProps: {
      // Limit to the first five users at most, else the dropdown becomes too big
      users: result.value.slice(0, 5),
    },
    trigger: 'select-user-input',
    side: 'bottom',
    alignment: 'start',
    showBackdrop: false,
  });
  await popover.present();
  const { role, data } = await popover.onDidDismiss();
  await popover.dismiss();
  if (role === MsModalResult.Confirm && data) {
    selectedUser.value = data.user;
    emits('update:modelValue', data.user);
  }
  querying.value = false;
}

async function onInput(value: string): Promise<void> {
  if (selectedUser.value) {
    selectedUser.value = undefined;
    emits('update:modelValue', undefined);
    return;
  }
  if (!value) {
    return;
  }
  queryCount += 1;
  const backup = queryCount;
  errorMessage.value = '';

  if (queryCount !== backup || querying.value) {
    return;
  }

  await queryMatchingUsers(value);
}

onMounted(async () => {
  await inputRef.value?.setFocus();
  if (props.modelValue) {
    selectedUser.value = props.modelValue;
  }
});
</script>

<style scoped lang="scss">
.user-select-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-helperText {
  color: var(--parsec-color-light-secondary-soft-text);
}
</style>
