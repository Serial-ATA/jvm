//! Globals related to the module system

pub static mut MODULE_SYSTEM_INITIALIZED: bool = false;

pub fn module_system_initialized() -> bool {
	unsafe { MODULE_SYSTEM_INITIALIZED }
}

pub unsafe fn set_module_system_initialized() {
	MODULE_SYSTEM_INITIALIZED = true;
}
