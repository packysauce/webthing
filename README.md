# huh
=====
install trunk via `cargo install`
use `trunk serve` and go to the address
code away

# tailwind setup
=====
from https://tailwindcss.com/docs/installation#installing-tailwind-css-as-a-post-css-plugin
`npm install -D tailwindcss@latest postcss@latest autoprefixer@latest`

## building tailwind (shouldnt really need but once)
"""this should be in a build.rs or something"""
`npx tailwindcss --jit -i src/static/styles.css -o target/static/styles.css`

# running
`trunk serve`
