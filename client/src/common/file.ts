// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { File } from '@/components/core/ms-image';
import { Path } from '@/parsec';
import { translate } from '@/services/translation';

// Shorten a file name by adding ellipsis in the middle if it is above a certain length.
// It may be a bit scuffed in some edge cases but the default config should work fine.
export function shortenFileName(filename: string, { maxLength = 15, prefixLength = 6, suffixLength = 6, filler = '...' } = {}): string {
  if (maxLength > 0 && filename.length > maxLength && prefixLength + suffixLength > 0) {
    // Check that we're keeping under the maxLength
    if (prefixLength + suffixLength + filler.length > maxLength) {
      // Recalculate prefixLength and suffixLength as a ratio of the filename length
      const totalLength = prefixLength + suffixLength + filler.length;
      if (prefixLength) {
        const prefixPercent = Math.ceil((prefixLength / totalLength) * 100);
        prefixLength = Math.ceil(((maxLength - filler.length) / 100) * prefixPercent) || 1;
      }
      if (suffixLength) {
        const suffixPercent = Math.ceil((suffixLength / totalLength) * 100);
        suffixLength = Math.ceil(((maxLength - filler.length) / 100) * suffixPercent) || 1;
      }
    }
    filename = `${filename.substring(0, prefixLength)}${filler}${filename.substring(filename.length - suffixLength)}`;
  }
  return filename;
}

/* File size */
/*
Format the specified number of bytes with the corresponding system.

More specifically:
- `0 <= bytes < 10`:          1 significant figures:     `X B`
- `10 <= bytes < 100`:        2 significant figures:    `XY B`
- `100 <= bytes < 1000`:      3 significant figures:   `XYZ B`
- `1000 <= bytes < 1024`:     2 significant figures: `0.9X KB`
- `1 <= kilobytes < 10`:      3 significant figures: `X.YZ KB`
- `10 <= kilobytes < 100`:    3 significant figures: `XY.Z KB`
- `100 <= kilobytes < 1000`:  3 significant figures:  `XYZ KB`
- `1000 <= kilobytes < 1024`: 2 significant figures: `0.9X MB`
- And so on for MB, GB and TB
*/

function size(bytes: number, system: [number, string][]): string {
  if (bytes < 0) {
    throw Error('Bytes must be >= 0');
  }

  // Iterate over factors, expecting them to be in increasing order
  let formattedAmount = '';
  let factor = 1;
  let suffix = '';
  for (const item of system) {
    // Stop when the right factor is reached
    factor = item[0];
    suffix = item[1];
    if (bytes / factor < 999.5) {
      break;
    }
  }
  // Convert to the right unit
  let amount = bytes / factor;
  // Truncate to two decimals in order to avoid misleading rounding to 1.00
  amount = Math.trunc(amount * 100) / 100;
  // Factor is one, the amount is an integer
  if (factor === 1) {
    formattedAmount = amount.toFixed();
  } else if (amount < 10.0) {
    formattedAmount = amount.toFixed(2);
  } else if (amount < 99.95) {
    formattedAmount = amount.toFixed(1);
  } else {
    formattedAmount = amount.toFixed();
  }
  return `${formattedAmount} ${suffix}`;
}

export function formatFileSize(bytesize: number): string {
  const SYSTEM: [number, string][] = [
    [Math.pow(1024, 0), translate('common.filesize.bytes')],
    [Math.pow(1024, 1), translate('common.filesize.kilobytes')],
    [Math.pow(1024, 2), translate('common.filesize.megabytes')],
    [Math.pow(1024, 3), translate('common.filesize.gigabytes')],
    [Math.pow(1024, 4), translate('common.filesize.terabytes')],
  ];
  return size(bytesize, SYSTEM);
}

/* File icons */
export function getFileIcon(file: string): string {
  const fileExtension = Path.getFileExtension(file);
  switch (fileExtension) {
    case 'apk':
      return File.Apk;
    case 'css':
    case 'scss':
    case 'sass':
    case 'less':
    case 'styl':
    case 'stylus':
    case 'pcss':
    case 'postcss':
      return File.Css;
    case 'disc':
    case 'iso':
      return File.Iso;
    case 'pdf':
      return File.Pdf;
    case 'doc':
    case 'docx':
    case 'odt':
      return File.Word;
    case 'xls':
    case 'xlsx':
    case 'ods':
    case 'csv':
      return File.Excel;
    case 'ppt':
    case 'pptx':
    case 'odp':
      return File.Powerpoint;
    case 'txt':
      return File.Text;
    case 'otf':
    case 'ttf':
    case 'woff':
    case 'woff2':
      return File.Font;
    case 'ai':
      return File.Illustrator;
    case 'psd':
      return File.Photoshop;
    case 'js':
    case 'ts':
    case 'jsx':
    case 'tsx':
      return File.Javascript;
    case 'mail':
    case 'eml':
      return File.Mail;
    case 'php':
      return File.Php;
    case 'zip':
    case 'rar':
    case '7z':
    case 'tar':
    case 'gz':
    case 'tgz':
    case 'bz2':
    case 'xz':
    case 'lz':
    case 'z':
    case 'lzma':
    case 'lzo':
    case 'zst':
    case 's7z':
    case 'ace':
    case 'cab':
    case 'jar':
      return File.Zip;
    case 'eps':
    case 'svg':
    case 'cdr':
    case 'vsd':
    case 'pub':
    case 'indd':
    case 'swf':
    case 'fla':
    case 'emf':
    case 'wmf':
    case 'dxf':
    case 'plt':
    case 'pict':
    case 'vml':
    case 'cgm':
    case 'xar':
    case 'sda':
    case 'v2d':
    case 'odg':
      return File.Vector;
    case 'jpeg':
    case 'jpg':
    case 'png':
    case 'gif':
    case 'bmp':
    case 'tiff':
    case 'tif':
    case 'raw':
    case 'ico':
    case 'webp':
    case 'jp2':
    case 'jxr':
    case 'wdp':
    case 'pcx':
    case 'tga':
    case 'exr':
    case 'hdr':
    case 'dng':
    case 'pic':
    case 'pct':
    case 'jfif':
    case 'pjpeg':
    case 'pgm':
    case 'ppm':
    case 'pbm':
    case 'pgf':
    case 'bat':
    case 'bpg':
    case 'heif':
    case 'heic':
    case 'int':
    case 'lbm':
    case 'iff':
    case 'mac':
    case 'ilbm':
    case 'avs':
    case 'cin':
    case 'dpx':
    case 'fits':
      return File.Image;
    case 'mp3':
    case 'wav':
    case 'ogg':
    case 'flac':
    case 'aac':
    case 'wma':
    case 'alac':
    case 'amr':
    case 'dct':
    case 'au':
    case 'mmf':
    case 'aiff':
    case 'm4a':
    case 'm4p':
    case 'mpc':
    case 'msv':
    case 'opus':
    case 'ra':
    case 'rm':
    case 'tta':
    case 'voc':
      return File.Music;
    case 'mp4':
    case 'avi':
    case 'mov':
    case 'wmv':
    case 'mkv':
    case 'webm':
    case 'flv':
    case 'vob':
    case 'ogv':
    case 'drc':
    case 'gifv':
    case 'mng':
    case 'qt':
    case 'yuv':
    case 'rmvb':
    case 'asf':
    case 'amv':
    case 'm4v':
    case 'mpg':
    case 'mp2':
    case 'mpeg':
    case 'mpe':
    case 'mpv':
    case 'm2v':
    case 'svi':
    case '3gp':
    case '3g2':
    case 'mxf':
    case 'roq':
    case 'nsv':
      return File.Video;
    default:
      return File.Default;
  }
}
