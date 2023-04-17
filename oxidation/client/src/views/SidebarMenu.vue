<template>
  <ion-page>
    <ion-split-pane
      content-id="main"
    >
      <ion-menu
        content-id="main"
        class="sidebar-menu"
      >
        <ion-header>
          <!-- active oragnization -->
          <ion-card class="menu-orga-card">
            <ion-card-header>
              <div class="text-content">
                <ion-avatar class="orga-avatar">
                  sm
                  <!-- <span>{{ device.organizationId?.substring(0, 2) }}</span> -->
                </ion-avatar>
                <div class="orga-title">
                  <ion-card-subtitle class="caption-info">
                    {{ $t('HomePage.organizationActionSheet.header') }}
                  </ion-card-subtitle>
                  <ion-card-title class="title-h4">
                    My company
                    <!-- {{ device.organizationId }} -->
                  </ion-card-title>
                </div>
              </div>
              <!-- new icon to provide -->
              <svg
                width="32"
                height="32"
                viewBox="0 0 32 32"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path d="M10.3495 20.4231L15.6049 26.795C15.6896 26.8977 15.7947 26.9801 15.9129 27.0366C16.0311 27.0931 16.1597 27.1223 16.2899 27.1223C16.42 27.1223 16.5486 27.0931 16.6669 27.0366C16.7851 26.9801 16.8902 26.8977 16.9749 26.795L22.2303 20.4231C22.7319 19.8149 22.316 18.8755 21.5453 18.8755L11.033 18.8755C10.2622 18.8755 9.84641 19.8149 10.3495 20.4231Z" fill="#F9F9FB"/>
                <path d="M22.2326 13.4558L16.9772 7.08389C16.8925 6.98124 16.7874 6.89884 16.6691 6.84234C16.5509 6.78585 16.4223 6.7566 16.2921 6.7566C16.162 6.7566 16.0334 6.78585 15.9151 6.84234C15.7969 6.89884 15.6918 6.98124 15.6071 7.08389L10.3517 13.4558C9.85015 14.064 10.266 15.0034 11.0367 15.0034L21.549 15.0034C22.3198 15.0034 22.7356 14.064 22.2326 13.4558Z" fill="#F9F9FB"/>
              </svg>
            </ion-card-header>

            <div class="orga-manage-btn">
              <ion-icon
                :icon="cog"
                slot="start"
                size="small"
              />
              <ion-text
                class="subtitles-sm"
                button
                @click="navigateToPage('settings')"
              >
                Gérer mon organisation
              </ion-text>
            </div>
          </ion-card>
          <!-- end of active organzation -->
        </ion-header>

        <ion-content class="ion-padding ">
          <!-- list of workspaces -->
          <ion-list class="menu-list-workspaces">
            <ion-header
              lines="none"
              button
              @click="navigateToPage('workspacesPages')"
            >
              All {{ $t('OrganizationPage.workspaces') }}
            </ion-header>

            <ion-item
              lines="none"
              button
              @click="navigateToPage('workspaces')"
              v-for="workspace in workspacesExampleData"
              :key="workspace.id"
            >
              <ion-icon
                :icon="business"
                slot="start"
              />
              <ion-label>{{ workspace.name }}</ion-label>
            </ion-item>
          </ion-list>
          <!-- list of workspaces -->
        </ion-content>
      </ion-menu>

      <!-- doit-on séparer les deux views ? -->
      <!-- <div class="ion-page" id="main">
      <ion-header>
        <ion-toolbar>
          <ion-title>Main View</ion-title>
        </ion-toolbar>
      </ion-header>
      <ion-content class="ion-padding">
        <div>
          <ion-button size="small">Default</ion-button>
          <ion-button size="small" fill="outline">Outline</ion-button>
          <ion-button size="small" fill="clear">Ghost</ion-button>
        </div>

        <div>
          <ion-button size="default">Default</ion-button>
          <ion-button size="default" fill="outline">Outline</ion-button>
          <ion-button size="default" fill="clear">Ghost</ion-button>
        </div>

        <div>
          <ion-button size="large">Default</ion-button>
          <ion-button size="large" fill="outline">Outline</ion-button>
          <ion-button size="large" fill="clear">Ghost</ion-button>
        </div>
      </ion-content>
    </div> -->
      <ion-router-outlet id="main" />
    </ion-split-pane>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonMenu,
  IonSplitPane,
  IonTitle,
  IonToolbar,
  IonIcon,
  IonText,
  IonButton,
  IonList,
  IonCard,
  IonCardTitle,
  IonCardSubtitle,
  IonCardHeader,
  IonLabel,
  IonPage,
  IonAvatar,
  IonItem,
  IonRouterOutlet,
  menuController
} from '@ionic/vue';
import {
  business,
  cog
} from 'ionicons/icons';
import { onMounted } from 'vue';
import { useI18n } from 'vue-i18n';

import { useRouter, useRoute } from 'vue-router';
import { AvailableDevice } from '../plugins/libparsec/definitions';

// const props =defineProps<{
//   device: AvailableDevice
// }>();

let device: any;

// fake data
const workspacesExampleData = [
  {
    id: 1234,
    name: 'Product Design',
    userRole: 'Owner',
    userCount: 3
  },
  {
    id: 2345,
    name: 'Marketing',
    userRole: 'Contributor',
    userCount: 1
  },
  {
    id: 3456,
    name: 'Engineering',
    userRole: 'Contributor',
    userCount: 4
  },
  {
    id: 4567,
    name: 'Research',
    userRole: 'Reader',
    userCount: 3
  }
];
const router = useRouter();
const currentRoute = useRoute();
const { t, d } = useI18n();

function navigateToPage(pageName: string): void {
  router.push({ name: pageName });
  menuController.close();
}

// code au chargement de la page
onMounted(() => {
  device = currentRoute.params.device;
  console.log(device);
});

</script>

<style lang="scss" scoped>

.sidebar-menu, .sidebar-menu ion-content {
  --background: var(--parsec-color-light-primary-800);
}
.menu-orga-card{
  --background: var(--parsec-color-light-primary-30-opacity15);
  box-shadow: none;
  margin: 0.5rem;
  ion-card-header{
    display: flex;
    justify-content: space-between;
    .text-content{
      box-shadow: none;
      display: flex;
      align-items: center;
      justify-content: left;
      gap: 0.75em;
      .orga-avatar{
        background-color: var(--parsec-color-light-primary-800);
        color: var(--parsec-color-light-primary-50);
        width: 42px;
        height: 42px;
        border-radius: 50%;
        text-align: center;
        display: flex;
        align-items: center;
        justify-content: center;
      }
      ion-card-subtitle{
        --color: var(--parsec-color-light-secondary-light);
      }
      ion-card-title{
        padding: 0.1875em 0;
        margin: 0;
        --color: var(--parsec-color-light-primary-50);
      }
    }
  }
  .orga-manage-btn{
    padding:0.625em 1em;
    display: flex;
    align-items: center;
    gap: 1em;
    color: var(--parsec-color-light-secondary-light);
    border-top: 1px solid var(--parsec-color-light-primary-30-opacity15);
    &:hover{
      background: var(--parsec-color-light-primary-30-opacity15);
    }
  }
}
.list-md{
  background: none;
}

.menu-list-workspaces{
  ion-header{
    opacity: 0.6;
    color: var(--parsec-color-light-primary-100);
    margin-bottom: 1.5rem;
    width: fit-content;
    &:hover::after{
      border-bottom: 1px solid var(--parsec-color-light-primary-100);
    }
  }
  .item-label{
    --background:none;
    border-radius: 4px;
    &:hover{
      --background: var(--parsec-color-light-primary-30-opacity15);
    }
    ion-label{
      --color: var(--parsec-color-light-primary-100);
    }
    ion-icon{
      color: var(--parsec-color-light-primary-100);
      font-size: 1.25em;
      margin-inline-end: 12px;
    }
  }
}

</style>
