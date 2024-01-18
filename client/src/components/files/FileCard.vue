<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="file-card-item ion-no-padding"
    :class="{ selected: isSelected }"
    @click="$emit('click', $event, file)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div class="card-checkbox">
      <ion-checkbox
        aria-label=""
        class="checkbox"
        v-model="isSelected"
        v-show="isSelected || isHovered || showCheckbox"
        @click.stop
        @ion-change="$emit('select', file, isSelected)"
      />
    </div>
    <div
      class="card-option"
      v-show="isHovered || menuOpened"
      @click.stop="onOptionsClick($event)"
    >
      <ion-icon :icon="ellipsisHorizontal" />
    </div>
    <div class="card-content">
      <ion-avatar class="card-content__icons">
        <ion-icon
          class="icon-item"
          :icon="file.isFile() ? document : folder"
        />
        <ion-icon
          class="cloud-overlay"
          :class="isFileSynced() ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
          :icon="isFileSynced() ? cloudDone : cloudOffline"
        />
      </ion-avatar>

      <ion-title class="card-content__title body">
        {{ file.name }}
      </ion-title>

      <ion-text class="card-content-last-update caption-caption">
        <span>{{ $t('FoldersPage.File.lastUpdate') }}</span>
        <span>{{ formatTimeSince(file.updated, '--', 'short') }}</span>
      </ion-text>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import { EntryStat } from '@/parsec';
import { IonAvatar, IonCheckbox, IonIcon, IonItem, IonText, IonTitle } from '@ionic/vue';
import { cloudDone, cloudOffline, document, ellipsisHorizontal, folder } from 'ionicons/icons';
import { ref } from 'vue';

const isHovered = ref(false);
const menuOpened = ref(false);
const isSelected = ref(false);

const props = defineProps<{
  file: EntryStat;
  showCheckbox: boolean;
}>();

const emits = defineEmits<{
  (e: 'click', event: Event, file: EntryStat): void;
  (e: 'menuClick', event: Event, file: EntryStat, onFinished: () => void): void;
  (e: 'select', file: EntryStat, selected: boolean): void;
}>();

defineExpose({
  isSelected,
  props,
});

function isFileSynced(): boolean {
  return !props.file.needSync;
}

async function onOptionsClick(event: Event): Promise<void> {
  menuOpened.value = true;
  emits('menuClick', event, props.file, () => {
    menuOpened.value = false;
  });
}
</script>

<style lang="scss" scoped>
.file-card-item {
  position: relative;
  cursor: pointer;
  text-align: center;
  --background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  user-select: none;
  border-radius: var(--parsec-radius-12);
  width: 10.5rem;

  &::part(native) {
    --inner-padding-end: 0px;
  }

  &:hover {
    --background: var(--parsec-color-light-primary-30);
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
  }

  &.selected {
    --background: var(--parsec-color-light-primary-100);
    --background-hover: var(--parsec-color-light-primary-100);
    border: 1px solid var(--parsec-color-light-primary-100);
  }
}

.card-option,
.card-checkbox {
  position: absolute;
}

.card-checkbox {
  left: 0.5rem;
  top: 0.5rem;
}

.card-option {
  color: var(--parsec-color-light-secondary-grey);
  text-align: right;
  display: flex;
  align-items: center;
  top: 0;
  right: 0;
  font-size: 1.5rem;
  padding: 0.5rem;
  cursor: pointer;

  &:hover {
    color: var(--parsec-color-light-primary-500);
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 1.5rem 0.5rem;
  width: 100%;
  margin: auto;

  &__icons {
    position: relative;
    color: var(--parsec-color-light-primary-600);
    height: fit-content;
    width: fit-content;
    margin: 0 auto 0.875rem;

    .icon-item {
      font-size: 3rem;
    }

    .cloud-overlay {
      position: absolute;
      font-size: 1.25rem;
      left: 58%;
      bottom: -2px;
      padding: 2px;
      background: var(--parsec-color-light-secondary-background);
      border-radius: 50%;
    }

    .cloud-overlay-ok {
      color: var(--parsec-color-light-primary-500);
    }

    .cloud-overlay-ko {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &__title {
    color: var(--parsec-color-light-primary-900);
    text-align: center;
    padding: 0 0 0.25rem;
    text-overflow: ellipsis;
    white-space: nowrap;
    width: inherit;

    ion-text {
      width: 100%;
      overflow: hidden;
    }
  }
}

.card-content-last-update {
  color: var(--parsec-color-light-secondary-grey);
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

/* No idea how to change the color of the ion-item */
.card-content__title::part(native),
.card-content-last-update::part(native) {
  background-color: var(--parsec-color-light-secondary-background);
}
</style>
