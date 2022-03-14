<template>
  <ion-page>
    <ion-header :translucent="true">
      <ion-toolbar>
        <ion-title>Parsec</ion-title>
      </ion-toolbar>
    </ion-header>

    <ion-content :fullscreen="true">
      <ion-header collapse="condense">
        <ion-toolbar>
          <ion-title size="large">Parsec</ion-title>
        </ion-toolbar>
      </ion-header>

      <div id="container">
        <strong>Ready to create Parsec?</strong>
        <div>
            <ion-button @click="onSubmit">Let's go!</ion-button>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script lang="ts">
import {
    IonContent,
    IonHeader,
    IonPage,
    IonTitle,
    IonToolbar,
    IonButton,
    toastController
} from '@ionic/vue';
import { defineComponent, ref } from 'vue';

const key = "1UT2bs6chdW4AnXbkSS18EuwOAgWIr7ROcHnicUhdAA="
const message = "Ym9ueW91ciE="

export default defineComponent({
  name: 'HomePage',
  components: {
    IonContent,
    IonHeader,
    IonPage,
    IonTitle,
    IonToolbar,
    IonButton
  },
  setup() {
    const content = ref();
    return { content, onSubmit }
  }
});

async function onSubmit(): Promise<any> {
    console.log("calling encrypt...");
    const encrypted = await encrypt(key, message);
    console.log(encrypted);
    console.log("calling decrypt...");
    const decrypted = await decrypt(key, encrypted);
    console.log(decrypted);
    toastController.create({
        message: decrypted,
        duration: 2000
    }).then((toast) => {
        toast.present();
    });
}

async function encrypt(key: string, cleartext: string): Promise<string> {
    return new Promise((resolve, reject) => {
        const w: any = window;
        w.libparsec.submitJob(resolve, reject, "encrypt", `${key}:${cleartext}`);
    });
}

async function decrypt(key: string, cyphertext: string): Promise<string> {
    return new Promise((resolve, reject) => {
        const w: any = window;
        w.libparsec.submitJob(resolve, reject, "decrypt", `${key}:${cyphertext}`);
    });
}
</script>

<style scoped>
#container {
  text-align: center;

  position: absolute;
  left: 0;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
}

#container strong {
  font-size: 20px;
  line-height: 26px;
}

#container a {
  text-decoration: none;
}
</style>
