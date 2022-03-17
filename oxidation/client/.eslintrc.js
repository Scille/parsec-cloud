module.exports = {
  root: true,
  env: {
    node: true
  },
  'extends': [
    // Don't forget to also update plugin configuration in .pre-commit-config.yaml !
    'plugin:vue/vue3-essential',
    'eslint:recommended',
    '@vue/typescript/recommended'
  ],
  parserOptions: {
    ecmaVersion: 2020
  },
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'vue/no-deprecated-slot-attribute': 'off',
    '@typescript-eslint/no-explicit-any': 'off',
    'no-var': 'error',
    'semi': 'error',
    'no-useless-return': 'error',
    'no-trailing-spaces': 'error',
    'no-multiple-empty-lines': ['error', { 'max': 1, 'maxEOF': 0 }],
    'prefer-const': 'error',
    'comma-dangle': 'error',
    'indent': ['error', 2],
    'vue/html-indent': ['error', 2],
    'camelcase': 'error',
    'max-len': ['error', 140],
    'quotes': ['error', 'single', {'avoidEscape': true}],
    'eqeqeq': 'error',
    '@typescript-eslint/explicit-function-return-type': 'off'
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
