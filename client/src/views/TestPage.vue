<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<!-- This page serve only for test purposes -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <ion-header collapse="condense">
        <ion-title size="large">
          Test libparsec
        </ion-title>
      </ion-header>
      <div v-html="logs"></div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonPage,
  IonTitle,
} from '@ionic/vue';
import { ref, onMounted } from 'vue';
import { libparsec } from '../plugins/libparsec';
import { ClientEvent, ClientConfig, DeviceAccessParamsPassword } from '../plugins/libparsec/definitions';

const logs = ref('');

async function testCase(name: string, cb: () => Promise<any> ): Promise<any> {
  console.log(`${name}...`);
  logs.value += `${name}...`;
  let ret;
  try {
    ret = await cb();
  } catch (error) {
    logs.value += ` <span style="color: red; font-weight: bold">☒</span><br>${error}`;
    throw error;
  }
  console.log(`${name}... ok !`);
  logs.value += ` <span style="color: green; font-weight: bold">☑</span><br>`;
  return ret;
}

// ARE YOU KIDDING ME JAVASCRIPT ??????
function compareArrays(a: Array<any>, b: Array<any>) {
  return a.length == b.length && a.every((value, index) => {b[index] === value});
}

function assert(outcome: boolean, msg: string) {
  if (!outcome) {
    throw `Error: ${msg}`;
  }
}
  // {
  //   "keyFilePath": "/parsec/testbed/1/Org20/alice@dev1.key",
  //   "organizationId": "Org20",
  //   "deviceId": "alice@dev1",
  //   "humanHandle": "Alicey McAliceFace <alice@example.com>",
  //   "deviceLabel": "My dev1 machine",
  //   "slug": "9ff2284ce2#Org20#alice@dev1",
  //   "ty": {
  //     "tag": "Password"
  //   }
  // }

onMounted(async () => {
  // Tests are runned here

  const configPath = await testCase("Init testbed", async () => {
    return await libparsec.testNewTestbed('coolorg', import.meta.env.VITE_TESTBED_SERVER_URL);
  });

  await testCase("Teardown testbed", async () => {
    const devices = await libparsec.clientListAvailableDevices(configPath);
    assert(compareArrays(devices, []), `Bad devices: ${JSON.stringify(devices, null, 2)}`)
  });

  await testCase("Teardown testbed", async () => {
    await libparsec.testDropTestbed(configPath);
  });

});

// const path = 'PATH/TO/.config/parsec/';
// const password = 'PASSWORD';
// // eslint-disable-next-line @typescript-eslint/explicit-function-return-type, @typescript-eslint/no-unused-vars
// const onEventCallback = (event: ClientEvent) => {
//   console.log('callback');
// };
// const config: ClientConfig = {
//   configDir: '',
//   dataBaseDir: '',
//   mountpointBaseDir: '',
//   preferredOrgCreationBackendAddr: 'parsec://alice_dev1.example.com:9999',
//   workspaceStorageCacheSize: { tag: 'Default' }
// };


// async function runTests(): Promise<void> {

//   // console.log('Submitting');
//   // const devices = await libparsec.clientListAvailableDevices(path);
//   // console.log(devices);
//   // const param: DeviceAccessParamsPassword = {
//   //   tag: 'Password',
//   //   path: devices[0].keyFilePath,
//   //   // path: devices[0].slug,
//   //   password
//   // };
//   // const handle = await libparsec.clientLogin(param, config, onEventCallback).then((x) => x.ok ? x.value : -1);
//   // console.log(handle);
//   // const deviceID = await libparsec.clientGetDeviceId(handle).then((x) => x.ok ? x.value : -1);
//   // console.log(deviceID);
// }
</script>
