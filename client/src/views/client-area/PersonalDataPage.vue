<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="personal-data-container"
    v-if="personalData"
  >
    <div class="personal-data-content">
      <ms-summary-card
        title="clientArea.personalDataPage.personalInfo.title"
        :rows="[
          {
            item: createItem('personalInfo.firstname', personalData.firstName) as MsSummaryCardItemData,
            secondItem: createItem('personalInfo.lastname', personalData.lastName),
          },
          {
            item: createItem('personalInfo.phone', personalData.phone) as MsSummaryCardItemData,
          },
        ]"
        @update="openPersonalInfoModal"
      />
      <ms-summary-card
        title="clientArea.personalDataPage.professionalInfo.title"
        :rows="[
          {
            item: createItem('professionalInfo.company', personalData.company) as MsSummaryCardItemData,
            secondItem: createItem('professionalInfo.job', personalData.job),
          },
        ]"
        @update="openProfessionalInfoModal"
      />
    </div>
    <div class="personal-data-content">
      <ms-summary-card
        title="clientArea.personalDataPage.authentication.title"
        :rows="[
          {
            item: createItem('authentication.email', personalData.email) as MsSummaryCardItemData,
          },
          {
            item: createItem('authentication.password', '*********') as MsSummaryCardItemData,
          },
        ]"
        @update="openAuthenticationModal"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { BmsAccessInstance, BmsOrganization, PersonalInformationResultData } from '@/services/bms';
import { createSummaryCardItem, I18n, MsSummaryCard, MsSummaryCardItemData, Translatable } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const personalData = ref<PersonalInformationResultData | null>(null);

defineProps<{
  organization: BmsOrganization;
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

onMounted(async () => {
  personalData.value = await BmsAccessInstance.get().getPersonalInformation();
});

function openAuthenticationModal(): void {}

function openPersonalInfoModal(): void {}

function openProfessionalInfoModal(): void {}
</script>

<style scoped lang="scss">
.personal-data-container {
  display: flex;
  gap: 1.5rem;
  padding: 1.5rem;
}

.personal-data-content {
  background: var(--parsec-color-light-primary-background);
  display: flex;
  gap: 1.5rem;
  flex: 1;
  flex-direction: column;
  max-width: 30em;
}
</style>
