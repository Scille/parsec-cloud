// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// POC: runs CryptPad's vendored x2t (see client/vendors/onlyoffice/README.md) to convert an office
// document (docx/xlsx/pptx/odt/ods/odp) into the internal binary format OnlyOffice's editor expects
// (see services/onlyoffice.ts), which is otherwise the only format we can open directly.
//
// x2t is an Emscripten build (a CLI tool compiled to wasm, driven through an in-memory filesystem
// rather than a JS-friendly API), and this mirrors CryptPad's own `www/common/outer/x2t.js` fairly
// closely: write the input file and a `params.xml` job description to `/working`, call the C
// `main1` entry point, then read the converted file back. It runs directly in the main app window
// (not sandboxed in an iframe like CryptPad does) since, unlike CryptPad, Parsec isn't hosting
// arbitrary third-party content here.
//
// Font substitution during conversion (see CryptPad's `fetchFonts`) isn't implemented: CryptPad
// itself only supplies fonts to x2t once an editor session already exists to source them from, so
// this first conversion (opening a file that was never in OnlyOffice's native format yet) doesn't
// have any to give it either. Image extraction isn't implemented yet.

interface X2TFileSystem {
  mkdir: (path: string) => void;
  writeFile: (path: string, data: Uint8Array | string) => void;
  readFile: (path: string) => Uint8Array;
}

interface X2TModule {
  FS: X2TFileSystem;
  ccall: (name: string, returnType: string, argTypes: string[], args: unknown[]) => number;
  onRuntimeInitialized?: () => void;
}

declare global {
  interface Window {
    Module?: X2TModule;
  }
}

// x2t.js's own bootstrap does `new URL(script.getAttribute('src')).search` to compute where to find
// x2t.wasm, which throws on a root-relative URL (no origin to resolve against) - needs to be
// absolute. Resolved against `document.baseURI` (the page's `<base href>`, see index.html) rather
// than `window.location.href`, since the app is routed client-side and the current path is
// unrelated to where static assets live.
const X2T_SCRIPT_URL = new URL(`${import.meta.env.BASE_URL}onlyoffice-x2t/x2t.js`, document.baseURI).href;

// Formats x2t doesn't convert to the native binary format directly: it needs to go through the
// equivalent OOXML (Microsoft) format first (mirrors CryptPad's own intermediary step).
const ODF_INTERMEDIARY_FORMAT: Record<string, string> = {
  odt: 'docx',
  ods: 'xlsx',
  odp: 'pptx',
};

let modulePromise: Promise<X2TModule> | undefined;

function loadX2T(): Promise<X2TModule> {
  if (!modulePromise) {
    modulePromise = new Promise<X2TModule>((resolve, reject) => {
      const module = (window.Module = window.Module || ({} as X2TModule));
      module.onRuntimeInitialized = (): void => {
        module.FS.mkdir('/working');
        module.FS.mkdir('/working/media');
        module.FS.mkdir('/working/fonts');
        module.FS.mkdir('/working/themes');
        resolve(module);
      };
      const script = document.createElement('script');
      script.src = X2T_SCRIPT_URL;
      script.onerror = (): void => reject(new Error(`Failed to load ${X2T_SCRIPT_URL}`));
      document.head.appendChild(script);
    });
  }
  return modulePromise;
}

// x2t's in-memory FS chokes on the same characters a real filesystem would.
// TODO: investigate if the FS actually follow UNIX rules (i.e. only `/` and `\x00` are not allowed)
function sanitizeFileName(name: string): string {
  const sanitized = name.replace(/[/\\?<>:*|"]/g, '');
  return sanitized || 'file';
}

async function runConversion(module: X2TModule, fileName: string, data: Uint8Array, outputFormat: string): Promise<Uint8Array> {
  module.FS.writeFile(`/working/${fileName}`, data);

  const outputFileName = `${fileName}.${outputFormat}`;
  const params =
    '<?xml version="1.0" encoding="utf-8"?>' +
    '<TaskQueueDataConvert xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">' +
    `<m_sFileFrom>/working/${fileName}</m_sFileFrom>` +
    '<m_sThemeDir>/working/themes</m_sThemeDir>' +
    `<m_sFileTo>/working/${outputFileName}</m_sFileTo>` +
    '<m_bIsNoBase64>false</m_bIsNoBase64>' +
    '</TaskQueueDataConvert>';
  module.FS.writeFile('/working/params.xml', params);

  module.ccall('main1', 'number', ['string'], ['/working/params.xml']);

  return module.FS.readFile(`/working/${outputFileName}`);
  // TODO: need to clean the FS ?
}

// Converts `data` (the raw content of a `.<extension>` file, e.g. `docx`) into OnlyOffice's native
// binary format.
export async function convertToNativeFormat(data: Uint8Array, fileName: string, extension: string): Promise<Uint8Array> {
  const module = await loadX2T();
  const safeName = sanitizeFileName(fileName);

  const intermediaryFormat = ODF_INTERMEDIARY_FORMAT[extension];
  if (intermediaryFormat) {
    const intermediaryData = await runConversion(module, safeName, data, intermediaryFormat);
    return runConversion(module, `${safeName}.${intermediaryFormat}`, intermediaryData, 'bin');
  }

  return runConversion(module, safeName, data, 'bin');
}
