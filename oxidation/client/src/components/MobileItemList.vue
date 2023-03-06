<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item
    button
    lines="none"
    :detail="false"
    :class="{ selected: isSelected, 'no-padding-end': !isSelected }"
  >
    <ion-icon
      :icon="getIconFromItemType()"
      color="secondary"
      class="thumbnail-icon"
    />
    <ion-label class="text-labels">
      {{ primaryLabel }}
      <p class="secondary-text">
        {{ secondaryLabel }}
      </p>
      <div
        v-if="thirdLabel"
        class="secondary-text"
      >
        <p>{{ thirdLabel }}</p>
      </div>
    </ion-label>
    <ion-button
      v-if="!isSelected && itemType==='workspace'"
      color="dark"
      fill="clear"
      size="default"
      class="action-button"
      @click.stop="{ $emit('trigger-share'); }"
    >
      <ion-icon
        :icon="shareSocial"
        slot="icon-only"
        size="small"
      />
    </ion-button>
    <ion-button
      v-if="!isSelected"
      color="dark"
      fill="clear"
      size="default"
      class="action-button"
      @click.stop="{ $emit('trigger-action-sheet'); }"
    >
      <ion-icon
        :icon="ellipsisVertical"
        slot="icon-only"
        size="small"
      />
    </ion-button>
    <ion-icon
      v-else
      :icon="checkmarkCircle"
      color="secondary"
    />
  </ion-item>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { IonIcon, IonItem, IonLabel, IonButton } from '@ionic/vue';
import { folder, document, ellipsisVertical, briefcase, person, help, checkmarkCircle, shareSocial } from 'ionicons/icons';

const props = defineProps<{
  itemType: 'folder' | 'file' | 'workspace' | 'user' | 'pendingUser'
  primaryLabel: string
  secondaryLabel: string
  thirdLabel?: string | number
}>();

const emit = defineEmits<{
  (event: 'trigger-action-sheet'): void
  (event: 'trigger-share'): void
}>();

const isSelected = ref(false);

function setSelected(bool: boolean): void {
  isSelected.value = bool;
}

function getIconFromItemType(): string {
  switch (props.itemType) {
  case 'folder':
    return folder;
  case 'file':
    return document;
  case 'workspace':
    return briefcase;
  case 'user':
    return person;
  case 'pendingUser':
    return help;
  default:
    return help;
  }
}
</script>

<style scoped>
.no-padding-end {
    --inner-padding-end: 0;
}
.text-labels {
    padding-inline: 1em;
}

p {
    margin: 0;
}

.secondary-text {
    color: var(--ion-color-dark-shade);
}

.selected {
    --background: var(--ion-color-medium);
}

.thumbnail-icon {
    font-size: 64px;
}

.action-button {
    margin: 0;
}

</style>
