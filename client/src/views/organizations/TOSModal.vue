<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="CreateOrganization.acceptTOS.update.title"
    subtitle="CreateOrganization.acceptTOS.update.subtitle"
    :close-button="{ visible: false }"
    :cancel-button="{
      disabled: false,
      label: 'CreateOrganization.acceptTOS.update.no',
      theme: MsReportTheme.Error,
    }"
    :confirm-button="{
      disabled: !checked,
      label: 'CreateOrganization.acceptTOS.update.yes',
      theme: MsReportTheme.Success,
    }"
  >
    <div class="modal-tos-content">
      <ion-button
        @click="openTOS"
        class="tos-button button-small"
      >
        <ion-icon
          slot="start"
          class="tos-icon"
          :icon="documentText"
        />
        {{ $msTranslate('CreateOrganization.acceptTOS.update.tosButton') }}
      </ion-button>
      <ms-checkbox
        v-model="checked"
        label-placement="end"
        justify="start"
        class="tos-checkbox"
      >
        {{ $msTranslate('CreateOrganization.acceptTOS.update.checkbox') }}
      </ms-checkbox>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { Env } from '@/services/environment';
import { IonButton, IonIcon } from '@ionic/vue';
import { documentText } from 'ionicons/icons';
import { I18n, MsCheckbox, MsModal, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  tosLinks: Map<string, string>;
}>();

const checked = ref(false);
const link = ref('');

function getLinkForLocale(): string {
  const currentLocale = (I18n.getLocale() as string).toLocaleLowerCase().replace('_', '-');

  if (props.tosLinks.size === 0) {
    return '';
  }

  const links = new Map<string, string>();
  // Try to clean the map a bit in case it was not properly set:
  // - set to lower case
  // - convert _ to -
  for (const entry of props.tosLinks.entries()) {
    links.set(entry[0].toLocaleLowerCase().replace('_', '-'), entry[1]);
  }

  // Direct match with the current locale
  if (links.has(currentLocale)) {
    return links.get(currentLocale) as string;
  }

  // Only the language part of the locale
  const lang = currentLocale.split('-')[0];
  for (const entry of links.entries()) {
    if (entry[0].split('-')[0] === lang) {
      return entry[1];
    }
  }

  const link = links.values().next().value ?? '';
  window.electronAPI.log(
    'info',
    `No terms of service link was found matching the locale \
'${currentLocale}' in ${JSON.stringify(props.tosLinks.entries())}, \
using '${link}'`,
  );
  // Still not found, worst cast scenario, return the first value
  return link;
}

onMounted(async () => {
  link.value = getLinkForLocale();
});

async function openTOS(): Promise<void> {
  await Env.Links.openTOS(link.value);
}
</script>

<style scoped lang="scss">
.modal-tos-content {
  margin-top: 1rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;

  .tos-button::part(native) {
    font-size: 1rem;
    font-weight: 500;
    text-transform: none;
    padding: 0.75rem 1.25rem;
    width: auto;
    background: var(--parsec-color-light-secondary-premiere);
    color: var(--parsec-color-light-secondary-text);
    --background-hover: var(--parsec-color-light-secondary-medium);
  }

  .tos-icon {
    font-size: 1.125rem;
    margin-right: 0.5rem;
  }

  .tos-checkbox {
    color: var(--parsec-color-light-secondary-text);
  }
}
</style>
