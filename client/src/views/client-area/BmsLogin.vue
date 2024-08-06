<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="saas-login">
    <div class="saas-login-container">
      <create-organization-modal-header
        v-if="!hideHeader"
        @close-clicked="$emit('closeRequested')"
        title="clientArea.app.title"
      />

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
          <!-- TODO: UPDATE THE LINK -->
          <!-- If changing the link, don't forget to check that it is allowed by electron! -->
          <ion-text
            class="saas-login-inputs__link button-small"
            target="_blank"
            @click="$event.stopPropagation()"
            :href="$msTranslate('clientArea.app.forgottenPasswordLink')"
          >
            {{ $msTranslate('clientArea.app.forgottenPassword') }}
          </ion-text>
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
        <div class="login-button">
          <ion-button
            v-if="!loading"
            :disabled="!emailInputRef || emailInputRef.validity !== Validity.Valid || !password.length || querying"
            @click="onLoginClicked"
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

        <!-- TODO: UPDATE THE LINK -->
        <!-- If changing the link, don't forget to check that it is allowed by electron! -->
        <div
          class="create-account"
          v-if="!loading"
        >
          <ion-text class="create-account__text body">{{ $msTranslate('clientArea.app.noAccount') }}</ion-text>
          <ion-button
            class="create-account__link"
            target="_blank"
            fill="clear"
            @click="$event.stopPropagation()"
            :href="$msTranslate('clientArea.app.createAccountUrl')"
          >
            {{ $msTranslate('clientArea.app.createAccount') }}
          </ion-button>
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
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonButton, IonText, IonFooter, IonIcon, IonSkeletonText } from '@ionic/vue';
import { MsInput, MsPasswordInput, Translatable, Validity, MsSpinner } from 'megashark-lib';
import { emailValidator } from '@/common/validators';
import { warning } from 'ionicons/icons';
import { onMounted, ref } from 'vue';
import { AuthenticationToken, BmsAccessInstance, PersonalInformationResultData } from '@/services/bms';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';

const props = defineProps<{
  email?: string;
  hideHeader?: boolean;
}>();

const emits = defineEmits<{
  (e: 'loginSuccess', token: AuthenticationToken, personalInformation: PersonalInformationResultData): void;
  (e: 'closeRequested'): void;
}>();

const email = ref<string>(props.email ?? '');
const password = ref<string>('');
const emailInputRef = ref();
const passwordInputRef = ref();
const querying = ref(false);
const loginError = ref<Translatable>('');
const loading = ref(true);

onMounted(async () => {
  if (BmsAccessInstance.get().isLoggedIn()) {
    emits('loginSuccess', await BmsAccessInstance.get().getToken(), await BmsAccessInstance.get().getPersonalInformation());
    return;
  }
  const loggedIn = await BmsAccessInstance.get().tryAutoLogin();
  if (loggedIn) {
    emits('loginSuccess', await BmsAccessInstance.get().getToken(), await BmsAccessInstance.get().getPersonalInformation());
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
  try {
    const response = await BmsAccessInstance.get().login(email.value, password.value);

    if (!response.ok) {
      loginError.value = 'clientArea.app.loginFailed';
    } else {
      emits('loginSuccess', await BmsAccessInstance.get().getToken(), await BmsAccessInstance.get().getPersonalInformation());
    }
  } catch (error: any) {
    window.electronAPI.log('error', `Connection to the BMS failed: ${error}`);
    loginError.value = 'clientArea.app.networkFailed';
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
.saas-login {
  display: flex;
  flex-direction: row;
  height: auto;
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

  &-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    width: 22rem;
  }

  // include inputs
  &-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    .input-password {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      .saas-login-inputs__link {
        cursor: pointer;
        color: var(--parsec-color-light-secondary-hard-grey);

        &:hover {
          text-decoration: underline;
        }
      }
    }

    .input-skeleton {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      border-radius: var(--parsec-radius-6);

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
  }

  // include buttons
  &-footer {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    margin-top: 2rem;
    gap: 2rem;

    .login-button {
      display: flex;
      gap: 1rem;
      align-items: center;
    }

    .skeleton-login-button {
      margin: 0;
      width: 40%;
      height: 2.25rem;
      border-radius: var(--parsec-radius-6);
    }

    .create-account {
      display: flex;
      gap: 0.5rem;
      align-items: center;

      &__text {
        color: var(--parsec-color-light-secondary-hard-grey);
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
    bottom: -2rem;
    display: flex;
    align-items: flex-end;
  }
}
</style>
