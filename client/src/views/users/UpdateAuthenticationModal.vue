<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page
    class="modal-stepper"
    :class="ChangeAuthenticationStep[pageStep]"
  >
    <!-- close button -->
    <ion-button
      slot="icon-only"
      @click="cancel()"
      class="closeBtn"
      v-if="isLargeDisplay"
    >
      <ion-icon
        :icon="close"
        class="closeBtn__icon"
      />
    </ion-button>

    <!-- modal content -->
    <div class="modal">
      <ion-header
        class="modal-header"
        v-if="isLargeDisplay"
      >
        <ion-title class="modal-header__title title-h3">
          {{ $msTranslate(texts.title) }}
        </ion-title>
      </ion-header>

      <small-display-modal-header
        v-else
        @close-clicked="cancel()"
        :title="texts.title"
      />

      <div class="modal-content inner-content">
        <div
          v-show="pageStep === ChangeAuthenticationStep.CurrentAuthentication"
          class="step"
        >
          <prompt-current-authentication
            :device="currentDevice"
            @authentication-selected="onCurrentAuthenticationSelected"
            @enter-pressed="nextStep"
          />
        </div>
        <div
          v-show="pageStep === ChangeAuthenticationStep.ChooseNewAuthMethod"
          class="step"
        >
          <choose-authentication
            ref="chooseAuth"
            :active-auth="currentDevice.ty.tag"
            :server-config="serverConfig"
          />
        </div>
      </div>

      <ms-report-text
        v-if="errorMessage"
        :theme="MsReportTheme.Error"
        class="modal-report-error"
      >
        {{ $msTranslate(errorMessage) }}
      </ms-report-text>

      <ion-footer class="modal-footer">
        <div class="modal-footer-buttons">
          <ion-button
            fill="clear"
            size="default"
            @click="cancel"
          >
            {{ $msTranslate('MyProfilePage.cancelButton') }}
          </ion-button>
          <ion-button
            fill="solid"
            size="default"
            id="next-button"
            @click="nextStep"
            :disabled="!canGoForward"
          >
            {{ $msTranslate(texts.button) }}
          </ion-button>
        </div>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import SmallDisplayModalHeader from '@/components/header/SmallDisplayModalHeader.vue';
import {
  AvailableDevice,
  AvailableDeviceTypeTag,
  DevicePrimaryProtectionStrategy,
  ServerConfig,
  UpdateDeviceErrorTag,
  constructAccessStrategy,
  isAuthenticationValid,
  updateDeviceChangeAuthentication,
} from '@/parsec';
import { DevicePrimaryProtectionStrategyTag } from '@/plugins/libparsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import PromptCurrentAuthentication from '@/views/users/PromptCurrentAuthentication.vue';
import { IonButton, IonFooter, IonHeader, IonIcon, IonPage, IonTitle, modalController } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { MsModalResult, MsReportText, MsReportTheme, Translatable, asyncComputed, useWindowSize } from 'megashark-lib';
import { Ref, computed, onMounted, ref, toRaw, useTemplateRef } from 'vue';

enum ChangeAuthenticationStep {
  Undefined,
  ChooseNewAuthMethod,
  CurrentAuthentication,
}

const props = defineProps<{
  currentDevice: AvailableDevice;
  informationManager: InformationManager;
  serverConfig?: ServerConfig;
}>();

const { isLargeDisplay } = useWindowSize();
const pageStep = ref(ChangeAuthenticationStep.Undefined);
const chooseAuthRef = useTemplateRef<InstanceType<typeof ChooseAuthentication>>('chooseAuth');
const errorMessage: Ref<Translatable> = ref('');
const currentAuth = ref<DevicePrimaryProtectionStrategy | undefined>(undefined);

const texts = computed(() => {
  switch (pageStep.value) {
    case ChangeAuthenticationStep.CurrentAuthentication:
      switch (props.currentDevice.ty.tag) {
        case AvailableDeviceTypeTag.Password:
          return { title: 'MyProfilePage.titleCurrentPassword', button: 'MyProfilePage.nextButton' };
        case AvailableDeviceTypeTag.OpenBao:
          return { title: 'MyProfilePage.titleCurrentOpenBao', button: 'MyProfilePage.nextButton' };
      }
    case ChangeAuthenticationStep.ChooseNewAuthMethod:
      return { title: 'MyProfilePage.titleNewAuthentication', button: 'MyProfilePage.changeAuthenticationButton' };
    default:
      return { title: '', button: '' };
  }
});

onMounted(async () => {
  pageStep.value = ChangeAuthenticationStep.CurrentAuthentication;
});

async function nextStep(): Promise<void> {
  if (pageStep.value === ChangeAuthenticationStep.CurrentAuthentication && currentAuth.value) {
    const access = constructAccessStrategy(props.currentDevice, toRaw(currentAuth.value));
    const result = await isAuthenticationValid(props.currentDevice, access);
    if (result) {
      errorMessage.value = '';
      pageStep.value = ChangeAuthenticationStep.ChooseNewAuthMethod;
    } else {
      errorMessage.value = 'MyProfilePage.errors.wrongAuthentication';
    }
  } else if (pageStep.value === ChangeAuthenticationStep.ChooseNewAuthMethod) {
    await changeAuthentication();
  }
}

const canGoForward = asyncComputed(async () => {
  if (pageStep.value === ChangeAuthenticationStep.CurrentAuthentication && currentAuth.value) {
    return true;
  } else if (pageStep.value === ChangeAuthenticationStep.ChooseNewAuthMethod && chooseAuthRef.value) {
    return await chooseAuthRef.value.areFieldsCorrect();
  }
  return false;
});

async function onCurrentAuthenticationSelected(protection?: DevicePrimaryProtectionStrategy): Promise<void> {
  currentAuth.value = protection;
  if (currentAuth.value) {
    if ([DevicePrimaryProtectionStrategyTag.Keyring, DevicePrimaryProtectionStrategyTag.PKI].includes(currentAuth.value.tag)) {
      pageStep.value = ChangeAuthenticationStep.ChooseNewAuthMethod;
    }
  }
}

async function changeAuthentication(): Promise<void> {
  const saveStrategy = chooseAuthRef.value?.getSaveStrategy();
  if (!saveStrategy || !currentAuth.value) {
    return;
  }

  const result = await updateDeviceChangeAuthentication(
    constructAccessStrategy(props.currentDevice, toRaw(currentAuth.value)),
    saveStrategy,
  );

  if (result.ok) {
    props.informationManager.present(
      new Information({
        message: 'MyProfilePage.authenticationUpdated',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await modalController.dismiss(undefined, MsModalResult.Confirm);
  } else {
    switch (result.error.tag) {
      case UpdateDeviceErrorTag.DecryptionFailed: {
        errorMessage.value = 'MyProfilePage.errors.wrongPassword';
        break;
      }
      default:
        props.informationManager.present(
          new Information({
            message: 'MyProfilePage.errors.cannotChangeAuthentication',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
    }
  }
}

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.modal-header__title {
  margin-right: 1.5rem;
}

.modal-footer-buttons-spinner {
  width: 1rem;
  height: 1rem;
  margin-left: 0.5rem;
}

.modal-report-error {
  margin-top: 1rem;
}

.provider-card {
  position: relative;
  margin-top: 1.25rem;

  .provider-card-spinner {
    position: absolute;
    top: 2rem;
    right: 2rem;
    transform: translate(-50%, -50%);
  }
}
</style>
