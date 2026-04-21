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
      <ion-text
        class="authentication-card-text__header subtitles-sm"
        v-if="state === AuthenticationCardState.Update"
      >
        {{ $msTranslate('Authentication.methodChosen') }}
      </ion-text>
      <ion-text class="authentication-card-text__title title-h4">{{ $msTranslate(config.methodName) }}</ion-text>
      <ion-text
        class="authentication-card-text__description body"
        v-if="config.description && showDescription && state !== AuthenticationCardState.Update"
      >
        {{ $msTranslate(config.description) }}
      </ion-text>
      <ion-text
        class="authentication-card-text__description body"
        v-if="
          (config.unavailableExplanation && state === AuthenticationCardState.Unavailable) || state === AuthenticationCardState.Forbidden
        "
      >
        {{
          state === AuthenticationCardState.Forbidden
            ? $msTranslate('Authentication.forbidden')
            : $msTranslate(config.unavailableExplanation)
        }}
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
import { DevicePrimaryProtectionStrategyTag, isDesktop, isKeyringAvailable, isWeb } from '@/parsec';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const props = defineProps<{
  state: AuthenticationCardState;
  authMethod: DevicePrimaryProtectionStrategyTag;
}>();

const showDescription = computed(() => {
  return ![
    AuthenticationCardState.Active,
    AuthenticationCardState.Current,
    AuthenticationCardState.Unavailable,
    AuthenticationCardState.Forbidden,
  ].includes(props.state);
});

const disabled = computed(() => {
  return [AuthenticationCardState.Forbidden, AuthenticationCardState.Unavailable, AuthenticationCardState.Disabled].includes(props.state);
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
  DevicePrimaryProtectionStrategyTag,
  {
    imageSrc: string;
    imageAlt: string;
    methodName: Translatable;
    description?: Translatable;
    unavailableExplanation?: Translatable;
  }
> = {
  [DevicePrimaryProtectionStrategyTag.Keyring]: {
    imageSrc: EllipsisGradient,
    imageAlt: 'Keyring',
    methodName: 'Authentication.method.system',
    unavailableExplanation: keyringUnavailableMessage(),
  },
  [DevicePrimaryProtectionStrategyTag.Password]: {
    imageSrc: KeypadGradient,
    imageAlt: 'Password',
    methodName: 'Authentication.method.password',
  },
  [DevicePrimaryProtectionStrategyTag.PKI]: {
    imageSrc: idCardGradient,
    imageAlt: 'Smartcard',
    methodName: 'Authentication.method.smartcard.title',
    description: 'Authentication.method.smartcard.description',
    unavailableExplanation: 'Authentication.method.smartcard.unavailable',
  },
  [DevicePrimaryProtectionStrategyTag.AccountVault]: {
    imageSrc: '',
    imageAlt: '',
    methodName: '',
    description: '',
  },
  [DevicePrimaryProtectionStrategyTag.OpenBao]: {
    imageSrc: personCircleGradient,
    imageAlt: 'OpenBao',
    methodName: 'Authentication.method.sso.title',
    description: 'Authentication.method.sso.description',
    unavailableExplanation: 'Authentication.method.sso.unavailable',
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
  padding-right: 1rem;
  border-radius: var(--parsec-radius-18);
  --overflow: visible;
  width: 100%;
  position: relative;
  z-index: 3;
  transition: all 0.1s ease-in;

  .image-container {
    background: var(--parsec-color-light-secondary-premiere);
    margin: 1rem;
    padding: 0.5rem;
    align-self: stretch;
    border-radius: var(--parsec-radius-12);
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
    padding: 0.625rem 0;
    gap: 0.25rem;
    overflow: hidden;

    &__header {
      color: var(--parsec-color-light-secondary-grey);
    }

    &__title {
      color: var(--parsec-color-light-primary-700);
      font-weight: 500;
    }

    &__description {
      color: var(--parsec-color-light-secondary-hard-grey);
      margin-top: -0.25rem;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: wrap;
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
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);

    &:hover {
      background: var(--parsec-color-light-primary-30);

      .image-container {
        background-color: var(--parsec-color-light-primary-100);
      }
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
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);

    .authentication-card__icon {
      color: var(--parsec-color-light-primary-400);
      display: block;
    }
  }

  &--selected {
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);
    border-color: var(--parsec-color-light-primary-400);
  }

  &--update {
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);
    border-color: var(--parsec-color-light-secondary-medium);

    .authentication-card__update-button {
      margin-left: auto;
      color: var(--parsec-color-light-secondary-white);
      --background: var(--parsec-color-light-secondary-text);
      --background-hover: var(--parsec-color-light-secondary-contrast);

      &::part(native) {
        padding: 0.75rem 1.125rem;
      }
    }
  }

  &--disabled {
    background: var(--parsec-color-light-secondary-premiere);

    .authentication-card__image {
      filter: grayscale(1);
    }

    .authentication-card-text__title,
    .authentication-card-text__description {
      color: var(--parsec-color-light-secondary-text);
      opacity: 0.7;
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
