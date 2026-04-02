<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-recovery-container">
    <section class="recovery-section recovery-section--file">
      <div
        class="recovery-method"
        :class="hasRecoveryDevice ? 'restore-password__advice--done' : ''"
      >
        <div class="recovery-method-content">
          <ms-image
            :image="RecoveryFileIcon"
            class="recovery-method-content-image"
          />

          <div class="recovery-method-content-text">
            <ion-text class="recovery-method-content-text__title title-h4">
              {{ $msTranslate('OrganizationRecovery.file.title') }}
            </ion-text>
            <ion-text class="recovery-method-content-text__description body-lg">
              {{ $msTranslate('OrganizationRecovery.file.description') }}
            </ion-text>
          </div>
        </div>

        <div class="recovery-method-actions">
          <ion-button
            class="button-normal button-default action-button"
            :class="{ actionDone: hasRecoveryDevice }"
            @click="exportRecoveryDevice"
            fill="clear"
          >
            {{ $msTranslate('OrganizationRecovery.file.action') }}
          </ion-button>
          <ion-text
            v-if="hasRecoveryDevice"
            class="button-medium action-validation"
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
            class="badge-active button-medium"
            v-if="hasRecoveryDevice"
          >
            {{ $msTranslate('OrganizationRecovery.state.active') }}
          </ion-text>
          <ion-text
            v-else
            class="badge-inactive button-medium"
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
            :image="RecoveryDeviceIcon"
            class="recovery-method-content-image"
          />

          <div class="recovery-method-content-text">
            <ion-text class="recovery-method-content-text__title title-h4">
              {{ $msTranslate('OrganizationRecovery.device.title') }}
            </ion-text>
            <ion-text class="recovery-method-content-text__description body-lg">
              {{ $msTranslate('OrganizationRecovery.device.description') }}
            </ion-text>
          </div>
        </div>

        <div class="recovery-method-actions">
          <ion-button
            class="button-normal button-default action-button"
            :class="{ actionDone: hasSecondaryDevice }"
            @click="onAddDeviceClick()"
            fill="clear"
          >
            {{ $msTranslate('OrganizationRecovery.device.action') }}
          </ion-button>
          <ion-text
            v-if="hasSecondaryDevice && !queryingDevices"
            class="button-medium action-validation"
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
            class="badge-active button-medium"
            v-else-if="hasSecondaryDevice && !queryingDevices"
          >
            {{ $msTranslate('OrganizationRecovery.state.active') }}
          </ion-text>
          <ion-text
            v-else
            class="badge-inactive button-medium"
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
            :image="RecoveryTrustIcon"
            class="recovery-method-content-image"
          />

          <div class="recovery-method-content-text">
            <ion-text class="recovery-method-content-text__title title-h4">
              {{ $msTranslate('OrganizationRecovery.shamir.title') }}
            </ion-text>
            <ion-text class="recovery-method-content-text__description body-lg">
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
            class="button-normal button-default action-button"
            fill="clear"
            @click="openShamirRecoveryModal(ShamirTab.Self)"
          >
            {{ $msTranslate('OrganizationRecovery.shamir.action') }}
          </ion-button>
          <ion-button
            v-show="!isExternal"
            class="button-normal button-default action-button actionDone"
            fill="clear"
            @click="openShamirRecoveryModal(ShamirTab.Others)"
          >
            {{ $msTranslate('OrganizationRecovery.shamir.actionSecondary') }}
          </ion-button>
        </div>

        <div class="recovery-method-state">
          <ion-text
            class="badge-active button-medium"
            v-if="shamirInfo?.isUsable()"
          >
            {{ $msTranslate('OrganizationRecovery.state.active') }}
          </ion-text>
          <ion-text
            v-if="shamirInfo && !shamirInfo.isUsable()"
            class="badge-inactive button-medium"
          >
            {{ $msTranslate('OrganizationRecovery.state.inactive') }}
          </ion-text>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import RecoveryDeviceIcon from '@/assets/images/recovery-device-icon.svg?raw';
import RecoveryFileIcon from '@/assets/images/recovery-file-icon.svg?raw';
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
import { Answer, askQuestion, MsImage, MsModalResult, MsSpinner, useWindowSize } from 'megashark-lib';
import { computed, inject, onMounted, Ref, ref } from 'vue';

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
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
    hasRecoveryDevice.value = devices.value.find((d) => d.isRecovery) !== undefined;
    queryingDevices.value = false;
  } else {
    informationManager.value.present(
      new Information({
        message: 'DevicesPage.greet.errors.retrieveDeviceInfoFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    window.electronAPI.log('error', `Failed to list devices ${JSON.stringify(result.error)}`);
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
  gap: 1rem;
}

.recovery-section {
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-white);
  padding: 1.75rem;
  box-shadow:
    0 1px 1px 0 rgba(0, 0, 0, 0.05),
    0 1px 4px 0 rgba(0, 0, 0, 0.03),
    0 0 1px 0 rgba(0, 0, 0, 0.2);
}

.recovery-method {
  display: flex;
  position: relative;
  flex-direction: column;
  gap: 1rem;

  &-content {
    display: flex;
    gap: 1rem;

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
      gap: 0.25rem;

      &__title {
        color: var(--parsec-color-light-secondary-text);
        padding-right: 3rem;
      }

      &__description {
        color: var(--parsec-color-light-secondary-hard-grey);
        font-size: 0.9375rem;
        padding-right: 2rem;

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
    gap: 1rem;

    @include ms.responsive-breakpoint('md') {
      margin-left: 0;
    }

    @include ms.responsive-breakpoint('sm') {
      flex-direction: column;
      align-items: flex-start;
    }

    .action-button {
      --background: var(--parsec-color-light-secondary-text);
      --background-hover: var(--parsec-color-light-secondary-contrast);
      color: var(--parsec-color-light-secondary-white);

      &.actionDone {
        --background: var(--parsec-color-light-secondary-background);
        --background-hover: var(--parsec-color-light-secondary-medium);
        box-shadow:
          0 1px 1px 0 rgba(0, 0, 0, 0.05),
          0 1px 4px 0 rgba(0, 0, 0, 0.03),
          0 0 1px 0 rgba(0, 0, 0, 0.2);
        color: var(--parsec-color-light-secondary-text);
        border-radius: var(--parsec-radius-8);
      }

      @include ms.responsive-breakpoint('md') {
        width: 100%;
      }
    }

    .action-validation {
      display: flex;
      color: var(--parsec-color-light-secondary-text);
      align-items: center;
      gap: 0.25rem;

      &-icon {
        font-size: 1rem;
        color: var(--parsec-color-light-success-700);
      }
    }
  }

  &-state {
    margin-left: auto;
    top: -0.5rem;
    right: -0.5rem;
    position: absolute;

    [class*='badge-'] {
      flex-shrink: 0;
      padding: 0.25rem 0.5rem;
      border-radius: var(--parsec-radius-12);
      white-space: nowrap;
    }

    .badge-active {
      background: var(--parsec-color-light-primary-50);
      color: var(--parsec-color-light-primary-500);
    }

    .badge-inactive {
      color: var(--parsec-color-light-danger-500);
      background: var(--parsec-color-light-danger-50);
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
  gap: 1rem;

  &__description {
    margin-top: -0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &__advice {
    display: flex;
    width: 100%;
    align-items: center;
    background: var(--parsec-color-light-info-50);
    padding: 0.625rem 0.75rem;
    gap: 0.5rem;
    border-radius: var(--parsec-radius-8);
    color: var(--parsec-color-light-info-500);

    .advice-icon {
      font-size: 1rem;
      color: var(--parsec-color-light-info-500);
      flex-shrink: 0;
    }
  }

  &-button {
    --background: var(--parsec-color-light-secondary-text);
    --background-hover: var(--parsec-color-light-secondary-contrast);
    color: var(--parsec-color-light-secondary-white);
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
