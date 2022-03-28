<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS -->

<template>
  <ion-page>
    <ion-menu content-id="main">
      <ion-header>
        <ion-toolbar translucent>
          <ion-title>Parsec</ion-title>
        </ion-toolbar>
      </ion-header>
      <ion-content>
        <ion-list>
          <ion-item
            button
            @click="presentPatchNote()"
          >
            <ion-icon
              :icon="newspaperSharp"
              slot="start"
            />
            <ion-label>Journal des modifications</ion-label>
          </ion-item>
          <ion-item
            button
            @click="presentAbout()"
          >
            <ion-icon
              :icon="helpCircleOutline"
              slot="start"
            />
            <ion-label>À propos</ion-label>
          </ion-item>
        </ion-list>
      </ion-content>
    </ion-menu>
    <ion-header :translucent="true">
      <ion-toolbar color="primary">
        <ion-buttons slot="start">
          <ion-menu-button auto-hide="false" />
        </ion-buttons>
        <ion-buttons slot="primary">
          <ion-button @click="presentOrganizationActionSheet">
            <ion-icon
              slot="icon-only"
              :ios="ellipsisHorizontal"
              :icon="ellipsisVertical"
              :md="ellipsisVertical"
            />
          </ion-button>
        </ion-buttons>
        <ion-title>Parsec</ion-title>
      </ion-toolbar>
    </ion-header>

    <ion-content
      :fullscreen="true"
      id="main"
    >
      <ion-header collapse="condense">
        <ion-toolbar color="primary">
          <ion-buttons slot="start">
            <ion-menu-button auto-hide="false" />
          </ion-buttons>
          <ion-buttons slot="primary">
            <ion-button @click="presentOrganizationActionSheet">
              <ion-icon
                slot="icon-only"
                :ios="ellipsisHorizontal"
                :icon="ellipsisVertical"
                :md="ellipsisVertical"
              />
            </ion-button>
          </ion-buttons>
          <ion-title size="large">
            Parsec
          </ion-title>
        </ion-toolbar>
      </ion-header>
      <div id="container">
        <p>Vous n'avez pas d'appareils sur ce téléphone.</p>
        <p>Pour ajouter un appareil, invitez-le depuis une organisation existante, ou créez une nouvelle organisation.</p>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonPage,
  IonTitle,
  IonToolbar,
  IonButton,
  IonIcon,
  IonMenuButton,
  IonItem,
  IonList,
  IonMenu,
  IonLabel,
  IonButtons,
  actionSheetController
} from '@ionic/vue';
import {
  ellipsisVertical,
  ellipsisHorizontal,
  add,
  link,
  qrCodeSharp,
  helpCircleOutline,
  newspaperSharp
} from 'ionicons/icons'; // We're forced to import icons for the moment, see : https://github.com/ionic-team/ionicons/issues/1032

function presentAbout(): void {
  console.log('presentAbout');
}

function presentPatchNote(): void {
  console.log('presentPatchNote');
}

async function presentOrganizationActionSheet(): Promise<void> {
  const actionSheet = await actionSheetController
    .create({
      header: 'Organisation',
      cssClass: 'organization-action-sheet',
      buttons: [
        {
          text: 'Créer',
          icon: add,
          data: {
            type: 'delete'
          },
          handler: (): void => {
            console.log('Create clicked');
          }
        },
        {
          text: 'Rejoindre par lien',
          icon: link,
          data: 10,
          handler: (): void => {
            console.log('Join by link clicked');
          }
        },
        {
          text: 'Rejoindre par QR code',
          icon: qrCodeSharp,
          data: 'Data value',
          handler: (): void => {
            console.log('Join by QR code clicked');
          }
        }
      ]
    });
  await actionSheet.present();

  const { role, data } = await actionSheet.onDidDismiss();
  console.log('onDidDismiss resolved with role and data', role, data);
}
</script>

<style lang="scss" scoped>
#container {
    text-align: center;
    position: absolute;
    left: 0;
    right: 0;
    top: 50%;
    transform: translateY(-50%);

    p {
        font-weight: bold;
    }
}
</style>
