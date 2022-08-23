<template>
  <ion-item
    button
    lines="none"
    :detail="false"
    :class="{ selected: isSelected }"
    @click="{ isSelected = !isSelected; }"
    @contextmenu.prevent="setSelected(true)"
  >
    <div class="item-grid-container">
      <ion-checkbox
        v-if="itemType==='file' || itemType==='folder'"
        class="checkbox"
        :checked="isSelected"
        :class="{ selected: isSelected, 'show-on-hover': !isSelected }"
      />
      <ion-icon
        :icon="getIconFromItemType()"
        color="secondary"
        class="thumbnail-icon ion-margin-bottom"
      />
      <div class="text-labels">
        <ion-label>
          {{ primaryLabel }}
        </ion-label>
        <ion-label class="secondary-label">
          {{ secondaryLabel }}
        </ion-label>
      </div>
      <div
        class="action-icons"
        :class="{ 'show-on-hover': !isSelected && !isPlatform('mobile')}"
      >
        <ion-button
          v-if="itemType==='workspace' && !isPlatform('mobile')"
          color="dark"
          fill="clear"
          class="ion-no-margin"
          @click="emit('trigger-share');"
        >
          <ion-icon
            :icon="shareSocial"
            size="small"
            slot="icon-only"
          />
        </ion-button>
        <ion-button
          color="dark"
          fill="clear"
          class="ion-no-margin"
          @click.stop="onEllipsisClick($event)"
        >
          <ion-icon
            :icon="ellipsisVertical"
            size="small"
            slot="icon-only"
          />
        </ion-button>
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { defineProps, ref, defineEmits } from 'vue';
import { IonButton, IonCheckbox, IonIcon, IonLabel, IonItem, isPlatform } from '@ionic/vue';
import { folder, document, ellipsisVertical, shareSocial, briefcase, person, help } from 'ionicons/icons';

const props = defineProps<{
  itemType: 'folder' | 'file' | 'workspace' | 'user'
  primaryLabel: string
  secondaryLabel: string
}>();

const emit = defineEmits<{
  (event: 'trigger-context-menu', ev: Event): void
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
  default:
    return help;
  }
}

function onEllipsisClick(ev: Event): void {
  if (isPlatform('mobile')){
    emit('trigger-action-sheet');
  } else {
    emit('trigger-context-menu', ev);
    setSelected(true);
  }
}
</script>

<style lang="scss" scoped>
ion-item {
    --padding-start: 0px;
    --inner-padding-end: 0px;
    &:hover .show-on-hover {
        visibility: visible;
    }
}

.item-grid-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    min-height: 180px;
    width: 100%;
    padding: 8px;
}

.checkbox {
    position: absolute;
    top: 0px;
    left: 18px;
}

.thumbnail-icon {
    font-size: 80px;
}

.text-labels {
    width: 100%;
    padding: 0px 20px;
}

ion-label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.secondary-label {
    color: var(--ion-color-dark-shade);
}

.action-icons {
    display: flex;
    flex-direction: column;
    justify-content: center;
    position: absolute;
    bottom: 0px;
    right: 0px;
    height: 80px;

    &:active {
        visibility: visible;
    }
}

.show-on-hover {
    visibility: hidden;
}

.selected {
    --background: var(--ion-color-medium);
    visibility: visible;
}

</style>
