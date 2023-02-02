// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { mount, VueWrapper } from '@vue/test-utils';
import MsSelectPopover from '@/components/MsSelectPopover.vue';
import MsSelect from '@/components/MsSelect.vue';
import { MsSelectOption, MsSelectSortByLabels } from '@/components/MsSelectOption';
import { popoverController } from '@ionic/vue';
import { clearEmitted } from './utils';

describe('MsSelect.vue', () => {
  const options: MsSelectOption[] = [
    { label: 'Label A', key: '1' },
    { label: 'Label B', key: '2' },
    { label: 'Label C', key: '3' }
  ];

  const sortLabels: MsSelectSortByLabels = {
    asc: 'Asc',
    desc: 'Desc'
  };

  const label = 'Select label';

  const wrapper = mount(MsSelect, {
    props: {
      options: options,
      label: label,
      sortByLabels: sortLabels,
      defaultOption: '1'
    }
  });

  let selectButton: VueWrapper;

  beforeAll(() => {
    selectButton = wrapper.findComponent('ion-button') as VueWrapper;
  });

  beforeEach(() => {
    jest.clearAllMocks();
    clearEmitted(wrapper);
  });

  it('renders select', () => {
    expect(wrapper.vm.sortByAsc).toBe(true);
    expect(wrapper.vm.selectedOption).toEqual(options[0]);
    expect(wrapper.vm.labelRef).toEqual('Label A');
  });

  it('should render select popover on select click', () => {
    const popoverCtrlSpy = jest.spyOn(popoverController, 'create');
    const openPopoverSpy = jest.spyOn(wrapper.vm, 'openPopover');
    selectButton.trigger('click');
    expect(popoverCtrlSpy).toHaveBeenCalledWith({
      component: MsSelectPopover,
      componentProps: {
        options: wrapper.props('options'),
        defaultOption: wrapper.vm.selectedOption.key,
        sortByLabels: wrapper.props('sortByLabels'),
        sortByAsc: wrapper.vm.sortByAsc
      },
      event: openPopoverSpy.mock.lastCall.at(0)
    });
  });

  it('should update select name, sort value and emit change on popover dismiss', async () => {
    await wrapper.vm.openPopover();
    await popoverController.dismiss({
      option: options[2],
      sortByAsc: false
    });

    expect(wrapper.vm.sortByAsc).toBe(false);
    expect(wrapper.vm.selectedOption).toEqual(options[2]);
    expect(wrapper.vm.labelRef).toEqual('Label C');
    expect(wrapper.emitted('change')).toEqual([[{
      option: options[2],
      sortByAsc: false
    }]]);
  });
});
