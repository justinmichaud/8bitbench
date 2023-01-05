# WasmEmu

A simple 8-bit emulator targeting wasm, written in rust, and designed as a microbenchmark for wasm engines.

This is a port of a project that I wrote in my first year of uni to learn rust. As such, the code is not particularily nice or effeicient.

This should hopefully be representative of a project that was quickly ported to wasm, without any optimization effort and without any biases towards the performance characteristics of wasm engines of today.

![](/screenshot.png?raw=true)

# Try it out

[https://justinmichaud.com/8bitbench/](https://justinmichaud.com/8bitbench/)

# TODO

- Better perf stats collection
- Warmup
- Verify resulting image

# Building

```
cd rust
cargo install wasm-pack
wasm-pack build --target web --release
```

Then add `"type": "module"` to rust/pkg/package.json (to support node)

# Building the rom file

This ensures that the assembler is also availible under an appropriate licence.

```
assets/nesdoug
make
```

# Running

Worker version (from root):

```
python3 -m http.server
```

Then go to `http://localhost:8000`

Native version:

```
cd rust/
cargo run --release
```

JSC CLI:

```
jsc -m cli.mjs
```

Node:

```
node cli.mjs
```

# Results

Results on M1:

- JSC (CLI & worker): 2.3 ms/frame
- Chrome / Node: 1.5 ms/frame
- Native: 0.9 ms / frame

# Licence

```
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
```

# Licence for dependencies

- Emscripten: MIT https://github.com/emscripten-core/emsdk/blob/main/LICENSE

- PHF: MIT https://github.com/rust-phf/rust-phf

- Dyn_clone: Apache https://github.com/dtolnay/dyn-clone

- image-rs: MIT https://github.com/image-rs/image

- wasm-bindgen: MIT https://github.com/rustwasm/wasm-bindgen

- wasm-pack: MIT https://github.com/rustwasm/wasm-pack

- coi-serviceworker: MIT https://github.com/gzuidhof/coi-serviceworker

- (for cli) fast-text-encoding: Apache https://github.com/samthor/fast-text-encoding

- console_error_panic_hook: MIT https://github.com/rustwasm/console_error_panic_hook

# Licence for rom file

Assembler: Zlib https://github.com/cc65/cc65

NESDOUG: MIT https://github.com/nesdoug/26_Full_Game

# Transitive dependencies

TODO

```
emu_bench v1.0.0 (/Volumes/WebKit/8bitbench/rust)
├── console_error_panic_hook v0.1.7
│   ├── cfg-if v1.0.0
│   └── wasm-bindgen v0.2.83
│       ├── cfg-if v1.0.0
│       └── wasm-bindgen-macro v0.2.83 (proc-macro)
│           ├── quote v1.0.23
│           │   └── proc-macro2 v1.0.49
│           │       └── unicode-ident v1.0.6
│           └── wasm-bindgen-macro-support v0.2.83
│               ├── proc-macro2 v1.0.49 (*)
│               ├── quote v1.0.23 (*)
│               ├── syn v1.0.107
│               │   ├── proc-macro2 v1.0.49 (*)
│               │   ├── quote v1.0.23 (*)
│               │   └── unicode-ident v1.0.6
│               ├── wasm-bindgen-backend v0.2.83
│               │   ├── bumpalo v3.11.1
│               │   ├── log v0.4.17
│               │   │   └── cfg-if v1.0.0
│               │   ├── once_cell v1.16.0
│               │   ├── proc-macro2 v1.0.49 (*)
│               │   ├── quote v1.0.23 (*)
│               │   ├── syn v1.0.107 (*)
│               │   └── wasm-bindgen-shared v0.2.83
│               └── wasm-bindgen-shared v0.2.83
├── dyn-clone v1.0.10
├── image v0.24.5
│   ├── bytemuck v1.12.3
│   ├── byteorder v1.4.3
│   ├── color_quant v1.1.0
│   ├── exr v1.5.2
│   │   ├── bit_field v0.10.1
│   │   ├── flume v0.10.14
│   │   │   ├── futures-core v0.3.25
│   │   │   ├── futures-sink v0.3.25
│   │   │   ├── nanorand v0.7.0
│   │   │   │   └── getrandom v0.2.8
│   │   │   │       ├── cfg-if v1.0.0
│   │   │   │       └── libc v0.2.139
│   │   │   ├── pin-project v1.0.12
│   │   │   │   └── pin-project-internal v1.0.12 (proc-macro)
│   │   │   │       ├── proc-macro2 v1.0.49 (*)
│   │   │   │       ├── quote v1.0.23 (*)
│   │   │   │       └── syn v1.0.107 (*)
│   │   │   └── spin v0.9.4
│   │   │       └── lock_api v0.4.9
│   │   │           └── scopeguard v1.1.0
│   │   │           [build-dependencies]
│   │   │           └── autocfg v1.1.0
│   │   ├── half v2.1.0
│   │   ├── lebe v0.5.2
│   │   ├── miniz_oxide v0.6.2
│   │   │   └── adler v1.0.2
│   │   ├── smallvec v1.10.0
│   │   └── threadpool v1.8.1
│   │       └── num_cpus v1.15.0
│   │           └── libc v0.2.139
│   ├── gif v0.11.4
│   │   ├── color_quant v1.1.0
│   │   └── weezl v0.1.7
│   ├── jpeg-decoder v0.3.0
│   │   └── rayon v1.6.1
│   │       ├── either v1.8.0
│   │       └── rayon-core v1.10.1
│   │           ├── crossbeam-channel v0.5.6
│   │           │   ├── cfg-if v1.0.0
│   │           │   └── crossbeam-utils v0.8.14
│   │           │       └── cfg-if v1.0.0
│   │           ├── crossbeam-deque v0.8.2
│   │           │   ├── cfg-if v1.0.0
│   │           │   ├── crossbeam-epoch v0.9.13
│   │           │   │   ├── cfg-if v1.0.0
│   │           │   │   ├── crossbeam-utils v0.8.14 (*)
│   │           │   │   ├── memoffset v0.7.1
│   │           │   │   │   [build-dependencies]
│   │           │   │   │   └── autocfg v1.1.0
│   │           │   │   └── scopeguard v1.1.0
│   │           │   │   [build-dependencies]
│   │           │   │   └── autocfg v1.1.0
│   │           │   └── crossbeam-utils v0.8.14 (*)
│   │           ├── crossbeam-utils v0.8.14 (*)
│   │           └── num_cpus v1.15.0 (*)
│   ├── num-rational v0.4.1
│   │   ├── num-integer v0.1.45
│   │   │   └── num-traits v0.2.15
│   │   │       [build-dependencies]
│   │   │       └── autocfg v1.1.0
│   │   │   [build-dependencies]
│   │   │   └── autocfg v1.1.0
│   │   └── num-traits v0.2.15 (*)
│   │   [build-dependencies]
│   │   └── autocfg v1.1.0
│   ├── num-traits v0.2.15 (*)
│   ├── png v0.17.7
│   │   ├── bitflags v1.3.2
│   │   ├── crc32fast v1.3.2
│   │   │   └── cfg-if v1.0.0
│   │   ├── flate2 v1.0.25
│   │   │   ├── crc32fast v1.3.2 (*)
│   │   │   └── miniz_oxide v0.6.2 (*)
│   │   └── miniz_oxide v0.6.2 (*)
│   ├── scoped_threadpool v0.1.9
│   └── tiff v0.8.1
│       ├── flate2 v1.0.25 (*)
│       ├── jpeg-decoder v0.3.0 (*)
│       └── weezl v0.1.7
├── phf v0.11.1
│   ├── phf_macros v0.11.1 (proc-macro)
│   │   ├── phf_generator v0.11.1
│   │   │   ├── phf_shared v0.11.1
│   │   │   │   └── siphasher v0.3.10
│   │   │   └── rand v0.8.5
│   │   │       └── rand_core v0.6.4
│   │   ├── phf_shared v0.11.1 (*)
│   │   ├── proc-macro2 v1.0.49 (*)
│   │   ├── quote v1.0.23 (*)
│   │   └── syn v1.0.107 (*)
│   └── phf_shared v0.11.1 (*)
└── wasm-bindgen v0.2.83 (*)
```