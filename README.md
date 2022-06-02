# ux-vg

<div align="center">

# ux-vg

[![API Docs][docrs-badge]][docrs-url]
[![Crates.io][crates-badge]][crates-url]
[![Code coverage][codecov-badge]][codecov-url]
[![Tests][tests-badge]][tests-url]
[![MPL-2.0 licensed][license-badge]][license-url]
[![Gitter chat][gitter-badge]][gitter-url]
[![loc][loc-badge]][loc-url]
</div>

[docrs-badge]: https://img.shields.io/docsrs/ux-vg?style=flat-square
[docrs-url]: https://docs.rs/ux-vg/
[crates-badge]: https://img.shields.io/crates/v/ux-vg.svg?style=flat-square
[crates-url]: https://crates.io/crates/ux-vg
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square
[license-url]: https://github.com/angular-rust/ux-vg/blob/master/LICENSE
[gitter-badge]: https://img.shields.io/gitter/room/angular_rust/community.svg?style=flat-square
[gitter-url]: https://gitter.im/angular_rust/community
[tests-badge]: https://img.shields.io/github/workflow/status/angular-rust/ux-vg/Tests?label=tests&logo=github&style=flat-square
[tests-url]: https://github.com/angular-rust/ux-vg/actions/workflows/tests.yml
[codecov-badge]: https://img.shields.io/codecov/c/github/angular-rust/ux-vg?logo=codecov&style=flat-square&token=L7KV27OLY0
[codecov-url]: https://codecov.io/gh/angular-rust/ux-vg
[loc-badge]: https://img.shields.io/tokei/lines/github/angular-rust/ux-vg?style=flat-square
[loc-url]: https://github.com/angular-rust/ux-vg

__This is fork of [femtovg v0.2.8](https://crates.io/crates/femtovg/0.2.8) library__

> This fork was created so that you can implement your own renders and to integrate with the AngularRust framework.

Antialiased 2D vector drawing library written in Rust.
Ported from https://github.com/memononen/nanovg

Most of the implementation is the same as the original C code with some bug fixes, some features added and several parts have been made more Rust-y. Rendering is done via one OpenGl (ES) 3.0+ backend.

## Features
* [x] Anti-aliasing
* [x] Bézier paths filling and stroking
* [x] Solid color and image pattern fills and strokes
* [x] Gradients - box, linear and radial
* [x] Stroke width and miterlimit
* [x] Stroke caps: butt, round and square
* [x] Stroke joins: miter, round and bevel
* [x] Fill rules - EvenOdd/NonZero
* [x] Rectangle scissoring
* [x] Composition modes (SourceOver, SourceIn, SourceOut, Atop, etc..)
* [x] Global alpha
* [x] Text filling and stroking
* [x] Text shaping
* [x] Text alignment: (left center right), (top, middle, alphabetic, bottom)
* [x] Nearest font matching
* [x] Path hit testing
* [x] OpenGl ES2 backend

## In progress
* [ ] [Metal backend](https://github.com/adamnemecek/gpucanvas/)

## Not supported
* [ ] Stroke dashing
* [ ] Path scissoring
* [ ] Custom shaders
* [ ] 3D transforms
* [ ] Color fonts

## License
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](docs/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](docs/LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

Fonts used in examples:
- Roboto licensed under [Apache license](http://www.apache.org/licenses/LICENSE-2.0)
- Entypo licensed under CC BY-SA 4.0.
- Amiri licensed under [SIL Open Font License, Version 1.1](http://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL)
