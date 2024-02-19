<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="input-container">
    <span
      id="passwordLabel"
      class="form-label"
    >
      {{ label }}
    </span>
    <ion-item
      class="input-item"
      :class="passwordIsInvalid ? 'input-invalid' : ''"
    >
      <ion-input
        class="form-input input"
        ref="inputRef"
        aria-labelledby="passwordLabel"
        :type="passwordVisible ? 'text' : 'password'"
        @ion-input="onChange($event.target.value)"
        :value="modelValue"
        id="ms-password-input"
        :clear-on-edit="false"
        @ion-focus="hasFocus = true"
        @ion-blur="hasFocus = false"
        @keyup.enter="onEnterPress()"
      />
      <div
        class="input-icon"
        @click="passwordVisible = !passwordVisible"
      >
        <ion-icon
          slot="icon-only"
          :icon="passwordVisible ? eyeOff : eye"
        />
      </div>
    </ion-item>
    <span
      v-show="errorMessage"
      class="form-error caption-caption"
    >
      <ion-icon
        class="form-error-icon"
        :icon="warning"
      />
      {{ errorMessage }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { Groups, HotkeyManager, HotkeyManagerKey, Hotkeys, Modifiers, Platforms } from '@/services/hotkeyManager';
import { IonIcon, IonInput, IonItem } from '@ionic/vue';
import { eye, eyeOff, warning } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref } from 'vue';

const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const inputRef = ref();
let hotkeys: Hotkeys | null = null;
const passwordVisible = ref(false);
const hasFocus = ref(false);

const props = defineProps<{
  label: string;
  modelValue?: string;
  errorMessage?: string;
  passwordIsInvalid?: boolean;
}>();

const emits = defineEmits<{
  (e: 'change', value: string): void;
  (e: 'onEnterKeyup', value: string): void;
  (e: 'update:modelValue', value: string): void;
}>();

defineExpose({
  setFocus,
});

onMounted(async () => {
  hotkeys = hotkeyManager.newHotkeys(Groups.Unique);

  hotkeys.add('/', Modifiers.Ctrl, Platforms.Desktop, async () => {
    if (hasFocus.value) {
      passwordVisible.value = !passwordVisible.value;
    }
  });
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
});

function setFocus(): void {
  setTimeout(() => {
    if (inputRef.value && inputRef.value.$el) {
      inputRef.value.$el.setFocus();
    }
  }, 200);
}

async function onChange(value: any): Promise<void> {
  emits('update:modelValue', value);
  emits('change', value);
}

function onEnterPress(): void {
  if (props.modelValue && props.modelValue.length > 0) {
    emits('onEnterKeyup', props.modelValue);
  }
}
</script>

<style lang="scss" scoped></style>
