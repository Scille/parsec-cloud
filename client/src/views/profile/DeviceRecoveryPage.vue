<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="organization-recovery-container">
    <section class="recovery-section recovery-section--file">
      <div
        class="recovery-method"
        :class="hasRecoveryDevice ? 'restore-password__advice--done' : ''"
      >
        <ms-image
          :image="RecoveryFileIcon"
          class="recovery-method-image"
        />

        <div class="recovery-method-content">
          <div class="recovery-method-content-text">
            <ion-text class="recovery-method-content-text__title title-h4">
              {{ $msTranslate('OrganizationRecovery.file.title') }}
            </ion-text>
            <ion-text class="recovery-method-content-text__description body-lg">
              {{ $msTranslate('OrganizationRecovery.file.description') }}
            </ion-text>
          </div>
          <div class="recovery-method-content-actions">
            <ion-button
              class="button-normal button-default action-button"
              :class="{ actionDone: hasRecoveryDevice }"
              @click="goToExportRecoveryDevice()"
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
        <ms-image
          :image="RecoveryDeviceIcon"
          class="recovery-method-image"
        />

        <div class="recovery-method-content">
          <div class="recovery-method-content-text">
            <ion-text class="recovery-method-content-text__title title-h4">
              {{ $msTranslate('OrganizationRecovery.device.title') }}
            </ion-text>
            <ion-text class="recovery-method-content-text__description body-lg">
              {{ $msTranslate('OrganizationRecovery.device.description') }}
            </ion-text>
          </div>
          <div class="recovery-method-content-actions">
            <ion-button
              class="button-normal button-default action-button"
              :class="{ actionDone: hasSecondaryDevice }"
              @click="onAddDeviceClick()"
              fill="clear"
            >
              {{ $msTranslate('OrganizationRecovery.device.action') }}
            </ion-button>
            <ion-text
              v-if="hasSecondaryDevice"
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
                    key: 'OrganizationRecovery.device.activeDevices',
                    data: { count: devices.length },
                    count: devices.length,
                  })
                }}
              </span>
            </ion-text>
          </div>
        </div>
        <div class="recovery-method-state">
          <ion-text
            class="badge-active button-medium"
            v-if="hasSecondaryDevice"
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
      v-if="false"
    >
      <div class="recovery-method">
        <ms-image
          :image="RecoveryTrustIcon"
          class="recovery-method-image"
        />

        <div class="recovery-method-content">
          <div class="recovery-method-content-text">
            <ion-text class="recovery-method-content-text__title title-h4">
              {{ $msTranslate('OrganizationRecovery.trustedUser.title') }}
            </ion-text>
            <ion-text class="recovery-method-content-text__description body-lg">
              {{ $msTranslate('OrganizationRecovery.trustedUser.description') }}
            </ion-text>
          </div>
          <div class="recovery-method-content-actions">
            <ion-button
              class="button-normal button-default action-button"
              fill="clear"
            >
              {{ $msTranslate('OrganizationRecovery.trustedUser.action') }}
            </ion-button>
            <ion-button
              class="button-normal button-default action-button actionDone"
              fill="clear"
            >
              {{ $msTranslate('OrganizationRecovery.trustedUser.actionSecondary') }}
            </ion-button>
          </div>
        </div>
        <div class="recovery-method-state">
          <ion-text class="badge-active button-medium">
            {{ $msTranslate('OrganizationRecovery.state.active') }}
          </ion-text>
          <ion-text class="badge-inactive button-medium">
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
import { getClientInfo, listOwnDevices, OwnDeviceInfo } from '@/parsec';
import { navigateTo, ProfilePages, Routes } from '@/router';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { addDeviceWithGreetModal, refreshOwnDevicesList } from '@/views/devices/utils';
import DeviceRecoveryModal from '@/views/profile/DeviceRecoveryModal.vue';
import { IonButton, IonIcon, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { Answer, askQuestion, MsImage, MsModalResult } from 'megashark-lib';
import { computed, inject, onMounted, Ref, ref } from 'vue';

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
const orgId = ref('');
const hasRecoveryDevice = ref(false);
const devices: Ref<OwnDeviceInfo[]> = ref([]);
const hasSecondaryDevice = computed(() => devices.value.length > 1);

onMounted(async (): Promise<void> => {
  const result = await getClientInfo();
  if (!result.ok) {
    return;
  }
  orgId.value = result.value.organizationId;

  await refreshDevicesList();
});

async function navigateToDevicesPage(): Promise<void> {
  await navigateTo(Routes.MyProfile, { replace: true, query: { profilePage: ProfilePages.Devices } });
}

async function goToExportRecoveryDevice(): Promise<void> {
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
    component: DeviceRecoveryModal,
    cssClass: 'export-recovery-modal',
    componentProps: {
      organizationId: orgId.value,
      informationManager: informationManager.value,
    },
    canDismiss: true,
    backdropDismiss: true,
    showBackdrop: true,
  });

  await modal.present();
  const { role, data } = await modal.onDidDismiss<{ recoveryActionsDone?: boolean }>();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm || !data?.recoveryActionsDone) {
    return;
  }

  await sendRecoveryDeviceCreatedEvent();
}

async function sendRecoveryDeviceCreatedEvent(): Promise<void> {
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

async function refreshDevicesList(): Promise<void> {
  await refreshOwnDevicesList(informationManager.value, devices);
}

async function onAddDeviceClick(): Promise<void> {
  await addDeviceWithGreetModal(informationManager.value, eventDistributor.value, devices);
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
  padding: 1.5rem;
  box-shadow:
    0 1px 1px 0 rgba(0, 0, 0, 0.05),
    0 1px 4px 0 rgba(0, 0, 0, 0.03),
    0 0 1px 0 rgba(0, 0, 0, 0.2);
}

.recovery-method {
  display: flex;
  gap: 1rem;

  &-image {
    width: fit-content;
    height: fit-content;
    flex-shrink: 0;
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;

      &__title {
        color: var(--parsec-color-light-secondary-text);
      }

      &__description {
        color: var(--parsec-color-light-secondary-hard-grey);
        font-size: 0.9375rem;
      }
    }

    &-actions {
      display: flex;
      align-items: center;
      gap: 1rem;

      @include ms.responsive-breakpoint('sm') {
        flex-direction: column;
        align-items: flex-start;
        gap: 0.5rem;
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
  }

  &-state {
    margin-left: auto;

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
