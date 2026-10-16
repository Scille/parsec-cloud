<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-image
    :image="isDarkTheme ? UpdateRocketDark : UpdateRocket"
    class="update-rocket"
  />
  <ms-modal
    :close-button="{ visible: false }"
    class="update-app"
  >
    <div class="update-content">
      <ion-text class="update__title">
        {{ $msTranslate('HomePage.topbar.updateAvailable') }}
      </ion-text>
      <div class="update-main">
        <div class="update-version">
          <ion-text
            class="update-version__item"
            @click="openChangelog"
            :title="$msTranslate('HomePage.topbar.changelog')"
          >
            {{ targetVersion }}
          </ion-text>
        </div>
        <div class="update-message">
          <span class="update-message__badge">{{ $msTranslate('HomePage.topbar.recommended') }}</span>
          <ion-text class="update-message__curent-version">
            {{ $msTranslate('HomePage.topbar.currentVersion') }}<span>{{ currentVersion ?? APP_VERSION }}</span>
          </ion-text>
          <ion-text class="update-message__question">
            {{ $msTranslate('HomePage.topbar.updateConfirmQuestion') }}
          </ion-text>
        </div>
      </div>
      <div class="update-footer">
        <ion-button
          @click="dismiss(MsModalResult.Confirm)"
          class="update-footer__button button-large"
        >
          {{ $msTranslate('HomePage.topbar.updateYes') }}
        </ion-button>
        <ion-button
          @click="dismiss(MsModalResult.Cancel)"
          class="update-footer__button button-large"
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
import { MsImage, MsModal, MsModalResult, UpdateRocket, UpdateRocketDark, useTheme } from 'megashark-lib';

const props = defineProps<{
  currentVersion?: string;
  targetVersion: string;
}>();

const { isDarkTheme } = useTheme();

async function openChangelog(): Promise<void> {
  await Env.Links.openChangelogLink(props.targetVersion);
}

async function dismiss(role: MsModalResult): Promise<void> {
  await modalController.dismiss(undefined, role);
}
</script>

<style lang="scss" scoped>
.update-rocket {
  background: ms.color('surface-brand-default-subtle');
  margin: -0.25rem;
}

.update-content {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');

  .update__title {
    @include ms.font('heading-h3');
    text-align: center;
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-image: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));
  }

  .update-main {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-4xl');

    .update-version {
      display: flex;
      justify-content: center;
      align-items: center;
      gap: ms.spacing('gap-lg');
      width: fit-content;
      margin: auto;

      &__item {
        @include ms.font('heading-h4');
        background: ms.color('surface-base-page');
        border-radius: ms.radius('lg');
        color: ms.color('text-base-body');
        padding: ms.spacing('padding-lg') 1.25rem;
        transition: outline 100ms ease-in-out;
        border: ms.border('thin') solid ms.color('border-base-default');

        &:hover {
          cursor: pointer;
          outline: ms.border('thin') solid ms.color('border-brand-default-hover');
          outline-offset: 2px;
          color: ms.color('text-brand-default-hover');
        }
      }
    }

    .update-message {
      color: ms.color('text-base-body');
      display: flex;
      flex-direction: column;

      &__question {
        @include ms.font('body-lg-regular');
      }

      &__badge {
        @include ms.font('label-md-medium');
        background: ms.color('surface-brand-default-subtle-hover');
        width: fit-content;
        color: ms.color('text-brand-default');
        padding: ms.spacing('padding-sm') ms.spacing('padding-2xl');
        border-radius: ms.radius('lg');
        margin-bottom: 0.5rem;
      }

      &__curent-version {
        @include ms.font('body-lg-regular');
      }

      &__curent-version span {
        @include ms.font('body-lg-medium');
        color: ms.color('text-base-body');
      }
    }
  }

  .update-footer {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: ms.spacing('gap-lg');

    &__button {
      @include ms.font('label-lg-medium');
      margin: auto;
      width: 100%;

      &:nth-child(1) {
        --background: linear-gradient(#{ms.color('surface-gradient-from')}, #{ms.color('surface-gradient-to')});
        --padding-end: #{ms.spacing('padding-5xl')};
        --padding-start: #{ms.spacing('padding-5xl')};
        --padding-bottom: #{ms.spacing('padding-2xl')};
        --padding-top: #{ms.spacing('padding-2xl')};
      }

      &:nth-child(2) {
        --background: transparent;
        --background-hover: #{ms.color('surface-neutral-default-subtle-hover')};
        --color: #{ms.color('text-base-body')};
        --padding-end: #{ms.spacing('padding-lg')};
        --padding-start: #{ms.spacing('padding-lg')};
      }
    }
  }
}
</style>
