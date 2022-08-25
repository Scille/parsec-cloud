<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <h4 class="ion-margin-start">
        {{ $t('WorkspacesPage.documents') }}
      </h4>
      <ion-item-divider class="workspace-toolbar ion-margin-bottom">
        <ion-button
          v-if="!isPlatform('mobile')"
          fill="clear"
          @click="openCreateWorkspaceModal()"
        >
          <ion-icon
            slot="icon-only"
            :icon="add"
          />
          <ion-label>{{ $t('WorkspacesPage.createWorkspace') }}</ion-label>
        </ion-button>
        <ion-button
          fill="clear"
          class="workspace-toolbar__change-view"
          @click="listView = !listView"
        >
          <ion-icon
            slot="icon-only"
            :icon="listView ? grid : list"
          />
        </ion-button>
      </ion-item-divider>
      <div v-if="listView">
        <MobileItemList
          v-for="workspace in workspacesExampleData"
          :item-type="'workspace'"
          :primary-label="workspace.name"
          :secondary-label="workspace.userRole"
          :third-label="userCountLabel(workspace.userCount)"
          :key="workspace.id"
          @contextmenu.prevent="openWorkspaceActionSheet()"
          @trigger-action-sheet="openWorkspaceActionSheet()"
          @trigger-share="openWorkspaceShareModal()"
        />
      </div>
      <div
        v-else
        class="workspaces-grid-container"
      >
        <ItemGrid
          v-for="workspace in workspacesExampleData"
          :item-type="'workspace'"
          :primary-label="workspace.name"
          :secondary-label="workspace.userRole"
          :key="workspace.id"
          @contextmenu.prevent="handleContextMenu($event)"
          @trigger-context-menu="openWorkspaceContextMenu($event)"
          @trigger-action-sheet="openWorkspaceActionSheet()"
          @trigger-share="openWorkspaceShareModal()"
        />
      </div>
      <ion-fab
        v-if="isPlatform('mobile')"
        vertical="bottom"
        horizontal="end"
        slot="fixed"
      >
        <ion-fab-button @click="openCreateWorkspaceModal()">
          <ion-icon :icon="add" />
        </ion-fab-button>
      </ion-fab>
    </ion-content>
  </ion-page>
</template>

<script setup lang = "ts" >
import {
  IonLabel,
  IonButton,
  IonIcon,
  IonPage,
  IonItemDivider,
  IonContent,
  popoverController,
  actionSheetController,
  isPlatform,
  IonFab,
  IonFabButton,
  modalController
} from '@ionic/vue';

import {
  add, grid, list, close, shareSocial, time
} from 'ionicons/icons';
import ItemGrid from '@/components/ItemGrid.vue';
import MobileItemList from '@/components/MobileItemList.vue';
import WorkspaceContextMenu from '@/components/WorkspaceContextMenu.vue';
import CreateWorkspaceModal from '@/components/CreateWorkspaceModal.vue';
import WorkspaceShareModal from '@/components/WorkspaceShareModal.vue';
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';

const { t } = useI18n();
const listView = ref(false);

async function openCreateWorkspaceModal(): Promise<void> {
  const modal = await modalController.create({
    component: CreateWorkspaceModal,
    cssClass: 'one-line-modal'
  });
  modal.present();

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    console.log(data);
  }
}

const workspacesExampleData = [
  {
    'id': 1234,
    'name': 'Product Design',
    'userRole': 'Owner',
    'userCount': 3
  },
  {
    'id': 2345,
    'name': 'Marketing',
    'userRole': 'Contributor',
    'userCount': 1
  },
  {
    'id': 3456,
    'name': 'Engineering',
    'userRole': 'Contributor',
    'userCount': 4
  },
  {
    'id': 4567,
    'name': 'Research',
    'userRole': 'Reader',
    'userCount': 3
  }
];

function userCountLabel(userCount: number): string {
  return userCount > 1 ? `${userCount} ${t('WorkspacesPage.user')}s` : `${userCount} ${t('WorkspacesPage.user')}`;
}

function handleContextMenu(ev: Event): void {
  if (isPlatform('mobile')) { // @contextmenu event is triggered by a long press on mobile
    openWorkspaceActionSheet();
  } else {
    openWorkspaceContextMenu(ev);
  }
}

async function openWorkspaceContextMenu(ev: Event): Promise<void> {
  const popover = await popoverController
    .create({
      component: WorkspaceContextMenu,
      event: ev,
      translucent: true,
      showBackdrop: false,
      dismissOnSelect: true,
      reference: 'event'
      /* componentProps: {
        dataTest: 'context menu data test'
      } */
    });
  await popover.present();

  const { role } = await popover.onDidDismiss();
  console.log('onDidDismiss resolved with role', role);
  if (role==='share') {
    openWorkspaceShareModal();
  }
}

async function openWorkspaceActionSheet(): Promise<void> {
  const actionSheet = await actionSheetController
    .create({
      header: 'Workspace name',
      buttons: [
        {
          text: t('WorkspacesPage.workspaceContextMenu.share'),
          icon: shareSocial,
          handler: () :void => {
            openWorkspaceShareModal();
          }
        },
        {
          text: t('WorkspacesPage.workspaceContextMenu.history'),
          icon: time,
          handler: ():void => {
            console.log('History clicked');
          }
        },
        {
          text: t('WorkspacesPage.workspaceContextMenu.cancel'),
          icon: close,
          role: 'cancel',
          handler: ():void => {
            console.log('Cancel clicked');
          }
        }
      ]
    });
  await actionSheet.present();
  const { role, data } = await actionSheet.onDidDismiss();
  console.log('onDidDismiss resolved with role and data: ', role, data);
}

async function openWorkspaceShareModal(): Promise<void> {
  const modal = await modalController.create({
    component: WorkspaceShareModal
  });
  modal.present();

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    console.log(data);
  }
}
</script>

<style lang="scss" scoped>
.workspaces-grid-container {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
}

.workspace-toolbar {
    --padding-start: 0px;
}

.workspace-toolbar__change-view {
    margin-left: auto;
}

</style>
