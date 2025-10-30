<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="button-list">
    <ion-button
      class="button-choice code"
      fill="outline"
      :disabled="disabled"
      @click="$emit('select', choices[0])"
    >
      {{ choices[0] }}
    </ion-button>
    <ion-button
      class="button-choice code"
      fill="outline"
      :disabled="disabled"
      @click="$emit('select', choices[1])"
    >
      {{ choices[1] }}
    </ion-button>
    <ion-button
      class="button-choice code"
      fill="outline"
      :disabled="disabled"
      @click="$emit('select', choices[2])"
    >
      {{ choices[2] }}
    </ion-button>
    <ion-button
      class="button-choice code"
      fill="outline"
      :disabled="disabled"
      @click="$emit('select', choices[3])"
    >
      {{ choices[3] }}
    </ion-button>
    <ion-button
      class="button-medium"
      id="noneChoicesButton"
      fill="clear"
      :disabled="disabled"
      @click="onNoneClicked"
    >
      {{ $msTranslate('SasCodeChoice.noneOfTheChoices') }}
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import { IonButton } from '@ionic/vue';
import { Answer, askQuestion } from 'megashark-lib';

defineProps<{
  choices: string[];
  disabled?: boolean;
}>();

const emits = defineEmits<{
  (e: 'select', value: string | null): void;
}>();

async function onNoneClicked(): Promise<void> {
  const answer = await askQuestion('SasCodeChoice.info.title', 'SasCodeChoice.info.subtitle', {
    yesText: 'SasCodeChoice.info.yesText',
    noText: 'SasCodeChoice.info.noText',
  });
  if (answer === Answer.Yes) {
    emits('select', null);
  }
}
</script>

<style scoped lang="scss">
.button-list {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  grid-template-rows: repeat(2, 1fr);
  grid-column-gap: 0px;
  grid-row-gap: 0px;
  gap: 1.25rem;
  align-self: center;
  width: 100%;
  max-width: 25rem;
  position: relative;

  @include ms.responsive-breakpoint('sm') {
    max-width: 100%;
    display: flex;
    flex-direction: column;
  }
}

ion-button {
  width: 100%;
  height: 4em;
  border-radius: var(--parsec-radius-8);

  &::part(native) {
    margin: auto;
    border: none;
  }

  &:last-child {
    grid-area: 3 / 1 / 4 / 3;
    position: relative;
  }

  @include ms.responsive-breakpoint('sm') {
    height: 4rem;
    margin: 0;
  }

  &:focus-within {
    outline: 2px solid var(--parsec-color-light-primary-400);
    outline-offset: 3px;
    background-color: var(--parsec-color-light-primary-100);
    border-color: var(--parsec-color-light-primary-600);
  }

  &:hover {
    background-color: var(--parsec-color-light-primary-100);
    border-color: var(--parsec-color-light-primary-600);
  }
}

.button-choice {
  background-color: var(--parsec-color-light-secondary-premiere);
  overflow: hidden;
  color: var(--parsec-color-light-primary-600);
  transition: all 0.2s ease;
  --background-focused: none;
  --background-focused-opacity: 0;
  --background-hover: none;
}
</style>
