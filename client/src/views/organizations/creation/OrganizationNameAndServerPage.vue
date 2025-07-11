<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="organization-name-and-server-page page-modal-container">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.nameAndServer"
      :small-display-stepper="true"
    />

    <div class="organization-name-and-server-page-content">
      <div class="organization-name">
        <ms-input
          :label="'CreateOrganization.organizationName'"
          :placeholder="'CreateOrganization.organizationNamePlaceholder'"
          id="org-name-input"
          v-model="organizationName"
          :disabled="disableOrganizationNameField"
          ref="organizationNameInputRef"
          :validator="organizationValidator"
        />
        <ion-text class="subtitles-sm org-name-criteria">
          {{ $msTranslate('CreateOrganization.organizationNameCriteria') }}
        </ion-text>
      </div>

      <ms-input
        :label="'CreateOrganization.serverAddress'"
        :placeholder="'CreateOrganization.serverAddressPlaceholder'"
        id="server-addr-input"
        v-model="serverAddr"
        :disabled="disableServerAddrField"
        ref="serverAddrInputRef"
        :validator="parsecAddrValidator"
      />
      <!-- error -->
      <ion-text
        class="form-error body login-button-error"
        v-show="error"
      >
        <ion-icon
          class="form-error-icon"
          :icon="warning"
        />{{ $msTranslate(error) }}
      </ion-text>
    </div>
    <ion-footer class="organization-name-and-server-page-footer">
      <div class="modal-footer-buttons">
        <ion-button
          fill="clear"
          size="default"
          id="previous-button"
          @click="$emit('goBackRequested')"
          v-show="!hidePrevious"
        >
          {{ $msTranslate('CreateOrganization.button.previous') }}
          <ion-icon
            slot="start"
            :icon="chevronBack"
            size="small"
          />
        </ion-button>
        <ion-button
          fill="solid"
          size="default"
          @click="$emit('organizationNameAndServerChosen', organizationName, serverAddr)"
          :disabled="!valid"
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
      </div>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import { OrganizationID } from '@/parsec';
import { IonPage, IonButton, IonText, IonFooter, IonIcon } from '@ionic/vue';
import { computed, onMounted, ref } from 'vue';
import { chevronForward, chevronBack } from 'ionicons/icons';
import { organizationValidator, parsecAddrValidator } from '@/common/validators';
import { Translatable, Validity, MsInput } from 'megashark-lib';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { warning } from 'ionicons/icons';

const props = defineProps<{
  organizationName?: OrganizationID;
  serverAddr?: string;
  disableServerAddrField?: boolean;
  disableOrganizationNameField?: boolean;
  error?: Translatable;
  hidePrevious?: boolean;
}>();

defineEmits<{
  (e: 'organizationNameAndServerChosen', name: OrganizationID, serverAddr: string): void;
  (e: 'closeRequested'): void;
  (e: 'goBackRequested'): void;
}>();

const organizationNameInputRef = ref();
const serverAddrInputRef = ref();
const organizationName = ref<OrganizationID>(props.organizationName ?? '');
const serverAddr = ref<string>(props.serverAddr ?? '');
const valid = computed(() => {
  return (
    organizationNameInputRef.value &&
    organizationNameInputRef.value.validity === Validity.Valid &&
    serverAddrInputRef.value &&
    serverAddrInputRef.value.validity === Validity.Valid
  );
});

onMounted(async () => {
  if (organizationNameInputRef.value && organizationName.value) {
    await organizationNameInputRef.value.validate(organizationName.value);
  }
  if (serverAddrInputRef.value && serverAddr.value) {
    await serverAddrInputRef.value.validate(serverAddr.value);
  }
});
</script>

<style scoped lang="scss">
.organization-name-and-server-page-content {
  display: flex;
  flex-direction: column;
  height: 100%;

  @include ms.responsive-breakpoint('sm') {
    padding: 0.5rem 2rem;
  }

  @include ms.responsive-breakpoint('xs') {
    padding: 0.5rem 1.5rem;
  }
}

.organization-name {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: 100%;
  margin-bottom: 1.5rem;
}

.org-name-criteria {
  color: var(--parsec-color-light-secondary-hard-grey);
}
</style>
