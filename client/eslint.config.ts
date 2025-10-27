// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import noRelativeImportPaths from 'eslint-plugin-no-relative-import-paths';
import pluginVue from 'eslint-plugin-vue';
import { defineConfig } from 'eslint/config';
import globals from 'globals';
import tseslint from 'typescript-eslint';
import vueParser from 'vue-eslint-parser';

export default defineConfig([
  ...pluginVue.configs['flat/recommended'],
  {
    ignores: [
      'pkg/**',
      'dist/**',
      'node_modules/**',
      'electron/build/**',
      'electron/app/**',
      'electron/src/**',
      'electron/dist/**',
      'electron/live-runner.js',
      'electron/node_modules/**',
      'src/views/testing/**',
      'bindings/**',
      'playwright.config.ts',
      'merge-playwright.ts',
      'src/parsec/mock_files/**',
      'scripts/vite_plugin_wasm_pack.ts',
      'src/vite-env.d.ts',
    ],
  },
  {
    files: ['**/*.ts'],
    languageOptions: {
      parser: tseslint.parser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
    plugins: {
      '@typescript-eslint': tseslint.plugin,
      'no-relative-import-paths': noRelativeImportPaths,
    },
  },
  {
    files: ['**/*.vue'],
    languageOptions: {
      parser: vueParser,
      parserOptions: {
        parser: tseslint.parser,
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },
  {
    plugins: {
      '@typescript-eslint': tseslint.plugin,
      'no-relative-import-paths': noRelativeImportPaths,
    },
  },
  {
    rules: {
      'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
      'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
      'no-var': 'error',
      semi: 'error',
      'no-useless-return': 'error',
      'no-trailing-spaces': 'error',
      'no-multiple-empty-lines': ['error', { max: 1, maxEOF: 0 }],
      'prefer-const': 'error',
      'comma-dangle': [
        'error',
        {
          arrays: 'always-multiline',
          objects: 'always-multiline',
          imports: 'always-multiline',
          exports: 'always-multiline',
          functions: 'always-multiline',
        },
      ],
      indent: ['error', 2, { SwitchCase: 1 }],
      camelcase: 'error',
      'max-len': ['error', 140],
      quotes: ['error', 'single', { avoidEscape: true }],
      eqeqeq: 'error',
      'dot-notation': 'error',
      'no-alert': 'error',
      'comma-spacing': 'error',
      'no-unused-vars': 'off',
      '@typescript-eslint/no-unused-vars': [
        'warn',
        {
          args: 'all',
          argsIgnorePattern: '^_',
          caughtErrors: 'all',
          caughtErrorsIgnorePattern: '^_',
          destructuredArrayIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          ignoreRestSiblings: true,
        },
      ],
      'eol-last': 'error',
      'no-useless-concat': 'error',
      'prefer-template': 'error',
      'spaced-comment': [
        'error',
        'always',
        {
          block: { balanced: true },
        },
      ],
      'array-bracket-spacing': 'error',
      'arrow-parens': 'error',
      'arrow-spacing': 'error',
      'block-spacing': 'error',
      'brace-style': 'error',
      '@typescript-eslint/explicit-function-return-type': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      'vue/html-indent': ['error', 2],
      'vue/no-deprecated-slot-attribute': 'off',
      'vue/component-name-in-template-casing': [
        'error',
        'kebab-case',
        {
          registeredComponentsOnly: false,
        },
      ],
      'vue/block-tag-newline': [
        'error',
        {
          singleline: 'always',
          multiline: 'always',
          maxEmptyLines: 0,
        },
      ],
      'vue/attributes-order': 'off',
      'vue/padding-line-between-blocks': 'error',
      'vue/no-spaces-around-equal-signs-in-attribute': 'error',
      'vue/html-self-closing': [
        'error',
        {
          html: {
            void: 'always',
            normal: 'always',
            component: 'always',
          },
        },
      ],
      'vue/singleline-html-element-content-newline': 'off',
      'no-relative-import-paths/no-relative-import-paths': [
        'error',
        {
          allowSameFolder: false,
          prefix: '@',
        },
      ],
    },
  },
]);
