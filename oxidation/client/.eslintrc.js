module.exports = {
  root: true,
  env: {
    node: true
  },
  'extends': [
    // Don't forget to also update plugin configuration in .pre-commit-config.yaml !
    'plugin:vue/vue3-strongly-recommended',
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    '@vue/typescript/recommended'
  ],
  parserOptions: {
    ecmaVersion: 2020
  },
  plugins: [
    'cypress'
  ],
  rules: {
    'no-console': 'off',
    'no-debugger': 'error',
    'no-var': 'error',
    'semi': 'error',
    'no-useless-return': 'error',
    'no-trailing-spaces': 'error',
    'no-multiple-empty-lines': ['error', { 'max': 1, 'maxEOF': 0 }],
    'prefer-const': 'error',
    'comma-dangle': 'error',
    'indent': ['error', 2],
    'camelcase': 'error',
    'max-len': ['error', 140],
    'quotes': ['error', 'single', {'avoidEscape': true}],
    'eqeqeq': 'error',
    'dot-notation': 'error',
    'no-alert': 'error',
    'comma-spacing': 'error',
    'eol-last': 'error',
    'no-useless-concat': 'error',
    'prefer-template': 'error',
    'spaced-comment': [
      'error',
      'always',
      {
        'block': {'balanced': true}
      }
    ],
    'array-bracket-spacing': 'error',
    'arrow-parens': 'error',
    'arrow-spacing': 'error',
    'block-spacing': 'error',
    'brace-style': 'error',
    '@typescript-eslint/explicit-function-return-type': 'off',
    '@typescript-eslint/no-explicit-any': 'off',
    'vue/html-indent': ['error', 2],
    'vue/no-deprecated-slot-attribute': 'off'
  },
  overrides: [
    {
      files: [
        '**/__tests__/*.{j,t}s?(x)',
        '**/tests/unit/**/*.spec.{j,t}s?(x)'
      ],
      env: {
        jest: true
      }
    },
    {
      files: ['*.ts', '*.vue'],
      rules: {
        '@typescript-eslint/explicit-function-return-type': 'error'
      }
    }
  ]
};
