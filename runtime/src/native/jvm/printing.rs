use std::ffi::{VaList, c_char};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jio_vsnprintf(
	_s: *const c_char,
	_count: usize,
	_fmt: *const c_char,
	_args: VaList<'_, '_>,
) {
	todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jio_snprintf(
	_s: *const c_char,
	_count: usize,
	_fmt: *const c_char,
	_: ...
) {
	todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jio_fprintf(_f: *const libc::FILE, _fmt: *const c_char, _: ...) {
	todo!()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn jio_vfprintf(_fmt: *const c_char, _: ...) {
	todo!()
}
