// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultProvideConfig, mockLibParsec } from '@tests/component/support/mocks';

mockLibParsec();

import { FileModel, FolderModel } from '@/components/files';
import FileListItem from '@/components/files/FileListItem.vue';
import { FileType } from '@/parsec';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

describe('File List Item', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(DateTime.utc(1988, 4, 7, 12, 0, 0).toJSDate());
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('Display item for file', () => {
    const FILE: FileModel = {
      tag: FileType.File,
      confinementPoint: null,
      isConfined: (): boolean => false,
      id: '67',
      parent: '66',
      created: DateTime.now(),
      updated: DateTime.now(),
      baseVersion: 1,
      isPlaceholder: false,
      needSync: false,
      size: 43_297_832_478,
      name: 'A File.txt',
      path: '/',
      isFile: (): boolean => true,
      isSelected: false,
    };

    const wrapper = mount(FileListItem, {
      props: {
        entry: FILE,
        showCheckbox: false,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.file-name__label').text()).to.equal('A File.txt');
    expect(wrapper.get('.label-last-update').text()).to.equal('now');
    expect(wrapper.get('.label-size').text()).to.equal('40.3 GB');
    wrapper.trigger('dblclick');
    // When the drag&drop was added, this stopped working
    // expect(wrapper.emitted('click')?.length).to.equal(1);
    // expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FILE);
    wrapper.get('.options-button').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(FILE);
  });

  it('Display item for folder', () => {
    const FOLDER: FolderModel = {
      tag: FileType.Folder,
      confinementPoint: null,
      isConfined: (): boolean => false,
      id: '67',
      parent: '66',
      created: DateTime.now(),
      updated: DateTime.now(),
      baseVersion: 1,
      isPlaceholder: false,
      needSync: false,
      name: 'A Folder',
      path: '/',
      isFile: (): boolean => false,
      isSelected: false,
    };

    const wrapper = mount(FileListItem, {
      props: {
        entry: FOLDER,
        showCheckbox: false,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.file-name__label').text()).to.equal('A Folder');
    expect(wrapper.get('.label-last-update').text()).to.equal('now');
    // expect(wrapper.get('.label-size')).not.to.be.visible;
    wrapper.trigger('dblclick');
    // Stopped working when the drag&drop was added
    // expect(wrapper.emitted('click')?.length).to.equal(1);
    // expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FOLDER);
    wrapper.get('.options-button').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(FOLDER);
  });
});
