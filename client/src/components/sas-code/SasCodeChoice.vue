<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      id="noneChoicesButton"
      class="color-secondary"
      fill="outline"
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

#noneChoicesButton {
  grid-column: 1 / -1;
  width: 100%;
}

.button-choice {
  border-radius: ms.radius('lg');
  font-family: var(--parsec-font-family-paragraph);
  font-size: var(--parsec-root-font-size-2xl) !important;
  font-weight: var(--parsec-font-weight-bold);
  letter-spacing: 9px;
  text-transform: uppercase;
  height: 4em;
  --color: #{ms.color('text-base-label')};
  --color-hover: #{ms.color('text-base-label')};
  --color-focused: #{ms.color('text-base-label')};
  --background: #{ms.color('surface-neutral-default-subtle')};
  --background-hover-opacity: 0;
  --background-focused-opacity: 0;
  --background-activated: #{ms.color('surface-neutral-default-subtle-pressed')};

  &::part(native) {
    border: ms.border('thin') solid ms.color('border-base-default');
  }

  @include ms.responsive-breakpoint('sm') {
    height: 4rem;
    margin: 0;
  }

  &:hover {
    --background: #{ms.color('surface-neutral-default-subtle-hover')};

    &::part(native) {
      border-color: ms.color('border-base-default-hover');
    }
  }

  &.ion-focused {
    --background: #{ms.color('surface-neutral-default-subtle-pressed')};

    &::part(native) {
      border-color: transparent;
      box-shadow: ms.shadow('focus-brand');
    }
  }
}
</style>
