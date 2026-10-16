<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
        class="saas-login__title"
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
            <ion-text class="saas-login-link__label">{{ $msTranslate('clientArea.app.saveLogin') }}</ion-text>
          </ms-checkbox>
          <ion-skeleton-text v-show="loading" />
          <ion-text
            v-show="!loading"
            class="saas-login-link__forgotten-password"
            target="_blank"
            @click="$emit('forgottenPasswordClicked')"
          >
            {{ $msTranslate('clientArea.app.forgottenPassword') }}
          </ion-text>
          <ion-skeleton-text
            v-show="loading"
            class="skeleton-forgotten-password"
          />
        </div>

        <!-- back and login buttons -->
        <div class="saas-login-button">
          <div class="saas-login-button__row">
            <ion-button
              v-show="showBackButton"
              :disabled="querying"
              fill="clear"
              slot="icon-only"
              class="saas-login-button__item color-secondary"
              @click="$emit('goBackRequested')"
            >
              <ion-icon :icon="chevronBack" />
            </ion-button>
            <ion-button
              v-show="!loading"
              @click="onLoginClicked"
              :disabled="!validEmail || !password.length || querying"
              class="saas-login-button__item saas-login-button__login"
            >
              <span>{{ $msTranslate('clientArea.app.login') }}</span>
              <ms-spinner
                v-show="querying"
                :size="14"
                :speed="2"
              />
            </ion-button>
          </div>
          <div class="saas-login-button__divider" />
          <ion-button
            class="saas-login-button__item"
            fill="clear"
            @click="
              $event.stopPropagation();
              Env.Links.openUrl(Env.getSignUrl());
            "
          >
            {{ $msTranslate('clientArea.app.createAccount') }}
          </ion-button>
          <ion-skeleton-text
            v-show="loading"
            class="skeleton-login-button"
            :animated="true"
          />
        </div>

        <!-- error -->
        <ion-text
          class="form-error login-button-error"
          v-show="loginError"
        >
          <ion-icon
            class="form-error-icon"
            :icon="warning"
          />{{ $msTranslate(loginError) }}
        </ion-text>
      </div>
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
import { IonButton, IonIcon, IonSkeletonText, IonText } from '@ionic/vue';
import { chevronBack, close, warning } from 'ionicons/icons';
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
  background: ms.color('surface-base-default');
  position: relative;
  padding: ms.spacing('padding-5xl');
  border: 1px solid ms.color('border-neutral-default-subtle');

  @include ms.responsive-breakpoint('md') {
    padding: ms.spacing('padding-4xl');
  }

  &__title {
    @include ms.font('heading-h2');
    color: ms.color('text-brand-default');
    margin-bottom: 2rem;

    @include ms.responsive-breakpoint('md') {
      @include ms.font('heading-h3');
      margin-bottom: 1rem;
      text-align: center;
      padding: ms.spacing('padding-none');
    }
  }

  &-container {
    display: flex;
    flex-direction: column;
    max-width: 24rem;
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
    gap: ms.spacing('gap-3xl');

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-lg') ms.spacing('padding-none');
    }

    .input-skeleton {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-lg');

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
        @include ms.font('label-md-medium');
        color: ms.color('text-brand-default');
        cursor: pointer;
        border-bottom: ms.border('thin') solid transparent;
        transition: border-bottom 150ms linear;
        padding: ms.spacing('padding-xs') ms.spacing('padding-none');

        &:hover {
          border-bottom: ms.border('thin') solid ms.color('border-brand-default-hover');
        }
      }
    }

    .saas-login-button {
      display: flex;
      gap: ms.spacing('gap-3xl');
      margin-top: 0.5rem;
      flex-direction: column;

      &__row {
        display: flex;
        gap: ms.spacing('gap-3xl');
        align-items: center;
      }

      &__login {
        flex-grow: 1;
      }

      &__divider {
        width: 100%;
        border-top: ms.border('thin') solid ms.color('border-neutral-default-subtle');
      }

      .skeleton-login-button {
        width: 100%;
        height: 2.75rem;
      }
    }

    .login-button-error {
      display: flex;
      gap: ms.spacing('gap-lg');

      .form-error-icon {
        margin-top: 0.25rem;
        font-size: 1rem;
        align-self: baseline;
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

.skeleton-forgotten-password {
  @include ms.font('label-md-medium');
}
</style>
