// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { mockI18n, getDefaultProvideConfig } from '@tests/component/support/mocks';
import FileListItem from '@/components/files/FileListItem.vue';
import { EntryStatFile, EntryStatFolder, FileType } from '@/parsec';

mockI18n();

describe('File List Item', () => {
  it('Display item for file', () => {
    const FILE: EntryStatFile = {
      tag: FileType.File,
      confinementPoint: null,
      id: '67',
      created: DateTime.now(),
      updated: DateTime.now(),
      baseVersion: 1,
      isPlaceholder: false,
      needSync: false,
      size: 43_297_832_478,
      name: 'A File.txt',
      isFile: (): boolean => true,
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
    const FOLDER: EntryStatFolder= {
      tag: FileType.Folder,
      confinementPoint: null,
      id: '67',
      created: DateTime.now(),
      updated: DateTime.now(),
      baseVersion: 1,
      isPlaceholder: false,
      needSync: false,
      name: 'A Folder',
      isFile: (): boolean => false,
      children: ['A File.txt', 'Another File.png'],
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
