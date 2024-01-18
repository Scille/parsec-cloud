// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import FileCard from '@/components/files/FileCard.vue';
import { EntryStatFile, EntryStatFolder, FileType } from '@/parsec';
import { getDefaultProvideConfig } from '@tests/component/support/mocks';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { vi } from 'vitest';

describe('File Card Item', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(DateTime.utc(1988, 4, 7, 12, 0, 0).toJSDate());
  });

  afterEach(() => {
    vi.useRealTimers();
  });

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

    const wrapper = mount(FileCard, {
      props: {
        file: FILE,
        showCheckbox: true,
        showOptions: true,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.card-content__title').text()).to.equal('A File.txt');
    expect(wrapper.get('.card-content-last-update').text()).to.equal('Last updated:one second ago');
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FILE);
    wrapper.get('.card-option').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(FILE);
  });

  it('Display item for folder', () => {
    const FOLDER: EntryStatFolder = {
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

    const wrapper = mount(FileCard, {
      props: {
        file: FOLDER,
        showCheckbox: true,
        showOptions: true,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.card-content__title').text()).to.equal('A Folder');
    expect(wrapper.get('.card-content-last-update').text()).to.equal('Last updated:one second ago');
    // expect(wrapper.get('.label-size')).not.to.be.visible;
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FOLDER);
    wrapper.get('.card-option').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(FOLDER);
  });
});
