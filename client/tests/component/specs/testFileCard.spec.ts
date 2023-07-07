// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import FileCard from '@/components/FileCard.vue';
import { MockFile } from '@/common/mocks';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { mockI18n, getDefaultProvideConfig } from '../support/mocks';

mockI18n();

describe('File Card Item', () => {

  it('Display item for file', () => {
    const FILE: MockFile = {
      id: '0',
      name: 'A File.txt',
      type: 'file',
      size: 43_297_832_478,
      lastUpdate: DateTime.now(),
      updater: 'Gordon Freeman',
      children: []
    };

    const wrapper = mount(FileCard, {
      props: {
        file: FILE
      },
      global: {
        provide: getDefaultProvideConfig()
      }
    });

    expect(wrapper.get('.card-content__title').text()).to.equal('A File.txt');
    expect(wrapper.get('.card-content-last-update').text()).to.equal('FoldersPage.File.lastUpdateOne minute ago');
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FILE);
    wrapper.get('.card-option').trigger('click');
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
        children: []
      }, {
        id: '2',
        name: 'Another File.png',
        type: 'file',
        size: 239_029_484,
        lastUpdate: DateTime.now(),
        updater: 'Gordon Freeman',
        children: []
      }]
    };

    const wrapper = mount(FileCard, {
      props: {
        file: FOLDER
      },
      global: {
        provide: getDefaultProvideConfig()
      }
    });

    expect(wrapper.get('.card-content__title').text()).to.equal('A Folder');
    expect(wrapper.get('.card-content-last-update').text()).to.equal('FoldersPage.File.lastUpdateOne minute ago');
    // expect(wrapper.get('.label-size')).not.to.be.visible;
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(FOLDER);
    wrapper.get('.card-option').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(FOLDER);
  });
});
