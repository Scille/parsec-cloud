<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
          <div
            class="background-shape"
            id="top-left"
          />
          <div
            class="background-shape"
            id="top-right"
          />
          <div
            class="background-shape"
            id="center"
          />
          <!-- prettier-ignore -->
          <ms-image
            :image="(ResourcesManager.instance().get(Resources.LogoIcon, AppIconParsec) as string)"
            class="image-logo"
            :class="ResourcesManager.instance().get(Resources.LogoIcon, AppIconParsec) === Resources.LogoIcon ? 'image-logo--custom' : ''"
          />
        </div>
        <ion-text class="server-choice-item__label">
          {{ $msTranslate('CreateOrganization.server.saas') }}
        </ion-text>
        <ion-icon
          v-if="serverChoice === ServerType.Saas && isSmallDisplay"
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
          <div
            v-if="I18n.getLocale() === 'fr-FR'"
            class="trial-visual"
          >
            <img
              :src="isDarkTheme ? TrialFRDark : TrialFR"
              alt="trial"
            />
            <ion-text class="trial-visual__badge">{{ $msTranslate('CreateOrganization.server.trial.badge') }}</ion-text>
          </div>
          <img
            v-else
            :src="isDarkTheme ? TrialUSDark : TrialUS"
            alt="trial"
          />
        </div>
        <ion-text class="server-choice-item__label">
          {{ $msTranslate('CreateOrganization.server.trial.title') }}
          <span>{{ $msTranslate('CreateOrganization.server.trial.description') }}</span>
        </ion-text>
        <ion-icon
          v-if="serverChoice === ServerType.Trial && isSmallDisplay"
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
import TrialFRDark from '@/assets/images/trial-FR-dark.svg';
import TrialFR from '@/assets/images/trial-FR.svg';
import TrialUSDark from '@/assets/images/trial-US-dark.svg';
import TrialUS from '@/assets/images/trial-US.svg';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { Env } from '@/services/environment';
import { ServerType } from '@/services/parsecServers';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonButton, IonFooter, IonIcon, IonPage, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { I18n, MsImage, useTheme, useWindowSize } from 'megashark-lib';
import { ref } from 'vue';

const { isSmallDisplay } = useWindowSize();
const { isDarkTheme } = useTheme();

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
</script>

<style scoped lang="scss">
.server-page {
  padding: ms.spacing('padding-4xl');
  display: flex;
  width: 100%;

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-none');
  }

  &-footer {
    display: flex;
    justify-content: space-between;
    margin-top: 2.5rem;

    @include ms.responsive-breakpoint('sm') {
      position: sticky;
      bottom: 0;
      padding-bottom: ms.spacing('padding-5xl');
      right: -2rem;
      background: ms.color('surface-base-default');
      flex-direction: column-reverse;
      border-radius: ms.radius('2xl') ms.radius('2xl') ms.radius('none') ms.radius('none');
      box-shadow: ms.shadow('strong');
    }

    @media only screen and (max-height: 600px) {
      max-height: 25rem;
    }
  }
}

.server-choice {
  display: flex;
  padding: ms.spacing('padding-lg') ms.spacing('padding-5xl');
  justify-content: space-evenly;
  height: fit-content;

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-lg') ms.spacing('padding-4xl');
    flex-direction: column;
    margin: auto;
    gap: ms.spacing('gap-5xl');
    max-width: 35rem;
  }

  &-item {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    cursor: pointer;
    flex: 1;
    position: relative;
    align-items: center;

    @include ms.responsive-breakpoint('sm') {
      outline: ms.border('thin') solid ms.color('border-base-default');
      border-radius: ms.radius('2xl');
      gap: ms.spacing('gap-none');
    }

    &__checkmark {
      @include ms.responsive-breakpoint('sm') {
        position: absolute;
        top: 0.5rem;
        right: 2rem;
        color: ms.color('text-brand-default');
        font-size: 1.5rem;
        z-index: 3;
        top: 1rem;
        right: 1rem;
      }
    }

    &__image {
      width: 13.1875rem;
      height: 9.875rem;
      background-color: ms.color('surface-brand-default-subtle');
      border-radius: ms.radius('2xl');
      display: flex;
      justify-content: center;
      align-items: center;
      position: relative;
      overflow: hidden;

      @include ms.responsive-breakpoint('sm') {
        width: 100%;
        height: 8rem;
      }

      .background-shape {
        position: absolute;
        width: 9.1875rem;
        height: 6.9375rem;
        border-radius: ms.radius('full');
        box-shadow:
          0 0 5px 0 #{ms.color('shadow-neutral-lighter')},
          inset 0 4px 20px 0 #{ms.color('shadow-brand-moderate')};
        opacity: 0.7;
        mix-blend-mode: darken;

        body.ms-theme-dark & {
          box-shadow:
            0 0 5px 0 #{ms.color('shadow-neutral-lighter')},
            inset 0 4px 20px 0 #{ms.color('shadow-brand-lighter')};
          opacity: 1;
          mix-blend-mode: normal;
        }
      }

      #top-left {
        top: -4rem;
        left: -6.75rem;
        rotate: 30deg;
        z-index: 2;
        transition:
          left 0.3s,
          top 0.3s;
      }

      #top-right {
        top: -4.5rem;
        right: -5.75rem;
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
        bottom: -6rem;
        left: 1.125rem;
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

      .trial-visual {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: ms.spacing('gap-2xl');

        &__badge {
          @include ms.font('label-lg-medium');
          color: ms.color('text-brand-default-hover');
          border: ms.border('thin') solid ms.color('border-brand-default-hover');
          border-radius: ms.radius('2xl');
          padding: ms.spacing('padding-xs') ms.spacing('padding-lg');
        }
      }
    }

    &__label {
      @include ms.font('body-lg-medium');
      text-align: center;
      display: flex;
      flex-direction: column;
      color: ms.color('text-base-body');
      max-width: 15rem;
      line-height: 150%; /* 150% */
      gap: ms.spacing('gap-sm');

      span {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
      }

      @include ms.responsive-breakpoint('sm') {
        max-width: 100%;
        padding: ms.spacing('padding-3xl') ms.spacing('padding-4xl');
        text-align: center;
      }
    }

    &:hover {
      .server-choice-item__image {
        background-color: ms.color('surface-brand-default-subtle');

        @include ms.responsive-breakpoint('sm') {
          outline: none;
        }

        #top-left {
          top: -3rem;
          left: -5.25rem;
        }

        #top-right {
          top: -2.5rem;
          right: -4.75rem;
        }

        #center {
          bottom: -4rem;
          left: 0.125rem;
        }
      }
    }

    &.selected {
      @include ms.responsive-breakpoint('sm') {
        outline: ms.border('thick') solid ms.color('border-brand-default');
      }

      .server-choice-item__image {
        outline: ms.border('thick') solid ms.color('border-brand-default');
        background-color: ms.color('surface-brand-default-subtle');

        @include ms.responsive-breakpoint('sm') {
          outline: none;
        }

        #top-left {
          top: -3rem;
          left: -5.25rem;
        }

        #top-right {
          top: -2.5rem;
          right: -4.75rem;
        }

        #center {
          bottom: -4rem;
          left: 0.125rem;

          @include ms.responsive-breakpoint('sm') {
            bottom: -5.5rem;
            left: 2rem;
          }
        }
      }
      .server-choice-item__label {
        color: ms.color('text-brand-default');

        span {
          color: ms.color('text-error-default');
        }
      }
    }
  }
}
</style>
