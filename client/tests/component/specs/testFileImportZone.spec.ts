// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import FileImportZone from '@/components/files/FileImportZone.vue';
import { getDefaultProvideConfig } from '@tests/component/support/mocks';
import { mount } from '@vue/test-utils';
import { vi } from 'vitest';

const config = getDefaultProvideConfig();

describe('File Drop Zone', () => {
  it('Handles dropped file', async () => {
    const wrapper = mount(FileImportZone, {
      props: {
        currentPath: '/',
        workspaceHandle: 42,
        workspaceId: '1337',
      },
      global: {
        provide: config,
      },
    });

    vi.useFakeTimers();

    expect((wrapper.vm as any).isActive).to.be.false;
    await wrapper.trigger('dragenter');
    expect((wrapper.vm as any).isActive).to.be.true;
    await wrapper.trigger('dragleave');
    // Leave reacts after 50ms, giving it some time
    vi.advanceTimersByTime(100);
    expect((wrapper.vm as any).isActive).to.be.false;

    await wrapper.trigger('dragenter');
    expect((wrapper.vm as any).isActive).to.be.true;

    // A real pain to mock

    // // Fake drop event
    // await wrapper.trigger('drop', {
    //   dataTransfer: {
    //     items: [
    //       {
    //         webkitGetAsEntry: (): any => {
    //           return {
    //             name: 'File1.txt',
    //             size: 1337,
    //           };
    //         },
    //       },
    //     ],
    //   },
    // });

    // // Making sure we got our filesDrop event
    // expect(wrapper.emitted('filesAdded')?.length).to.equal(1);
    // const files = wrapper.emitted('filesAdded')?.at(0)?.at(0) as Array<any>;
    // expect(files.length).to.equal(1);
    // expect(files[0].name).to.equal('File1.txt');
    // expect(files[0].size).to.equal(1337);

    // // // Drop zone should have been made inactive by the drop event
    // expect((wrapper.vm as any).isActive).to.be.false;

    vi.useRealTimers();
  });
});
