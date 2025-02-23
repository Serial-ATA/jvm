mod attach_args;
mod error;
mod init_args;

use crate::env::JniEnv;
use crate::error::{JniError, Result};
use crate::version::JniVersion;
pub use attach_args::*;
pub use error::*;
pub use init_args::*;

use std::path::PathBuf;

use jni_sys::JNIEnv;

#[derive(Default, Debug, Clone)]
pub struct JavaVmBuilder {
	jvm_lib_path: Option<PathBuf>,
	args: Option<VmInitArgs>,
}

type CreateJavaVmFn = unsafe extern "system" fn(
	*mut *mut jni_sys::JavaVM,
	*mut *mut (),
	*mut jni_sys::JavaVMInitArgs,
) -> jni_sys::jint;

impl JavaVmBuilder {
	/// Create a new `JavaVmBuilder`
	pub fn new() -> Self {
		Self {
			jvm_lib_path: None,
			args: None,
		}
	}

	/// Set the path of `libjvm-runtime`
	pub fn jvm_lib_path<P>(mut self, path: P) -> Self
	where
		P: Into<PathBuf>,
	{
		self.jvm_lib_path = Some(path.into());
		self
	}

	/// Set the initialization args
	pub fn args(mut self, args: VmInitArgs) -> Self {
		self.args = Some(args);
		self
	}

	pub fn build(self) -> Result<(JavaVm, JniEnv)> {
		let libjvm_path = self.jvm_lib_path.unwrap_or_else(default_libjvm_path);
		let args = self.args.unwrap_or_default().finish();

		let ret;
		let mut javavm_raw = core::ptr::null_mut();
		let mut jni_env_raw = core::ptr::null_mut();
		unsafe {
			let libjvm =
				libloading::Library::new(libjvm_path).map_err(|_| Error::LibJvmNotFound)?;

			let create_java_vm: libloading::Symbol<'_, CreateJavaVmFn> = libjvm
				.get(b"JNI_CreateJavaVM\0")
				.map_err(|_| Error::SymbolNotFound(b"JNI_CreateJavaVM\0"))?;

			ret = create_java_vm(&mut javavm_raw, &mut jni_env_raw, args.raw() as _);
		}

		if let Some(err) = JniError::from_jint(ret) {
			return Err(err);
		}

		if javavm_raw.is_null() {
			return Err(Error::JavaVmNull.into());
		}

		let java_vm = unsafe { JavaVm::from_raw(javavm_raw) };
		let jni_env = unsafe { JniEnv::from_raw(jni_env_raw as *mut JNIEnv) };

		Ok((java_vm, jni_env))
	}
}

#[cfg(debug_assertions)]
fn default_libjvm_path() -> PathBuf {
	let target = std::env::var("CARGO_TARGET_DIR").map_or_else(
		|_| {
			PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.parent()
				.unwrap()
				.join("target")
				.join("x86_64-unknown-linux-gnu")
		},
		PathBuf::from,
	);

	target.join("debug").join("libjvm_runtime.so")
}

#[cfg(not(debug_assertions))]
fn default_libjvm_path() -> PathBuf {
	let target = std::env::var("CARGO_TARGET_DIR").map_or_else(
		|_| {
			PathBuf::from(env!("CARGO_MANIFEST_DIR"))
				.parent()
				.unwrap()
				.join("target")
				.join("x86_64-unknown-linux-gnu")
		},
		PathBuf::from,
	);

	target.join("release").join("libjvm_runtime.so")
}

/// A wrapper around a built [`jni_sys::JavaVM`]
///
/// See [`JavaVmBuilder`].
#[derive(PartialEq, Eq)]
pub struct JavaVm(*mut jni_sys::JavaVM);

impl JavaVm {
	/// Unloads the Java VM and reclaims its resources.
	pub fn destroy(self) -> Result<()> {
		let ret;
		unsafe {
			let invoke_interface = self.as_invoke_interface();
			ret = ((*invoke_interface).DestroyJavaVM)(self.0);
		}

		if let Some(err) = JniError::from_jint(ret) {
			return Err(err);
		}

		Ok(())
	}

	pub fn attach_current_thread(&self, args: Option<VmAttachArgs>) -> Result<()> {
		let mut env = core::ptr::null_mut();
		let args = args.map(VmAttachArgs::finish);

		let ret;
		unsafe {
			let invoke_interface = self.as_invoke_interface();
			let mut args_ptr = core::ptr::null_mut();
			if let Some(args) = args {
				args_ptr = args.raw() as _;
			}

			ret = ((*invoke_interface).AttachCurrentThread)(self.0, &mut env, args_ptr);
		}

		if let Some(err) = JniError::from_jint(ret) {
			return Err(err);
		}

		todo!("AttachCurrentThread")
	}

	pub fn detach_current_thread(&self) -> Result<()> {
		todo!("DetachCurrentThread")
	}

	pub fn get_env(&self, version: JniVersion) -> Result<JniEnv> {
		let mut env = core::ptr::null_mut();

		let ret;
		unsafe {
			let invoke_interface = self.as_invoke_interface();
			ret = ((*invoke_interface).GetEnv)(self.0, &mut env, version.into());
		}

		if let Some(err) = JniError::from_jint(ret) {
			return Err(err);
		}

		Ok(unsafe { JniEnv::from_raw(env as _) })
	}

	pub fn attach_current_thread_as_daemon(&self) -> Result<()> {
		todo!("AttachCurrentThreadAsDaemon")
	}

	unsafe fn as_invoke_interface(&self) -> *const jni_sys::JNIInvokeInterface_ {
		self.0 as _
	}
}

impl JavaVm {
	pub const fn raw(&self) -> *const jni_sys::JavaVM {
		self.0 as _
	}

	pub const unsafe fn from_raw(ptr: *mut jni_sys::JavaVM) -> Self {
		Self(ptr)
	}
}
