#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JByteArray, JClass, JObject};
use jni::sys::{jint, jlong, jsize};
use native_macros::jni_call;

pub const SUPPORTS_OPENAT: jint = 1 << 1; // syscalls
pub const SUPPORTS_XATTR: jint = 1 << 3;
pub const SUPPORTS_BIRTHTIME: jint = 1 << 16; // other features

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_init(
	env: JniEnv,
	_this: JClass,
) -> jint {
	macro_rules! field_exists {
		($env:ident, $sig:literal, $class:ident, $field_name:ident) => {
			let Ok(_field) = $env.get_field_id($class, stringify!($field_name), $sig) else {
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

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_getcwd(
	env: JniEnv,
	_this: JClass,
) -> Option<JByteArray> {
	// + 1 for null terminator
	let mut buf = vec![0; libc::PATH_MAX as usize + 1];

	let cwd = unsafe { libc::getcwd(buf.as_mut_ptr() as *mut libc::c_char, buf.capacity()) };
	if cwd.is_null() {
		todo!("Throw sun/util/UnixException")
	}

	let cwd_len;
	unsafe {
		cwd_len = libc::strlen(cwd) + 1;
		buf.set_len(cwd_len);
	}

	let buf_without_terminator = &mut buf[..cwd_len - 1];

	let Ok(byte_array) = env.new_byte_array((cwd_len - 1) as jsize) else {
		return None;
	};

	if env
		.set_byte_array_region(byte_array, 0, buf_without_terminator)
		.is_err()
	{
		return None;
	}

	Some(byte_array)
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_strerror(
	_env: JniEnv,
	_this: JClass,
	_error: jint,
) -> JByteArray {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#strerror");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_dup(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#strerror");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_rewind(
	_env: JniEnv,
	_this: JClass,
	_stream: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#rewind");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_getlinelen(
	_env: JniEnv,
	_this: JClass,
	_stream: jlong,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#getlinelen");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_open0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_oflags: jint,
	_mode: jint,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#open0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_openat0(
	_env: JniEnv,
	_this: JClass,
	_dfd: jint,
	_path_address: jlong,
	_oflags: jint,
	_mode: jint,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#openat0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_close0(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#close0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_read0(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
	_address: jlong,
	_nbytes: jint,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#read0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_write0(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
	_address: jlong,
	_nbytes: jint,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#write0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_stat0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_attrs: JObject,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#stat0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_lstat0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_attrs: JObject,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#lstat0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_fstat0(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
	_attrs: JObject,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#fstat0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_fstatat0(
	_env: JniEnv,
	_this: JClass,
	_dfd: jint,
	_path_address: jlong,
	_flag: jint,
	_attrs: JObject,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#fstatat0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_chmod0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_mode: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#chmod0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_fchmod0(
	_env: JniEnv,
	_this: JClass,
	_filedes: jint,
	_mode: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#fchmod0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_chown0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_uid: jint,
	_gid: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#chown0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_lchown0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_uid: jint,
	_gid: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#lchown0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_fchown0(
	_env: JniEnv,
	_this: JClass,
	_filedes: jint,
	_uid: jint,
	_gid: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#fchown0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_utimes0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_access_time: jlong,
	_modification_time: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#utimes0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_futimens0(
	_env: JniEnv,
	_this: JClass,
	_filedes: jint,
	_access_time: jlong,
	_modification_time: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#futimens0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_utimensat0(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
	_path_address: jlong,
	_access_time: jlong,
	_modification_time: jlong,
	_flags: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#utimensat0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_opendir0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
) -> jlong {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#opendir0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_fdopendir(
	_env: JniEnv,
	_this: JClass,
	_dfd: jint,
) -> jlong {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#fdopendir");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_closedir(
	_env: JniEnv,
	_this: JClass,
	_dir: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#closedir");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_readdir0(
	_env: JniEnv,
	_this: JClass,
	_value: jlong,
) -> JByteArray {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#readdir0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_mkdir0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_mode: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#mkdir0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_rmdir0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#rmdir0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_link0(
	_env: JniEnv,
	_this: JClass,
	_existing_address: jlong,
	_new_address: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#link0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_unlink0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#unlink0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_unlinkat0(
	_env: JniEnv,
	_this: JClass,
	_dfd: jint,
	_path_address: jlong,
	_flags: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#unlinkat0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_rename0(
	_env: JniEnv,
	_this: JClass,
	_from_address: jlong,
	_to_address: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#rename0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_renameat0(
	_env: JniEnv,
	_this: JClass,
	_fromfd: jint,
	_from_address: jlong,
	_tofd: jint,
	_to_address: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#renameat0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_symlink0(
	_env: JniEnv,
	_this: JClass,
	_target_address: jlong,
	_link_address: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#symlink0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_readlink0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
) -> JByteArray {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#readlink0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_realpath0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
) -> JByteArray {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#realpath0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_access0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_amode: jint,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#access0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_statvfs0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_attrs: JObject,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#statvfs0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_mknod0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_mode: jint,
	_dev: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#mknod0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_getpwuid(
	_env: JniEnv,
	_this: JClass,
	_uid: jint,
) -> JByteArray {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#getpwuid");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_getgrgid(
	_env: JniEnv,
	_this: JClass,
	_gid: jint,
) -> JByteArray {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#getgrgid");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_getpwnam0(
	_env: JniEnv,
	_this: JClass,
	_name_address: jlong,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#getpwnam0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_getgrnam0(
	_env: JniEnv,
	_this: JClass,
	_name_address: jlong,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#getgrnam0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_fgetxattr0(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
	_name_address: jlong,
	_value_address: jlong,
	_value_len: jint,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#fgetxattr0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_fsetxattr0(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
	_name_address: jlong,
	_value_address: jlong,
	_value_len: jint,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#fsetxattr0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_fremovexattr0(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
	_name_address: jlong,
) {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#fremovexattr0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixNativeDispatcher_flistxattr(
	_env: JniEnv,
	_this: JClass,
	_fd: jint,
	_list_address: jlong,
	_size: jint,
) -> jint {
	unimplemented!("sun.nio.fs.UnixNativeDispatcher#flistxattr");
}
