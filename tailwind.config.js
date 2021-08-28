module.exports = {
  mode: 'jit',
  purge: {
    content: [
      'index.html',
      'src/**/*.rs',
    ],
    options: {
      safelist: [
        /data-theme$/,
      ]
    }
  },
  darkMode: 'media', // false or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
}
