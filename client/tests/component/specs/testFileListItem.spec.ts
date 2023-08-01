// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MockFile } from '@/common/mocks';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { mockI18n, getDefaultProvideConfig } from 'tests/component/support/mocks';
import FileListItem from '@/components/files/FileListItem.vue';

mockI18n();

describe('File List Item', () => {
  it('Display item for file', () => {
    const FILE: MockFile = {
      id: '0',
      name: 'A File.txt',
      type: 'file',
      size: 43_297_832_478,
      lastUpdate: DateTime.now(),
      updater: 'Gordon Freeman',
      children: [],
    };

    const wrapper = mount(FileListItem, {
      props: {
        file: FILE,
        showCheckbox: false,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect((wrapper.vm as any).isSelected).to.be.false;
    expect(wrapper.get('.file-name__label').text()).to.equal('A File.txt');
    expect(wrapper.get('.label-last-update').text()).to.equal('One minute ago');
    expect(wrapper.get('.label-size').text()).to.equal('1MB');
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FILE);
    wrapper.get('.options-button').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(FILE);
  });

  it('Display item for folder', () => {
    const FOLDER: MockFile = {
      id: '0',
      name: 'A Folder',
      type: 'folder',
      size: 0,
      lastUpdate: DateTime.now(),
      updater: 'Gordon Freeman',
      children: [{
        id: '1',
        name: 'A File.txt',
        type: 'file',
        size: 8_932_472_384,
        lastUpdate: DateTime.now(),
        updater: 'Gordon Freeman',
        children: [],
      }, {
        id: '2',
        name: 'Another File.png',
        type: 'file',
        size: 239_029_484,
        lastUpdate: DateTime.now(),
        updater: 'Gordon Freeman',
        children: [],
      }],
    };

    const wrapper = mount(FileListItem, {
      props: {
        file: FOLDER,
        showCheckbox: false,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect((wrapper.vm as any).isSelected).to.be.false;
    expect(wrapper.get('.file-name__label').text()).to.equal('A Folder');
    expect(wrapper.get('.label-last-update').text()).to.equal('One minute ago');
    // expect(wrapper.get('.label-size')).not.to.be.visible;
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FOLDER);
    wrapper.get('.options-button').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(FOLDER);
  });
});
