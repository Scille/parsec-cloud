<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="page">
    <div>
      <ion-text class="title-h2">
        {{ $msTranslate('WebRedirectPage.title') }}
      </ion-text>
      <ion-text class="title-h3">
        {{ $msTranslate('WebRedirectPage.subtitle') }}
      </ion-text>
      <a
        :href="redirectLink"
        target="_blank"
        class="link"
      >
        {{ $msTranslate('WebRedirectPage.desktop') }}
      </a>
      <ion-text @click="openLinkInWeb()">
        {{ $msTranslate('WebRedirectPage.web') }}
      </ion-text>
      <ion-text>
        {{ $msTranslate('WebRedirectPage.downloadText') }}
      </ion-text>
      <a
        :href="$msTranslate('MenuPage.downloadParsecLink')"
        target="_blank"
        class="link"
      >
        {{ $msTranslate('WebRedirectPage.downloadLink') }}
      </a>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { getCurrentRouteQuery } from '@/router';
import { InformationManager, InformationManagerKey } from '@/services/informationManager';
import { handleParsecLink } from '@/services/linkHandling';
import { IonPage, IonText } from '@ionic/vue';
import { inject } from 'vue';

const informationManager: InformationManager = inject(InformationManagerKey)!;

const query = getCurrentRouteQuery();
const redirectLink = query.webRedirectUrl ?? '';

async function openLinkInWeb(): Promise<void> {
  await handleParsecLink(redirectLink, informationManager);
}
</script>

<style lang="scss" scoped>
</style>
