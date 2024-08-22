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
        @click="serverChoice = ServerType.Saas"
        :class="{ selected: serverChoice === ServerType.Saas }"
      >
        <!-- <ms-image
          :image="ParsecMockup"
          class="server-choice-item__image"
        /> -->
        <!-- should be replaced with megashark-lib -->
        <div class="server-choice-item__image">
          <img
            src="@/assets/images/mockup.svg"
            alt="trial"
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
        <!-- <ms-image
          :image="Trial"
          class="server-choice-item__image"
        /> -->
        <!-- should be replaced with megashark-lib -->
        <div class="server-choice-item__image">
          <img
            src="@/assets/images/trial.svg"
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
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';

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
      }
    }

    &.selected {
      .server-choice-item__image {
        outline: 3px solid var(--parsec-color-light-primary-300);
        background-color: var(--parsec-color-light-primary-30);
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
