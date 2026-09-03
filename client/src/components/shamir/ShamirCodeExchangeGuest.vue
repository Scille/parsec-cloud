<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="code-exchange">
    <div class="code-exchange-header">
      <ion-text class="code-exchange-header__title">
        {{ $msTranslate(codeExchangeStepTexts.title) }}
      </ion-text>
      <ion-text class="code-exchange-header__description">
        {{ $msTranslate({ key: codeExchangeStepTexts.description, data: { name: greeterName.label } }) }}
      </ion-text>
    </div>

    <sas-code-choice
      v-if="step === Steps.ChooseCode"
      :choices="claimer.greeterSasChoices"
      @select="selectGreeterCode"
      :disabled="querying"
    />

    <template v-if="step === Steps.ProvideCode">
      <sas-code-provide
        class="code-exchange__code"
        :code="claimer.claimerSasCode"
      />
      <div class="code-exchange-waiting">
        <ion-text class="code-exchange-waiting__text">
          {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.claim.exchange.waiting') }}
        </ion-text>
        <ms-spinner :size="16" />
      </div>
    </template>
  </div>
</template>

<script lang="ts">
export enum CodeExchangeInterruptedReason {
  NoCodeSelected = 'no-code-selected',
  WrongCodeSelected = 'wrong-code-selected',
  SignifyTrustFailed = 'signify-trust-failed',
  WaitTrustFailed = 'wait-trust-failed',
  GreeterCancelled = 'greeter-cancelled',
}
</script>

<script setup lang="ts">
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import { ClaimInProgressErrorTag, HumanHandle, ShamirClaimer } from '@/parsec';
import { IonText } from '@ionic/vue';
import { MsSpinner } from 'megashark-lib';
import { computed, nextTick, ref } from 'vue';

enum Steps {
  ChooseCode = 'shamir-guest-choose-code',
  ProvideCode = 'shamir-guest-provide-code',
}

const step = ref<Steps>(Steps.ChooseCode);
const querying = ref(false);
const codeExchangeStepTexts = computed(() => {
  switch (step.value) {
    case Steps.ChooseCode:
      return {
        title: 'ImportRecoveryDevicePage.modal.shamir.claim.exchange.chooseCode.title',
        description: 'ImportRecoveryDevicePage.modal.shamir.claim.exchange.chooseCode.description',
      };
    case Steps.ProvideCode:
      return {
        title: 'ImportRecoveryDevicePage.modal.shamir.claim.exchange.provideCode.title',
        description: 'ImportRecoveryDevicePage.modal.shamir.claim.exchange.provideCode.description',
      };
    default:
      return {
        title: 'ImportRecoveryDevicePage.modal.shamir.claim.exchange.chooseCode.title',
        description: 'ImportRecoveryDevicePage.modal.shamir.claim.exchange.chooseCode.description',
      };
  }
});

const props = defineProps<{
  claimer: ShamirClaimer;
  greeterName: HumanHandle;
}>();

const emits = defineEmits<{
  (e: 'interrupted', reason: CodeExchangeInterruptedReason): void;
  (e: 'codeExchanged'): void;
}>();

async function selectGreeterCode(code: string | null): Promise<void> {
  querying.value = true;
  try {
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
    const result = await props.claimer.signifyTrust();
    if (!result.ok) {
      emits(
        'interrupted',
        result.error.tag === ClaimInProgressErrorTag.GreetingAttemptCancelled
          ? CodeExchangeInterruptedReason.GreeterCancelled
          : CodeExchangeInterruptedReason.SignifyTrustFailed,
      );
      return;
    }
    step.value = Steps.ProvideCode;
    await nextTick();
    const waitResult = await props.claimer.waitGreeterTrust();
    if (!waitResult.ok) {
      emits(
        'interrupted',
        waitResult.error.tag === ClaimInProgressErrorTag.GreetingAttemptCancelled
          ? CodeExchangeInterruptedReason.GreeterCancelled
          : CodeExchangeInterruptedReason.WaitTrustFailed,
      );
      return;
    }
    emits('codeExchanged');
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
.code-exchange {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: ms.spacing('gap-5xl');
}

.code-exchange-header {
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

.code-exchange__code.code-exchange__code {
  width: 10.9375rem;
  height: auto;
  padding: ms.spacing('padding-5xl') ms.spacing('padding-4xl');
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-default-secondary');
  color: ms.color('text-brand-default-hover');
  box-shadow:
    0 3px 5px rgba(0, 0, 0, 0.02),
    0 1px 1px rgba(0, 0, 0, 0.04);
}

.code-exchange-waiting {
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
