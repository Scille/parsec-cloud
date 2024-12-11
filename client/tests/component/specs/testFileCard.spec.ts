// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FileModel, FolderModel } from '@/components/files';
import FileCard from '@/components/files/FileCard.vue';
import { FileType } from '@/parsec';
import { getDefaultProvideConfig } from '@tests/component/support/mocks';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

describe('File Card Item', () => {
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

    const wrapper = mount(FileCard, {
      props: {
        entry: FILE,
        showCheckbox: true,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.file-card__title').text()).to.equal('A File.txt');
    // expect(wrapper.get('.card-content-last-update').text()).to.equal('now');
    wrapper.trigger('dblclick');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FILE);
    wrapper.get('.card-option').trigger('click');
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

    const wrapper = mount(FileCard, {
      props: {
        entry: FOLDER,
        showCheckbox: true,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.file-card__title').text()).to.equal('A Folder');
    // expect(wrapper.get('.file-card-last-update').text()).to.equal('now');
    // expect(wrapper.get('.label-size')).not.to.be.visible;
    wrapper.trigger('dblclick');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FOLDER);
    wrapper.get('.card-option').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(FOLDER);
  });
});
