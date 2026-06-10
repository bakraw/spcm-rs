# SPCM-rs (WIP)

> **NOT AFFILIATED WITH SPECTRUM INSTRUMENTATION GMBH**. THIS IS NOT OFFICIALLY SUPPORTED SOFTWARE FOR SPECTRUM'S PRODUCTS. USE AT YOUR OWN RISK AND LIABILITY (MIT LICENSE).

Bindgen'd Rust bindings for the Spectrum M5i digitizer cards' C SDK (probably also older models I guess).

Also includes (in ``lib.rs``) a few util functions that were useful to me when programming the cards (page aligned alloc, converting that buffer to a ``void*``, reading ``char*`` errors to an ``&str``, etc.). Tried to keep them as idiomatic / clean / simple as possible.

Those utils use no ``unsafe`` blocks apart from individual calls to the SDK's functions and strictly necessary things (in which case only objects directly returned by the SDK are mannipulated), e.g. ``&str`` from ``*mut i8``. Also they use standard Rust naming convention, as opposed to the SDK's functions (no shot you'll ever catch me using systems hungarian notation).

### Installation

I don't think I'd be legally allowed to redistribute the actual SDK in this repo and I don't want Spectrum's legal dept to send a hit squad on me, so the files required to build the bindings aren't included. You can grab them from [Spectrum's website](https://spectrum-instrumentation.com/support/downloads.php), then copy-paste them in ``src/C_SDK``. Similarly I won't risk including the built ``bindings.rs`` file itself, so you'll have to build it.

```bash
cargo build --release
```
The generated ``bindings.rs`` will be somewhere in ``target/release/build/``.


**The building process might be more complex**, depending on your source and target. In my case, I was
cross-compiling from Linux to Windows, so I needed to do a few things :
1. Snatch ``spcm_win64.dll`` from the ``System32`` directory of the Windows server were the cards and their drivers are installed.
2. Generate the required files from that, for use by the GCC toolchain :
   ```bash
   gendef spcm_win64.dll
   ```
   Then :
   ```bash
   x86_64-w64-mingw32-dlltool -d spcm_win64.def -l libspcm.a
   ```
3.	Install the relevant build target :
	```bash
	rustup target add x86_64-pc-windows-gnu
	```
	Then build with :
	```bash
	cargo build --target=x86_64-pc-windows-gnu --release
	```

### Usage

Use like any other local Rust crate :
1. Add to your project's ``Cargo.toml`` :
	```
	[dependencies]
	spcm-rs = {path="../spcm-rs"}
	```

2. Call from your Rust code:
   ```rust
   use spcm_rs as spcm;

   fn main() {
	let card_handle: spcm::drv_handle;
	card_handle = unsafe{spcm::spcm_hOpen(c"/dev/spcm0".as_ptr())}

	// set up card...

	unsafe{spcm::spcm_dwSetParam_i32(card_handle, spcm::SPC_M2CMD as i32, spcm::M2CMD_CARD_START as i32 | spcm::M2CMD_CARD_ENABLETRIGGER as i32 | spcm::M2CMD_CARD_WAITREADY as i32);}

	// do something...

	unsafe{spcm::spcm_vClose(card_handle);}
   }
   ```
   > As you can see, bindgen struggles to correctly infer the proper type for C macros (most of them become ``u32`` by default and need to be cast for use). [TODO: config build.rs to avoid that if possible.]  