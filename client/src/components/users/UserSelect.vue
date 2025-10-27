<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="user-select-container">
    <ms-input
      id="select-user-input"
      ref="input"
      :label="label || 'UserSelect.defaultLabel'"
      :placeholder="placeholder || 'UserSelect.defaultPlaceholder'"
      @input="onInputChange"
      :model-value="inputValue"
    />
    <ion-text
      class="body form-helperText"
      v-if="errorMessage"
    >
      {{ $msTranslate(errorMessage) }}
    </ion-text>
    <user-select-dropdown
      class="user-select-dropdown-overlay"
      v-if="matchingUsers.length > 0"
      :users="matchingUsers"
      :current-user="currentUser"
      @select="onUserSelect"
    />
  </div>
</template>

<script setup lang="ts">
import UserSelectDropdown from '@/components/users/UserSelectDropdown.vue';
import { listUsers, UserInfo } from '@/parsec';
import { IonText } from '@ionic/vue';
import { MsInput, Translatable } from 'megashark-lib';
import { onMounted, Ref, ref, useTemplateRef, watch } from 'vue';

const props = defineProps<{
  label?: Translatable;
  placeholder?: Translatable;
  excludeUsers?: UserInfo[];
  modelValue?: UserInfo;
  currentUser: UserInfo;
}>();

const emits = defineEmits<{
  (e: 'update:modelValue', value: UserInfo | undefined): void;
}>();

const querying = ref(false);
const matchingUsers = ref<UserInfo[]>([]);
const inputRef = useTemplateRef<InstanceType<typeof MsInput>>('input');
const inputValue: Ref<string> = ref('');
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

  querying.value = false;

  if (!result.ok || result.value.length === 0) {
    matchingUsers.value = [];

    if (!result.ok) {
      errorMessage.value = 'UserSelect.queryFailed';
    } else if (result.value.length === 0) {
      errorMessage.value = 'UserSelect.noMatch';
    }
    return;
  }

  matchingUsers.value = result.value;
  errorMessage.value = '';
}

function onInputChange(event: Event): void {
  const target = event.target as HTMLInputElement;
  inputValue.value = target?.value || '';
}

async function onUserSelect(user: UserInfo): Promise<void> {
  selectedUser.value = user;
  inputValue.value = `${user.humanHandle.label} (${user.humanHandle.email})`;
  emits('update:modelValue', user);

  queryCount = 0;
  matchingUsers.value = [];
  errorMessage.value = '';
}

watch(inputValue, async (newValue) => {
  if (selectedUser.value) {
    const expectedValue = `${selectedUser.value.humanHandle.label} (${selectedUser.value.humanHandle.email})`;
    if (newValue !== expectedValue) {
      selectedUser.value = undefined;
      emits('update:modelValue', undefined);
    }
    return;
  }

  if (!newValue || newValue.trim().length === 0) {
    matchingUsers.value = [];
    errorMessage.value = '';
    return;
  }

  queryCount += 1;
  const backup = queryCount;
  errorMessage.value = '';

  if (queryCount !== backup || querying.value) {
    return;
  }

  await queryMatchingUsers(newValue);
});

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
  position: relative;
  margin-bottom: 1rem;
}

.form-helperText {
  position: absolute;
  bottom: -2rem;
  color: var(--parsec-color-light-secondary-soft-text);
}

.user-select-dropdown-overlay {
  position: absolute;
  box-shadow: var(--parsec-shadow-strong);
  border-radius: var(--parsec-radius-8);
  z-index: 100;
  width: 100%;
  top: 4.5rem;
  background: var(--parsec-color-light-secondary-white);
}
</style>
