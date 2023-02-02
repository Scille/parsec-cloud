// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { mount, VueWrapper } from '@vue/test-utils';
import MsSelectPopover from '@/components/MsSelectPopover.vue';
import { MsSelectOption, MsSelectSortByLabels } from '@/components/MsSelectOption';
import {
  arrowUp,
  arrowDown
} from 'ionicons/icons';
import { popoverController } from '@ionic/vue';

describe('MsSelectPopover.vue', () => {
  const options: MsSelectOption[] = [
    { label: 'Label A', key: '1' },
    { label: 'Label B', key: '2' },
    { label: 'Label C', key: '3' }
  ];

  const sortLabels: MsSelectSortByLabels = {
    asc: 'Asc',
    desc: 'Desc'
  };

  const wrapper = mount(MsSelectPopover, {
    props: {
      options: options,
      sortByLabels: sortLabels,
      sortByAsc: true
    }
  });

  let sortIcon: VueWrapper;

  beforeAll(() => {
    sortIcon = wrapper.findComponent('ion-icon') as VueWrapper;
  });

  beforeEach(() => {
    jest.clearAllMocks();
  });

  function findItemAt(wrapper: VueWrapper, index: number): VueWrapper | undefined {
    for (const [i, item] of wrapper.findAllComponents('ion-item').entries()) {
      if (index === i) {
        return item as VueWrapper;
      }
    }
  }

  it('renders select popover', () => {
    expect(wrapper.vm.sortByAsc).toBe(true);
    expect(wrapper.vm.selectedOption).toBe(undefined);

    wrapper.findAllComponents('ion-item').forEach((item, index) => {
      if (index === 0) {
        expect(item.text()).toBe('Asc');
        expect(sortIcon.props('icon')).toBe(arrowUp);
      } else {
        expect(item.text()).toBe(options[index - 1].label);
        expect(item.attributes('class')).not.toContain('selected');
      }
    });
  });

  it('allows item selection', () => {
    const popoverCtrlSpy = jest.spyOn(popoverController, 'dismiss').mockImplementation();

    expect(wrapper.vm.selectedOption).toBe(undefined);
    const item = findItemAt(wrapper, 2);
    expect(item).toBeDefined();

    item?.trigger('click');
    expect(popoverCtrlSpy).toHaveBeenCalledWith({
      option: options[1],
      sortByAsc: true
    });
  });

  it('inverts sort order', async () => {
    const popoverCtrlSpy = jest.spyOn(popoverController, 'dismiss').mockImplementation();

    expect(wrapper.vm.sortByAsc).toBe(true);
    const item = findItemAt(wrapper, 0);
    expect(item).toBeDefined();
    expect(sortIcon.props('icon')).toBe(arrowUp);
    await item?.trigger('click');
    expect(popoverCtrlSpy).toHaveBeenCalledWith({
      option: options[1],
      sortByAsc: false
    });
    expect(sortIcon.props('icon')).toBe(arrowDown);
  });
});
