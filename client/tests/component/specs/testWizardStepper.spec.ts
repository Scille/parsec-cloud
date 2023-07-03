// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import WizardStepper from '@/components/WizardStepper.vue';
import { VueWrapper, mount } from '@vue/test-utils';
import { IonChip } from '@ionic/vue';

describe('Wizard stepper', () => {
  let wrapper: VueWrapper;

  beforeEach(() => {
    wrapper = mount(WizardStepper, {
      props: {
        titles: ['A', 'B', 'C', 'D', 'E'],
        currentIndex: 2
      }
    });
  });

  it('display steps', async () => {
    const chips = wrapper.get('div').findAllComponents(IonChip);

    expect(chips.length).to.equal(5);
    expect(chips.at(0).text()).to.contain('A');
    expect(chips.at(4).text()).to.contain('E');

    // For some reason, the ion-chip has no attributes and no classes.
    // console.log(chips.at(0).attributes());
    // console.log(chips.at(0).classes());
    // expect(chips.at(0)?.attributes().color).to.equal('medium');
    // expect(chips.at(1)?.attributes('color')).to.equal('medium');
    // expect(chips.at(2)?.attributes('color')).to.equal('primary');
    // expect(chips.at(3)?.attributes('color')).to.equal('secondary');
    // expect(chips.at(4)?.attributes('color')).to.equal('secondary');
  });
});
