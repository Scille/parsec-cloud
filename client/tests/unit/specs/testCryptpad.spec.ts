// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FileContentType } from '@/common/fileTypes';
import { Cryptpad, CryptpadDocumentType, getCryptpadDocumentType, isEnabledCryptpadDocumentType } from '@/services/cryptpad';
import { Env } from '@/services/environment';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

const SERVER_URL = 'serverurl';

// Mocks
vi.mock('@/services/environment', () => ({
  Env: {
    isEditicsEnabled: vi.fn(),
  },
}));

const mockElectronAPI = {
  log: vi.fn(),
};
const mockCryptPadAPI = vi.fn() as any;

describe('CryptPad Service', () => {
  let containerElement: HTMLElement;
  let mockScript: HTMLScriptElement;

  beforeEach(() => {
    // Reset all mocks
    vi.clearAllMocks();

    // Setup DOM environment
    document.head.innerHTML = '';
    containerElement = document.createElement('div');
    containerElement.id = 'cryptpad-container';
    document.body.appendChild(containerElement);

    // Setup window mocks
    (global as any).window = {
      ...global.window,
      electronAPI: mockElectronAPI,
      CryptPadAPI: undefined,
    };

    // Mock createElement to return a controllable script element
    mockScript = document.createElement('script');
    // Prevent actual script loading by overriding the src setter
    Object.defineProperty(mockScript, 'src', {
      set: vi.fn(),
      get: vi.fn(() => ''),
      configurable: true,
    });

    vi.spyOn(document, 'createElement').mockImplementation((tagName: string) => {
      if (tagName === 'script') {
        return mockScript;
      }
      return document.createElement(tagName);
    });

    // Mock Env.isEditicsEnabled to return true by default
    vi.mocked(Env.isEditicsEnabled).mockReturnValue(true);
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('init()', () => {
    let cryptpad: Cryptpad;

    beforeEach(() => {
      cryptpad = new Cryptpad(containerElement, SERVER_URL);
    });

    afterEach(() => {
      (global as any).window.CryptPadAPI = undefined;
    });

    it('should throw error when Editics is not enabled', async () => {
      vi.mocked(Env.isEditicsEnabled).mockReturnValue(false);

      await expect(cryptpad.init()).rejects.toThrow('Failed to initialize CryptPad: not-enabled');
    });

    it('should throw error if script element creation fails', async () => {
      vi.spyOn(document, 'createElement').mockImplementation((tagName: string) => {
        if (tagName === 'script') {
          return null as any;
        }
        return document.createElement(tagName);
      });

      await expect(cryptpad.init()).rejects.toThrow('Failed to initialize CryptPad: script-element-creation-failed');
    });

    it('should initialize successfully when script loads', async () => {
      const initPromise = cryptpad.init();
      (global as any).window.CryptPadAPI = mockCryptPadAPI;
      mockScript.onload?.({} as Event);
      await initPromise;

      expect(mockElectronAPI.log).toHaveBeenCalledWith('info', 'CryptPad API script loaded successfully.');
    });

    it('should reuse global script if new instance', async () => {
      const initPromise = cryptpad.init();
      (global as any).window.CryptPadAPI = mockCryptPadAPI;
      mockScript.onload?.({} as Event);
      await initPromise;

      // Clear the log mock to test second init
      vi.clearAllMocks();

      const cryptpad2 = new Cryptpad(containerElement, SERVER_URL);
      const initPromise2 = cryptpad2.init();
      mockScript.onload?.({} as Event);
      await initPromise2;

      expect(mockElectronAPI.log).toHaveBeenCalledWith('info', 'CryptPad API script previously loaded, reusing');
    });

    it('should handle script loading errors', async () => {
      const initPromise = cryptpad.init();

      // Simulate script error
      mockScript.onerror?.('Script load failed');

      await expect(initPromise).rejects.toThrow();
      expect(mockElectronAPI.log).toHaveBeenCalledWith('error', 'Failed to load CryptPad script: Script load failed');
      expect(mockElectronAPI.log).toHaveBeenCalledWith(
        'error',
        'This might be due to HTTPS requirements. Check if CryptPad server requires secure context.',
      );
    });

    it('should handle script loading when CryptPadAPI is not available', async () => {
      // Don't set CryptPadAPI on window to simulate it not being available
      (global as any).window.CryptPadAPI = undefined;

      const initPromise = cryptpad.init();

      // Simulate script load but without CryptPadAPI being available
      mockScript.onload?.({} as Event);

      await expect(initPromise).rejects.toThrow('Cryptpad error: init-failed');
      expect(mockElectronAPI.log).toHaveBeenCalledWith('error', 'CryptPad API script loaded but CryptPadAPI function is not available.');
    });

    it('should not reinitialize if already loaded', async () => {
      (global as any).window.CryptPadAPI = mockCryptPadAPI;

      const initPromise = cryptpad.init();
      (global as any).window.CryptPadAPI = mockCryptPadAPI;
      mockScript.onload?.({} as Event);
      await initPromise;

      // Clear the log mock to test second init
      vi.clearAllMocks();

      // Second initialization should return immediately
      await cryptpad.init();

      expect(mockElectronAPI.log).toHaveBeenCalledWith('info', 'CryptPad API script already loaded.');
    });
  });

  describe('open()', () => {
    let cryptpad: Cryptpad;

    beforeEach(async () => {
      cryptpad = new Cryptpad(containerElement, SERVER_URL);
      const initPromise = cryptpad.init();
      (global as any).window.CryptPadAPI = mockCryptPadAPI;
      mockScript.onload?.({} as Event);
      await initPromise;
    });

    it('should open document with valid config', async () => {
      const config = {
        document: {
          url: 'https://example.com/doc',
          fileType: 'txt',
          title: 'Test Document',
          key: 'test-key',
        },
        documentType: CryptpadDocumentType.Pad,
        editorConfig: {
          lang: 'en',
        },
        autosave: 5000,
        events: {
          onSave: vi.fn(),
          onReady: vi.fn(),
        },
      };

      // Mock CryptPadAPI to call onReady immediately
      mockCryptPadAPI.mockImplementation((_containerId: string, cfg: any) => {
        if (cfg.events.onReady) {
          cfg.events.onReady();
        }
      });

      await cryptpad.open(config);

      expect(mockCryptPadAPI).toHaveBeenCalledWith(
        containerElement.id,
        expect.objectContaining({
          document: config.document,
          documentType: config.documentType,
          editorConfig: config.editorConfig,
          autosave: config.autosave,
        }),
      );
    });

    it('should throw error for unsupported document types', async () => {
      const config = {
        document: {
          url: 'https://example.com/doc',
          fileType: 'txt',
          title: 'Test Document',
        },
        documentType: 'unsupported' as CryptpadDocumentType,
        editorConfig: {
          lang: 'en',
        },
        autosave: 5000,
        events: {
          onSave: vi.fn(),
        },
      };

      await expect(cryptpad.open(config)).rejects.toThrow(
        "Failed to open document type 'unsupported' with Cryptpad: document-type-not-enabled",
      );
      expect(mockCryptPadAPI).not.toHaveBeenCalled();
    });

    it('should throw error if container element is not available', async () => {
      // Now set container to null to test the error
      (cryptpad as any).containerElement = null;

      const config = {
        document: {
          url: 'https://example.com/doc',
          fileType: 'txt',
          title: 'Test Document',
        },
        documentType: CryptpadDocumentType.Pad,
        editorConfig: {
          lang: 'en',
        },
        autosave: 5000,
        events: {
          onSave: vi.fn(),
        },
      };

      await expect(cryptpad.open(config)).rejects.toThrow("Failed to open document type 'pad' with Cryptpad: not-initialized");
      expect(mockElectronAPI.log).toHaveBeenCalledWith('error', 'Container element is not initialized. Please call init() before open().');
      expect(mockCryptPadAPI).not.toHaveBeenCalled();
    });
  });

  describe('getCryptpadDocumentType()', () => {
    function expectExtensionToGiveDocumentType(filetype: FileContentType, type: CryptpadDocumentType): void {
      expect(getCryptpadDocumentType(filetype)).toBe(type);
    }

    it('should return correct document types for each supported file content type', () => {
      expectExtensionToGiveDocumentType(FileContentType.Text, CryptpadDocumentType.Code);
      expectExtensionToGiveDocumentType(FileContentType.Document, CryptpadDocumentType.Doc);
      expectExtensionToGiveDocumentType(FileContentType.Spreadsheet, CryptpadDocumentType.Sheet);
    });

    it('should return unsupported type for each unsupported file content type', () => {
      expectExtensionToGiveDocumentType(FileContentType.Audio, CryptpadDocumentType.Unsupported);
      expectExtensionToGiveDocumentType(FileContentType.Video, CryptpadDocumentType.Unsupported);
      expectExtensionToGiveDocumentType(FileContentType.Image, CryptpadDocumentType.Unsupported);
      expectExtensionToGiveDocumentType(FileContentType.PdfDocument, CryptpadDocumentType.Unsupported);
      expectExtensionToGiveDocumentType(FileContentType.Unknown, CryptpadDocumentType.Unsupported);
    });
  });

  describe('isEnabledCryptpadDocumentType()', () => {
    function expectDocumentTypeToBeEnabled(fileType: FileContentType, expected: boolean): void {
      expect(isEnabledCryptpadDocumentType(fileType)).toBe(expected);
    }

    it('should return true for enabled document types', () => {
      expectDocumentTypeToBeEnabled(FileContentType.Text, true);
      expectDocumentTypeToBeEnabled(FileContentType.Document, true);
      expectDocumentTypeToBeEnabled(FileContentType.Spreadsheet, true);
    });

    it('should return false for unsupported document types', () => {
      expectDocumentTypeToBeEnabled(FileContentType.Audio, false);
      expectDocumentTypeToBeEnabled(FileContentType.Video, false);
      expectDocumentTypeToBeEnabled(FileContentType.Image, false);
      expectDocumentTypeToBeEnabled(FileContentType.PdfDocument, false);
      expectDocumentTypeToBeEnabled(FileContentType.Unknown, false);
    });
  });
});
