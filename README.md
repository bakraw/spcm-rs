# SPCM-rs (WIP)

> **NOT AFFILIATED WITH SPECTRUM INSTRUMENTATION GMBH**. THIS IS NOT OFFICIALLY SUPPORTED SOFTWARE FOR SPECTRUM'S PRODUCTS. LICENSED UNDER MIT.

Bindgen'd Rust bindings for the Spectrum M5i digitizer cards' C SDK (haven't tested but probably also older models I guess).

Also includes (in ``utils.rs``) a few low-abstraction util functions that were useful to me when programming the cards, accessible from the ``utils`` module :
- 4096-aligned ``PageAlignedBuffer<N>([u8;N])`` struct for easy page aligned byte buffer allocation ;
- ``.as_mut_void_ptr()`` method to get a ``void*`` to that buffer ;
- ``.as_i16_samples()`` and ``.as_i8_samples()`` methods to get a slice of 2-bytes unpacked or 1-byte packed samples from that buffer ;
- ``get_error(drv_handle) -> Option<String>`` function that reads and returns the error string of a card if there is one.
  
Those utils try to limit ``unsafe`` blocks to individual calls to the SDK's functions and strictly necessary pointer manipulation. Also they use standard Rust naming convention, and not the SDK's (no shot you'll ever catch me using systems hungarian notation).

![](https://github.com/user-attachments/assets/2a07e7ab-087b-4aed-b4c0-c42f8846bf6e)

## Status

Still in use, will change without notice as I find the need to add new features.

## Requirements

I don't think I'd be legally allowed to redistribute the actual SDK in this repo and I don't want Spectrum's legal dept to send a drone strike on me, so the files required to build the bindings aren't included. Similarly I won't risk including the built ``bindings.rs`` file itself, so you'll have to build it. You'll need :

- [Rust toolchain](https://rust-lang.org/tools/install/)
- [clang/libclang](https://github.com/llvm/llvm-project/releases/)
- [Spectrum's C SDK](https://spectrum-instrumentation.com/support/downloads.php) (download ``c_header`` then copy-paste all the files in ``src/C_SDK``)

## Installation

I've only ever targeted Windows for this project so I can only give info on that. Look at ``dlltyp.h`` in the SDK to see the ``#ifdef`` differences if targeting another OS.

### Windows -> Windows

Because of macro / typedef shenanigans in the SDK, you have to build using the GCC toolchain rather than MSVC, even if building from Windows.
Add the build target :
```bash
rustup target add x86_64-pc-windows-gnu
```
Then build with :
```bash
cargo build --target=x86_64-pc-windows-gnu --release
```
Bindings are generated during the build process and written to Cargo's build output directory.

### Linux -> Windows

**The building process might be more complex**, depending on your source and target. In my case, I was
cross-compiling from Linux to Windows, so I needed to do a few things :
1. Snatch ``spcm_win64.dll`` from the ``System32`` directory of the Windows server where the cards and their drivers are installed.
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

## Usage

Use like any other local Rust crate :
1. Add to your project's ``Cargo.toml`` :
	```
	[dependencies]
	spcm-rs = {path="../spcm-rs"}
	```

2. Call from your Rust code, e.g.:
   ```rust
   use spcm_rs as spcm;

   fn main() {
	let card_handle: spcm::drv_handle = unsafe{spcm::spcm_hOpen(c"/dev/spcm0".as_ptr())};
	let mut buffer = Box::new(spcm::utils::PageAlignedBuffer([0; /*size*/ * 2]));
	let buffer_ptr: *mut raw::c_void = buffer.as_mut_void_ptr();

	// set up card...

	unsafe{spcm::spcm_dwSetParam_i32(card_handle, spcm::SPC_M2CMD, spcm::M2CMD_CARD_START | spcm::M2CMD_CARD_ENABLETRIGGER | spcm::M2CMD_CARD_WAITREADY);}
	
	match spcm::utils::get_error(card_handle) {
		Some(err) => {
			println!("{}\n", err);
			process::exit(1);
		}
		None => print!("No error.\n")
	};

	// acquire data...

	unsafe{spcm::spcm_vClose(card_handle);}

	let samples: &[i16] = buffer.as_i16_samples();
	// read samples...
   }
   ```
   > I set up bindgen to use i32 as the default type for converted macros, as that's the type most frequently expected by SPCM's functions. Casting can still be needed occasionally.