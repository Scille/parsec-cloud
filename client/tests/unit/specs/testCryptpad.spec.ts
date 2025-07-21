// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Cryptpad, CryptpadDocumentType, getDocumentTypeFromExtension, isEnabledCryptpadDocumentType } from '@/services/cryptpad';
import { Env } from '@/services/environment';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

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
    document.body.innerHTML = '';
    document.head.innerHTML = '';
    vi.restoreAllMocks();
  });

  describe('Constructor', () => {
    it('should create Cryptpad instance when Editics is enabled', () => {
      vi.mocked(Env.isEditicsEnabled).mockReturnValue(true);

      const cryptpad = new Cryptpad(containerElement, 'https://cryptpad.example.com');

      expect(cryptpad).toBeInstanceOf(Cryptpad);
      // Check that the script was created and configured correctly
      expect(mockScript.async).toBe(true);
      expect(mockScript.id).toBe('cryptpad-api-js');
      expect(document.createElement).toHaveBeenCalledWith('script');
    });

    it('should throw error when Editics is not enabled', () => {
      vi.mocked(Env.isEditicsEnabled).mockReturnValue(false);

      expect(() => {
        new Cryptpad(containerElement, 'https://cryptpad.example.com');
      }).toThrow('Editics is not enabled. Cannot initialize CryptPad.');
    });

    it('should reuse existing script element if already present', () => {
      const existingScript = document.createElement('script');
      existingScript.id = 'cryptpad-api-js';
      existingScript.src = 'https://existing.com/cryptpad-api.js';
      document.head.appendChild(existingScript);

      // Clear the mock since we've already called createElement
      vi.clearAllMocks();

      const cryptpad = new Cryptpad(containerElement, 'https://cryptpad.example.com');

      expect(cryptpad).toBeInstanceOf(Cryptpad);
      // Should not call createElement for script since it already exists
      expect(document.createElement).not.toHaveBeenCalledWith('script');
    });

    it('should throw error if script element creation fails', () => {
      vi.spyOn(document, 'createElement').mockImplementation((tagName: string) => {
        if (tagName === 'script') {
          return null as any;
        }
        return document.createElement(tagName);
      });

      expect(() => {
        new Cryptpad(containerElement, 'https://cryptpad.example.com');
      }).toThrow('Failed to create script element for CryptPad API.');
    });
  });

  describe('init()', () => {
    let cryptpad: Cryptpad;

    beforeEach(() => {
      cryptpad = new Cryptpad(containerElement, 'https://cryptpad.example.com');
    });

    it('should initialize successfully when script loads', async () => {
      (global as any).window.CryptPadAPI = mockCryptPadAPI;

      const initPromise = cryptpad.init();

      // Simulate successful script load
      mockScript.onload?.({} as Event);

      await initPromise;

      expect(mockElectronAPI.log).toHaveBeenCalledWith('info', 'CryptPad API script loaded successfully.');
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

      await expect(initPromise).rejects.toThrow('CryptPad API script loaded but CryptPadAPI function is not available.');
      expect(mockElectronAPI.log).toHaveBeenCalledWith('error', 'CryptPad API script loaded but CryptPadAPI function is not available.');
    });

    it('should not reinitialize if already loaded', async () => {
      (global as any).window.CryptPadAPI = mockCryptPadAPI;

      // First initialization
      const firstInitPromise = cryptpad.init();
      mockScript.onload?.({} as Event);
      await firstInitPromise;

      // Clear the log mock to test second init
      vi.clearAllMocks();

      // Second initialization should return immediately
      await cryptpad.init();

      expect(mockElectronAPI.log).not.toHaveBeenCalled();
    });
  });

  describe('open()', () => {
    let cryptpad: Cryptpad;

    beforeEach(() => {
      cryptpad = new Cryptpad(containerElement, 'https://cryptpad.example.com');
      (global as any).window.CryptPadAPI = mockCryptPadAPI;
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
        },
      };

      // Initialize first
      const initPromise = cryptpad.init();
      mockScript.onload?.({} as Event);
      await initPromise;

      await cryptpad.open(config);

      expect(mockCryptPadAPI).toHaveBeenCalledWith(containerElement.id, config);
    });

    it('should warn and return for unsupported document types', async () => {
      const consoleSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

      const config = {
        document: {
          url: 'https://example.com/doc',
          fileType: 'unsupported',
          title: 'Test Document',
        },
        documentType: CryptpadDocumentType.Unsupported,
        editorConfig: {
          lang: 'en',
        },
        autosave: 5000,
        events: {
          onSave: vi.fn(),
        },
      };

      // Initialize first
      const initPromise = cryptpad.init();
      mockScript.onload?.({} as Event);
      await initPromise;

      await cryptpad.open(config);

      expect(consoleSpy).toHaveBeenCalledWith('CryptPad edition for document type unsupported is not enabled.');
      expect(mockCryptPadAPI).not.toHaveBeenCalled();

      consoleSpy.mockRestore();
    });

    it('should warn if container element is not available', async () => {
      const consoleSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

      // Create cryptpad and initialize it first
      const cryptpad = new Cryptpad(containerElement, 'https://cryptpad.example.com');
      (global as any).window.CryptPadAPI = mockCryptPadAPI;

      // Initialize the script first
      const initPromise = cryptpad.init();
      mockScript.onload?.({} as Event);
      await initPromise;

      // Now set container to null to test the warning
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

      await cryptpad.open(config);

      expect(consoleSpy).toHaveBeenCalledWith('Container element is not initialized. Please call init() before open().');
      expect(mockCryptPadAPI).not.toHaveBeenCalled();

      consoleSpy.mockRestore();
    });
  });

  describe('getDocumentTypeFromExtension()', () => {
    function expectExtensionToGiveDocumentType(extension: string, type: CryptpadDocumentType): void {
      expect(getDocumentTypeFromExtension(extension)).toBe(type);
    }

    it('should return correct document type for text files', () => {
      expectExtensionToGiveDocumentType('txt', CryptpadDocumentType.Pad);
      expectExtensionToGiveDocumentType('TXT', CryptpadDocumentType.Pad);
      expectExtensionToGiveDocumentType('rtf', CryptpadDocumentType.Pad);
    });

    it('should return correct document type for spreadsheet files', () => {
      expectExtensionToGiveDocumentType('csv', CryptpadDocumentType.Sheet);
      expectExtensionToGiveDocumentType('xlsx', CryptpadDocumentType.Sheet);
      expectExtensionToGiveDocumentType('XLSX', CryptpadDocumentType.Sheet);
    });

    it('should return correct document type for document files', () => {
      expectExtensionToGiveDocumentType('docx', CryptpadDocumentType.Doc);
      expectExtensionToGiveDocumentType('odt', CryptpadDocumentType.Doc);
    });

    it('should return correct document type for presentation files', () => {
      expectExtensionToGiveDocumentType('pptx', CryptpadDocumentType.Presentation);
      expectExtensionToGiveDocumentType('odp', CryptpadDocumentType.Presentation);
    });

    it('should return correct document type for code files', () => {
      expectExtensionToGiveDocumentType('js', CryptpadDocumentType.Code);
      expectExtensionToGiveDocumentType('ts', CryptpadDocumentType.Code);
      expectExtensionToGiveDocumentType('py', CryptpadDocumentType.Code);
      expectExtensionToGiveDocumentType('md', CryptpadDocumentType.Code);
    });

    it('should return unsupported for unknown extensions', () => {
      expectExtensionToGiveDocumentType('unknown', CryptpadDocumentType.Unsupported);
      expectExtensionToGiveDocumentType('xyz', CryptpadDocumentType.Unsupported);
      expectExtensionToGiveDocumentType('', CryptpadDocumentType.Unsupported);
    });

    it('should handle case insensitivity', () => {
      expectExtensionToGiveDocumentType('JS', CryptpadDocumentType.Code);
      expectExtensionToGiveDocumentType('TXT', CryptpadDocumentType.Pad);
      expectExtensionToGiveDocumentType('DOCX', CryptpadDocumentType.Doc);
      expectExtensionToGiveDocumentType('PY', CryptpadDocumentType.Code);
    });
  });

  describe('isEnabledCryptpadDocumentType()', () => {
    function expectDocumentTypeToBeEnabled(extension: string, expected: boolean): void {
      expect(isEnabledCryptpadDocumentType(extension)).toBe(expected);
    }

    it('should return true for enabled document types', () => {
      expectDocumentTypeToBeEnabled('txt', true);
      expectDocumentTypeToBeEnabled('csv', true);
      expectDocumentTypeToBeEnabled('docx', true);
      expectDocumentTypeToBeEnabled('pptx', true);
      expectDocumentTypeToBeEnabled('js', true);
    });

    it('should return false for unsupported document types', () => {
      expectDocumentTypeToBeEnabled('unknown', false);
      expectDocumentTypeToBeEnabled('xyz', false);
      expectDocumentTypeToBeEnabled('', false);
    });

    it('should handle case insensitivity', () => {
      expectDocumentTypeToBeEnabled('TXT', true);
      expectDocumentTypeToBeEnabled('UNKNOWN', false);
    });
  });
});
