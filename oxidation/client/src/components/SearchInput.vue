<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <!-- Here we still use ion-item to wrap ion-input because of lack of support of ion-icon with new v7 standalone ion-input -->
  <!-- TODO: Migrate from legacy to modern syntax following this issue: https://github.com/ionic-team/ionic-framework/issues/26297 -->
  <ion-item fill="solid">
    <ion-icon
      :icon="search"
      slot="start"
    />
    <ion-input
      id="search-input"
      :label="label"
      label-placement="floating"
      v-model="searchRef"
      :clear-input="true"
      @ion-change="$emit('change', $event.detail.value)"
      @keyup.enter="onEnterPress()"
    />
  </ion-item>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { IonItem, IonInput, IonIcon } from '@ionic/vue';
import { search } from 'ionicons/icons';

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
