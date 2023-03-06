<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item fill="solid">
    <ion-label
      position="floating"
    >
      {{ label }}
    </ion-label>
    <ion-icon
      :icon="searchOutline"
      slot="start"
    />
    <ion-input
      id="search-input"
      v-model="searchRef"
      :clear-input="true"
      @ion-change="$emit('change', $event.detail.value)"
      @keyup.enter="onEnterPress()"
    />
  </ion-item>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { IonItem, IonInput, IonIcon, IonLabel } from '@ionic/vue';
import { searchOutline } from 'ionicons/icons';

defineProps<{
  label: string
}>();

const searchRef = ref('');

const emits = defineEmits<{
  (e: 'change', value: string): void
  (e: 'enter'): void
}>();

function onEnterPress() : void {
  if (searchRef.value.length > 0) {
    emits('enter');
  }
}
</script>
