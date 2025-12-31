<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="personal-data-page"
    v-if="personalData"
  >
    <div class="personal-data-content">
      <ms-summary-card
        title="clientArea.personalDataPage.personalInfo.title"
        :rows="getPersonalInfoRows()"
        @update="openPersonalInfoModal"
        class="personal-data-content__item"
      />
      <ms-summary-card
        title="clientArea.personalDataPage.professionalInfo.title"
        :rows="getProfessionalInfoRows()"
        @update="openProfessionalInfoModal"
        class="personal-data-content__item"
      />
    </div>
    <div class="personal-data-content">
      <ms-summary-card
        title="clientArea.personalDataPage.authentication.title"
        :rows="getAuthenticationRows()"
        @update="openAuthenticationModal"
        :read-only="personalData.billingSystem !== BillingSystem.Stripe"
        class="personal-data-content__item"
      />
      <ms-summary-card
        title="clientArea.personalDataPage.security.title"
        :rows="getSecurityRows()"
        @update="openSecurityModal"
        class="personal-data-content__item"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { BillingSystem, BmsAccessInstance, BmsOrganization, PersonalInformationResultData } from '@/services/bms';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import AuthenticationModal from '@/views/client-area/personal-data/AuthenticationModal.vue';
import PersonalInfoModal from '@/views/client-area/personal-data/PersonalInfoModal.vue';
import ProfessionalInfoModal from '@/views/client-area/personal-data/ProfessionalInfoModal.vue';
import SecurityModal from '@/views/client-area/personal-data/SecurityModal.vue';
import { modalController, ModalOptions } from '@ionic/vue';
import {
  createSummaryCardItem,
  I18n,
  MsModalResult,
  MsSummaryCard,
  MsSummaryCardItemData,
  MsSummaryCardRowData,
  Translatable,
} from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const personalData = ref<PersonalInformationResultData | null>(null);
const isRepresentingCompany = computed(() => !!personalData.value?.company && !!personalData.value?.job);

const props = defineProps<{
  organization: BmsOrganization;
  informationManager: InformationManager;
}>();

function createItem(
  label: Translatable,
  text: any,
  error?: Translatable,
  secondLineText?: any,
  secondLineError?: Translatable,
): MsSummaryCardItemData | undefined {
  return createSummaryCardItem(
    `clientArea.personalDataPage.${label}`,
    I18n.valueAsTranslatable(text),
    error,
    secondLineText ? I18n.valueAsTranslatable(secondLineText) : undefined,
    secondLineError,
  );
}

function createItemWithTranslatable(
  label: Translatable,
  text: Translatable,
  error?: Translatable,
  secondLineText?: Translatable,
  secondLineError?: Translatable,
): MsSummaryCardItemData | undefined {
  return createSummaryCardItem(
    `clientArea.personalDataPage.${label}`,
    `clientArea.personalDataPage.${text}`,
    error,
    secondLineText ? `clientArea.personalDataPage.${secondLineText}` : undefined,
    secondLineError,
  );
}

onMounted(async () => {
  await getPersonalData();
});

async function getPersonalData(): Promise<void> {
  personalData.value = BmsAccessInstance.get().getPersonalInformation();
}

function getPersonalInfoRows(): MsSummaryCardRowData[] {
  return [
    {
      item: createItem('personalInfo.firstname', personalData.value?.firstName) as MsSummaryCardItemData,
      secondItem: createItem('personalInfo.lastname', personalData.value?.lastName),
    },
    {
      item:
        personalData.value?.phone === undefined || personalData.value?.phone === null
          ? (createItemWithTranslatable('personalInfo.phone', 'personalInfo.notDefined') as MsSummaryCardItemData)
          : (createItem('personalInfo.phone', personalData.value?.phone) as MsSummaryCardItemData),
    },
  ];
}

function getProfessionalInfoRows(): MsSummaryCardRowData[] {
  const rows = [
    {
      item: createItemWithTranslatable(
        'professionalInfo.representCompany.title',
        isRepresentingCompany.value ? 'professionalInfo.representCompany.yes' : 'professionalInfo.representCompany.no',
      ) as MsSummaryCardItemData,
    },
  ];
  if (isRepresentingCompany.value) {
    rows.push({
      item: createItem('professionalInfo.company', personalData.value?.company) as MsSummaryCardItemData,
      secondItem: createItem('professionalInfo.job', personalData.value?.job),
    } as MsSummaryCardRowData);
  }
  return rows;
}

function getAuthenticationRows(): MsSummaryCardRowData[] {
  return [
    {
      item: createItem('authentication.email', personalData.value?.email) as MsSummaryCardItemData,
    },
  ];
}

function getSecurityRows(): MsSummaryCardRowData[] {
  return [
    {
      item: createItem('security.password', '*********') as MsSummaryCardItemData,
    },
  ];
}

async function openAuthenticationModal(): Promise<void> {
  const role = await openModal({
    component: AuthenticationModal,
    canDismiss: true,
    cssClass: 'authentication-modal',
    backdropDismiss: false,
    componentProps: {
      email: personalData.value?.email,
    },
  });

  if (role === MsModalResult.Confirm) {
    onDataUpdated('clientArea.personalDataPage.modals.authentication.success');
  }
}

async function openSecurityModal(): Promise<void> {
  const role = await openModal({
    component: SecurityModal,
    canDismiss: true,
    cssClass: 'security-modal',
    backdropDismiss: false,
  });

  if (role === MsModalResult.Confirm) {
    onDataUpdated('clientArea.personalDataPage.modals.security.success');
  }
}

async function openPersonalInfoModal(): Promise<void> {
  const role = await openModal({
    component: PersonalInfoModal,
    canDismiss: true,
    cssClass: 'personal-info-modal',
    backdropDismiss: false,
    componentProps: {
      firstname: personalData.value?.firstName,
      lastname: personalData.value?.lastName,
      phone: personalData.value?.phone,
    },
  });

  if (role === MsModalResult.Confirm) {
    onDataUpdated('clientArea.personalDataPage.modals.personalInfo.success');
  }
}

async function openProfessionalInfoModal(): Promise<void> {
  const role = await openModal({
    component: ProfessionalInfoModal,
    canDismiss: true,
    cssClass: 'professional-info-modal',
    backdropDismiss: false,
    componentProps: {
      company: personalData.value?.company,
      job: personalData.value?.job,
    },
  });

  if (role === MsModalResult.Confirm) {
    onDataUpdated('clientArea.personalDataPage.modals.professionalInfo.success');
  }
}

async function openModal(options: ModalOptions): Promise<MsModalResult | undefined> {
  if (!personalData.value) {
    return;
  }
  const modal = await modalController.create(options);
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();

  return role as MsModalResult;
}

async function onDataUpdated(message: Translatable): Promise<void> {
  await getPersonalData();
  props.informationManager.present(
    new Information({
      message,
      level: InformationLevel.Success,
    }),
    PresentationMode.Toast,
  );
}
</script>

<style scoped lang="scss">
.personal-data-page {
  display: flex;
  gap: 1.5rem;
  flex-wrap: wrap;
}

.personal-data-content {
  display: flex;
  gap: 1.5rem;
  flex: 1;
  flex-direction: column;
  max-width: 30em;

  &__item {
    padding: 1.5rem;
    background: var(--parsec-color-light-secondary-background);
    border-radius: var(--parsec-radius-12);
    box-shadow: var(--parsec-shadow-input);
  }

  @include ms.responsive-breakpoint('sm') {
    max-width: 100%;
  }
}
</style>
