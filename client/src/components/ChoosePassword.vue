<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="info-password">
    <img
      src="@/assets/images/password.svg"
      alt="password-image"
      class="info-password__img"
    >
    <ion-text
      class="subtitles-sm info-password__text"
    >
      {{ $t('Password.passwordInfo') }}
    </ion-text>
  </div>
  <custom-input
    :label="$t('Password.password')"
    v-model="password"
    name="password"
    type="password"
  />
  <custom-input
    :label="$t('Password.confirmPassword')"
    v-model="passwordConfirm"
    name="confirmPassword"
    type="password"
  />
  <span
    class="error-message"
    v-if="password !== passwordConfirm"
  >
    {{ $t('Password.noMatch') }}
  </span>
  <div
    class="container-password-level"
    v-show="getPasswordStrength(password) !== PasswordStrength.None"
  >
    <div
      class="password-level"
      :class="getPasswordLevelClass()"
    >
      <div class="password-level-bar">
        <div class="bar-item" />
        <div class="bar-item" />
        <div class="bar-item" />
      </div>
      <ion-text
        class="subtitles-sm password-level__text"
      >
        {{ getPasswordStrengthText($t, getPasswordStrength(password)) }}
      </ion-text>
    </div>
    <ion-text
      class="subtitles-sm password-criteria"
    >
      {{ $t('Password.passwordCriteria') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import { ref } from 'vue';
import CustomInput from '@/components/CustomInput.vue';
import { PasswordStrength, getPasswordStrength, getPasswordStrengthText } from '@/common/passwordValidation';

const password = ref('');
const passwordConfirm = ref('');

defineExpose({
  areFieldsCorrect,
  password
});

function areFieldsCorrect(): boolean {
  return getPasswordStrength(password.value) === PasswordStrength.High && password.value === passwordConfirm.value;
}

function getPasswordLevelClass() : string {
  const strength = getPasswordStrength(password.value);

  if (strength === PasswordStrength.Low) {
    return 'password-level-low';
  } else if (strength === PasswordStrength.Medium) {
    return 'password-level-medium';
  } else if (strength === PasswordStrength.High) {
    return 'password-level-high';
  }
  return '';
}
</script>

<style scoped lang="scss">
.info-password {
  background: var(--parsec-color-light-primary-30);
  display: flex;
  align-items: center;
  gap: .625rem;
  width: 100%;
  padding: .625rem;
  border-radius: var(--parsec-radius-6);
  margin-bottom: .5rem;

  &__img {
    width: 3.25rem;
    height: 3.25rem;
    margin-right: .5rem;
  }

  &__text {
    color: var(--parsec-color-light-secondary-text);
  }
}

.container-password-level {
  display: flex;
  flex-direction: column;
  gap: .5rem;
}

.password-level {
  display: flex;
  align-items: center;
  gap: 1rem;

  &__text {
    color: var(--parsec-color-light-secondary-text);
  }

  &-bar {
    display: flex;
    gap: .5rem;

    .bar-item {
      width: 3rem;
      height: .375rem;
      border-radius: var(--parsec-radius-6);
      background: var(--parsec-color-light-primary-50);
      position: relative;
    }
  }

  &.password-level-low {
    .password-level__text {
      color: var(--parsec-color-light-danger-500);
    }

    .bar-item:nth-child(-n+1) {
      background: var(--parsec-color-light-danger-500);
    }
  }

  &.password-level-medium {
    .password-level__text {
      color: var(--parsec-color-light-warning-500);
    }

    .bar-item:nth-child(-n+2) {
      background: var(--parsec-color-light-warning-500);
    }
  }

  &.password-level-high {
    .password-level__text {
      color: var(--parsec-color-light-success-500);
    }

    .bar-item:nth-child(-n+3) {
      background: var(--parsec-color-light-success-500);
    }
  }
}

.password-criteria {
  color: var(--parsec-color-light-secondary-grey);
}
</style>
