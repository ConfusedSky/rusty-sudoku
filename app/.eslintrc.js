module.exports = {
  env: {
    browser: true,
    es2020: true,
    node: true,
  },
  extends: [
    "plugin:react/recommended",
    "airbnb",
  ],
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: 11,
    sourceType: "module",
  },
  plugins: [
    "react",
  ],
  rules: {
    quotes: ["error", "double"],
    "no-console": 0,
  },
  settings: {
    "import/resolver": {
      node: {
        paths: ["js"],
      },
    },
  },
};
