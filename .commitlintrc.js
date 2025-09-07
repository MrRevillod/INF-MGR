export default {
    extends: ['@commitlint/config-conventional'],
    rules: {
        'subject-max-length': [2, 'always', 72],
        'subject-min-length': [2, 'always', 10],
    },
};