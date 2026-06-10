# SPCM-rs (WIP)

> **NOT AFFILIATED WITH SPECTRUM INSTRUMENTATION GMBH**. THIS IS NOT OFFICIALLY SUPPORTED SOFTWARE FOR SPECTRUM'S PRODUCTS. USE AT YOUR OWN RISK AND LIABILITY.

Bindgen'd Rust bindings for the Spectrum M5i digitizer cards' C SDK (probably also older models I guess).

Also includes (in *lib.rs*) a few util functions that were useful to me when programming the cards (page aligned alloc, converting that buffer to a ``void*``, reading ``char*`` errors to an ``&str``, etc.). Tried to keep them as idiomatic / clean / simple as possible.

Those utils use no ``unsafe`` blocks apart from individual calls to the SDK's functions and strictly necessary things (in which case only objects directly returned by the SDK are mannipulated), e.g. ``&str`` from ``*mut i8``. Also they use standard Rust naming convention, as opposed to the SDK's functions (no shot you'll ever catch me using systems hungarian notation).

### Installation

I don't think I'd be legally allowed to redistribute the actual SDK in this repo and I don't want Spectrum's legal dept to send a hit squad on me, so the files required to build the bindings aren't included. You can grab them from [Spectrum's website](https://spectrum-instrumentation.com/support/downloads.php), then copy-paste them in *src/C_SDK*. Similarly I won't risk including the built *bindings.rs* file itself, so you'll have to build it.

```bash
cargo build
```

The building process will depend on your source and target. In my case, I was
cross-compiling from Linux to Windows, 