use {crate::core,
	 std::{ffi, os::raw, ptr, slice}};

/// * 4096-aligned buffer to use for data transfers.
/// * ``N`` is the size of the buffer in bytes.
/// * Sadly align() has to be hardcoded so I settled for the standard 4 KiB page size,
///		modify that number if a different page size is used.
/// * Instantiate on the heap with:
///		```rust 
/// 		let mut buf = Box::new(PageAlignedBuffer</*size*/>(0, /*size*/));
/// 	``` 
#[repr(align(4096))]
pub struct PageAlignedBuffer<const N: usize>(pub [u8;N]);

impl<const N: usize> PageAlignedBuffer<N> {
	/// * Use to read the buffer if the card is in 16-bits unpacked samples mode.
	/// * Negligible performance overhead (effectively just constructs a fat pointer).
	///	* O(1).
	/// * **If N is odd, the last byte will be ignored** (which is most likely
	/// 	not what you want).
	pub fn as_i16_samples(&self) -> &[i16] {
		// Unsafe block, but casting a pair of u8 to a i16 is memory-safe if the buffer
		// has proper 2-bytes alignment (which it has in our case, as it's page-aligned).
		// Even if N % 2 != 0, N / 2 will round down, such that the last u8 will get ignored.
		unsafe {
			slice::from_raw_parts(self.0.as_ptr() as *const i16, N/2)
		}
	}

	/// * Use to read the buffer if the card is in 8-bits packed samples mode.
	/// * Negligible performance overhead (effectively just constructs a fat pointer).
	/// * O(1).
	pub fn as_i8_samples(&self) -> &[i8] {
		// Unsafe block, but casting a u8 to a i8 is memory-safe.
		unsafe {
			slice::from_raw_parts(self.0.as_ptr() as *const i8, N)
		}
	}

	// TODO: as_i12_samples()

	/// * Use to read the buffer if it contains timestamps.
	/// * Negligible performance overhead (effectively just constructs a fat pointer).
	///	* O(1).
	/// * **If N % 8 != 0, the last few bytes will be ignored** (which is most likely
	/// 	not what you want).
	pub fn as_u64_timestamps(&self) -> &[u64] {
		// Unsafe block, but casting eight u8 to a u64 is memory-safe if the buffer
		// has proper 8-bytes alignment (which it has in our case, as it's page-aligned).
		// Even if N % 8 != 0, N / 8 will round down, such that the last few u8 will get ignored.
		unsafe {
			slice::from_raw_parts(self.0.as_ptr() as *const u64, N/8)
		}
	}

	/// * Returns a C-like ``void*`` to the given ``PageAlignedBuffer``.
	pub fn as_mut_void_ptr(&self) -> *mut raw::c_void {
		self as *const PageAlignedBuffer<N> as *mut raw::c_void
	}
}


/// * Reads the specified card's error string, and returns it as ``Some(String)`` if
///		it's not null (else ``None``).
/// * I initially tried to get it to return ``Some(&str)``, but the conversion required
/// 	UTF-8 while the SDK returns bytes usually interpreted as ASCII. Getting a proper
/// 	conversion seems to be a PITA and would increase complexity a bunch. At least the
/// 	heap allocation only happens when an error occurs (at which point you'd probably
/// 	want to kill the program anyway).
pub fn get_error(device: core::drv_handle) -> Option<String> {
	let mut err_text_buf = [0u8; core::ERRORTEXTLEN as usize];
	let err_text_ptr: *mut i8 = err_text_buf.as_mut_ptr() as *mut ffi::c_char;
	if unsafe{core::spcm_dwGetErrorInfo_i64(
				device, ptr::null_mut(), ptr::null_mut(), err_text_ptr)} != 0 {
		return Some(unsafe{ffi::CStr::from_ptr(err_text_ptr)}.to_string_lossy().into_owned());
	}
	
	None
}