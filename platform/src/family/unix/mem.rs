use std::sync::OnceLock;

/// [**UNIX**] Get the size of a page in bytes
///
/// This shouldn't ever change during execution, so the result is cached after the first call.
pub fn get_page_size() -> usize {
	static ONCE: OnceLock<usize> = OnceLock::new();
	*ONCE.get_or_init(|| unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize })
}
