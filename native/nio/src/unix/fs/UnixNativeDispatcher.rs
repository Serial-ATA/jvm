use jni::env::JniEnv;
use jni::objects::JClass;
use jni::sys::jint;
use native_macros::jni_call;

pub const SUPPORTS_OPENAT: jint = 1 << 1; // syscalls
pub const SUPPORTS_XATTR: jint = 1 << 3;
pub const SUPPORTS_BIRTHTIME: jint = 1 << 16; // other features

/// Generated C bindings for sun.io.fs.UnixNativeDispatcher methods
pub mod raw {
	pub use super::raw_Java_sun_nio_fs_UnixNativeDispatcher_init::*;
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_init(
	env: JniEnv,
	_this: JClass,
) -> jint {
	macro_rules! field_exists {
		($env:ident, $sig:literal, $class:ident, $field_name:ident) => {
			let Ok(_field) = $env.get_field_id($class, stringify!($field_name), stringify!($sig))
			else {
				return false;
			};
		};
	}

	fn verify_unix_file_attributes(env: JniEnv) -> bool {
		let Ok(class) = env.find_class("sun/nio/fs/UnixFileAttributes") else {
			return false;
		};

		field_exists!(env, "I", class, st_mode);
		field_exists!(env, "J", class, st_ino);
		field_exists!(env, "J", class, st_dev);
		field_exists!(env, "J", class, st_rdev);
		field_exists!(env, "I", class, st_nlink);
		field_exists!(env, "I", class, st_uid);
		field_exists!(env, "I", class, st_gid);
		field_exists!(env, "J", class, st_size);
		field_exists!(env, "J", class, st_atime_sec);
		field_exists!(env, "J", class, st_atime_nsec);
		field_exists!(env, "J", class, st_mtime_sec);
		field_exists!(env, "J", class, st_mtime_nsec);
		field_exists!(env, "J", class, st_ctime_sec);
		field_exists!(env, "J", class, st_ctime_nsec);

		if cfg!(target_os = "linux") {
			field_exists!(env, "J", class, st_birthtime_sec);
			field_exists!(env, "J", class, st_birthtime_nsec);
		}

		field_exists!(env, "Z", class, birthtime_available);

		true
	}

	fn verify_unix_file_store_attributes(env: JniEnv) -> bool {
		let Ok(class) = env.find_class("sun/nio/fs/UnixFileStoreAttributes") else {
			return false;
		};

		field_exists!(env, "J", class, f_frsize);
		field_exists!(env, "J", class, f_blocks);
		field_exists!(env, "J", class, f_bfree);
		field_exists!(env, "J", class, f_bavail);

		true
	}

	fn verify_unix_mount_entry(env: JniEnv) -> bool {
		let Ok(class) = env.find_class("sun/nio/fs/UnixMountEntry") else {
			return false;
		};

		field_exists!(env, "[B", class, name);
		field_exists!(env, "[B", class, dir);
		field_exists!(env, "[B", class, fstype);
		field_exists!(env, "[B", class, opts);
		field_exists!(env, "J", class, dev);

		true
	}

	if !verify_unix_file_attributes(env)
		|| !verify_unix_file_store_attributes(env)
		|| !verify_unix_mount_entry(env)
	{
		return 0;
	}

	let mut capabilities = 0;

	// TODO: BSDs
	let openat = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"openat64".as_ptr()) };
	let fstatat = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"fstatat64".as_ptr()) };
	let unlinkat = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"unlinkat".as_ptr()) };
	let renameat = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"renameat".as_ptr()) };
	let fdopendir = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"fdopendir".as_ptr()) };

	if !openat.is_null()
		&& !fstatat.is_null()
		&& !unlinkat.is_null()
		&& !renameat.is_null()
		&& !fdopendir.is_null()
	{
		capabilities |= SUPPORTS_OPENAT;
	}

	#[cfg(target_os = "linux")]
	{
		let statx = unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"statx".as_ptr()) };
		if !statx.is_null() {
			capabilities |= SUPPORTS_BIRTHTIME;
		}
	}

	capabilities
}
