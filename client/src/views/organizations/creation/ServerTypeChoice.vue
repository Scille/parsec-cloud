<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="server-modal">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.create"
      subtitle="CreateOrganization.subtitle.needs"
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
          <img
            src="@/assets/images/app-icon-parsec.svg"
            class="icon"
          />
        </div>
        <ion-text class="server-choice-item__label subtitles-normal">
          {{ $msTranslate('CreateOrganization.server.saas') }}
        </ion-text>
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
      </div>
    </div>

    <ion-footer class="server-modal-footer">
      <ion-button
        @click="$emit('serverChosen', ServerType.Custom)"
        fill="clear"
      >
        {{ $msTranslate('CreateOrganization.server.ownServer') }}
      </ion-button>
      <ion-button
        @click="onChoiceMade"
        :disabled="serverChoice === undefined"
      >
        {{ $msTranslate('CreateOrganization.button.continue') }}
      </ion-button>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonButton, IonText, IonFooter } from '@ionic/vue';
import { ref } from 'vue';
import { ServerType } from '@/services/parsecServers';
import { I18n } from 'megashark-lib';
import TrialUS from '@/assets/images/trial-US.svg';
import TrialFR from '@/assets/images/trial-FR.svg';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { Env } from '@/services/environment';

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
.server-modal {
  padding: 2.5rem;
  display: flex;
  height: auto;
  width: 100%;

  &-footer {
    display: flex;
    justify-content: space-between;
    margin-top: 2.5rem;
  }
}

.server-choice {
  display: flex;
  gap: 4rem;
  justify-content: center;

  &-item {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    cursor: pointer;
    flex: 1;
    position: relative;

    &:first-child {
      align-items: end;
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

      .icon {
        position: relative;
        z-index: 3;
      }
    }

    &__label {
      text-align: center;
      display: flex;
      flex-direction: column;
      color: var(--parsec-color-light-secondary-text);
      max-width: 12rem;
      line-height: 150%; /* 150% */
      gap: 0.25rem;

      span {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }

    &:hover {
      .server-choice-item__image {
        outline: 3px solid var(--parsec-color-light-primary-100);
        color: var(--parsec-color-light-secondary-grey);

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
      .server-choice-item__image {
        outline: 3px solid var(--parsec-color-light-primary-300);
        background-color: var(--parsec-color-light-primary-30);

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
