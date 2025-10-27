<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="page">
    <div class="redirect-container">
      <ms-image
        :image="LogoIconGradient"
        class="redirect-logo"
      />

      <div class="redirect-text">
        <ion-text class="redirect-text__title title-h1">
          {{ $msTranslate('WebRedirectPage.title') }}
        </ion-text>
        <ion-text class="redirect-text__subtitle body-lg">
          {{ $msTranslate('WebRedirectPage.subtitle') }}
        </ion-text>
      </div>

      <div class="redirect-buttons">
        <ion-button class="redirect-buttons__item button-large button-primary">
          <a
            :href="redirectLink"
            target="_blank"
            class="link"
          >
            {{ $msTranslate('WebRedirectPage.desktop') }}
          </a>
        </ion-button>
        <ion-button
          @click="openLinkInWeb()"
          class="redirect-buttons__item button-large button-default"
          fill="clear"
        >
          {{ $msTranslate('WebRedirectPage.web') }}
        </ion-button>
      </div>

      <div class="redirect-download">
        <ion-text class="redirect-download__text body">
          {{ $msTranslate('WebRedirectPage.downloadText') }}
        </ion-text>
        <a
          :href="$msTranslate('MenuPage.downloadParsecLink')"
          target="_blank"
          class="redirect-download__link button-medium"
        >
          {{ $msTranslate('WebRedirectPage.downloadLink') }}
        </a>
      </div>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { getCurrentRouteQuery } from '@/router';
import { InformationManager, InformationManagerKey } from '@/services/informationManager';
import { handleParsecLink } from '@/services/linkHandler';
import { IonButton, IonPage, IonText } from '@ionic/vue';
import { LogoIconGradient, MsImage } from 'megashark-lib';
import { inject } from 'vue';

const informationManager: InformationManager = inject(InformationManagerKey)!;

const query = getCurrentRouteQuery();
const redirectLink = query.webRedirectUrl ?? '';

async function openLinkInWeb(): Promise<void> {
  await handleParsecLink(redirectLink, informationManager);
}
</script>

<style lang="scss" scoped>
.page {
  background-color: var(--parsec-color-light-secondary-white);
}

.redirect-container {
  max-width: 28rem;
  display: flex;
  margin: auto;
  width: 100%;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

.redirect-logo {
  width: 4rem;
  margin-bottom: 1.75rem;
}

.redirect-text {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;

  &__title {
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-image: var(--parsec-color-light-gradient-background);
  }

  &__subtitle {
    color: var(--parsec-color-light-secondary-hard-grey);
    text-align: center;
  }
}

.redirect-buttons {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  max-width: 22rem;
  width: 100%;

  &__item {
    width: 100%;

    .link {
      color: var(--parsec-color-light-secondary-white);
    }
  }
}

.redirect-download {
  margin-top: 2.5rem;
  padding-top: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  border-top: 1px solid var(--parsec-color-light-secondary-medium);

  &__text {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &__link {
    color: var(--parsec-color-light-primary-500);
    transition: all 0.2s ease-in-out;

    &:hover {
      text-decoration: underline;
      color: var(--parsec-color-light-primary-600);
    }
  }
}
</style>
