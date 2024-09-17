// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FIFO } from '@/common/queue';
import { it } from 'vitest';

describe('Queues', () => {
  it('Test FIFO', async () => {
    const fifo = new FIFO<number>(5);

    expect(fifo.getEntries()).to.deep.equal([]);
    expect(fifo.pop()).to.equal(undefined);
    expect(fifo.length()).to.equal(0);
    fifo.push(1);
    expect(fifo.getEntries()).to.deep.equal([1]);
    fifo.push(2);
    fifo.push(3);
    fifo.push(4);
    expect(fifo.push(5)).to.equal(undefined);
    expect(fifo.getEntries()).to.deep.equal([5, 4, 3, 2, 1]);
    expect(fifo.push(6)).to.equal(1);
    expect(fifo.getEntries()).to.deep.equal([6, 5, 4, 3, 2]);
    expect(fifo.push(7)).to.equal(2);
    expect(fifo.getEntries()).to.deep.equal([7, 6, 5, 4, 3]);
    expect(fifo.length()).to.equal(5);
    expect(fifo.pop()).to.equal(3);
    expect(fifo.getEntries()).to.deep.equal([7, 6, 5, 4]);
    expect(fifo.length()).to.equal(4);
  });
});
