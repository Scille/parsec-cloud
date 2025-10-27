<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-login">
    <ion-button
      v-show="!hideHeader"
      slot="icon-only"
      class="closeBtn"
      @click="$emit('closeRequested')"
    >
      <ion-icon
        :icon="close"
        class="closeBtn__icon"
      />
    </ion-button>
    <div class="saas-login-container">
      <create-organization-modal-header
        v-if="!hideHeader"
        @close-clicked="$emit('closeRequested')"
        title="clientArea.app.titleLinkOrganization"
        :hide-close-button="true"
      />
      <ion-text
        v-show="hideHeader"
        class="saas-login__title title-h2"
      >
        {{ $msTranslate('clientArea.app.titleLogin') }}
      </ion-text>

      <div class="saas-login-content">
        <!-- email -->
        <ms-input
          v-show="!loading"
          class="saas-login-content__input"
          ref="emailInput"
          v-model="email"
          label="clientArea.app.emailLabel"
          @on-enter-keyup="onLoginClicked()"
          :validator="emailValidator"
        />
        <div
          v-show="loading"
          class="input-skeleton"
        >
          <ion-skeleton-text
            class="input-skeleton__label"
            :animated="true"
          />
          <ion-skeleton-text
            class="input-skeleton__input"
            :animated="true"
          />
        </div>
        <!-- password -->
        <div
          class="input-password"
          v-show="!loading"
        >
          <ms-password-input
            class="saas-login-content__input"
            ref="passwordInput"
            v-model="password"
            label="clientArea.app.password"
            @on-enter-keyup="onLoginClicked()"
          />
        </div>
        <div
          v-show="loading"
          class="input-skeleton"
        >
          <ion-skeleton-text
            class="input-skeleton__label"
            :animated="true"
          />
          <ion-skeleton-text
            class="input-skeleton__input"
            :animated="true"
          />
          <ion-skeleton-text
            class="input-skeleton__button"
            :animated="true"
          />
        </div>

        <!-- forgotten password && remember me checkbox-->
        <div class="saas-login-link">
          <ms-checkbox
            v-show="!loading"
            v-model="storeCredentials"
            class="saas-login-link__checkbox"
            label-placement="end"
          >
            <ion-text class="body">{{ $msTranslate('clientArea.app.saveLogin') }}</ion-text>
          </ms-checkbox>
          <ion-skeleton-text v-show="loading" />
          <ion-text
            v-show="!loading"
            class="saas-login-link__forgotten-password button-small"
            target="_blank"
            @click="$emit('forgottenPasswordClicked')"
          >
            {{ $msTranslate('clientArea.app.forgottenPassword') }}
          </ion-text>
          <ion-skeleton-text
            v-show="loading"
            class="button-small"
          />
        </div>

        <!-- back and login buttons -->
        <div class="saas-login-button">
          <ion-text
            v-show="showBackButton"
            :disabled="querying"
            class="button-medium custom-button saas-login-button__item"
            button
            @click="$emit('goBackRequested')"
          >
            <ion-icon
              :icon="chevronBack"
              class="custom-button__icon"
            />
            <span>{{ $msTranslate('clientArea.app.backButton') }}</span>
          </ion-text>
          <ion-button
            v-show="!loading"
            :disabled="!validEmail || !password.length || querying"
            @click="onLoginClicked"
            class="saas-login-button__item button-large"
            size="large"
          >
            {{ $msTranslate('clientArea.app.login') }}
          </ion-button>
          <ion-skeleton-text
            v-show="loading"
            class="skeleton-login-button"
            :animated="true"
          />
          <ms-spinner v-show="querying" />
        </div>

        <!-- error -->
        <ion-text
          class="form-error body login-button-error"
          v-show="loginError"
        >
          <ion-icon
            class="form-error-icon"
            :icon="warning"
          />{{ $msTranslate(loginError) }}
        </ion-text>
      </div>

      <ion-footer class="saas-login-footer">
        <div
          class="create-account"
          v-show="!loading"
        >
          <ion-text class="create-account__text body">{{ $msTranslate('clientArea.app.noAccount') }}</ion-text>
          <a
            class="create-account__link button-medium"
            @click="
              $event.stopPropagation();
              Env.Links.openUrl(Env.getSignUrl());
            "
          >
            {{ $msTranslate('clientArea.app.createAccount') }}
            <ion-icon :icon="arrowForward" />
          </a>
        </div>
        <div
          v-show="loading"
          class="create-account-skeleton"
        >
          <ion-skeleton-text
            class="create-account-skeleton__text"
            :animated="true"
          />
          <ion-skeleton-text
            class="create-account-skeleton__link"
            :animated="true"
          />
        </div>
      </ion-footer>
    </div>
    <div class="saas-login-mockup">
      <img
        src="@/assets/images/mockup-parsec-client.svg"
        alt="mockup"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { emailValidator } from '@/common/validators';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { AuthenticationToken, BmsAccessInstance, PersonalInformationResultData } from '@/services/bms';
import { Env } from '@/services/environment';
import { IonButton, IonFooter, IonIcon, IonSkeletonText, IonText } from '@ionic/vue';
import { arrowForward, chevronBack, close, warning } from 'ionicons/icons';
import { MsCheckbox, MsInput, MsPasswordInput, MsSpinner, Translatable, Validity } from 'megashark-lib';
import { computed, onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  email?: string;
  hideHeader?: boolean;
  showBackButton?: boolean;
}>();

const emits = defineEmits<{
  (e: 'loginSuccess', token: AuthenticationToken, personalInformation: PersonalInformationResultData): void;
  (e: 'closeRequested'): void;
  (e: 'goBackRequested'): void;
  (e: 'forgottenPasswordClicked'): void;
}>();

const email = ref<string>(props.email ?? '');
const password = ref<string>('');
const emailInputRef = useTemplateRef<InstanceType<typeof MsInput>>('emailInput');
const passwordInputRef = useTemplateRef<InstanceType<typeof MsPasswordInput>>('passwordInput');
const querying = ref(false);
const loginError = ref<Translatable>('');
const loading = ref(true);
const storeCredentials = ref<boolean>(false);

const validEmail = computed(() => {
  return Boolean(email.value.length > 0 && emailInputRef.value && emailInputRef.value.validity === Validity.Valid);
});

onMounted(async () => {
  querying.value = false;
  loading.value = true;
  if (BmsAccessInstance.get().isLoggedIn()) {
    emits('loginSuccess', await BmsAccessInstance.get().getToken(), BmsAccessInstance.get().getPersonalInformation());
    return;
  }
  const loggedIn = await BmsAccessInstance.get().tryAutoLogin();
  if (loggedIn) {
    emits('loginSuccess', await BmsAccessInstance.get().getToken(), BmsAccessInstance.get().getPersonalInformation());
    return;
  }

  if (email.value.length > 0) {
    await emailInputRef.value?.validate(email.value);
    await passwordInputRef.value?.setFocus();
  } else {
    await emailInputRef.value?.setFocus();
  }
  loading.value = false;
});

async function onLoginClicked(): Promise<void> {
  if (email.value.length === 0 || password.value.length === 0) {
    return;
  }
  querying.value = true;
  const response = await BmsAccessInstance.get().login(email.value.toLowerCase(), password.value, storeCredentials.value);

  if (response.ok) {
    emits('loginSuccess', await BmsAccessInstance.get().getToken(), BmsAccessInstance.get().getPersonalInformation());
    return;
  }
  loginError.value = 'clientArea.app.networkFailed';
  if (response.errors) {
    loginError.value = 'clientArea.app.loginFailed';
  }
  querying.value = false;
}
</script>

<style scoped lang="scss">
.saas-login {
  display: flex;
  flex-direction: row;
  width: 100%;
  background: var(--parsec-color-light-primary-50);
  position: relative;
  padding: 2rem;

  @include ms.responsive-breakpoint('md') {
    padding: 1.5rem;
  }

  &::before {
    content: url('@/assets/images/background/background-shapes.svg');
    position: absolute;
    left: 20rem;
    top: -20rem;
    background: var(--parsec-color-light-primary-50);
    z-index: -1;
  }

  &__title {
    color: var(--parsec-color-light-primary-800);
    margin-bottom: 2rem;

    @include ms.responsive-breakpoint('md') {
      padding: 1rem;
    }
  }

  &-container {
    display: flex;
    flex-direction: column;
    max-width: 22rem;
    width: 100%;
    position: relative;
    z-index: 2;

    @include ms.responsive-breakpoint('md') {
      max-width: 100%;
    }
  }

  // include inputs
  &-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    @include ms.responsive-breakpoint('sm') {
      padding: 0.5rem 0;
    }

    .input-skeleton {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &__label {
        margin: 0;
        width: 30%;
        height: 1rem;
      }

      &__input {
        margin: 0;
        width: 100%;
        height: 2.75rem;
      }

      &__button {
        margin: 0;
        width: 30%;
        height: 0.75rem;
      }
    }

    .saas-login-link {
      display: flex;
      align-items: center;
      justify-content: space-between;

      &__forgotten-password {
        color: var(--parsec-color-light-primary-600);
        cursor: pointer;
        border-bottom: 1px solid transparent;
        transition: border-bottom 150ms linear;
        padding: 0.125rem 0;

        &:hover {
          border-bottom: 1px solid var(--parsec-color-light-primary-600);
        }
      }

      &__checkbox {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        color: var(--parsec-color-light-secondary-text);
        cursor: pointer;
        user-select: none;
      }
    }

    .saas-login-button {
      display: flex;
      gap: 1rem;
      align-items: center;
      justify-content: end;
      margin-top: 0.5rem;
      width: 100%;

      &__item {
        border-radius: var(--parsec-radius-6);
        border: 1px solid transparent;

        &:nth-child(1) ion-icon {
          @include ms.responsive-breakpoint('sm') {
            font-size: 1.5rem;
          }
        }

        &:nth-child(2) {
          @include ms.responsive-breakpoint('sm') {
            flex-grow: 1;
          }
        }
      }

      .custom-button {
        flex-shrink: 0;
        width: fit-content;
        color: var(--parsec-color-light-secondary-soft-text);

        &__icon {
          font-size: 1rem;
          width: 1rem;
        }
      }

      .skeleton-login-button {
        width: 100%;
        height: 2.75rem;
      }

      @include ms.responsive-breakpoint('sm') {
        justify-content: space-between;
      }
    }

    .login-button-error {
      display: flex;
      gap: 0.5rem;

      .form-error-icon {
        margin-top: 0.25rem;
        font-size: 1rem;
        align-self: baseline;
      }
    }
  }

  // include buttons
  &-footer {
    margin-top: 2rem;

    .create-account {
      display: flex;
      gap: 1rem;
      flex-direction: column;

      @include ms.responsive-breakpoint('sm') {
        justify-content: space-between;
        text-align: center;
        gap: 0.5rem;
      }

      &::before {
        content: '';
        width: 100%;
        height: 1px;
        background: var(--parsec-color-light-secondary-disabled);
        margin-bottom: 0.5rem;

        @include ms.responsive-breakpoint('sm') {
          display: none;
        }
      }

      &__text {
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      &__link {
        display: flex;
        align-items: center;
        width: fit-content;
        cursor: pointer;
        color: var(--parsec-color-light-primary-800);
        padding-bottom: 0.25rem;
        border-bottom: 1px solid transparent;
        transition: border-bottom 150ms linear;
        position: relative;

        @include ms.responsive-breakpoint('sm') {
          padding: 0.75rem 0;
          text-wrap: nowrap;
          justify-content: center;
          width: 100%;
          gap: 0.75rem;
          border: none;
          border-radius: var(--parsec-radius-8);
        }

        ion-icon {
          position: absolute;
          right: -2rem;
          transition: right 150ms linear;
          color: transparent;

          @include ms.responsive-breakpoint('sm') {
            position: initial;
            color: var(--parsec-color-light-primary-800);
          }
        }

        &:hover {
          border-bottom: 1px solid var(--parsec-color-light-primary-800);

          @include ms.responsive-breakpoint('sm') {
            border: none;
            background: var(--parsec-color-light-secondary-white);
            box-shadow: var(--parsec-shadow-light);
          }

          ion-icon {
            right: -1.25rem;
            color: var(--parsec-color-light-primary-800);
          }
        }
      }

      &-skeleton {
        display: flex;
        gap: 2.5rem;
        align-items: center;

        &__text {
          margin: 0;
          width: 50%;
          height: 1rem;
          border-radius: var(--parsec-radius-6);
        }

        &__link {
          margin: 0;
          width: 30%;
          height: 0.75rem;
          border-radius: var(--parsec-radius-6);
        }
      }
    }
  }

  &-mockup {
    position: absolute;
    right: 0;
    top: 7rem;
    display: flex;
    align-items: flex-end;
    scale: 1.2;

    @include ms.responsive-breakpoint('xl') {
      right: -1rem;
    }

    @include ms.responsive-breakpoint('lg') {
      right: -6rem;
    }

    @include ms.responsive-breakpoint('md') {
      display: none;
    }
  }
}
</style>
