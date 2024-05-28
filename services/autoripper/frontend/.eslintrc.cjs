module.exports = {
  env: { browser: true, es2020: true, node: true },
  extends: [
    'airbnb-base',
    'airbnb-typescript/base',
    'plugin:tailwindcss/recommended',
    'plugin:react-hooks/recommended',
    'plugin:react/recommended',
    'plugin:import/recommended',
    'plugin:import/typescript',
    'plugin:jsx-a11y/recommended',
    'plugin:@typescript-eslint/recommended-type-checked',
    'plugin:prettier/recommended',
    'prettier',
  ],
  settings: {
    react: {
      version: 'detect',
    },
    'import/parsers': {
      '@typescript-eslint/parser': ['.ts', '.tsx'],
    },
    'import/resolver': {
      typescript: {
        project: ['./tsconfig.json', './tsconfig.node.json'],
      },
    },
  },
  ignorePatterns: ['dist', '.eslintrc.cjs', '*.css'],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: ['./tsconfig.json', './tsconfig.node.json'],
  },
  plugins: ['react-refresh', '@typescript-eslint', 'import'],
  rules: {
    'react-refresh/only-export-components': ['warn', { allowConstantExport: true }],
    'react/react-in-jsx-scope': 'off',

    // turn on errors for missing imports
    'import/no-unresolved': 'error',

    // Disabled for void functions as this also has problems with buttons which trigger async actions on click
    '@typescript-eslint/no-misused-promises': [
      'error',
      {
        checksVoidReturn: false,
      },
    ],

    "import/prefer-default-export": "off",

    // Imports should be grouped with spacing and should be alphabetized
    'import/order': [
      'error',
      {
        groups: ['builtin', 'external', 'internal', 'parent', 'sibling', 'index', 'object', 'type'],
        alphabetize: {
          order: 'asc',
          caseInsensitive: true,
        },
        'newlines-between': 'always',
      },
    ],

    // Dosencourage something, with is stated in the chakra ui documentation and so should be
    // disabled
    '@typescript-eslint/unbound-method': 'off',
  },
};
