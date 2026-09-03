<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-modal :close-button="{ visible: true, onClick: onCloseClicked }">
    <div class="claim-shamir">
      <div
        v-if="!isCodeExchangeStep"
        class="claim-shamir-header"
        :class="{ 'claim-shamir-header--done': finishedInfo }"
      >
        <div class="claim-shamir-header__text">
          <ms-image
            :image="theme === Theme.Dark ? ShamirIconCircleDark : ShamirIconCircleLight"
            class="claim-shamir-header__image"
          />
          <ion-text class="claim-shamir-header__title">
            {{
              $msTranslate(
                finishedInfo
                  ? 'ImportRecoveryDevicePage.modal.shamir.claim.done.title'
                  : 'ImportRecoveryDevicePage.modal.shamir.link.title',
              )
            }}
          </ion-text>
          <ion-text class="claim-shamir-header__subtitle">
            {{
              $msTranslate(
                finishedInfo
                  ? 'ImportRecoveryDevicePage.modal.shamir.claim.done.subtitle'
                  : 'ImportRecoveryDevicePage.modal.shamir.link.subtitle',
              )
            }}
          </ion-text>
        </div>
        <div class="claim-shamir-progress">
          <ms-progress
            :progress="progress"
            :appearance="MsProgressAppearance.Bar"
          />
          <div class="claim-shamir-progress__labels">
            <ion-text>{{ $msTranslate(progressLabel) }}</ion-text>
            <ion-text>
              {{
                $msTranslate({
                  key: 'ImportRecoveryDevicePage.modal.shamir.claim.progressCount',
                  data: { current: currentShare, total: info.threshold },
                })
              }}
            </ion-text>
          </div>
        </div>
      </div>

      <div class="claim-shamir-content">
        <shamir-code-exchange-guest
          v-if="claimer && isCodeExchangeStep && greeterName"
          :claimer="claimer"
          :greeter-name="greeterName"
          @code-exchanged="onCodeExchanged"
          @interrupted="onCodeExchangeInterrupted"
        />

        <template v-else-if="!finishedInfo">
          <div class="recipients">
            <ms-search-input
              v-model="search"
              placeholder="ImportRecoveryDevicePage.modal.shamir.claim.searchPlaceholder"
            />
            <div class="recipient-list">
              <div
                class="recipient"
                v-for="recipient in filteredRecipients"
                :key="recipient.userId"
              >
                <div class="recipient__avatar">
                  <ion-icon :icon="personCircle" />
                </div>
                <div class="recipient__info">
                  <ion-text class="recipient__name">{{ recipient.humanHandle.label }}</ion-text>
                  <ion-text class="recipient__email">{{ recipient.humanHandle.email }}</ion-text>
                </div>
                <ion-icon
                  v-if="recipientHasContributed(recipient)"
                  class="recipient__check"
                  :icon="checkmarkCircle"
                />
                <div
                  v-else-if="exchangeStartedWithUserId === recipient.userId"
                  class="recipient__waiting"
                >
                  <ms-spinner :size="14" />
                  <ion-text class="recipient__waiting-text">
                    {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.claim.waiting') }}
                  </ion-text>
                  <ion-button
                    class="recipient__button recipient__cancel"
                    fill="clear"
                    @click="cancelWait"
                  >
                    {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.claim.cancel') }}
                  </ion-button>
                </div>
                <ion-button
                  v-else
                  class="recipient__button recipient__choose"
                  fill="clear"
                  :disabled="exchangeStartedWithUserId !== undefined || recipient.revokedOn !== null"
                  @click="chooseRecipient(recipient)"
                >
                  {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.claim.choose') }}
                </ion-button>
              </div>
              <ion-text
                v-if="filteredRecipients.length === 0"
                class="recipient-list__empty"
              >
                {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.claim.noMatch') }}
              </ion-text>
            </div>
          </div>

          <div class="claim-shamir-reminder">
            <ion-icon
              class="claim-shamir-reminder__icon"
              :icon="informationCircle"
            />
            <div class="claim-shamir-reminder__text">
              <ion-text class="claim-shamir-reminder__title">
                {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.claim.reminderTitle') }}
              </ion-text>
              <ion-text class="claim-shamir-reminder__description">
                <ms-rich-text text="ImportRecoveryDevicePage.modal.shamir.claim.reminderDescription" />
              </ion-text>
            </div>
          </div>
        </template>

        <ion-button
          v-else
          class="claim-shamir-done-button"
          @click="finish"
        >
          {{ $msTranslate('ImportRecoveryDevicePage.modal.shamir.claim.done.continue') }}
        </ion-button>
      </div>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import ShamirIconCircleDark from '@/assets/images/shamir-icon-circle-dark.svg?raw';
import ShamirIconCircleLight from '@/assets/images/shamir-icon-circle-light.svg?raw';
import ShamirCodeExchangeGuest, { CodeExchangeInterruptedReason } from '@/components/shamir/ShamirCodeExchangeGuest.vue';
import {
  AnyClaimRetrievedInfoShamirRecovery,
  ClaimInProgressErrorTag,
  HumanHandle,
  ShamirClaimer,
  shamirPickRecipient,
  ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient,
  ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice,
  ShamirRecoveryClaimMaybeRecoverDeviceInfoTag,
  ShamirRecoveryRecipient,
  UserID,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle, informationCircle, personCircle } from 'ionicons/icons';
import {
  Answer,
  askQuestion,
  MsImage,
  MsModal,
  MsModalResult,
  MsProgress,
  MsProgressAppearance,
  MsRichText,
  MsSearchInput,
  MsSpinner,
  Theme,
  Translatable,
  useTheme,
} from 'megashark-lib';
import { computed, ref } from 'vue';

const props = defineProps<{
  invitationInfo: AnyClaimRetrievedInfoShamirRecovery;
  informationManager: InformationManager;
}>();

const { theme } = useTheme();

const info = ref<AnyClaimRetrievedInfoShamirRecovery | ShamirRecoveryClaimMaybeRecoverDeviceInfoPickRecipient>(props.invitationInfo);
const finishedInfo = ref<ShamirRecoveryClaimMaybeRecoverDeviceInfoRecoverDevice | undefined>(undefined);
const exchangeStartedWithUserId = ref<UserID | undefined>(undefined);
const claimer = ref<ShamirClaimer | undefined>(undefined);
const greeterName = ref<HumanHandle | undefined>(undefined);
const search = ref('');
const isCodeExchangeStep = computed(() => claimer.value !== undefined && claimer.value.greeterSasCode !== '');
const shares = computed(() => {
  if (info.value.tag === ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.PickRecipient) {
    return [...info.value.recoveredShares.values()].reduce((sum, value) => sum + value, 0);
  }
  return 0;
});

const currentShare = computed(() => (finishedInfo.value ? info.value.threshold : Math.min(shares.value, info.value.threshold)));
const progressLabel = computed(() =>
  finishedInfo.value
    ? 'ImportRecoveryDevicePage.modal.shamir.claim.progressDone'
    : 'ImportRecoveryDevicePage.modal.shamir.claim.progressLabel',
);

const progress = computed(() => (finishedInfo.value ? 100 : Math.max((shares.value / info.value.threshold) * 100, 2)));

const filteredRecipients = computed(() => {
  const query = search.value.toLocaleLowerCase();
  return props.invitationInfo.recipients.filter(
    (recipient) =>
      recipient.humanHandle.label.toLocaleLowerCase().includes(query) || recipient.humanHandle.email.toLocaleLowerCase().includes(query),
  );
});

function recipientHasContributed(recipient: ShamirRecoveryRecipient): boolean {
  if (info.value.tag === ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.PickRecipient) {
    return info.value.recoveredShares.has(recipient.userId);
  }
  return false;
}

async function cancelWait(): Promise<void> {
  if (claimer.value) {
    await claimer.value.cancel();
  }
  claimer.value = undefined;
  greeterName.value = undefined;
}

async function chooseRecipient(recipient: ShamirRecoveryRecipient): Promise<void> {
  try {
    greeterName.value = undefined;
    exchangeStartedWithUserId.value = recipient.userId;
    const result = await shamirPickRecipient(info.value.handle, recipient.userId);
    if (!result.ok) {
      props.informationManager.present(
        new Information({
          message: 'ImportRecoveryDevicePage.modal.shamir.claim.errors.pickRecipientFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return;
    }
    claimer.value = new ShamirClaimer(result.value);
    const waitResult = await claimer.value.initialWaitGreeter();
    if (waitResult.ok) {
      greeterName.value = recipient.humanHandle;
    } else if (!waitResult.ok && waitResult.error.tag !== ClaimInProgressErrorTag.Cancelled) {
      props.informationManager.present(
        new Information({
          message: 'ImportRecoveryDevicePage.modal.shamir.claim.errors.waitFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      claimer.value = undefined;
    }
  } finally {
    exchangeStartedWithUserId.value = undefined;
  }
}

async function onCodeExchangeInterrupted(reason: CodeExchangeInterruptedReason): Promise<void> {
  let message!: Translatable;
  switch (reason) {
    case CodeExchangeInterruptedReason.GreeterCancelled: {
      message = 'ImportRecoveryDevicePage.modal.shamir.claim.errors.greeterCancelled';
      break;
    }
    case CodeExchangeInterruptedReason.NoCodeSelected: {
      message = 'ImportRecoveryDevicePage.modal.shamir.claim.errors.noCodeSelected';
      break;
    }
    case CodeExchangeInterruptedReason.WrongCodeSelected: {
      message = 'ImportRecoveryDevicePage.modal.shamir.claim.errors.wrongCodeSelected';
      break;
    }
    default: {
      message = 'ImportRecoveryDevicePage.modal.shamir.claim.errors.exchangeFailed';
      break;
    }
  }
  props.informationManager.present(
    new Information({
      message: message,
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  claimer.value = undefined;
  greeterName.value = undefined;
}

async function onCodeExchanged(): Promise<void> {
  if (!claimer.value) {
    window.nativeAPI.log('error', 'Claimer is undefined after code exchange, should not happen');
    return;
  }
  try {
    const claimResult = await claimer.value.claim();
    if (!claimResult.ok) {
      if (claimResult.error.tag !== ClaimInProgressErrorTag.Cancelled) {
        props.informationManager.present(
          new Information({
            message: 'ImportRecoveryDevicePage.modal.shamir.claim.errors.claimFailed',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
        return;
      }
    }
    const shareResult = await claimer.value.addShare(info.value.handle);
    if (!shareResult.ok) {
      props.informationManager.present(
        new Information({
          message: 'ImportRecoveryDevicePage.modal.shamir.claim.errors.addShareFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return;
    }
    if (shareResult.value.tag === ShamirRecoveryClaimMaybeRecoverDeviceInfoTag.PickRecipient) {
      info.value = shareResult.value;
    } else {
      finishedInfo.value = shareResult.value;
    }
  } finally {
    claimer.value = undefined;
  }
}

async function onCloseClicked(): Promise<boolean> {
  const answer = await askQuestion(
    'ImportRecoveryDevicePage.modal.shamir.claim.interrupt.title',
    'ImportRecoveryDevicePage.modal.shamir.claim.interrupt.message',
    {
      yesText: 'ImportRecoveryDevicePage.modal.shamir.claim.interrupt.yes',
      noText: 'ImportRecoveryDevicePage.modal.shamir.claim.interrupt.no',
    },
  );
  if (answer !== Answer.Yes) {
    return false;
  }
  if (claimer.value) {
    await claimer.value.cancel();
    await claimer.value.abort();
  }
  return await modalController.dismiss(undefined, MsModalResult.Cancel);
}

async function finish(): Promise<void> {
  if (finishedInfo.value) {
    await modalController.dismiss({ info: finishedInfo.value }, MsModalResult.Confirm);
  }
}
</script>

<style scoped lang="scss">
.claim-shamir {
  display: flex;
  flex-direction: column;
}

.claim-shamir-header {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
  padding: ms.spacing('padding-5xl') ms.spacing('padding-6xl') ms.spacing('padding-3xl');
  background: ms.color('surface-base-default-secondary');

  &--done {
    padding: ms.spacing('padding-4xl') ms.spacing('padding-4xl') ms.spacing('padding-lg');
    background: none;
  }

  &__text {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: ms.spacing('gap-lg');
    text-align: center;
  }

  &__image {
    width: 4.625rem;
    height: 4.625rem;
  }

  &__title {
    @include ms.font('heading-h4');
    color: ms.color('text-base-heading');
  }

  &__subtitle {
    @include ms.font('body-md-medium');
    color: ms.color('text-base-description');
  }
}

.claim-shamir-progress {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');

  :deep(.progress) {
    background: ms.color('border-base-default');
  }

  &__labels {
    @include ms.font('body-md-medium');
    display: flex;
    justify-content: space-between;
    color: ms.color('text-base-description');
  }
}

.claim-shamir-content {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
  padding: ms.spacing('padding-4xl');
}

.recipients {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
}

.recipient-list {
  display: flex;
  flex-direction: column;
  max-height: 20rem;
  overflow-y: auto;
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-page-secondary');

  &__empty {
    @include ms.font('body-md-regular');
    padding: ms.spacing('padding-3xl');
    color: ms.color('text-base-description');
    text-align: center;
  }
}

.recipient {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-lg');
  min-height: 3.75rem;
  background: ms.color('surface-base-default');
  padding: ms.spacing('padding-lg') ms.spacing('padding-lg') ms.spacing('padding-lg') ms.spacing('padding-3xl');

  &:not(:last-child) {
    border-bottom: ms.border('thin') solid ms.color('border-base-default');
  }

  &:hover {
    background: ms.color('surface-base-default-secondary');
  }

  &__avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 2rem;
    height: 2rem;
    border: ms.border('thin') solid ms.color('border-base-default');
    border-radius: ms.radius('full');
    background: ms.color('surface-brand-default-subtle-hover');
    color: ms.color('icon-brand-default');
    font-size: 1rem;
  }

  &__info {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
  }

  &__name {
    @include ms.font('label-md-emphasis');
    color: ms.color('text-base-body');
  }

  &__email {
    @include ms.font('body-sm-regular');
    color: ms.color('text-base-description');
  }

  &__waiting {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    color: ms.color('text-base-description');

    &-text {
      @include ms.font('body-sm-regular');
    }
  }

  &__check {
    font-size: 1.25rem;
    margin-right: 0.75rem;
    color: ms.color('icon-success-default');
  }
}

.claim-shamir-reminder {
  display: flex;
  align-items: flex-start;
  gap: ms.spacing('gap-xl');
  padding: ms.spacing('padding-2xl');
  border-radius: ms.radius('2xl');
  background: ms.color('surface-brand-default-subtle-hover');

  &__icon {
    flex-shrink: 0;
    width: 1.125rem;
    height: 1.125rem;
    color: ms.color('icon-brand-default');
  }

  &__text {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');
  }

  &__title {
    @include ms.font('heading-h6');
    color: ms.color('text-brand-default');
  }

  &__description {
    @include ms.font('body-md-regular');
    color: ms.color('text-base-body');
  }
}

.claim-shamir-done-button {
  width: 100%;
}
</style>
