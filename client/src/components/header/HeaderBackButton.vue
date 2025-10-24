<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="back-button-container">
    <ion-button
      fill="clear"
      @click="onBackClick()"
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
import { Path } from '@/parsec';
import { currentRouteIs, getDocumentPath, getWorkspaceHandle, navigateTo, routerGoBack, Routes } from '@/router';
import { IonButton, IonIcon, IonLabel } from '@ionic/vue';
import { chevronBack } from 'ionicons/icons';
import { useWindowSize } from 'megashark-lib';

const { isLargeDisplay } = useWindowSize();

defineProps<{
  short: boolean;
}>();

async function onBackClick(): Promise<void> {
  console.log('[DEBUG] HeaderBackButton clicked');
  routerGoBack();

  // Possible fix for issue #11344
  // if (!currentRouteIs(Routes.FileHandler)) {
  //   routerGoBack();
  // } else {
  //   console.log('[DEBUG] FileHandler back navigation - navigating to documents');
  //   const workspaceHandle = getWorkspaceHandle();
  //   const path = getDocumentPath();
  //   if (workspaceHandle && path) {
  //     const parentPath = await Path.parent(path);
  //     await navigateTo(Routes.Documents, {
  //       query: {
  //         workspaceHandle: workspaceHandle,
  //         documentPath: parentPath,
  //       },
  //     });
  //   }
  // }
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
