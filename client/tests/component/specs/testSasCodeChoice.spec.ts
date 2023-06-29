// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import SasCodeChoice from '@/components/SasCodeChoice.vue';
import { VueWrapper, mount } from '@vue/test-utils';
import { IonButton } from '@ionic/vue';
import { mockI18n } from '../support/mocks';

mockI18n();

describe('Sas code choice', () => {
  let wrapper: VueWrapper;
  beforeEach(() => {
    wrapper = mount(SasCodeChoice, {
      props: {
        choices: ['AAAA', 'BBBB', 'CCCC', 'DDDD']
      }
    });
  });

  it('should display all for choices and a none option', async () => {
    const buttons = wrapper.findAllComponents(IonButton);

    expect(buttons.length).to.equal(5);
    expect(buttons.at(0)?.text()).to.contain('AAAA');
    expect(buttons.at(1)?.text()).to.contain('BBBB');
    expect(buttons.at(2)?.text()).to.contain('CCCC');
    expect(buttons.at(3)?.text()).to.contain('DDDD');
    expect(buttons.at(4)?.text()).to.contain('SasCodeChoice.noneOfTheChoices');
  });

  it('should emit select with the clicked value', async () => {
    const buttons = wrapper.findAllComponents(IonButton);

    await buttons.at(1)?.trigger('click');
    expect(wrapper.emitted('select')?.length).to.equal(1);
    expect(wrapper.emitted('select')?.at(0)).to.have.same.members(['BBBB']);
  });

  it('should emit select with null if none is clicked', async () => {
    const buttons = wrapper.findAllComponents(IonButton);

    await buttons.at(4)?.trigger('click');
    expect(wrapper.emitted('select')?.length).to.equal(1);
    expect(wrapper.emitted('select')?.at(0)).to.have.same.members([null]);
  });
});
