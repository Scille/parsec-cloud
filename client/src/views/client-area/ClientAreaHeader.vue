<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="header-content">
    <ion-title class="header-title title-h1">{{ $msTranslate(title) }}</ion-title>

    <div class="header-right">
      <!-- settings -->
      <ion-text
        class="button-medium custom-button custom-button-fill"
        button
        @click="openSettingsModal"
      >
        <ion-icon :icon="cog" />
        {{ $msTranslate('clientArea.header.settings') }}
      </ion-text>

      <!-- profile -->
      <div
        class="header-right-profile"
        @click="goToPageClicked(ClientAreaPages.PersonalData)"
      >
        <user-avatar-name
          :user-avatar="getUserName()"
          :user-name="getUserName()"
          :clickable="true"
          class="avatar medium"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { BmsAccessInstance, PersonalInformationResultData } from '@/services/bms';
import { ClientAreaPages } from '@/views/client-area/types';
import { openSettingsModal } from '@/views/settings';
import { IonIcon, IonText, IonTitle } from '@ionic/vue';
import { cog } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';
import { onMounted, ref } from 'vue';

defineProps<{
  title: Translatable;
}>();

const personalInformation = ref<PersonalInformationResultData | null>(null);

onMounted(async () => {
  if (BmsAccessInstance.get().isLoggedIn()) {
    personalInformation.value = await BmsAccessInstance.get().getPersonalInformation();
  }
});

const emits = defineEmits<{
  (e: 'pageSelected', page: ClientAreaPages): void;
}>();

async function goToPageClicked(page: ClientAreaPages): Promise<void> {
  emits('pageSelected', page);
}

function getUserName(): string {
  const info = personalInformation.value;
  if (info === null || !info.firstName || !info.lastName) {
    return '';
  }
  return `${info.firstName} ${info.lastName}`;
}
</script>

<style scoped lang="scss">
.header-content {
  background: var(--parsec-color-light-secondary-white);
  padding: 1.5rem 2rem;
  display: flex;
  justify-content: space-around;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
}

.header-title {
  background: var(--parsec-color-light-gradient-background);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 1rem;
}
</style>
