// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MsOptions, MsSorterLabels } from '@/components/core';
import MsSorterPopover from '@/components/core/ms-sorter/MsSorterPopover.vue';
import { VueWrapper, mount } from '@vue/test-utils';

describe('Select popover', () => {
  let wrapper: VueWrapper;

  const defaultOptions: MsOptions = new MsOptions([
    { label: 'Label A', key: '1' },
    { label: 'Label B', key: '2' },
    { label: 'Label C', key: '3' },
  ]);

  const defaultSortLabels: MsSorterLabels = {
    asc: 'Asc',
    desc: 'Desc',
  };

  beforeEach(() => {
    wrapper = mount(MsSorterPopover, {
      props: {
        options: defaultOptions,
        sorterLabels: defaultSortLabels,
        sortByAsc: true,
      },
    });
  });

  it('display the select popover', async () => {
    expect(wrapper.find('#sort-order-button').text()).to.equal('Asc');
    const items = wrapper.findAll('ion-item');
    expect(items.length).to.equal(4);
    // Use first option as default
    expect((wrapper.vm as any).selectedOption).to.deep.equal(defaultOptions.at(0));
    expect((wrapper.vm as any).sortByAsc).to.be.true;
    expect(items.at(1)?.text()).to.equal('Label A');
    expect(items.at(2)?.text()).to.equal('Label B');
    expect(items.at(3)?.text()).to.equal('Label C');

    expect(items.at(1)?.find('ion-icon').classes()).to.include('checked');
    expect(items.at(2)?.find('ion-icon').exists()).to.be.false;
    expect(items.at(3)?.find('ion-icon').exists()).to.be.false;
  });

  it('changes order when clicked', () => {
    // No idea how to mock popoverController.dismiss()
    // const sortButton = wrapper.find('#sort-order-button');
    // expect(sortButton.text()).to.equal('Asc');
    // sortButton.trigger('click');
    //   expect(sortButton.text()).to.equal('Desc');
    //   sortButton.trigger('click');
    //   expect(sortButton.text()).to.equal('Asc');
    // });
  });

  it('changes selected when clicked', () => {
    // NOP
  });
});
