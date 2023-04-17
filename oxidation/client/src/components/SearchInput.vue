<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <!-- Here we still use ion-item to wrap ion-input because of lack of support of ion-icon with new v7 standalone ion-input -->
  <!-- TODO: Migrate from legacy to modern syntax following this issue: https://github.com/ionic-team/ionic-framework/issues/26297 -->
  <ion-item fill="solid">
    <ion-icon
      :icon="search"
      slot="start"
      class="icon"
    />
    <ion-input
      id="search-input"
      class="search-input form-input"
      v-model="searchRef"
      :placeholder="label"
      :clear-input="true"
      @ion-input="$emit('change', $event.detail.value)"
      @keyup.enter="onEnterPress()"
      mode="ios"
    />
    <!-- mode='ios' in order to change the clear icon style -->
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

<style scoped lang="scss">
.container {
  border: 1px solid var(--parsec-color-light-secondary-light);
  border-radius: 6px;
  --min-height: 1rem;
  height: fit-content;
  --highlight-color-focused: blue;

  &:focus {
    --background: var(--parsec-color-light-secondary-background);
    --border-color: var(--parsec-color-light-primary-700);
    --border-style: solid;
    --border-width: 1px;
  }

  .icon {
    font-size: 1.125em;
    margin-inline-end: 1rem;
    color: var(--parsec-color-light-secondary-light);;
  }

}
.search-input {
  --placeholder-color: var(--parsec-color-light-secondary-light);
  --placeholder-opacity: .8;
  min-height: 1rem;
}

</style>