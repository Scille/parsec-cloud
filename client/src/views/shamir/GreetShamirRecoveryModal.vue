<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="HELP SOMEONE RECOVER THEIR ACCOUNT"
    :close-button="{ visible: true, onClick: onCloseClicked }"
  >
    <div v-if="step === Steps.WaitClaimer">
      <ms-spinner class="spinner" />
    </div>
    <div
      v-if="step === Steps.ProvideCode"
      class="provide-code"
    >
      <sas-code-provide :code="greeter.greeterSasCode" />
    </div>
    <div
      v-if="step === Steps.ChooseCode"
      class="choose-code"
    >
      <sas-code-choice
        :choices="greeter.claimerSasChoices"
        @select="selectClaimerCode"
      />
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import { GreetInProgressErrorTag, ShamirGreeter } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { modalController } from '@ionic/vue';
import { Answer, askQuestion, MsModal, MsModalResult, MsSpinner, Translatable } from 'megashark-lib';
import { nextTick, onMounted, ref } from 'vue';

enum Steps {
  WaitClaimer = 'shamir-greet-wait-claimer',
  ProvideCode = 'shamir-greet-provide-code',
  ChooseCode = 'shamir-greet-choose-code',
}

const props = defineProps<{
  greeter: ShamirGreeter;
  informationManager: InformationManager;
}>();

const step = ref<Steps>(Steps.WaitClaimer);

onMounted(async () => {
  startProcess();
});

async function startProcess(): Promise<void> {
  const waitResult = await props.greeter.initialWaitClaimer();
  if (!waitResult.ok) {
    props.informationManager.present(
      new Information({
        message: 'WAIT FAILED',
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
    let message!: Translatable;

    if (waitTrustResult.error.tag === GreetInProgressErrorTag.GreetingAttemptCancelled) {
      message = 'CLAIMER CANCELLED';
    } else {
      message = 'UNKNOWN ERROR';
    }
    props.informationManager.present(
      new Information({
        message: message,
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
  if (code === null) {
    props.informationManager.present(
      new Information({
        message: 'NO CODE SELECTED',
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
        message: 'WRONG CODE SELECTED',
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
        message: 'SIGNIFY TRUST FAILED',
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
        message: 'FINALIZE FAILED',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );

    await modalController.dismiss(undefined, MsModalResult.Cancel);
  } else {
    props.informationManager.present(
      new Information({
        message: 'SUCCESS',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await modalController.dismiss(undefined, MsModalResult.Confirm);
  }
}

async function onCloseClicked(): Promise<boolean> {
  const answer = await askQuestion('INTERRUPT THE PROCESS', 'ARE YOU SURE?', { yesText: 'INTERRUPT', noText: 'KEEP GOING' });
  if (answer !== Answer.Yes) {
    return false;
  }
  await props.greeter.cancel();
  await props.greeter.abort();
  return await modalController.dismiss(undefined, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss"></style>
