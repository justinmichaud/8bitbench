# WasmEmu

A simple 8-bit emulator targeting wasm, written is rust.

# Building

```
cd rust
cargo install wasm-pack
wasm-pack build --target web
```

# Running

From root:

```
npm install http-server -g
http-server -p 8000 --cors
```

Then go to `http://localhost:8000`

# Licence

MIT License

Copyright (c) 2023 Justin Michaud

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

# Licence for dependencies

- Emscripten: MIT https://github.com/emscripten-core/emsdk/blob/main/LICENSE

- PHF: MIT https://github.com/rust-phf/rust-phf

- Dyn_clone: Apache https://github.com/dtolnay/dyn-clone

- image-rs: MIT https://github.com/image-rs/image

- wasm-bindgen: MIT https://github.com/rustwasm/wasm-bindgen

- wasm-pack: MIT https://github.com/rustwasm/wasm-pack

# Licence for rom file

TODO on this one