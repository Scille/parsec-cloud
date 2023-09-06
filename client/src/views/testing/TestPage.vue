<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<!-- This page serve only for test purposes -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <ion-header collapse="condense">
        <ion-title size="large">
          Test libparsec
        </ion-title>
      </ion-header>
      <div v-html="logs" />
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
import { libparsec } from '@/plugins/libparsec';
import { ClientEvent, ClientConfig } from '@/plugins/libparsec/definitions';

const logs = ref('');

async function testCase<T>(name: string, cb: () => Promise<T>): Promise<T> {
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
  logs.value += ' <span style="color: green; font-weight: bold">☑</span><br>';
  return ret;
}

// ARE YOU KIDDING ME JAVASCRIPT ??????
function compareArrays(a: Array<any>, b: Array<any>): boolean {
  return a.length === b.length && a.every((value, index) => {
    b[index] === value;
  });
}

function assert(outcome: boolean, msg: string): void {
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
  // Tests are ran here

  await testBootstrapOrganization();
});

/*
 * Bootstrap organization
 */

async function testBootstrapOrganization(): Promise<void> {
  const configPath = await testCase('Init empty testbed', async () => {
    return await libparsec.testNewTestbed('empty', import.meta.env.VITE_TESTBED_SERVER_URL);
  });

  const config: ClientConfig = {
    configDir: configPath,
    dataBaseDir: configPath,
    mountpointBaseDir: configPath,
    workspaceStorageCacheSize: {
      tag: 'Default',
    },
  };

  // eslint-disable-next-line @typescript-eslint/explicit-function-return-type
  const onEventCallback = (e: ClientEvent) => {
    console.log(`Event received from libparsec: ${e}`);
  };
  const bootstrapAddr = await libparsec.testGetTestbedBootstrapOrganizationAddr(configPath);
  if (bootstrapAddr === null) {
    throw new Error("Couldn't retrieve bootstrap organization addr");
  }

  const humanHandle = { label: 'John', email: 'john@example.com' };
  const availableDevice = await testCase('Bootstrap organization', async () => {
    const outcome = await libparsec.bootstrapOrganization(
      config,
      onEventCallback,
      bootstrapAddr,
      {
        tag: 'Password',
        password: 'P@ssw0rd.',
      },
      humanHandle,
      'PC1',
      null,
    );
    switch (outcome.ok) {
      case true:
        return outcome.value;

      default:
        throw new Error(`Returned error: ${JSON.stringify(outcome, null, 2)}`);
    }
  });
  assert(availableDevice.humanHandle === humanHandle, `Invalid available device: ${JSON.stringify(availableDevice, null, 2)}`);
  assert(availableDevice.deviceLabel === 'PC1', `Invalid available device: ${JSON.stringify(availableDevice, null, 2)}`);

  // Cannot re-bootstrap the organization !
  await testCase('Bootstrap organization bad outcome', async () => {
    const outcome = await libparsec.bootstrapOrganization(
      config,
      onEventCallback,
      bootstrapAddr,
      {
        tag: 'Password',
        password: 'P@ssw0rd.',
      },
      null,
      null,
      null,
    );
    switch (outcome.ok) {
      case true:
        throw new Error(`Returned success but expected error ! ${JSON.stringify(outcome, null, 2)}`);

      case false:
        switch (outcome.error.tag) {
          case 'AlreadyUsedToken':
            break;

          default:
            throw new Error(`Returned expected error: ${JSON.stringify(outcome, null, 2)}`);
        }
    }
  });

  await testCase('Teardown testbed', async () => {
    await libparsec.testDropTestbed(configPath);
  });

}
</script>
