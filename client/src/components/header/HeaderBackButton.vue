<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="back-button-container">
    <ion-button
      fill="clear"
      @click="goBack()"
      class="back-button"
    >
      <ion-icon
        :icon="chevronBack"
        class="back-button__icon"
      />
      <ion-label
        v-if="!short && isLargeDisplay"
        class="back-button__label"
      >
        {{ $msTranslate('HeaderPage.previous') }}
      </ion-label>
    </ion-button>

    <div
      v-if="short && isLargeDisplay"
      class="vertical-spacer"
    />
  </div>
</template>

<script setup lang="ts">
import { routerGoBack } from '@/router';
import useFileOpener from '@/services/pathOpener';
import { IonButton, IonIcon, IonLabel } from '@ionic/vue';
import { chevronBack } from 'ionicons/icons';
import { useWindowSize } from 'megashark-lib';

const { isLargeDisplay } = useWindowSize();
const fileOpener = useFileOpener();

defineProps<{
  short: boolean;
}>();

async function goBack(): Promise<void> {
  fileOpener.pathOpened();
  await routerGoBack();
}
</script>

<style scoped lang="scss">
.back-button {
  margin-inline: 0px;
  margin-top: 0px;
  margin-bottom: 0px;
  min-height: 0;
  flex-shrink: 0;

  @include ms.responsive-breakpoint('sm') {
    background: var(--parsec-color-light-secondary-premiere);
    border-radius: var(--parsec-radius-8);
  }

  &::part(native) {
    padding: 0.5rem;

    @include ms.responsive-breakpoint('sm') {
      padding: 0.375rem;
    }
  }

  &__icon {
    @include ms.responsive-breakpoint('sm') {
      color: var(--parsec-color-light-secondary-hard-grey);
    }
  }

  &__label {
    margin-left: 0.625rem;
  }
}

.back-button-container {
  display: flex;
  align-items: center;
}

.vertical-spacer {
  display: block;
  border-right: 1px solid var(--parsec-color-light-secondary-light);
  height: 1.5rem;
  margin: 0 1rem;
}
</style>
