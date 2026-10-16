<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="organization-recovery-container">
    <section class="recovery-section recovery-section--file">
      <div
        class="recovery-method"
        :class="hasRecoveryDevice ? 'restore-password__advice--done' : ''"
      >
        <div class="recovery-method-content">
          <ms-image
            :image="isDarkTheme ? RecoveryFileIconDark : RecoveryFileIcon"
            class="recovery-method-content-image"
          />

          <div class="recovery-method-content-text">
            <ion-text class="recovery-method-content-text__title">
              {{ $msTranslate('OrganizationRecovery.file.title') }}
            </ion-text>
            <ion-text class="recovery-method-content-text__description">
              {{ $msTranslate('OrganizationRecovery.file.description') }}
            </ion-text>
          </div>
        </div>

        <div class="recovery-method-actions">
          <ion-button
            class="action-button color-secondary"
            :class="{ actionDone: hasRecoveryDevice }"
            @click="exportRecoveryDevice"
          >
            {{ $msTranslate('OrganizationRecovery.file.action') }}
          </ion-button>
          <ion-text
            v-if="hasRecoveryDevice"
            class="action-validation"
          >
            <ion-icon
              :icon="checkmarkCircle"
              class="action-validation-icon"
            />
            {{ $msTranslate('OrganizationRecovery.file.downloaded') }}
          </ion-text>
        </div>

        <div class="recovery-method-state">
          <ion-text
            class="badge-active"
            v-if="hasRecoveryDevice"
          >
            {{ $msTranslate('OrganizationRecovery.state.active') }}
          </ion-text>
          <ion-text
            v-else
            class="badge-inactive"
          >
            {{ $msTranslate('OrganizationRecovery.state.inactive') }}
          </ion-text>
        </div>
      </div>
    </section>

    <section class="recovery-section recovery-section--device">
      <div
        class="recovery-method"
        :class="hasSecondaryDevice ? 'restore-password__advice--done' : ''"
      >
        <div class="recovery-method-content">
          <ms-image
            :image="isDarkTheme ? RecoveryDeviceIconDark : RecoveryDeviceIcon"
            class="recovery-method-content-image"
          />

          <div class="recovery-method-content-text">
            <ion-text class="recovery-method-content-text__title">
              {{ $msTranslate('OrganizationRecovery.device.title') }}
            </ion-text>
            <ion-text class="recovery-method-content-text__description">
              {{ $msTranslate('OrganizationRecovery.device.description') }}
            </ion-text>
          </div>
        </div>

        <div class="recovery-method-actions">
          <ion-button
            class="action-button color-secondary"
            :class="{ actionDone: hasSecondaryDevice }"
            @click="onAddDeviceClick()"
          >
            {{ $msTranslate('OrganizationRecovery.device.action') }}
          </ion-button>
          <ion-text
            v-if="hasSecondaryDevice && !queryingDevices"
            class="action-validation"
            @click="navigateToDevicesPage()"
          >
            <ion-icon
              :icon="checkmarkCircle"
              class="action-validation-icon"
            />
            <span class="has-devices">
              {{
                $msTranslate({
                  key: 'OrganizationRecovery.device.otherActiveDevices',
                  data: { count: devices.length - 1 },
                  count: devices.length,
                })
              }}
            </span>
          </ion-text>
        </div>

        <div class="recovery-method-state">
          <ms-spinner v-if="queryingDevices" />
          <ion-text
            class="badge-active"
            v-else-if="hasSecondaryDevice && !queryingDevices"
          >
            {{ $msTranslate('OrganizationRecovery.state.active') }}
          </ion-text>
          <ion-text
            v-else
            class="badge-inactive"
          >
            {{ $msTranslate('OrganizationRecovery.state.inactive') }}
          </ion-text>
        </div>
      </div>
    </section>

    <section
      class="recovery-section recovery-section--trust"
      v-if="Env.isShamirEnabled()"
    >
      <div class="recovery-method">
        <div class="recovery-method-content">
          <ms-image
            :image="isDarkTheme ? RecoveryTrustIconDark : RecoveryTrustIcon"
            class="recovery-method-content-image"
          />

          <div class="recovery-method-content-text">
            <ion-text class="recovery-method-content-text__title">
              {{ $msTranslate('OrganizationRecovery.shamir.title') }}
            </ion-text>
            <ion-text class="recovery-method-content-text__description">
              {{
                $msTranslate({
                  key: 'OrganizationRecovery.shamir.description',
                  data: { count: shamirThreshold },
                })
              }}
            </ion-text>
          </div>
        </div>

        <div class="recovery-method-actions">
          <ion-button
            class="action-button color-secondary"
            @click="openShamirRecoveryModal(ShamirTab.Self)"
          >
            {{ $msTranslate('OrganizationRecovery.shamir.action') }}
          </ion-button>
          <ion-button
            v-show="!isExternal"
            class="action-button color-secondary"
            fill="outline"
            @click="openShamirRecoveryModal(ShamirTab.Others)"
          >
            {{ $msTranslate('OrganizationRecovery.shamir.actionSecondary') }}
          </ion-button>
        </div>

        <div class="recovery-method-state">
          <ion-text
            class="badge-active"
            v-if="shamirInfo?.isUsable()"
          >
            {{ $msTranslate('OrganizationRecovery.state.active') }}
          </ion-text>
          <ion-text
            v-if="shamirInfo && !shamirInfo.isUsable()"
            class="badge-inactive"
          >
            {{ $msTranslate('OrganizationRecovery.state.inactive') }}
          </ion-text>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import RecoveryDeviceIconDark from '@/assets/images/recovery-device-icon-dark.svg?raw';
import RecoveryDeviceIcon from '@/assets/images/recovery-device-icon.svg?raw';
import RecoveryFileIconDark from '@/assets/images/recovery-file-icon-dark.svg?raw';
import RecoveryFileIcon from '@/assets/images/recovery-file-icon.svg?raw';
import RecoveryTrustIconDark from '@/assets/images/recovery-trusted-user-icon-dark.svg?raw';
import RecoveryTrustIcon from '@/assets/images/recovery-trusted-user-icon.svg?raw';
import { ShamirTab } from '@/components/profile/types';
import {
  getClientInfo,
  getRequiredShamirThreshold,
  getSelfShamirRecovery,
  listOwnDevices,
  OwnDeviceInfo,
  SelfShamirRecoveryInfo,
  UserProfile,
} from '@/parsec';
import { navigateTo, ProfilePages, Routes } from '@/router';
import { Env } from '@/services/environment';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { openDeviceGreetModal } from '@/views/devices/utils';
import ExportRecoveryDevice from '@/views/profile/ExportRecoveryDeviceModal.vue';
import ShamirRecoveryModal from '@/views/profile/ShamirRecoveryModal.vue';
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { Answer, askQuestion, MsImage, MsModalResult, MsSpinner, useTheme, useWindowSize } from 'megashark-lib';
import { computed, inject, onMounted, Ref, ref } from 'vue';

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
const { isDarkTheme } = useTheme();
const { isLargeDisplay } = useWindowSize();

const shamirThreshold = ref(3);
const orgId = ref('');
const hasRecoveryDevice = ref(false);
const queryingDevices = ref(true);
const isExternal = ref(false);
const devices: Ref<OwnDeviceInfo[]> = ref([]);
const hasSecondaryDevice = computed(() => devices.value.length > 1);
const shamirInfo = ref<SelfShamirRecoveryInfo | undefined>(undefined);
const error = ref('');

onMounted(async (): Promise<void> => {
  const [clientInfoResult, threshold] = await Promise.all([
    getClientInfo(),
    getRequiredShamirThreshold(),
    refreshDevicesList(),
    refreshShamirStatus(),
  ]);
  if (clientInfoResult.ok) {
    orgId.value = clientInfoResult.value.organizationId;
    isExternal.value = clientInfoResult.value.currentProfile === UserProfile.Outsider;
    shamirThreshold.value = threshold;
  }
});

async function refreshShamirStatus(): Promise<void> {
  const result = await getSelfShamirRecovery();

  if (!result.ok) {
    error.value = 'OrganizationRecovery.shamir.errors.generic';
  } else {
    shamirInfo.value = result.value;
  }
}

async function refreshDevicesList(): Promise<void> {
  const result = await listOwnDevices();
  if (result.ok) {
    devices.value = result.value.filter((d) => !d.isRecovery && !d.isShamir && !d.isRegistration).sort((d) => (d.isCurrent ? -1 : 1));
    hasRecoveryDevice.value = result.value.find((d) => d.isRecovery) !== undefined;
    queryingDevices.value = false;
  } else {
    informationManager.value.present(
      new Information({
        message: 'DevicesPage.greet.errors.retrieveDeviceInfoFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    window.nativeAPI.log('error', `Failed to list devices ${JSON.stringify(result.error)}`);
  }
}

async function navigateToDevicesPage(): Promise<void> {
  await navigateTo(Routes.MyProfile, { replace: true, query: { profilePage: ProfilePages.Devices } });
}

async function exportRecoveryDevice(): Promise<void> {
  if (hasRecoveryDevice.value) {
    const answer = await askQuestion(
      'OrganizationRecovery.done.recreateQuestionTitle',
      'OrganizationRecovery.done.recreateQuestionMessage',
      { yesText: 'OrganizationRecovery.done.recreateYes', noText: 'OrganizationRecovery.done.recreateNo' },
    );
    if (answer === Answer.No) {
      return;
    }
  }

  const modal = await modalController.create({
    component: ExportRecoveryDevice,
    cssClass: 'export-recovery-device-modal',
    componentProps: {
      organizationId: orgId.value,
      informationManager: informationManager.value,
    },
    canDismiss: true,
    backdropDismiss: true,
    showBackdrop: true,
  });

  await modal.present();
  const { role } = await modal.onDidDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm) {
    return;
  }
  const devicesResult = await listOwnDevices();
  if (devicesResult.ok) {
    const lastDevice = devicesResult.value.toSorted((d1, d2) => (d1.createdOn > d2.createdOn ? -1 : 1))[0];
    if (lastDevice) {
      eventDistributor.value.dispatchEvent(Events.DeviceCreated, { info: lastDevice });
      informationManager.value.present(
        new Information({
          message: 'OrganizationRecovery.file.downloaded',
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
    hasRecoveryDevice.value = true;
  }
}

async function openShamirRecoveryModal(tab: ShamirTab): Promise<void> {
  const modal = await modalController.create({
    component: ShamirRecoveryModal,
    cssClass: 'shamir-recovery-modal',
    componentProps: {
      informationManager: informationManager.value,
      tab: tab,
    },
    canDismiss: true,
    backdropDismiss: true,
    showBackdrop: true,
    handle: false,
    breakpoints: [1],
    initialBreakpoint: isLargeDisplay.value ? undefined : 1,
  });
  await modal.present();
  await modal.onDidDismiss();
  await modal.dismiss();
  await refreshShamirStatus();
}

async function onAddDeviceClick(): Promise<void> {
  const result = await openDeviceGreetModal(informationManager.value);
  if (result === MsModalResult.Confirm) {
    await refreshDevicesList();
  }
}
</script>

<style scoped lang="scss">
.organization-recovery-container {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
}

.recovery-section {
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-default');
  border: 1px solid ms.color('border-neutral-default-subtle');
  padding: ms.spacing('padding-3xl');
  box-shadow: ms.shadow('elevation-sm');
}

.recovery-method {
  display: flex;
  position: relative;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');

  &-content {
    display: flex;
    gap: ms.spacing('gap-3xl');

    &-image {
      width: fit-content;
      height: fit-content;
      flex-shrink: 0;

      @include ms.responsive-breakpoint('sm') {
        width: 2.5rem;
        height: 2.5rem;
      }
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-sm');

      &__title {
        @include ms.font('heading-h5');
        color: ms.color('text-base-heading');
        padding-right: ms.spacing('padding-7xl');
      }

      &__description {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
        padding-right: ms.spacing('padding-5xl');

        @include ms.responsive-breakpoint('md') {
          padding-right: 0;
        }
      }
    }
  }

  &-actions {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    margin-left: 3.75rem;
    gap: ms.spacing('gap-2xl');

    @include ms.responsive-breakpoint('md') {
      margin-left: 0;
    }

    @include ms.responsive-breakpoint('sm') {
      flex-direction: column;
      align-items: flex-start;
    }

    .action-button {
      @include ms.responsive-breakpoint('md') {
        width: 100%;
      }
    }

    .action-validation {
      @include ms.font('label-md-medium');
      display: flex;
      color: ms.color('text-success-default');
      align-items: center;
      gap: ms.spacing('gap-sm');

      &-icon {
        font-size: 1rem;
        color: ms.color('icon-success-default');
      }
    }
  }

  &-state {
    margin-left: auto;
    top: -0rem;
    right: -0rem;
    position: absolute;

    [class*='badge-'] {
      @include ms.font('label-md-medium');
      flex-shrink: 0;
      padding: ms.spacing('padding-sm') ms.spacing('padding-lg');
      border-radius: ms.radius('2xl');
      white-space: nowrap;
    }

    .badge-active {
      background: ms.color('surface-brand-default-subtle');
      color: ms.color('text-brand-default');
    }

    .badge-inactive {
      color: ms.color('text-error-default');
      background: ms.color('surface-error-default-subtle');
    }
  }
}

.recovery-section--device {
  .has-devices:hover {
    text-decoration: underline;
    cursor: pointer;
  }
}

.restore-password {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');

  &__description {
    margin-top: -0.5rem;
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
    color: ms.color('text-base-description');
  }

  &__advice {
    display: flex;
    width: 100%;
    align-items: center;
    background: ms.color('surface-information-default-subtle-hover');
    padding: ms.spacing('padding-xl') ms.spacing('padding-2xl');
    gap: ms.spacing('gap-lg');
    border-radius: ms.radius('lg');
    color: ms.color('text-information-default');

    .advice-icon {
      font-size: 1rem;
      color: ms.color('icon-information-default');
      flex-shrink: 0;
    }
  }

  &-button {
    --background: #{ms.color('surface-neutral-default')};
    --background-hover: #{ms.color('surface-neutral-default-hover')};
    color: ms.color('text-neutral-on-color');
    width: fit-content;

    @include ms.responsive-breakpoint('xs') {
      position: fixed;
      bottom: 2rem;
      left: 2rem;
      width: calc(100% - 4rem);
      margin: auto;
      z-index: 2;
      box-shadow: var(--parsec-shadow-strong);
    }
  }
}
</style>
