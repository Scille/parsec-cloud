<template>
  <ion-header>
    <ion-toolbar>
      <ion-title>{{ $t('WorkspacesPage.WorkspacesShareModal.pageTitle') }}</ion-title>
      <ion-buttons slot="end">
        <ion-button
          @click="closeModal()"
        >
          <ion-icon
            slot="icon-only"
            :icon="close"
            size="large"
          />
        </ion-button>
      </ion-buttons>
    </ion-toolbar>
    <ion-toolbar v-if="isPlatform('mobile')">
      <ion-buttons slot="primary">
        <ion-button
          fill="solid"
          @click="goToInvitationPage()"
          color="primary"
        >
          {{ $t('WorkspacesPage.WorkspacesShareModal.inviteNewUser') }}
          <ion-icon
            slot="end"
            :icon="arrowForward"
          />
        </ion-button>
      </ion-buttons>
    </ion-toolbar>
    <ion-searchbar
      v-model="searchUsertInput"
      v-else
    />
  </ion-header>
  <ion-content class="ion-padding">
    test: {{ searchUsertInput }}
    <WorkspaceShareModalUserItem
      v-for="user in filteredUsers"
      :name="user.name"
      :email="user.email"
      :role="user.userRole"
      :key="user.id"
    />
  </ion-content>
  <ion-footer>
    <ion-toolbar v-if="!isPlatform('mobile')">
      <ion-buttons slot="primary">
        <ion-button
          fill="solid"
          @click="goToInvitationPage()"
          color="primary"
        >
          {{ $t('WorkspacesPage.WorkspacesShareModal.inviteNewUser') }}
          <ion-icon
            slot="end"
            :icon="arrowForward"
          />
        </ion-button>
      </ion-buttons>
    </ion-toolbar>
    <ion-searchbar
      v-model="searchUsertInput"
      v-else
    />
  </ion-footer>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonTitle,
  IonToolbar,
  IonButtons,
  IonButton,
  modalController,
  IonFooter,
  IonIcon,
  IonSearchbar,
  isPlatform
} from '@ionic/vue';
import {
  close, arrowForward
} from 'ionicons/icons';
import { ref, computed } from 'vue';
import WorkspaceShareModalUserItem from '@/components/WorkspaceShareModalUserItem.vue';

const searchUsertInput=ref('');

const usersExampleData = [
  {
    'id': 345,
    'name': 'Jean Paul',
    'email': 'jean.paul@test.test',
    'userRole': 'Contributor'
  },
  {
    'id': 456,
    'name': 'Alice Dupont',
    'email': 'alicedupont@test.test',
    'userRole': 'Contributor'
  },
  {
    'id': 567,
    'name': 'Alexandre Dubois long name test',
    'email': 'alexandre.dubois@secondtest.test.test.test',
    'userRole': 'Reader'
  },
  {
    'id': 678,
    'name': 'Jean Martin',
    'email': 'jean.martin@thirdtest.test',
    'userRole': 'Owner'
  },
  {
    'id': 789,
    'name': 'Pierre Martin',
    'email': 'pierremartin@test.test',
    'userRole': 'Contributor'
  },
  {
    'id': 901,
    'name': 'Guillaume Dupont',
    'email': 'guillaume.dupont@dupont.test',
    'userRole': 'Contributor'
  },
  {
    'id': 102,
    'name': 'Clara Dubois',
    'email': 'clara.dubois@dubois.test',
    'userRole': 'Reader'
  }
];

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, 'cancel');
}

function goToInvitationPage(): void {
  // TO DO: go to organization page with new user invitation modal opened
}

const filteredUsers = computed(() => {
  let tempUsersExampleData = usersExampleData;
  if (searchUsertInput.value !== '' && searchUsertInput.value) {
    tempUsersExampleData=tempUsersExampleData.filter((item) => {
      return item.name
        .toLowerCase()
        .includes(searchUsertInput.value.toLowerCase());
    });
  }
  return tempUsersExampleData;
});
</script>
