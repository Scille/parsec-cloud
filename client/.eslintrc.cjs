const path = require('path');
const BASE_DIR = (process.cwd().endsWith(path.normalize('client'))) ? '.' : 'client';

module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    // Don't forget to also update plugin configuration in .pre-commit-config.yaml !
    'plugin:vue/vue3-strongly-recommended',
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    '@vue/typescript/recommended',
    'plugin:vue-scoped-css/vue3-recommended',
  ],
  overrides: [{
    files: ['*.ts', '*.vue'],
    rules: {
      '@typescript-eslint/explicit-function-return-type': 'error',
    },
  }],
  parser: 'vue-eslint-parser',
  parserOptions: {
    ecmaVersion: 'latest',
    project: `${BASE_DIR}/tsconfig.eslint.json`,
    extraFileExtensions: ['.vue'],
  },
  plugins: [
    'vue',
    'cypress',
    'no-relative-import-paths',
  ],
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-var': 'error',
    'semi': 'error',
    'no-useless-return': 'error',
    'no-trailing-spaces': 'error',
    'no-multiple-empty-lines': ['error', { 'max': 1, 'maxEOF': 0 }],
    'prefer-const': 'error',
    'comma-dangle': ['error', {
      'arrays': 'always-multiline',
      'objects': 'always-multiline',
      'imports': 'always-multiline',
      'exports': 'always-multiline',
      'functions': 'always-multiline',
    }],
    'indent': ['error', 2, { 'SwitchCase': 1 }],
    'camelcase': 'error',
    'max-len': ['error', 140],
    'quotes': ['error', 'single', { 'avoidEscape': true }],
    'eqeqeq': 'error',
    'dot-notation': 'error',
    'no-alert': 'error',
    'comma-spacing': 'error',
    'no-unused-vars': 'off',
    '@typescript-eslint/no-unused-vars': [
      'warn',
      {
        'argsIgnorePattern': '^_[a-z]\\S*$',
      },
    ],
    'eol-last': 'error',
    'no-useless-concat': 'error',
    'prefer-template': 'error',
    'spaced-comment': [
      'error',
      'always',
      {
        'block': { 'balanced': true },
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
    'vue/component-name-in-template-casing': ['error', 'kebab-case', {
      'registeredComponentsOnly': false,
    }],
    'vue/block-tag-newline': ['error', {
      'singleline': 'always',
      'multiline': 'always',
      'maxEmptyLines': 0,
    }],
    'vue/padding-line-between-blocks': 'error',
    'vue/no-spaces-around-equal-signs-in-attribute': 'error',
    'no-relative-import-paths/no-relative-import-paths': [
      'error', {
        'allowSameFolder': false,
        'prefix': '@',
      },
    ],
  },
};
