<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="card"
    @click="$emit('click', $event, file)"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <div
      class="card-option"
      v-show="isHovered || menuOpened"
      @click.stop="
        menuOpened = true;
        $emit('menuClick', $event, file, () => {
          menuOpened = false;
        });
      "
    >
      <ion-icon :icon="ellipsisHorizontal" />
    </div>
    <div class="card-content">
      <ion-avatar class="card-content-icons">
        <ion-icon
          class="card-content-icons__item"
          :icon="file.isFile() ? document : folder"
        />
        <ion-icon
          class="cloud-overlay"
          :class="isFileSynced() ? 'cloud-overlay-ok' : 'cloud-overlay-ko'"
          :icon="isFileSynced() ? cloudDone : cloudOffline"
        />
      </ion-avatar>

      <ion-title class="card-content__title body-lg">
        {{ file.name }}
      </ion-title>

      <ion-text class="card-content-last-update caption-caption">
        <span>{{ $t('FoldersPage.File.lastUpdate') }}</span>
        <span>{{ timeSince(file.updated, '--', 'short') }}</span>
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { folder, document, ellipsisHorizontal, cloudDone, cloudOffline } from 'ionicons/icons';
import { IonAvatar, IonIcon, IonText, IonTitle } from '@ionic/vue';
import { inject, ref } from 'vue';
import { FormattersKey, Formatters } from '@/common/injectionKeys';
import { EntryStat } from '@/parsec';

const isHovered = ref(false);
const menuOpened = ref(false);

const props = defineProps<{
  file: EntryStat;
}>();

defineEmits<{
  (e: 'click', event: Event, file: EntryStat): void;
  (e: 'menuClick', event: Event, file: EntryStat, onFinished: () => void): void;
}>();

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { timeSince } = inject(FormattersKey)! as Formatters;

function isFileSynced(): boolean {
  return !props.file.needSync;
}
</script>

<style lang="scss" scoped>
.card {
  padding: 2rem 1em 1em;
  cursor: pointer;
  text-align: center;
  background-color: var(--parsec-color-light-secondary-background);
  user-select: none;
  border-radius: 8px;
  width: 14rem;

  &:hover {
    background-color: var(--parsec-color-light-primary-30);
    // box-shadow: var(--parsec-shadow-light);
  }
}

.card-option {
  color: var(--parsec-color-light-secondary-grey);
  text-align: right;
  position: absolute;
  display: flex;
  align-items: center;
  top: 0;
  right: 1rem;
  font-size: 1.5rem;
  padding: 0.5rem;

  &:hover {
    color: var(--parsec-color-light-primary-500);
  }
}

.card-content-icons {
  margin: 0 auto 0.5rem;
  position: relative;
  height: fit-content;
  display: flex;
  justify-content: center;
  align-items: center;
  color: var(--parsec-color-light-primary-600);
  width: 100%;

  &__item {
    font-size: 2.5rem;
  }

  .cloud-overlay {
    position: absolute;
    font-size: 1.25rem;
    bottom: -10px;
    left: 54%;
    padding: 2px;
    background: white;
    border-radius: 50%;
  }

  .cloud-overlay-ok {
    color: var(--parsec-color-light-primary-500);
  }

  .cloud-overlay-ko {
    color: var(--parsec-color-light-secondary-text);
  }
}

.card-content__title {
  color: var(--parsec-color-light-primary-900);
  font-size: 18px;
  text-align: center;

  ion-text {
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}

.card-content-last-update {
  color: var(--parsec-color-light-secondary-grey);
  text-align: center;
  margin: 0.5rem 0 2rem;
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
