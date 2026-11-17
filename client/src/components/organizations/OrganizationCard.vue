<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="organization-card">
    <div class="organization-card__avatar">
      <ms-image
        v-if="isTrialOrg"
        :image="ResourcesManager.instance().get(Resources.LogoIcon, LogoIconWhite) as string"
        class="organization-card__logo"
      />
      <ion-text
        v-else
        class="organization-card__initials"
      >
        {{ device.organizationId.substring(0, 2) }}
      </ion-text>
    </div>
    <ion-text class="organization-card__name">{{ device.organizationId }}</ion-text>
  </div>
</template>

<script setup lang="ts">
import { isTrialOrganizationDevice } from '@/common/organization';
import { AvailableDevice } from '@/parsec';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonText } from '@ionic/vue';
import { LogoIconWhite, MsImage } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  device: AvailableDevice;
}>();

const isTrialOrg = ref(false);

onMounted(async () => {
  isTrialOrg.value = await isTrialOrganizationDevice(props.device);
});
</script>

<style lang="scss" scoped>
.organization-card {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-2xl');
  min-width: 0;
  user-select: none;

  &__avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 2.5rem;
    height: 2.5rem;
    border-radius: ms.radius('2xl');
    background: linear-gradient(113deg, ms.color('surface-gradient-from') 1.49%, ms.color('surface-gradient-to') 100%);
  }

  &__initials {
    @include ms.font('label-lg-medium');
    color: ms.color('text-on-color-label');
  }

  &__logo {
    width: 1.5rem;
    flex-shrink: 0;
  }

  &__name {
    @include ms.font('heading-h3');
    color: ms.color('text-brand-default-hover');
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
}
</style>
