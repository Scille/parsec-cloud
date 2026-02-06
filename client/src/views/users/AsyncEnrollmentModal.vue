<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="join-async-modal">
    <ms-modal
      :title="{ key: 'HomePage.organizationRequest.asyncEnrollmentModal.title', data: { organization: addr.organizationId } }"
      :close-button="{ visible: true }"
      :confirm-button="{
        disabled: !nextButtonEnabled,
        label: 'HomePage.organizationRequest.asyncEnrollmentModal.buttons.next',
        onClick: onNextButtonClicked,
        queryingSpinner: sendingRequest,
      }"
      :cancel-button="{
        label: previousButton.label,
        disabled: previousButton.disabled,
        onClick: onPreviousClicked,
      }"
    >
      <div class="modal-content">
        <div
          v-if="state === JoinRequestState.ChooseMethod"
          class="choose-method"
        >
          <ion-text class="choose-method-text subtitles-normal">
            {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.chooseMethod') }}
          </ion-text>
          <div class="choose-method-options">
            <ion-text
              @click="method = SubmitAsyncEnrollmentIdentityStrategyTag.PKI"
              v-if="pkiAvailable"
              class="choose-method-options-item"
              :class="{ selected: method === SubmitAsyncEnrollmentIdentityStrategyTag.PKI }"
            >
              <span class="title-h4">{{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.pki.label') }}</span>
              <ion-icon
                class="choose-method-options-item__icon"
                v-if="method === SubmitAsyncEnrollmentIdentityStrategyTag.PKI"
                :icon="checkmarkCircle"
              />
            </ion-text>
            <ion-text
              @click="method = SubmitAsyncEnrollmentIdentityStrategyTag.OpenBao"
              v-if="serverConfig && serverConfig.openbao && serverConfig.openbao.auths.length > 0"
              class="choose-method-options-item"
              :class="{ selected: method === SubmitAsyncEnrollmentIdentityStrategyTag.OpenBao }"
            >
              <span class="title-h4">{{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.sso.label') }}</span>
              <ion-icon
                v-if="method === SubmitAsyncEnrollmentIdentityStrategyTag.OpenBao"
                :icon="checkmarkCircle"
              />
            </ion-text>
          </div>
        </div>
        <div
          v-if="state === JoinRequestState.MethodPKI"
          class="modal-info"
        >
          <div class="async-authentication-modal-header">
            <ms-image
              :image="CertificateIcon"
              alt="Certificate Icon"
              class="async-authentication-modal-header__icon"
            />
            <ion-text class="async-authentication-modal-header__title subtitles-lg">
              {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.pki.title') }}
            </ion-text>
          </div>
          <ion-text class="async-authentication-modal-text body-lg">
            {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.pki.description') }}
          </ion-text>
          <choose-certificate @certificate-selected="certificate = $event" />
        </div>
        <div v-if="state === JoinRequestState.MethodSSO && serverConfig?.openbao">
          <div
            class="modal-info"
            v-if="!openBaoClient"
          >
            <div class="async-authentication-modal-header">
              <ms-image
                :image="CertificateIcon"
                alt="Certificate Icon"
                class="async-authentication-modal-header__icon"
              />
              <ion-text class="async-authentication-modal-header__title subtitles-lg">
                {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.sso.description') }}
              </ion-text>
            </div>
            <connect-sso
              class="async-authentication-modal-sso"
              :server-config="serverConfig"
              @open-bao-connected="onOpenBaoConnected"
            />
          </div>
          <div
            class="user-information"
            v-if="openBaoClient && openBaoEmails?.at(0)"
          >
            <div class="user-information-header">
              <ion-text class="user-information-header__title title-h4">
                {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.sso.join') }}
              </ion-text>
              <div class="user-information-header-connected title-h4">
                <ion-icon
                  :icon="checkmarkCircle"
                  class="user-information-header-connected__icon"
                />
                <ion-text class="user-information-header-connected__label">
                  {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.sso.connected') }}
                </ion-text>
              </div>
            </div>
            <ms-input
              label="CreateOrganization.fullname"
              placeholder="CreateOrganization.fullnamePlaceholder"
              name="fullname"
              v-model="fullName"
              :validator="userNameValidator"
            />
            <ms-dropdown
              title="CreateOrganization.email"
              :options="openBaoEmails"
              :default-option-key="openBaoEmails.at(0)?.key"
              @change="email = $event.option.key"
            />
          </div>
          <ms-report-text
            v-if="openBaoClient && !openBaoEmails?.at(0)"
            :theme="MsReportTheme.Warning"
          >
            {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.sso.noEmail') }}
          </ms-report-text>
        </div>
        <ms-report-text
          v-if="error"
          :theme="MsReportTheme.Error"
        >
          {{ $msTranslate(error) }}
        </ms-report-text>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import CertificateIcon from '@/assets/images/certificate-icon.svg?raw';
import { userNameValidator } from '@/common/validators';
import { ChooseCertificate } from '@/components/devices';
import ConnectSso from '@/components/devices/ConnectSso.vue';
import {
  getOpenBaoEmails,
  makeRequestOpenBaoIdentityStrategy,
  makeRequestPkiIdentityStrategy,
  ParsecAsyncEnrollmentAddr,
  ParsedParsecAddrAsyncEnrollment,
  requestJoinOrganization,
  ServerConfig,
  SubmitAsyncEnrollmentErrorTag,
  SubmitAsyncEnrollmentIdentityStrategyTag,
  X509CertificateReference,
} from '@/parsec';
import { OpenBaoClient } from '@/services/openBao';
import { IonIcon, IonPage, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import {
  I18n,
  MsDropdown,
  MsImage,
  MsInput,
  MsModal,
  MsModalResult,
  MsOptions,
  MsReportText,
  MsReportTheme,
  Translatable,
} from 'megashark-lib';
import { computed, onMounted, ref, toRaw } from 'vue';

enum JoinRequestState {
  ChooseMethod = 'choose-method',
  MethodPKI = 'method-pki',
  MethodSSO = 'method-sso',
}

const props = defineProps<{
  link: ParsecAsyncEnrollmentAddr;
  addr: ParsedParsecAddrAsyncEnrollment;
  serverConfig?: ServerConfig;
  pkiAvailable?: boolean;
}>();

const certificate = ref<X509CertificateReference | undefined>(undefined);
const sendingRequest = ref(false);
const error = ref('');
const state = ref<JoinRequestState>(JoinRequestState.ChooseMethod);
const method = ref<SubmitAsyncEnrollmentIdentityStrategyTag | undefined>(undefined);
const openBaoClient = ref<OpenBaoClient | undefined>(undefined);
const openBaoEmails = ref<MsOptions | undefined>(undefined);
const fullName = ref<string>('');
const email = ref<string>('');

const nextButtonEnabled = computed(() => {
  if (state.value === JoinRequestState.ChooseMethod && method.value !== undefined) {
    return true;
  }
  if (state.value === JoinRequestState.MethodPKI && certificate.value) {
    return true;
  }
  if (state.value === JoinRequestState.MethodSSO && openBaoClient.value && fullName.value && email.value) {
    return true;
  }
  return false;
});

const multipleMethodsAvailable = computed((): boolean => {
  return Boolean(props.pkiAvailable && props.serverConfig?.openbao && props.serverConfig.openbao.auths.length > 0);
});

const previousButton = computed((): { label: Translatable; disabled: boolean } => {
  if (state.value === JoinRequestState.ChooseMethod) {
    return { label: 'HomePage.organizationRequest.asyncEnrollmentModal.buttons.cancel', disabled: false };
  }
  if (!multipleMethodsAvailable.value) {
    return { label: 'HomePage.organizationRequest.asyncEnrollmentModal.buttons.cancel', disabled: false };
  }
  return { label: 'HomePage.organizationRequest.asyncEnrollmentModal.buttons.previous', disabled: false };
});

onMounted(async () => {
  if ((!props.serverConfig || !props.serverConfig.openbao || !props.serverConfig.openbao.auths.length) && !props.pkiAvailable) {
    throw new Error('No PKI and no openBao available');
  }
  if (!multipleMethodsAvailable.value) {
    if (props.pkiAvailable) {
      method.value = SubmitAsyncEnrollmentIdentityStrategyTag.PKI;
      state.value = JoinRequestState.MethodPKI;
    } else {
      method.value = SubmitAsyncEnrollmentIdentityStrategyTag.OpenBao;
      state.value = JoinRequestState.MethodSSO;
    }
  }
});

async function onOpenBaoConnected(client: OpenBaoClient) {
  openBaoClient.value = client;
  const emailsResult = await getOpenBaoEmails(client);
  if (!emailsResult.ok) {
    error.value = 'HomePage.organizationRequest.asyncEnrollmentModal.errors.failedToGetEmailsFromOpenBao';
    return;
  }
  openBaoEmails.value = new MsOptions(
    emailsResult.value.map((email) => {
      return { key: email, label: I18n.valueAsTranslatable(email) };
    }),
  );
  email.value = emailsResult.value[0] ?? '';
}

async function onNextButtonClicked(): Promise<boolean> {
  if (state.value === JoinRequestState.ChooseMethod && method.value) {
    if (method.value === SubmitAsyncEnrollmentIdentityStrategyTag.PKI) {
      state.value = JoinRequestState.MethodPKI;
    } else {
      state.value = JoinRequestState.MethodSSO;
    }
  } else if (state.value === JoinRequestState.MethodPKI) {
    if (!certificate.value) {
      window.electronAPI.log('error', 'Invalid state for async enrollment with PKI');
      return false;
    }
    const result = await requestJoinOrganization(props.link, makeRequestPkiIdentityStrategy(toRaw(certificate.value)));
    if (!result.ok) {
      switch (result.error.tag) {
        case SubmitAsyncEnrollmentErrorTag.EmailAlreadyEnrolled: {
          error.value = 'HomePage.organizationRequest.asyncEnrollmentModal.errors.existingEmail';
          break;
        }
        case SubmitAsyncEnrollmentErrorTag.PKICannotOpenCertificateStore:
        case SubmitAsyncEnrollmentErrorTag.PKIServerInvalidX509Trustchain:
        case SubmitAsyncEnrollmentErrorTag.PKIUnusableX509CertificateReference: {
          error.value = 'HomePage.organizationRequest.asyncEnrollmentModal.errors.problemWithRequestCertificate';
          break;
        }
        case SubmitAsyncEnrollmentErrorTag.Offline: {
          error.value = 'HomePage.organizationRequest.asyncEnrollmentModal.errors.offline';
          break;
        }
        default: {
          error.value = 'HomePage.organizationRequest.asyncEnrollmentModal.errors.failedToJoinWithPKI';
          break;
        }
      }
      return false;
    }
    return modalController.dismiss(null, MsModalResult.Confirm);
  } else if (state.value === JoinRequestState.MethodSSO) {
    if (!fullName.value || !email.value || !openBaoClient.value) {
      window.electronAPI.log('error', 'Invalid state for async enrollment with OpenBao');
      return false;
    }
    const result = await requestJoinOrganization(
      props.link,
      makeRequestOpenBaoIdentityStrategy(openBaoClient.value, { label: fullName.value, email: email.value }),
    );
    if (!result.ok) {
      switch (result.error.tag) {
        case SubmitAsyncEnrollmentErrorTag.EmailAlreadyEnrolled: {
          error.value = 'HomePage.organizationRequest.asyncEnrollmentModal.errors.existingEmail';
          break;
        }
        case SubmitAsyncEnrollmentErrorTag.OpenBaoBadServerResponse:
        case SubmitAsyncEnrollmentErrorTag.OpenBaoBadURL:
        case SubmitAsyncEnrollmentErrorTag.OpenBaoNoServerResponse: {
          error.value = 'HomePage.organizationRequest.asyncEnrollmentModal.errors.couldNotContactServer';
          break;
        }
        case SubmitAsyncEnrollmentErrorTag.Offline: {
          error.value = 'HomePage.organizationRequest.asyncEnrollmentModal.errors.offline';
          break;
        }
        default: {
          error.value = 'HomePage.organizationRequest.asyncEnrollmentModal.errors.failedToJoinWithOpenBao';
          break;
        }
      }
      return false;
    }
    return modalController.dismiss(null, MsModalResult.Confirm);
  } else {
    window.electronAPI.log('error', 'Invalid state for async enrollment modal');
    return false;
  }
  return true;
}

async function onPreviousClicked(): Promise<boolean> {
  if (state.value === JoinRequestState.ChooseMethod) {
    return modalController.dismiss(null, MsModalResult.Cancel);
  } else {
    state.value = JoinRequestState.ChooseMethod;
    openBaoClient.value = undefined;
    return false;
  }
}
</script>

<style scoped lang="scss">
.modal-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.choose-method {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &-text {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &-options {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    &-item {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 1rem;
      color: var(--parsec-color-light-secondary-hard-grey);
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-12);
      cursor: pointer;
      user-select: none;
      transition:
        background-color 0.2s,
        border-color 0.2s;

      &:hover {
        background-color: var(--parsec-color-light-secondary-premiere);
        border-color: var(--parsec-color-light-secondary-light);
      }

      &__icon {
        font-size: 1.125rem;
        color: var(--parsec-color-light-primary-600);
      }

      &.selected {
        background-color: var(--parsec-color-light-primary-50);
        border-color: var(--parsec-color-light-primary-600);
        color: var(--parsec-color-light-primary-600);
      }
    }
  }
}

.modal-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  background: var(--parsec-color-light-secondary-premiere);
  box-shadow: var(--parsec-shadow-input);
  border-radius: var(--parsec-radius-12);
  padding: 1rem;

  .async-authentication-modal-text {
    padding: 0 0.75rem;
  }
}

.user-information {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 100%;

  &-header {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
    padding: 1rem;
    color: var(--parsec-color-light-secondary-hard-grey);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-12);
    background: var(--parsec-color-light-secondary-background);
    box-shadow: var(--parsec-shadow-input);
    margin-bottom: 1rem;

    &__title {
      color: var(--parsec-color-light-primary-700);
    }

    &-connected {
      display: flex;
      align-items: center;
      gap: 0.5rem;

      &__icon {
        font-size: 1.125rem;
        color: var(--parsec-color-light-success-700);
      }

      &__label {
        font-weight: 500;
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }
}
</style>
