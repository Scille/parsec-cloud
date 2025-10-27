<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="server-page">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.create"
      subtitle="CreateOrganization.subtitle.needs"
      :small-display-stepper="true"
    />
    <div class="server-choice">
      <!-- Saas -->
      <div
        class="server-choice-item"
        v-show="!Env.isStripeDisabled()"
        @click="serverChoice = ServerType.Saas"
        :class="{ selected: serverChoice === ServerType.Saas }"
      >
        <div class="server-choice-item__image">
          <img
            src="@/assets/images/background/background-shapes-small.svg"
            id="top-left"
          />
          <img
            src="@/assets/images/background/background-shapes-small.svg"
            id="top-right"
          />
          <img
            src="@/assets/images/background/background-shapes-small.svg"
            id="center"
          />
          <!-- prettier-ignore -->
          <ms-image
            :image="(ResourcesManager.instance().get(Resources.LogoIcon, AppIconParsec) as string)"
            class="image-logo"
            :class="ResourcesManager.instance().get(Resources.LogoIcon, AppIconParsec) === Resources.LogoIcon ? 'image-logo--custom' : ''"
          />
        </div>
        <ion-text class="server-choice-item__label subtitles-normal">
          {{ $msTranslate('CreateOrganization.server.saas') }}
        </ion-text>
        <ion-icon
          v-if="serverChoice === ServerType.Saas"
          class="server-choice-item__checkmark"
          :icon="checkmarkCircle"
        />
      </div>

      <!-- Trial -->
      <div
        class="server-choice-item"
        @click="serverChoice = ServerType.Trial"
        :class="{ selected: serverChoice === ServerType.Trial }"
      >
        <div class="server-choice-item__image">
          <img
            :src="getImagePath()"
            alt="trial"
          />
        </div>
        <ion-text class="server-choice-item__label subtitles-normal">
          {{ $msTranslate('CreateOrganization.server.trial.title') }}
          <span class="body">{{ $msTranslate('CreateOrganization.server.trial.description') }}</span>
        </ion-text>
        <ion-icon
          v-if="serverChoice === ServerType.Trial"
          class="server-choice-item__checkmark"
          :icon="checkmarkCircle"
        />
      </div>
    </div>

    <ion-footer class="server-page-footer">
      <ion-button
        @click="$emit('serverChosen', ServerType.Custom)"
        fill="clear"
        class="button-large"
      >
        {{ $msTranslate('CreateOrganization.server.ownServer') }}
      </ion-button>
      <ion-button
        @click="onChoiceMade"
        size="large"
        :disabled="serverChoice === undefined"
        class="button-large"
      >
        {{ $msTranslate('CreateOrganization.button.continue') }}
      </ion-button>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import AppIconParsec from '@/assets/images/app-icon-parsec.svg?raw';
import TrialFR from '@/assets/images/trial-FR.svg';
import TrialUS from '@/assets/images/trial-US.svg';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { Env } from '@/services/environment';
import { ServerType } from '@/services/parsecServers';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonButton, IonFooter, IonIcon, IonPage, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { I18n, MsImage } from 'megashark-lib';
import { ref } from 'vue';

const emits = defineEmits<{
  (e: 'serverChosen', serverType: ServerType): void;
  (e: 'closeRequested'): void;
}>();

const serverChoice = ref<ServerType | undefined>(undefined);

async function onChoiceMade(): Promise<void> {
  if (serverChoice.value === undefined) {
    return;
  }
  emits('serverChosen', serverChoice.value);
}

function getImagePath(): string {
  const locale = I18n.getLocale();
  switch (locale) {
    case 'fr-FR':
      return TrialFR;
    default:
      return TrialUS;
  }
}
</script>

<style scoped lang="scss">
.server-page {
  padding: 2rem;
  display: flex;
  width: 100%;

  @include ms.responsive-breakpoint('sm') {
    padding: 0;
  }

  &-footer {
    display: flex;
    justify-content: space-between;
    margin-top: 2.5rem;

    @include ms.responsive-breakpoint('sm') {
      position: sticky;
      bottom: 0;
      padding-bottom: 2rem;
      right: -2rem;
      background: var(--parsec-color-light-secondary-white);
      flex-direction: column-reverse;
      border-radius: var(--parsec-radius-12) var(--parsec-radius-12) 0 0;
      box-shadow: var(--parsec-shadow-strong);
    }

    @media only screen and (max-height: 600px) {
      max-height: 25rem;
    }
  }
}

.server-choice {
  display: flex;
  padding: 0.5rem 2rem;
  justify-content: space-evenly;
  height: fit-content;

  @include ms.responsive-breakpoint('sm') {
    padding: 0.5rem 1.5rem;
    flex-direction: column;
    margin: auto;
    gap: 2rem;
    max-width: 35rem;
  }

  &-item {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    cursor: pointer;
    flex: 1;
    position: relative;
    align-items: center;

    @include ms.responsive-breakpoint('sm') {
      outline: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-12);
      gap: 0;
    }

    &__checkmark {
      position: absolute;
      top: 0.5rem;
      right: 2rem;
      color: var(--parsec-color-light-primary-600);
      font-size: 1.5rem;
      z-index: 3;

      @include ms.responsive-breakpoint('sm') {
        top: 1rem;
        right: 1rem;
      }
    }

    &__image {
      width: 12em;
      height: 12em;
      background-color: var(--parsec-color-light-secondary-premiere);
      border-radius: var(--parsec-radius-12);
      display: flex;
      justify-content: center;
      align-items: center;
      position: relative;
      overflow: hidden;
      transition: all 0.1s ease-out;

      @include ms.responsive-breakpoint('sm') {
        width: 100%;
        height: 8rem;
      }

      #top-left {
        position: absolute;
        top: -4rem;
        left: -7rem;
        rotate: 30deg;
        z-index: 2;
        transition:
          left 0.3s,
          top 0.3s;
      }

      #top-right {
        position: absolute;
        top: -5rem;
        right: -6rem;
        rotate: -30deg;
        z-index: 2;
        transition:
          right 0.3s,
          top 0.3s;

        @include ms.responsive-breakpoint('sm') {
          top: -3rem;
          right: -4rem;
        }
      }

      #center {
        position: absolute;
        bottom: -6.5rem;
        left: 2rem;
        rotate: 30deg;
        z-index: 2;
        transition:
          left 0.3s,
          bottom 0.3s;
      }

      .image-logo {
        position: relative;
        z-index: 3;

        &--custom {
          max-width: 5rem;
          height: auto;
        }
      }
    }

    &__label {
      text-align: center;
      display: flex;
      flex-direction: column;
      color: var(--parsec-color-light-secondary-text);
      max-width: 15rem;
      line-height: 150%; /* 150% */
      gap: 0.25rem;

      span {
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      @include ms.responsive-breakpoint('sm') {
        max-width: 100%;
        padding: 1rem 1.5rem;
        text-align: center;
      }
    }

    &:hover {
      @include ms.responsive-breakpoint('sm') {
        outline: 2px solid var(--parsec-color-light-primary-300);
      }

      .server-choice-item__image {
        outline: 2px solid var(--parsec-color-light-primary-300);
        background-color: var(--parsec-color-light-primary-30);

        @include ms.responsive-breakpoint('sm') {
          outline: none;
        }

        #top-left {
          top: -3rem;
          left: -5.5rem;
        }

        #top-right {
          top: -3rem;
          right: -5rem;
        }

        #center {
          bottom: -4.5rem;
          left: 1rem;
        }
      }
    }

    &.selected {
      @include ms.responsive-breakpoint('sm') {
        outline: 2px solid var(--parsec-color-light-primary-600);
      }

      .server-choice-item__image {
        outline: 2px solid var(--parsec-color-light-primary-600);
        background-color: var(--parsec-color-light-primary-30);

        @include ms.responsive-breakpoint('sm') {
          outline: none;
        }

        #top-left {
          top: -3rem;
          left: -5.5rem;
        }

        #top-right {
          top: -3rem;
          right: -5rem;
        }

        #center {
          bottom: -4.5rem;
          left: 1rem;

          @include ms.responsive-breakpoint('sm') {
            bottom: -5.5rem;
            left: 2rem;
          }
        }
      }
      .server-choice-item__label {
        color: var(--parsec-color-light-primary-600);

        span {
          color: var(--parsec-color-light-danger-700);
        }
      }
    }
  }
}
</style>
