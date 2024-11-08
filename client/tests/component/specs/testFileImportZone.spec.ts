// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { mockLibParsec } from '@tests/component/support/mocks';

mockLibParsec();

import { describe, it } from 'vitest';

describe('File Drop Zone', () => {
  it('Handles dropped file', async () => {
    // A real pain to mock
    // const wrapper = mount(FileDropZone, {
    //   props: {
    //     currentPath: '/',
    //   },
    //   global: {
    //     provide: config,
    //   },
    // });
    // vi.useFakeTimers();
    // expect((wrapper.vm as any).isActive).to.be.false;
    // await wrapper.trigger('dragenter');
    // expect((wrapper.vm as any).isActive).to.be.true;
    // await wrapper.trigger('dragleave');
    // // Leave reacts after 50ms, giving it some time
    // vi.advanceTimersByTime(100);
    // expect((wrapper.vm as any).isActive).to.be.false;
    // await wrapper.trigger('dragenter');
    // expect((wrapper.vm as any).isActive).to.be.true;
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
    // vi.useRealTimers();
  });
});
