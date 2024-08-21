<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-login">
    <ion-buttons
      slot="end"
      class="closeBtn-container"
      v-if="!hideHeader"
    >
      <ion-button
        slot="icon-only"
        class="closeBtn"
        @click="$emit('closeRequested')"
      >
        <ion-icon
          :icon="close"
          size="large"
          class="closeBtn__icon"
        />
      </ion-button>
    </ion-buttons>
    <div class="saas-login-container">
      <create-organization-modal-header
        v-if="!hideHeader"
        @close-clicked="$emit('closeRequested')"
        title="clientArea.app.titleLinkOrganization"
        :hide-close-button="true"
      />
      <ion-text
        v-if="hideHeader"
        class="saas-login__title title-h2"
      >
        {{ $msTranslate('clientArea.app.titleLogin') }}
      </ion-text>

      <div class="saas-login-content">
        <!-- email -->
        <ms-input
          v-if="!loading"
          class="saas-login-content__input"
          ref="emailInputRef"
          v-model="email"
          label="clientArea.app.emailLabel"
          :validator="emailValidator"
        />
        <div
          v-else
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
          v-if="!loading"
        >
          <ms-password-input
            class="saas-login-content__input"
            ref="passwordInputRef"
            v-model="password"
            label="clientArea.app.password"
          />
        </div>
        <div
          v-else
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
            v-model="checkboxValue"
            class="saas-login-link__checkbox"
            label-placement="end"
            @click="!checkboxValue"
          >
            <ion-text class="body">{{ $msTranslate('clientArea.app.saveLogin') }}</ion-text>
          </ms-checkbox>
          <ion-text
            class="saas-login-link__forgotten-password button-small"
            target="_blank"
            @click="$emit('forgottenPasswordClicked')"
          >
            {{ $msTranslate('clientArea.app.forgottenPassword') }}
          </ion-text>
        </div>

        <!-- login button -->
        <div class="saas-login-button">
          <ion-button
            v-if="!loading"
            :disabled="!emailInputRef || emailInputRef.validity !== Validity.Valid || !password.length || querying"
            @click="onLoginClicked"
            class="saas-login-button__item"
            size="large"
          >
            {{ $msTranslate('clientArea.app.login') }}
          </ion-button>
          <ion-skeleton-text
            v-else
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
          v-if="!loading"
        >
          <ion-text class="create-account__text body">{{ $msTranslate('clientArea.app.noAccount') }}</ion-text>
          <a
            class="create-account__link button-medium"
            target="_blank"
            @click="$event.stopPropagation()"
            :href="Env.getSignUrl()"
          >
            {{ $msTranslate('clientArea.app.createAccount') }}
            <ion-icon :icon="arrowForward" />
          </a>
        </div>
        <div
          v-else
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
import { IonButton, IonText, IonButtons, IonFooter, IonIcon, IonSkeletonText } from '@ionic/vue';
import { MsInput, MsPasswordInput, Translatable, Validity, MsSpinner, MsCheckbox } from 'megashark-lib';
import { emailValidator } from '@/common/validators';
import { warning, arrowForward, close } from 'ionicons/icons';
import { onMounted, ref } from 'vue';
import { AuthenticationToken, BmsAccessInstance, PersonalInformationResultData } from '@/services/bms';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { Env } from '@/services/environment';

const props = defineProps<{
  email?: string;
  hideHeader?: boolean;
}>();

const emits = defineEmits<{
  (e: 'loginSuccess', token: AuthenticationToken, personalInformation: PersonalInformationResultData): void;
  (e: 'closeRequested'): void;
  (e: 'forgottenPasswordClicked'): void;
}>();

const email = ref<string>(props.email ?? '');
const password = ref<string>('');
const emailInputRef = ref();
const passwordInputRef = ref();
const querying = ref(false);
const loginError = ref<Translatable>('');
const loading = ref(true);
const checkboxValue = ref<boolean>(false);

onMounted(async () => {
  if (BmsAccessInstance.get().isLoggedIn()) {
    emits('loginSuccess', await BmsAccessInstance.get().getToken(), BmsAccessInstance.get().getPersonalInformation());
    return;
  }
  const loggedIn = await BmsAccessInstance.get().tryAutoLogin();
  if (loggedIn) {
    emits('loginSuccess', await BmsAccessInstance.get().getToken(), BmsAccessInstance.get().getPersonalInformation());
  }

  if (emailInputRef.value) {
    if (email.value.length > 0) {
      await emailInputRef.value.validate(email.value);
      await passwordInputRef.value.setFocus();
    } else {
      await emailInputRef.value.setFocus();
    }
  }
  loading.value = false;
});

async function onLoginClicked(): Promise<void> {
  if (email.value.length === 0 || password.value.length === 0) {
    return;
  }
  querying.value = true;
  const response = await BmsAccessInstance.get().login(email.value, password.value);

  if (response.ok) {
    emits('loginSuccess', await BmsAccessInstance.get().getToken(), BmsAccessInstance.get().getPersonalInformation());
    querying.value = false;
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
  padding: 2.5rem;
  min-height: 28em;

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
  }

  &-container {
    display: flex;
    flex-direction: column;
    max-width: 22rem;
    width: 100%;
    position: relative;
    z-index: 2;
  }

  // include inputs
  &-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;

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
      margin-top: 0.5rem;
      width: 100%;

      &__item {
        height: 2.5rem;
        border-radius: var(--parsec-radius-6);
      }

      .skeleton-login-button {
        width: 100%;
        height: 2.75rem;
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

      &::before {
        content: '';
        width: 100%;
        height: 1px;
        background: var(--parsec-color-light-secondary-disabled);
        margin-bottom: 0.5rem;
      }

      &__text {
        color: var(--parsec-color-light-secondary-hard-grey);
      }

      &__link {
        display: flex;
        align-items: center;
        width: fit-content;
        color: var(--parsec-color-light-primary-800);
        padding-bottom: 0.25rem;
        border-bottom: 1px solid transparent;
        transition: border-bottom 150ms linear;
        position: relative;

        ion-icon {
          position: absolute;
          right: -2rem;
          transition: right 150ms linear;
          color: transparent;
        }

        &:hover {
          border-bottom: 1px solid var(--parsec-color-light-primary-800);

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
    right: -0.5rem;
    bottom: -1rem;
    display: flex;
    align-items: flex-end;
  }
}
</style>
