// Export family specific impls

#[cfg(target_family = "unix")]
use super::unix::containers as imp;
#[cfg(target_family = "windows")]
use super::windows::containers as imp;

/// Whether the VM is (assumed) to be running in a container
pub fn is_containerized() -> bool {
	false
}
