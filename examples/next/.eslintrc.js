module.exports = {
  'root': true,
  'parser': '@typescript-eslint/parser',
  'plugins': [
    '@typescript-eslint',
    'react-hooks',
  ],
  'extends': [
    'eslint:recommended',
    'plugin:@typescript-eslint/eslint-recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:import/errors',
    'plugin:import/warnings',
  ],
  'settings': {
    'import/resolver': {
      'node': {
        'moduleDirectory': ['node_modules', './'],
        'extensions': ['.js', '.jsx', '.ts', '.tsx'],
      },
    },
  },
  'rules': {
    'no-multi-spaces': 'error',
    'no-multiple-empty-lines': ['error', { 'max': 1 }],
    'func-call-spacing': ['error', 'never'],
    'no-unneeded-ternary': 'error',
    'quotes': ['error', 'single'],
    'object-curly-spacing': ['error', 'always'],
    'array-bracket-spacing': ['error', 'never'],
    'space-infix-ops': 'error',
    '@typescript-eslint/type-annotation-spacing': ['error', {
      'before': false,
      'after': true,
      'overrides': { 'arrow': { 'before': true, 'after': true } },
    }],
    'arrow-spacing': ['error', { 'before': true, 'after': true }],
    'key-spacing': ['error', { 'beforeColon': false, 'afterColon': true }],

    'no-var': 'error',
    'no-unused-vars': 'warn',
    '@typescript-eslint/no-unused-vars': 'warn',
    '@typescript-eslint/no-unused-expressions': 'warn',

    'indent': ['error', 2],
    '@typescript-eslint/indent': ['error', 2],
    '@typescript-eslint/ban-ts-comment': 0,

    // semi
    // 'semi': ['error', 'always'],
    'semi': 'off',
    '@typescript-eslint/semi': ['error'],
    'semi-spacing': ['error', { 'before': false, 'after': true }],
    'semi-style': ['error', 'last'],
    'no-extra-semi': 'error',
    'no-unexpected-multiline': 'error',
    'no-unreachable': 'error',
    'max-len': ['warn', { 'code': 100 }],

    'linebreak-style': ['error', 'unix'],
    'eol-last': 'error',
    'no-trailing-spaces': 'error',
    'space-in-parens': ['error', 'never'],
    'no-console': 'off',
    'comma-dangle': ['error', 'always-multiline'],
    'comma-spacing': 'error',
    'computed-property-spacing': 'error',
    'keyword-spacing': 'error',

    'sort-imports': 'off',
    'import/order': ['error', {
      'newlines-between': 'always',
    }],
  },
};
