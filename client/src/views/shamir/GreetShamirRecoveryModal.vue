<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-modal :close-button="{ visible: true, onClick: onCloseClicked }">
    <div class="greet-shamir">
      <div class="greet-shamir-header">
        <ion-text class="greet-shamir-header__title">
          {{ $msTranslate({ key: greetShamirStepTexts.title, data: { name: claimerName.label } }) }}
        </ion-text>
        <ion-text class="greet-shamir-header__description">
          {{ $msTranslate({ key: greetShamirStepTexts.description, data: { name: claimerName.label } }) }}
        </ion-text>
      </div>

      <ms-spinner
        v-if="step === Steps.WaitClaimer"
        class="greet-shamir__spinner"
      />

      <template v-if="step === Steps.ProvideCode">
        <sas-code-provide :code="greeter.greeterSasCode" />
        <div class="greet-shamir-waiting">
          <ion-text class="greet-shamir-waiting__text">
            {{ $msTranslate({ key: 'OrganizationRecovery.shamir.greet.waiting', data: { name: claimerName.label } }) }}
          </ion-text>
          <ms-spinner :size="16" />
        </div>
      </template>

      <sas-code-choice
        v-if="step === Steps.ChooseCode"
        :choices="greeter.claimerSasChoices"
        @select="selectClaimerCode"
        :disabled="querying"
      />
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import { GreetInProgressErrorTag, HumanHandle, ShamirGreeter } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonText, modalController } from '@ionic/vue';
import { Answer, askQuestion, MsModal, MsModalResult, MsSpinner } from 'megashark-lib';
import { computed, nextTick, onMounted, ref } from 'vue';

enum Steps {
  WaitClaimer = 'shamir-greet-wait-claimer',
  ProvideCode = 'shamir-greet-provide-code',
  ChooseCode = 'shamir-greet-choose-code',
}

const props = defineProps<{
  greeter: ShamirGreeter;
  claimerName: HumanHandle;
  informationManager: InformationManager;
}>();

const querying = ref(false);
const step = ref<Steps>(Steps.WaitClaimer);
const greetShamirStepTexts = computed(() => {
  switch (step.value) {
    case Steps.WaitClaimer:
      return {
        title: 'OrganizationRecovery.shamir.greet.waitClaimer.title',
        description: 'OrganizationRecovery.shamir.greet.waitClaimer.description',
      };
    case Steps.ProvideCode:
      return {
        title: 'OrganizationRecovery.shamir.greet.provideCode.title',
        description: 'OrganizationRecovery.shamir.greet.provideCode.description',
      };
    case Steps.ChooseCode:
      return {
        title: 'OrganizationRecovery.shamir.greet.chooseCode.title',
        description: 'OrganizationRecovery.shamir.greet.chooseCode.description',
      };
    default:
      return {
        title: 'OrganizationRecovery.shamir.greet.waitClaimer.title',
        description: 'OrganizationRecovery.shamir.greet.waitClaimer.description',
      };
  }
});

onMounted(async () => {
  startProcess();
});

async function startProcess(): Promise<void> {
  const waitResult = await props.greeter.initialWaitClaimer();
  if (!waitResult.ok) {
    props.informationManager.present(
      new Information({
        message: { key: 'OrganizationRecovery.shamir.greet.errors.waitFailed', data: { name: props.claimerName.label } },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    await modalController.dismiss(undefined, MsModalResult.Cancel);
    return;
  }
  step.value = Steps.ProvideCode;
  await nextTick();
  const waitTrustResult = await props.greeter.waitClaimerTrust();
  if (!waitTrustResult.ok) {
    props.informationManager.present(
      new Information({
        message: {
          key:
            waitTrustResult.error.tag === GreetInProgressErrorTag.GreetingAttemptCancelled
              ? 'OrganizationRecovery.shamir.greet.errors.claimerCancelled'
              : 'OrganizationRecovery.shamir.greet.errors.waitFailed',
          data: { name: props.claimerName.label },
        },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    await modalController.dismiss(undefined, MsModalResult.Cancel);
    return;
  }
  step.value = Steps.ChooseCode;
}

async function selectClaimerCode(code: string | null): Promise<void> {
  querying.value = true;
  try {
    if (code === null) {
      props.informationManager.present(
        new Information({
          message: { key: 'OrganizationRecovery.shamir.greet.errors.noCodeSelected', data: { name: props.claimerName.label } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      await props.greeter.denyTrust();
      await modalController.dismiss(undefined, MsModalResult.Cancel);
      return;
    }
    if (code !== props.greeter.claimerSasCode) {
      props.informationManager.present(
        new Information({
          message: { key: 'OrganizationRecovery.shamir.greet.errors.wrongCodeSelected', data: { name: props.claimerName.label } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      await props.greeter.denyTrust();
      await modalController.dismiss(undefined, MsModalResult.Cancel);
      return;
    }
    const trustResult = await props.greeter.signifyTrust();
    if (!trustResult.ok) {
      props.informationManager.present(
        new Information({
          message: { key: 'OrganizationRecovery.shamir.greet.errors.signifyTrustFailed', data: { name: props.claimerName.label } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      await modalController.dismiss(undefined, MsModalResult.Cancel);
      return;
    }
    const finalizeResult = await props.greeter.finalize();
    if (!finalizeResult.ok) {
      props.informationManager.present(
        new Information({
          message: { key: 'OrganizationRecovery.shamir.greet.errors.finalizeFailed', data: { name: props.claimerName.label } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      await modalController.dismiss(undefined, MsModalResult.Cancel);
    } else {
      props.informationManager.present(
        new Information({
          message: { key: 'OrganizationRecovery.shamir.greet.success', data: { name: props.claimerName.label } },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
      await modalController.dismiss(undefined, MsModalResult.Confirm);
    }
  } finally {
    querying.value = false;
  }
}

async function onCloseClicked(): Promise<boolean> {
  const answer = await askQuestion(
    'OrganizationRecovery.shamir.greet.interrupt.title',
    { key: 'OrganizationRecovery.shamir.greet.interrupt.message', data: { name: props.claimerName.label } },
    {
      yesText: 'OrganizationRecovery.shamir.greet.interrupt.yes',
      noText: 'OrganizationRecovery.shamir.greet.interrupt.no',
    },
  );
  if (answer !== Answer.Yes) {
    return false;
  }
  await props.greeter.cancel();
  await props.greeter.abort();
  return await modalController.dismiss(undefined, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.greet-shamir {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: ms.spacing('gap-5xl');
  padding: ms.spacing('padding-4xl');

  @include ms.responsive-breakpoint('sm') {
    padding-top: ms.spacing('padding-none');
  }
}

.greet-shamir-header {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
  width: 100%;
  padding-right: ms.spacing('padding-5xl');

  &__title {
    @include ms.font('heading-h4');
    color: ms.color('text-base-heading');
  }

  &__description {
    @include ms.font('body-lg-regular');
    color: ms.color('text-base-description');
  }
}

.greet-shamir-waiting {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: ms.spacing('gap-4xl');

  &__text {
    @include ms.font('body-lg-medium');
    color: ms.color('text-base-description');
  }
}
</style>
