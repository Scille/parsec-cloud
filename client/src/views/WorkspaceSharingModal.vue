<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <!-- header -->
    <ion-header class="modal-header">
      <ion-toolbar class="modal-header__toolbar">
        <ion-title class="title-h2">
          {{ $t('WorkspaceSharing.title') }}
        </ion-title>
      </ion-toolbar>
      <ion-buttons
        slot="end"
        class="closeBtn-container"
      >
        <ion-button
          slot="icon-only"
          @click="closeModal()"
          class="closeBtn"
        >
          <ion-icon
            :icon="close"
            size="large"
            class="closeBtn__icon"
          />
        </ion-button>
      </ion-buttons>
    </ion-header>
    <!-- content -->
    <ion-content>
      <custom-input
        v-model="search"
        :label="$t('WorkspaceSharing.searchLabel')"
        :placeholder="$t('WorkspaceSharing.searchPlaceholder')"
      />
      <ion-list class="user-list">
        <user-workspace-role
          v-for="entry in userRoles.entries()"
          :key="entry[0]"
          :user="entry[0]"
          :role="entry[1]"
          @role-update="updateUserRole"
        />
      </ion-list>
    </ion-content>
    <ion-footer class="modal-footer">
      <ion-buttons
        class="modal-footer-buttons"
      >
        <ion-button
          fill="clear"
          size="default"
          id="cancel-button"
          @click="closeModal()"
        >
          {{ $t('WorkspaceSharing.cancel') }}
        </ion-button>
      </ion-buttons>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonButton,
  IonPage,
  IonHeader,
  IonContent,
  IonButtons,
  IonTitle,
  IonToolbar,
  IonIcon,
  IonList,
  IonFooter,
  modalController,
} from '@ionic/vue';
import {
  close,
} from 'ionicons/icons';
import { ref, watch, onUnmounted } from 'vue';
import { getWorkspaceUsers, WorkspaceRole } from '@/common/mocks';
import UserWorkspaceRole from '@/components/UserWorkspaceRole.vue';
import CustomInput from '@/components/CustomInput.vue';

const search = ref('');

const props = defineProps<{
  workspaceId: string
}>();

const userRoles = ref(new Map<string, WorkspaceRole | null>());

getWorkspaceUsers(props.workspaceId).then((result) => {
  userRoles.value = result;
});

// Would prefere to use a computed instead of a watch but
// Vue doesn't handle async in computed.
const unwatchSearch = watch(search, async() => {
  const allRoles = await getWorkspaceUsers(props.workspaceId);
  const roles = new Map<string, WorkspaceRole | null>();
  const lowerCaseSearch = search.value.toLocaleLowerCase();

  for (const entry of allRoles) {
    if (entry[0].toLocaleLowerCase().includes(lowerCaseSearch)) {
      roles.set(entry[0], entry[1]);
    }
  }
  userRoles.value = roles;
});

onUnmounted(() => {
  unwatchSearch();
});

async function updateUserRole(user: string, role: WorkspaceRole | null): Promise<void> {
  console.log(`Update user ${user} role to ${role}`);
}

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, 'cancel');
}
</script>

<style scoped lang="scss">
.modal {
  padding: 2.5rem;
}

.modal-header {
  position: relative;

  &__toolbar {
    --min-height: 0;
  }
}

.title-h2 {
  color: var(--parsec-color-light-primary-600);
  padding-inline: 0;
  margin-bottom: 2rem;
}

.closeBtn-container, .closeBtn {
  margin: 0;
  --padding-start: 0;
  --padding-end: 0;
}

.closeBtn-container {
  position: absolute;
  top: 0;
  right: 0;
}

.closeBtn {
  border-radius: var(--parsec-radius-4);
  width: fit-content;
  height: fit-content;

  &:hover {
    --background-hover: var(--parsec-color-light-primary-50);
    --border-radius: var(--parsec-radius-4);
  }

  &:active {
    background: var(--parsec-color-light-primary-100);
    --border-radius: var(--parsec-radius-4);
  }

  &__icon {
    padding: 4px;
    color: var(--parsec-color-light-primary-500);
  }
}

.user-list {
  padding: .5rem;
}

.modal-footer {
  margin-top: 2.5rem;

  .modal-footer-buttons {
    display: flex;
    justify-content: flex-end;
    gap: 1.5rem;
  }
}
</style>
