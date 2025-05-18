mod attach_args;
pub use attach_args::*;
mod error;
pub use error::*;
mod init_args;
pub use init_args::*;

use crate::env::JniEnv;
use crate::error::{JniError, Result};
use crate::version::JniVersion;

use std::ffi::c_void;
use std::path::PathBuf;

use jni_sys::JNIEnv;

#[derive(Default, Debug, Clone)]
pub struct JavaVmBuilder {
	jvm_lib_path: Option<PathBuf>,
	args: Option<VmInitArgs>,
}

type CreateJavaVmFn = unsafe extern "system" fn(
	*mut *mut jni_sys::JavaVM,
	*mut *mut c_void,
	*mut c_void,
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

	/// Finalize this `JavaVmBuilder` and create the Java VM
	///
	/// TODO: Document default libjvm_path
	///
	/// # Errors
	///
	/// TODO: Document errors
	///
	/// # Examples
	///
	/// ```rust
	/// use jni::java_vm::JavaVm;
	///
	/// # fn main() -> jni::error::Result<()> {
	/// // Simplest case, create a VM with all default options
	/// let (vm, env) = JavaVm::builder().build()?;
	///
	/// // VM is active now, it can be interacted with through the VM and env interfaces
	/// match env.find_class("java/lang/Object") {
	/// 	Ok(_) => println!("Found java/lang/Object"),
	/// 	Err(e) => eprintln!("Couldn't find java/lang/Object: {e}"),
	/// }
	///
	/// println!("Shutting down VM!");
	/// vm.destroy()?;
	/// # Ok(()) }
	/// ```
	pub fn build(self) -> Result<(JavaVm, JniEnv)> {
		let libjvm_path = self.jvm_lib_path.unwrap_or_else(default_libjvm_path);
		let args = self.args.unwrap_or_default().finish();

		let libjvm;
		let ret;
		let mut javavm_raw = core::ptr::null_mut::<jni_sys::JavaVM>();
		let mut jni_env_raw = core::ptr::null_mut::<c_void>();
		unsafe {
			libjvm = libloading::Library::new(libjvm_path).map_err(|_| Error::LibJvmNotFound)?;

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

		let java_vm = JavaVm {
			inner: javavm_raw,
			_libjvm: Some(libjvm),
		};
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
		},
		PathBuf::from,
	);

	target.join("release").join("libjvm_runtime.so")
}

/// A wrapper around a built [`jni_sys::JavaVM`]
///
/// See [`JavaVmBuilder`].
pub struct JavaVm {
	inner: *mut jni_sys::JavaVM,
	// Not used outside of the original load, just kept here to prevent unloading.
	// Optional since it's also used in libjvm, where it, of course, isn't applicable.
	_libjvm: Option<libloading::Library>,
}

impl PartialEq for JavaVm {
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

impl Eq for JavaVm {}

impl JavaVm {
	pub fn builder() -> JavaVmBuilder {
		JavaVmBuilder::new()
	}
}

impl JavaVm {
	/// Unloads the Java VM and reclaims its resources.
	pub fn destroy(self) -> Result<()> {
		let ret;
		unsafe {
			let invoke_interface = self.as_invoke_interface();
			ret = ((*invoke_interface).DestroyJavaVM)(self.inner);
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

			ret = ((*invoke_interface).AttachCurrentThread)(self.inner, &mut env, args_ptr);
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
			ret = ((*invoke_interface).GetEnv)(self.inner, &mut env, version.into());
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
		self.inner as _
	}
}

impl JavaVm {
	pub const fn raw(&self) -> *const jni_sys::JavaVM {
		self.inner as _
	}

	pub const unsafe fn from_raw(ptr: *mut jni_sys::JavaVM) -> Self {
		Self {
			inner: ptr,
			_libjvm: None,
		}
	}
}
