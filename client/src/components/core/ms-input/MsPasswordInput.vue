<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-grid class="input-grid">
    <ion-row>
      <ion-col class="input-container">
        <ion-text
          id="passwordLabel"
          class="form-label"
        >
          {{ label }}
        </ion-text>
        <ion-item class="input-item ion-no-padding">
          <ion-input
            class="form-input input"
            ref="inputRef"
            aria-labelledby="passwordLabel"
            :type="passwordVisible ? 'text' : 'password'"
            @ion-input="onChange($event.target.value)"
            :value="modelValue"
            @keyup.enter="onEnterPress()"
            id="ms-password-input"
            :clear-on-edit="false"
            @ion-focus="hasFocus = true"
            @ion-blur="hasFocus = false"
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
      </ion-col>
    </ion-row>
  </ion-grid>
</template>

<script setup lang="ts">
import { Groups, HotkeyManager, HotkeyManagerKey, Hotkeys, Modifiers, Platforms } from '@/services/hotkeyManager';
import { IonCol, IonGrid, IonIcon, IonInput, IonItem, IonRow, IonText } from '@ionic/vue';
import { eye, eyeOff } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref } from 'vue';

const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const inputRef = ref();
let hotkeys: Hotkeys | null = null;
const passwordVisible = ref(false);
const hasFocus = ref(false);

const props = defineProps<{
  label: string;
  modelValue?: string;
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

function onChange(value: any): void {
  emits('update:modelValue', value);
  emits('change', value);
}

function onEnterPress(): void {
  if (props.modelValue && props.modelValue.length > 0) {
    emits('onEnterKeyup', props.modelValue);
  }
}
</script>

<style lang="scss" scoped>
.input-grid {
  width: 100%;
}
</style>
