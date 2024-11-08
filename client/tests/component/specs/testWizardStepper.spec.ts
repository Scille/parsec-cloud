// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { IonText } from '@ionic/vue';
import { VueWrapper, mount } from '@vue/test-utils';
import { MsWizardStepper } from 'megashark-lib';
import { beforeEach, describe, expect, it } from 'vitest';

describe('Wizard stepper', () => {
  let wrapper: VueWrapper;

  beforeEach(() => {
    wrapper = mount(MsWizardStepper, {
      props: {
        titles: ['A', 'B', 'C', 'D', 'E'],
        currentIndex: 2,
      },
    });
  });

  it('display steps', async () => {
    const step = wrapper.get('div').findAllComponents(IonText);

    expect(step.length).to.equal(5);
    expect(step.at(0).text()).to.contain('A');
    expect(step.at(4).text()).to.contain('E');
  });
});
