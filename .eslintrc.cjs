module.exports = {
    "ignorePatterns": ["vite-env.d.ts", "**/tests/*.ts"],
    "extends": [
        'eslint:recommended',
        'plugin:vue/vue3-recommended',
        'plugin:@typescript-eslint/recommended',
    ],
    "parser": "vue-eslint-parser",
    "parserOptions": {
        "parser": "@typescript-eslint/parser"
    },
    plugins: ['@typescript-eslint'],
}