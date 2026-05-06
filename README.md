# iloader-next

A combined web and desktop app for sideloading iOS apps. The next generation of iloader.

## Architecture

To make maintainability easier (though it may actually make it harder...) this repo is a single codebase that serves both the web and desktop versions of the app.

Layout:

```
- frontend
  - src # React frontend code
  - src-tauri # Tauri desktop app code, exposes tauri commands for iloader-core
- rust
  - iloader-core # Core rust code powering both apps
  - server # Proxy server for the web app
  - wasm # WASM bindings for iloader-core
```

### Development

For your convenience, a `dev` script is available to handle compiling the wasm, running the Tauri desktop app, and starting the web server. It will watch for changes and automatically rebuild as needed. It's very janky though.

This guide assumes you have [bun](https://bun.sh) and [rust](https://rust-lang.org/learn/get-started/) installed.

Install dependencies:
```
bun i && cd frontend && bun i && cd ..
```
Install wasm-pack:
```
cargo install wasm-pack
```
Install Tauri CLI:
```
cargo install tauri-cli
```

Start dev server:
```
RUSTFLAGS="--cfg=web_sys_unstable_apis" bun dev
```
You can also add the rustflags to your global cargo config to avoid having to specify it manually.
