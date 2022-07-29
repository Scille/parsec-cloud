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
            <ion-label>{{ $t('HomePage.mainMenu.changelog') }}</ion-label>
          </ion-item>
          <ion-item
            button
            @click="router.push({ name: 'settings' })"
          >
            <ion-icon
              :icon="settingsOutline"
              slot="start"
            />
            <ion-label>{{ $t('HomePage.mainMenu.settings') }}</ion-label>
          </ion-item>
          <ion-item
            button
            @click="router.push({ name: 'about' })"
          >
            <ion-icon
              :icon="helpCircleOutline"
              slot="start"
            />
            <ion-label>{{ $t('HomePage.mainMenu.about') }}</ion-label>
          </ion-item>
        </ion-list>
        <ion-accordion-group ref="langAccordionGroup">
          <ion-accordion value="langs">
            <ion-item slot="header">
              <ion-icon
                :icon="language"
                slot="start"
              />
              <ion-label>{{ $t(`HomePage.mainMenu.lang.${$i18n.locale.replace('-', '')}`) }}</ion-label>
            </ion-item>
            <ion-list
              slot="content"
              class="lang-list"
            >
              <ion-item
                button
                @click="changeLang('fr-FR')"
              >
                <ion-label>{{ $t('HomePage.mainMenu.lang.frFR') }}</ion-label>
              </ion-item>
              <ion-item
                button
                @click="changeLang('en-US')"
              >
                <ion-label>{{ $t('HomePage.mainMenu.lang.enUS') }}</ion-label>
              </ion-item>
            </ion-list>
          </ion-accordion>
        </ion-accordion-group>
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
        <p>{{ $t('HomePage.pleaseConnectToAnOrganisation') }}</p>
        <ion-button
          @click="openCreateOrganizationModal()"
          expand="full"
          size="large"
        >
          <ion-icon
            slot="start"
            :icon="add"
          />
          {{ $t('HomePage.createOrganization') }}
        </ion-button>
        <ion-button
          @click="openJoinByLinkModal()"
          expand="full"
          size="large"
        >
          <ion-icon
            slot="start"
            :icon="link"
          />
          {{ $t('HomePage.joinByLink') }}
        </ion-button>
        <ion-button
          v-if="isPlatform('hybrid')"
          expand="full"
          size="large"
        >
          <ion-icon
            slot="start"
            :icon="qrCodeSharp"
          />
          {{ $t('HomePage.joinByQRcode') }}
        </ion-button>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonAccordionGroup,
  IonAccordion,
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
  actionSheetController,
  isPlatform,
  modalController
} from '@ionic/vue';
import {
  ellipsisVertical,
  ellipsisHorizontal,
  add,
  link,
  qrCodeSharp,
  helpCircleOutline,
  newspaperSharp,
  language,
  settingsOutline
} from 'ionicons/icons'; // We're forced to import icons for the moment, see : https://github.com/ionic-team/ionicons/issues/1032
import { useI18n } from 'vue-i18n';
import { ref } from 'vue';
import { Storage } from '@ionic/storage';
import { useRouter } from 'vue-router';
import JoinByLinkModal from '@/components/JoinByLinkModal.vue';
import CreateOrganisation from '@/components/CreateOrganisationModal.vue';

const { t, locale } = useI18n();
const langAccordionGroup = ref();
const router = useRouter();

function closeLangAccordion(): void {
  if (langAccordionGroup.value) {
    langAccordionGroup.value.$el.value = undefined;
  }
}

async function changeLang(selectedLang: string): Promise<void> {
  locale.value = selectedLang;
  const store = new Storage();
  await store.create();
  await store.set('userLocale', selectedLang);
  closeLangAccordion();
}

function presentPatchNote(): void {
  console.log('presentPatchNote');
}

async function presentOrganizationActionSheet(): Promise<void> {
  const actionSheet = await actionSheetController
    .create({
      header: t('HomePage.organizationActionSheet.header'),
      cssClass: 'organization-action-sheet',
      buttons: [
        {
          text: t('HomePage.organizationActionSheet.create'),
          icon: add,
          data: {
            type: 'delete'
          },
          handler: (): void => {
            console.log('Create clicked');
          }
        },
        {
          text: t('HomePage.organizationActionSheet.joinByLink'),
          icon: link,
          data: 10,
          handler: (): void => {
            console.log('Join by link clicked');
          }
        },
        {
          text: t('HomePage.organizationActionSheet.joinByQRcode'),
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

async function openJoinByLinkModal(): Promise<void> {
  const modal = await modalController.create({
    component: JoinByLinkModal,
    cssClass: 'join-by-link-modal'
  });
  modal.present();

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    console.log(data);
  }
}

async function openCreateOrganizationModal(): Promise<void> {
  const modal = await modalController.create({
    component: CreateOrganisation
  });
  modal.present();

  const { data, role } = await modal.onWillDismiss();

  if (role === 'confirm') {
    console.log(data);
  }

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

	max-width: 680px;
	margin: 0 auto;

	p {
		font-weight: bold;
	}
}

.lang-list {
	.item {
		ion-label {
			margin-left: 3.5em;
		}
	}
}
</style>
