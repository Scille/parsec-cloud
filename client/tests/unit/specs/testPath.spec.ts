// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { parse, join } from '@/common/path';

describe('Path', () => {
  it('test path parsing', async () => {
    expect(parse('/')).to.deep.equal(['/']);
    expect(parse('/a/b')).to.deep.equal(['/', 'a', 'b']);
    expect(parse('/a/')).to.deep.equal(['/', 'a']);
    expect(parse('/a//b')).to.deep.equal(['/', 'a', 'b']);
    expect(parse('/////a')).to.deep.equal(['/', 'a']);
    expect(parse('//')).to.deep.equal(['/']);
    expect(parse('///////')).to.deep.equal(['/']);
    expect(() => parse('')).to.throw('Path should be at least "/" and start with "/"');
    expect(() => parse('a/b')).to.throw('Path should be at least "/" and start with "/"');
  });

  it('test path joining', async () => {
    expect(join('/', 'a')).to.equal('/a');
    expect(join('', '/')).to.equal('/');
    expect(join('', '/a')).to.equal('/a');
    expect(join('/a', 'b')).to.equal('/a/b');
    expect(join('/a/', '/b')).to.equal('/a/b');
    expect(join('/a/', '/b')).to.equal('/a/b');
  });
});
