<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="back-button-container">
    <ion-button
      :fill="isSmallDisplay ? 'outline' : 'clear'"
      @click="goBack()"
      class="back-button"
      :slot="short || isSmallDisplay ? 'icon-only' : ''"
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

const { isLargeDisplay, isSmallDisplay } = useWindowSize();
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
    box-shadow: ms.shadow('light');
  }

  &::part(native) {
    padding: ms.spacing('padding-lg');
  }

  &__icon {
    font-size: 1.375rem;
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
  border-right: ms.border('thin') solid ms.color('border-base-default');
  height: 1.5rem;
  margin: 0 1rem;
}
</style>
