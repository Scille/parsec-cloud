<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="step === Steps.ChooseCode"
    class="choose-code"
  >
    <sas-code-choice
      :choices="claimer.greeterSasChoices"
      @select="selectGreeterCode"
    />
  </div>

  <div
    v-if="step === Steps.ProvideCode"
    class="provide-code"
  >
    <sas-code-provide :code="claimer.claimerSasCode" />
  </div>
</template>

<script lang="ts">
export enum CodeExchangeInterruptedReason {
  NoCodeSelected = 'no-code-selected',
  WrongCodeSelected = 'wrong-code-selected',
  SignifyTrustFailed = 'signify-trust-failed',
  WaitTrustFailed = 'wait-trust-failed',
}
</script>

<script setup lang="ts">
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import { ShamirClaimer } from '@/parsec';
import { nextTick, ref } from 'vue';

enum Steps {
  ChooseCode = 'shamir-guest-choose-code',
  ProvideCode = 'shamir-guest-provide-code',
}

const step = ref<Steps>(Steps.ChooseCode);

const props = defineProps<{
  claimer: ShamirClaimer;
}>();

const emits = defineEmits<{
  (e: 'interrupted', reason: CodeExchangeInterruptedReason): void;
  (e: 'codeExchanged'): void;
}>();

async function selectGreeterCode(code: string | null): Promise<void> {
  if (code === null) {
    await props.claimer.denyTrust();
    emits('interrupted', CodeExchangeInterruptedReason.NoCodeSelected);
    return;
  }
  if (code !== props.claimer.greeterSasCode) {
    await props.claimer.denyTrust();
    emits('interrupted', CodeExchangeInterruptedReason.WrongCodeSelected);
    return;
  }
  console.log('SIGNIFY TRUST');
  const result = await props.claimer.signifyTrust();
  if (!result) {
    emits('interrupted', CodeExchangeInterruptedReason.SignifyTrustFailed);
    return;
  }
  console.log('SIGNIFY TRUST RETURNED');
  step.value = Steps.ProvideCode;
  await nextTick();
  console.log('WAIT TRUST', props.claimer.claimerSasCode);
  const waitResult = await props.claimer.waitGreeterTrust();
  if (!waitResult.ok) {
    emits('interrupted', CodeExchangeInterruptedReason.WaitTrustFailed);
    return;
  }
  emits('codeExchanged');
}
</script>

<style scoped lang="scss"></style>
