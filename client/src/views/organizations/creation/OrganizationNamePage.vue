<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="organization-name-page">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.create"
      subtitle="CreateOrganization.subtitle.nameYourOrg"
    />

    <div>
      <ms-input
        :label="'CreateOrganization.organizationName'"
        :placeholder="'CreateOrganization.organizationNamePlaceholder'"
        name="organization"
        id="org-name-input"
        v-model="organizationName"
        :disabled="disableOrganizationNameField"
        ref="organizationNameInputRef"
        @on-enter-keyup="$emit('organizationNameChosen', organizationName)"
        :validator="organizationValidator"
      />

      <ion-text class="subtitles-sm org-name-criteria">
        {{ $msTranslate('CreateOrganization.organizationNameCriteria') }}
      </ion-text>
      <p v-show="error">
        {{ $msTranslate(error) }}
      </p>

      <ion-footer class="organization-name-page-footer">
        <ion-buttons
          slot="primary"
          class="modal-footer-buttons"
        >
          <ion-button
            fill="solid"
            size="default"
            @click="$emit('organizationNameChosen', organizationName)"
            :disabled="!organizationNameInputRef || organizationNameInputRef.validity !== Validity.Valid"
          >
            <span>
              {{ $msTranslate('CreateOrganization.button.next') }}
            </span>
            <ion-icon
              slot="start"
              :icon="chevronForward"
              size="small"
            />
          </ion-button>
        </ion-buttons>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { OrganizationID } from '@/parsec';
import { IonPage, IonButton, IonText, IonButtons, IonFooter, IonIcon } from '@ionic/vue';
import { onMounted, ref } from 'vue';
import { chevronForward } from 'ionicons/icons';
import { organizationValidator } from '@/common/validators';
import { Translatable, Validity, MsInput } from 'megashark-lib';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';

const props = defineProps<{
  organizationName?: OrganizationID;
  error?: Translatable;
  disableOrganizationNameField?: boolean;
}>();

defineEmits<{
  (e: 'organizationNameChosen', name: OrganizationID): void;
  (e: 'closeRequested'): void;
}>();

const organizationNameInputRef = ref();
const organizationName = ref<OrganizationID>(props.organizationName ?? '');

onMounted(async () => {
  if (organizationNameInputRef.value && organizationName.value) {
    await organizationNameInputRef.value.validate(organizationName.value);
  }
});
</script>

<style scoped lang="scss">
.organization-name-page {
  padding: 2.5rem;
  display: flex;
  height: auto;
  width: 100%;

  &-footer {
    display: flex;
    justify-content: space-between;
    margin-top: 2.5rem;
  }
}
</style>
