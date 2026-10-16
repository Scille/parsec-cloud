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
        class="authentication-card-text__header"
        v-if="state === AuthenticationCardState.Update"
      >
        {{ $msTranslate('Authentication.methodChosen') }}
      </ion-text>
      <ion-text class="authentication-card-text__title">{{ $msTranslate(config.methodName) }}</ion-text>
      <ion-text
        class="authentication-card-text__description"
        v-if="config.description && showDescription && state !== AuthenticationCardState.Update"
      >
        {{ $msTranslate(config.description) }}
      </ion-text>
      <ion-text
        class="authentication-card-text__description"
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
      class="authentication-card__info-active"
    >
      {{ $msTranslate('Authentication.active') }}
    </ion-text>
    <ion-button
      class="authentication-card__update-button"
      v-if="state === AuthenticationCardState.Update"
      :disabled="disabled"
      @click="$emit('update-clicked')"
      fill="outline"
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
import { DevicePrimaryProtectionStrategyTag, isKeyringAvailable, isWeb } from '@/parsec';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const props = defineProps<{
  state: AuthenticationCardState;
  authMethod: DevicePrimaryProtectionStrategyTag;
}>();

const showDescription = computed(() => {
  return ![AuthenticationCardState.Current, AuthenticationCardState.Unavailable, AuthenticationCardState.Forbidden].includes(props.state);
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
  if (isWeb() && !keyringAvailable.value) {
    return 'Authentication.keyringUnavailableOnWeb';
  }
  return 'Authentication.keyringUnavailableOnSystem';
}
</script>

<style scoped lang="scss">
.authentication-card {
  background: ms.color('surface-base-page');
  border: 1px solid ms.color('border-neutral-default-subtle');
  display: flex;
  align-items: center;
  padding-right: ms.spacing('padding-2xl');
  border-radius: ms.radius('4xl');
  --overflow: visible;
  width: 100%;
  position: relative;
  z-index: 3;
  transition: all 0.1s ease-in;

  .image-container {
    background: ms.color('surface-base-default-secondary');
    box-shadow: ms.shadow('input');
    margin: 0.75rem;
    padding: ms.spacing('padding-lg');
    border-radius: ms.radius('2xl');
    display: flex;
    align-items: center;
  }

  &__image {
    min-width: 1.25rem;
    object-fit: contain;
  }

  &-text {
    display: flex;
    flex-direction: column;
    width: 100%;
    padding: ms.spacing('padding-xl') ms.spacing('padding-none');
    gap: ms.spacing('gap-xs');
    overflow: hidden;

    &__header {
      @include ms.font('body-md-medium');
      color: ms.color('text-base-description');
    }

    &__title {
      @include ms.font('body-lg-medium');
      color: ms.color('text-label-default');
    }

    &__description {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');
      margin-top: -0.25rem;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: wrap;
    }
  }

  &__icon {
    font-size: 1.25rem;
    flex-shrink: 0;
    color: ms.color('icon-brand-default-hover');
    display: none;
  }

  &__info-active {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-description');
    margin-left: auto;
  }

  // Manage different states
  &--default {
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);

    &:hover {
      background: ms.color('surface-neutral-default-subtle-hover');
    }
  }

  &--current {
    .authentication-card__icon {
      color: ms.color('icon-brand-default');
      display: block;
    }
  }

  &--active {
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);

    &:hover {
      background: ms.color('surface-brand-default-subtle');

      .image-container {
        background-color: ms.color('surface-brand-default-subtle-pressed');
      }
    }

    .authentication-card__icon {
      color: ms.color('icon-brand-default');
      display: block;
    }
  }

  &--selected {
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);
    border-color: ms.color('border-brand-default');
  }

  &--update {
    background: ms.color('surface-base-default-secondary');
    box-shadow:
      0 1px 1px 0 rgba(0, 0, 0, 0.05),
      0 1px 4px 0 rgba(0, 0, 0, 0.03),
      0 0 1px 0 rgba(0, 0, 0, 0.2);
    border-color: ms.color('border-neutral-default-subtle');

    .image-container {
      background: ms.color('surface-base-default');
    }

    .authentication-card__update-button {
      margin-left: auto;
      color: ms.color('text-base-body');
      --background-hover: #{ms.color('surface-base-page-secondary')};

      &::part(native) {
        border-color: ms.color('border-base-default');
        padding: ms.spacing('padding-2xl') 1.125rem;
      }
    }
  }

  &--disabled {
    background: ms.color('surface-base-page-secondary');
    opacity: ms.opacity('disabled');

    .authentication-card__image {
      filter: grayscale(1);
    }

    .authentication-card-text__title,
    .authentication-card-text__description {
      color: ms.color('text-base-body');
      opacity: ms.opacity('7');
    }
  }

  &--unavailable {
    background: ms.color('surface-base-page-secondary');
    border-color: ms.color('border-neutral-default-subtle');
    cursor: not-allowed;

    .authentication-card__image {
      filter: grayscale(1);
      opacity: ms.opacity('5');
    }

    .authentication-card-text__title {
      color: ms.color('text-base-body');
      opacity: ms.opacity('7');
    }

    .authentication-card-text__description {
      color: ms.color('text-error-default-hover');
    }
  }
}
</style>
