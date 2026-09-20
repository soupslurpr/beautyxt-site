# BeauTyXT website

Source for <https://beautyxt.app>. A dependency-free Rust program generates a
static site in `docs/`, the existing GitHub Pages publishing directory.
The site uses no JavaScript, external fonts, analytics, or remotely embedded
assets. Native HTML disclosures work without scripting; color scheme follows
the browser's system preference.

## Preview locally

```sh
cargo run --offline
python -m http.server 4173 --bind 127.0.0.1 --directory docs
```

Open <http://127.0.0.1:4173>. Serve only `docs/`, not the repository root.
The preview command does not publish anything. GitHub Pages serves `docs/`
from `main`, the default branch. Pushing to `main` publishes the generated
site, so review and verify changes locally first.

## Edit and verify

Edit page content in `src/pages/`, the shared layout in `src/layout.html`, and
styles in `src/main.css`. Run `cargo run --offline` to regenerate the published
HTML, CSS, sitemap, and license. Static assets live directly in `docs/`.

```sh
cargo fmt --check
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings
cargo run --offline -- --check
```

`--check` exits unsuccessfully when a generated file is missing or differs from
its source. It never writes files. Check narrow/mobile and wide layouts in both
color schemes, keyboard navigation, expanded disclosures, and local links
before publishing. The production app's README and architecture documents are
the source of truth for feature and privacy claims.

Screenshots show release 84 in an Android 17 emulator using the sample document
in `docs/samples/`. They are lossless WebP captures of the actual app with a
normalized status bar, not rendered mockups. The 1080 × 2424 and 2160 × 4848
versions are separate native captures; the larger versions double emulator
resolution and density together, without upscaling the smaller images.
Responsive image markup selects the resolution and color scheme. Keep its
`sizes` values aligned with the layout breakpoints in `src/main.css`.
The icon uses the released app's artwork and foreground scale. Local review
screenshots and temporary tooling belong in ignored `captures/`, not the
published directory.

The website is licensed under the [MIT License](LICENSE). The generator
publishes a copy at `/LICENSE`.
