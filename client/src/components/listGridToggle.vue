<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="list-grid-toggle">
    <!-- grid -->
    <ion-button
      fill="clear"
      class="button-view"
      id="grid-view"
      :disabled="!listView"
      @click="toggleView()"
    >
      <ion-icon
        :icon="grid"
      />
      <span v-if="!listView">
        {{ $t('WorkspacesPage.viewDisplay.grid') }}
      </span>
    </ion-button>
    <!-- list -->
    <ion-button
      fill="clear"
      class="button-view"
      id="list-view"
      :disabled="listView"
      @click="toggleView()"
    >
      <ion-icon
        :icon="list"
      />
      <span v-if="listView">
        {{ $t('WorkspacesPage.viewDisplay.list') }}
      </span>
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonIcon } from '@ionic/vue';
import { defineEmits } from 'vue';
import { grid, list } from 'ionicons/icons';
import { ref } from 'vue';

const props = defineProps<{
  listView: boolean
}>();

const listView = ref(props.listView);

// create a custom event
const emits = defineEmits<{
  (e: 'toggleView', value: boolean): void
}>();

// trigger the custom event
function toggleView() : void {
  listView.value = !listView.value;
  emits('toggleView', listView.value);
}
</script>

<style scoped lang="scss">
.list-grid-toggle {
  display: flex;
  align-items: center;
}

.button-view {
  color: var(--parsec-color-light-primary-700);
  padding: 0.25rem;
  border-radius: 4px;
  height: auto;

  span {
    margin-left: .5rem;
  }

  &:not(.button-disabled) {
    cursor: pointer;
    --background-hover: none;
  }
}

.button-disabled {
  background: var(--parsec-color-light-secondary-inversed-contrast);
  opacity: 1;
}

ion-button::part(native) {
  padding-inline-start: 0px;
  padding-inline-end: 0px;
  padding-top: 0px;
  padding-bottom: 0px;
}
</style>
