// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { join, parse } from '@/common/path';
import { Path } from '@/parsec';

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

  it('test get extension', () => {
    expect(Path.getFileExtension('video.mp4')).to.equal('mp4');
    expect(Path.getFileExtension('video.MP4')).to.equal('mp4');
    expect(Path.getFileExtension('.config')).to.equal('');
    expect(Path.getFileExtension('')).to.equal('');
    expect(Path.getFileExtension('file')).to.equal('');
    expect(Path.getFileExtension('archive.tar.gz')).to.equal('gz');
    expect(Path.getFileExtension('.config.cfg')).to.equal('cfg');
    expect(Path.getFileExtension('...config.cfg')).to.equal('cfg');
    expect(Path.getFileExtension('a.long.file.name.txt')).to.equal('txt');
  });
});
