<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-image
    :image="UpdateRocket"
    class="update-rocket"
  />
  <ms-modal
    :close-button="{ visible: false }"
    class="update-app"
  >
    <div class="update-content">
      <ion-text class="update__title title-h2">
        {{ $msTranslate('HomePage.topbar.updateAvailable') }}
      </ion-text>
      <div class="update-main">
        <div class="update-version">
          <ion-text class="title-h3 update-version__item">
            {{ targetVersion }}
          </ion-text>
          <ion-button
            class="update-version__button"
            @click="openChangelog"
            fill="clear"
          >
            {{ $msTranslate('HomePage.topbar.changelog') }}
          </ion-button>
        </div>
        <div class="update-message body">
          <ion-text class="update-message__curent-version">
            {{ $msTranslate('HomePage.topbar.currentVersion') }}<span class="subtitles-sm">{{ currentVersion ?? APP_VERSION }}</span>
          </ion-text>
          <ion-text>
            {{ $msTranslate('HomePage.topbar.updateConfirmQuestion') }}
          </ion-text>
        </div>
      </div>
      <div class="update-footer">
        <ion-button
          @click="dismiss(MsModalResult.Confirm)"
          class="update-footer__button"
        >
          {{ $msTranslate('HomePage.topbar.updateYes') }}
        </ion-button>
        <ion-button
          @click="dismiss(MsModalResult.Cancel)"
          class="update-footer__button"
        >
          {{ $msTranslate('HomePage.topbar.updateNo') }}
        </ion-button>
      </div>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { APP_VERSION, Env } from '@/services/environment';
import { IonButton, IonText, modalController } from '@ionic/vue';
import { MsImage, MsModal, MsModalResult, UpdateRocket } from 'megashark-lib';

const props = defineProps<{
  currentVersion?: string;
  targetVersion: string;
}>();

async function openChangelog(): Promise<void> {
  await Env.Links.openChangelogLink(props.targetVersion);
}

async function dismiss(role: MsModalResult): Promise<void> {
  await modalController.dismiss(undefined, role);
}
</script>

<style lang="scss" scoped>
.update-rocket {
  background: var(--parsec-color-light-primary-30);
  margin: -0.25rem;
}

.update-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;

  .update__title {
    text-align: center;
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-image: var(--parsec-color-light-gradient-background);
  }

  .update-main {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;

    .update-version {
      display: flex;
      justify-content: center;
      align-items: center;
      display: flex;
      gap: 0.5rem;
      width: fit-content;
      margin: auto;

      &__item {
        background: var(--parsec-color-light-secondary-background);
        border-radius: var(--parsec-radius-8);
        color: var(--parsec-color-light-secondary-text);
        padding: 0.5rem 1.25rem;
      }

      &__button {
        --color: var(--parsec-color-light-secondary-text);
        --background-hover: transparent;
        --padding-end: 0.5rem;
        --padding-start: 0.5rem;

        &:hover {
          text-decoration: underline;
        }
      }
    }

    .update-message {
      color: var(--parsec-color-light-secondary-hard-grey);
      display: flex;
      flex-direction: column;

      &__curent-version span {
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }

  .update-footer {
    display: flex;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 0.5rem;

    &__button {
      margin: auto;
      width: fit-content;

      &:nth-child(1) {
        --background: var(--parsec-color-light-gradient-background);
        --padding-end: 2rem;
        --padding-start: 2rem;
        --padding-bottom: 0.75rem;
        --padding-top: 0.75rem;
      }

      &:nth-child(2) {
        position: absolute;
        right: 2.5rem;
        --background: transparent;
        --background-hover: var(--parsec-color-light-secondary-premiere);
        --color: var(--parsec-color-light-secondary-text);
        --padding-end: 0.5rem;
        --padding-start: 0.5rem;
      }
    }
  }
}
</style>
