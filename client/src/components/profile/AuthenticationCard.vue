<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<template>
  <div
    class="authentication-card"
    :class="disabled ? 'authentication-card--disabled' : `authentication-card--${state}`"
  >
    <div class="image-container">
      <img
        :src="config.imageSrc"
        :alt="config.imageAlt"
        class="authentication-card__image"
      />
    </div>
    <div class="authentication-card-text">
      <ion-text class="authentication-card-text__title body-lg">{{ $msTranslate(config.methodName) }}</ion-text>
      <ion-text
        class="authentication-card-text__description body"
        v-if="config.description && showDescription"
      >
        {{ $msTranslate(config.description) }}
      </ion-text>
      <ion-text
        class="authentication-card-text__description body"
        v-if="config.unavailableExplanation && state === AuthenticationCardState.Unavailable"
      >
        {{ $msTranslate(config.unavailableExplanation) }}
      </ion-text>
    </div>
    <ion-icon
      v-if="state === AuthenticationCardState.Active"
      :icon="checkmarkCircle"
      class="authentication-card__icon"
    />
    <ion-text
      v-if="state === AuthenticationCardState.Disabled"
      class="authentication-card__info-active body"
    >
      {{ $msTranslate('Authentication.active') }}
    </ion-text>
    <ion-button
      class="authentication-card__update-button"
      v-if="state === AuthenticationCardState.Update"
      fill="clear"
      :disabled="disabled"
      @click="$emit('update-clicked')"
    >
      {{ $msTranslate('Authentication.update') }}
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import EllipsisGradient from '@/assets/images/ellipsis-gradient.svg';
import idCardGradient from '@/assets/images/id-card-gradient.svg';
import KeypadGradient from '@/assets/images/keypad-gradient.svg';
import personCircleGradient from '@/assets/images/person-circle-gradient.svg';
import { AuthenticationCardState } from '@/components/profile/types';
import { DeviceSaveStrategyTag, isDesktop, isKeyringAvailable, isWeb } from '@/parsec';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const props = defineProps<{
  state: AuthenticationCardState;
  authMethod: DeviceSaveStrategyTag;
  disabled?: boolean;
}>();

const showDescription = computed(() => {
  return ![AuthenticationCardState.Active, AuthenticationCardState.Current, AuthenticationCardState.Unavailable].includes(props.state);
});

const config = computed(() => methodConfig[props.authMethod]);

const keyringAvailable = ref(false);

onMounted(async () => {
  keyringAvailable.value = await isKeyringAvailable();
});

defineEmits<{
  (e: 'update-clicked'): void;
}>();

const methodConfig: Record<
  DeviceSaveStrategyTag,
  {
    imageSrc: string;
    imageAlt: string;
    methodName: Translatable;
    description?: Translatable;
    unavailableExplanation?: Translatable;
  }
> = {
  [DeviceSaveStrategyTag.Keyring]: {
    imageSrc: EllipsisGradient,
    imageAlt: 'Keyring',
    methodName: 'Authentication.method.system',
    unavailableExplanation: keyringUnavailableMessage(),
  },
  [DeviceSaveStrategyTag.Password]: {
    imageSrc: KeypadGradient,
    imageAlt: 'Password',
    methodName: 'Authentication.method.password',
  },
  [DeviceSaveStrategyTag.Smartcard]: {
    imageSrc: idCardGradient,
    imageAlt: 'Smartcard',
    methodName: 'Authentication.method.smartcard.title',
    description: 'Authentication.method.smartcard.description',
    unavailableExplanation: 'Authentication.method.smartcard.unavailable',
  },
  [DeviceSaveStrategyTag.PKI]: {
    imageSrc: idCardGradient,
    imageAlt: 'Smartcard',
    methodName: 'Authentication.method.smartcard.title',
    description: 'Authentication.method.smartcard.description',
    unavailableExplanation: 'Authentication.method.smartcard.unavailable',
  },
  [DeviceSaveStrategyTag.AccountVault]: {
    imageSrc: '',
    imageAlt: '',
    methodName: '',
    description: '',
  },
  [DeviceSaveStrategyTag.OpenBao]: {
    imageSrc: personCircleGradient,
    imageAlt: 'OpenBao',
    methodName: 'Authentication.method.sso.title',
    description: 'Authentication.method.sso.description',
  },
};

function keyringUnavailableMessage(): Translatable {
  if (isDesktop() && !keyringAvailable.value) {
    return 'Authentication.keyringUnavailableOnSystem';
  } else if (isWeb() && !keyringAvailable.value) {
    return 'Authentication.keyringUnavailableOnWeb';
  } else {
    return '';
  }
}
</script>

<style scoped lang="scss">
.authentication-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding-right: 1rem;
  border: 1px solid var(--parsec-color-light-secondary-medium);
  border-radius: var(--parsec-radius-12);
  box-shadow: var(--parsec-shadow-1);
  overflow: hidden;
  width: 100%;
  position: relative;
  z-index: 3;
  transition: all 0.2s ease-in-out;

  .image-container {
    background: var(--parsec-color-light-secondary-background);
    padding: 1.25rem 1rem;
    align-self: stretch;
    display: flex;
    align-items: center;
  }

  &__image {
    min-width: 1.5rem;
    object-fit: contain;
  }

  &-text {
    display: flex;
    flex-direction: column;
    width: 100%;
    padding: 0.25rem 0;

    &__title {
      color: var(--parsec-color-light-primary-700);
    }

    &__description {
      color: var(--parsec-color-light-secondary-grey);
      margin-top: -0.25rem;
    }
  }

  &__icon {
    font-size: 1.25rem;
    flex-shrink: 0;
    color: var(--parsec-color-light-primary-600);
    display: none;
  }

  &__info-active {
    color: var(--parsec-color-light-secondary-grey);
    margin-left: auto;
  }

  // Manage different states
  &--default {
    &:hover {
      border-color: var(--parsec-color-light-secondary-light);
    }
  }

  &--current {
    .authentication-card__icon {
      color: var(--parsec-color-light-primary-400);
      display: block;
    }
  }

  &--active {
    background: var(--parsec-color-light-secondary-premiere);

    .authentication-card__icon {
      color: var(--parsec-color-light-primary-400);
      display: block;
    }
  }

  &--selected {
    border-color: var(--parsec-color-light-primary-400);
  }

  &--update {
    border-color: var(--parsec-color-light-secondary-medium);

    .authentication-card__update-button {
      margin-left: auto;
      color: var(--parsec-color-light-secondary-text);
      --background-hover: var(--parsec-color-light-secondary-premiere);
    }
  }

  &--disabled {
    background: var(--parsec-color-light-secondary-premiere);

    .authentication-card__image {
      filter: grayscale(1);
    }

    .authentication-card-text__title {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  &--unavailable {
    background: var(--parsec-color-light-secondary-premiere);
    border-color: var(--parsec-color-light-secondary-premiere);
    cursor: not-allowed;

    .authentication-card__image {
      filter: grayscale(1);
      opacity: 0.5;
    }

    .authentication-card-text__title {
      color: var(--parsec-color-light-secondary-text);
      opacity: 0.7;
    }

    .authentication-card-text__description {
      color: var(--parsec-color-light-danger-700);
    }
  }
}
</style>
