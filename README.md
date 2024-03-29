# Setup

src: https://rustwasm.github.io/docs/book/game-of-life/setup.html

Install Rust toolchain:

- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
- source $HOME/.cargo/env
- test if installed correctly: rustc --version

Install wasm-pack for Rust generated WebAssembly:

- https://rustwasm.github.io/wasm-pack/installer/
- curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 

Install cargo-generate to generate a new Rust project:

- cargo install cargo-generate
- this takes like 10 minutes

Switch to latest Node:

- nvm use 15

Generate a project from a repo:

- cargo generate --git https://github.com/rustwasm/wasm-pack-template
- name: nidoran
- remove or update email address in generated Cargo.toml
- cd nidoran/nidoran
- nvm use 15
- npm init wasm-app www
- in www: npm i

Add Crates/dependencies:

- when adding a crate with e.g. `extern crate web_sys;`
- On crates.io find the crate: https://crates.io/crates/web-sys
- This lists the instruction: Add the following line to your Cargo.toml file: `web-sys = "0.3.50"`
- in nidoran/nidoran, run, `cargo check`
- Rebuild with `wasm-pack build`

Build:

- in nidoran/nidoran/www: `wasm-pack build` (can also be run in nidoran/nidoran, but in www is easier to switch between this and npm start)
- in nidoran/nidoran/www: `npm start`

# TODO

- TypeScript instead of JS!
- DONE: Remove name/email from generated pkg/package.json (comes from Cargo.toml)

=====================

<div align="center">

  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
