#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_initIDs(
	_env: JniEnv,
	_this: JClass,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#initIDs");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_CreateEvent(
	_env: JniEnv,
	_this: JClass,
	_b_manual_reset: jboolean,
	_b_initial_state: jboolean,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#CreateEvent");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_FormatMessage(
	_env: JniEnv,
	_this: JClass,
	_error_code: jint,
) -> JString {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#FormatMessage");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_LocalFree(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#LocalFree");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_CreateFile0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_dw_desired_access: jint,
	_dw_share_mode: jint,
	_sd_address: jlong,
	_dw_creation_disposition: jint,
	_dw_flags_and_attributes: jint,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#CreateFile0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_DeviceIoControlSetSparse(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#DeviceIoControlSetSparse");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_DeviceIoControlGetReparsePoint(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
	_buffer_address: jlong,
	_buffer_size: jint,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#DeviceIoControlGetReparsePoint");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_DeleteFile0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#DeleteFile0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_CreateDirectory0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_sd_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#CreateDirectory0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_RemoveDirectory0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#RemoveDirectory0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_CloseHandle(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#CloseHandle");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetFileSizeEx(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetFileSizeEx");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_FindFirstFile0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_obj: JObject,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#FindFirstFile0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_FindFirstFile1(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_data_address: jlong,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#FindFirstFile1");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_FindNextFile0(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
	_data_address: jlong,
) -> JString {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#FindNextFile0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_FindFirstStream0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_obj: JObject,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#FindFirstStream0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_FindNextStream0(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
) -> JString {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#FindNextStream0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_FindClose(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#FindClose");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetFileInformationByHandle0(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
	_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetFileInformationByHandle0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_CopyFileEx0(
	_env: JniEnv,
	_this: JClass,
	_existing_address: jlong,
	_new_address: jlong,
	_flags: jint,
	_cancel_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#CopyFileEx0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_MoveFileEx0(
	_env: JniEnv,
	_this: JClass,
	_existing_address: jlong,
	_new_address: jlong,
	_flags: jint,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#MoveFileEx0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetLogicalDrives(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetLogicalDrives");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetFileAttributes0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) -> jint {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetFileAttributes0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_SetFileAttributes0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_value: jint,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#SetFileAttributes0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetFileAttributesEx0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_data_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetFileAttributesEx0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_SetFileTime0(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
	_create_time: jlong,
	_last_access_time: jlong,
	_last_write_time: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#SetFileTime0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_SetEndOfFile(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#SetEndOfFile");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetVolumeInformation0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_obj: JObject,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetVolumeInformation0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetDriveType0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) -> jint {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetDriveType0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetDiskFreeSpaceEx0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_obj: JObject,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetDiskFreeSpaceEx0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetDiskFreeSpace0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_obj: JObject,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetDiskFreeSpace0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetVolumePathName0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) -> JString {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetVolumePathName0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_InitializeSecurityDescriptor(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#InitializeSecurityDescriptor");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_InitializeAcl(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_size: jint,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#InitializeAcl");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_SetFileSecurity0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_requested_information: jint,
	_desc_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#SetFileSecurity0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetFileSecurity0(
	_env: JniEnv,
	_this: JClass,
	_path_address: jlong,
	_requested_information: jint,
	_desc_address: jlong,
	_n_length: jint,
) -> jint {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetFileSecurity0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetSecurityDescriptorOwner(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetSecurityDescriptorOwner");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_SetSecurityDescriptorOwner(
	_env: JniEnv,
	_this: JClass,
	_desc_address: jlong,
	_owner_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#SetSecurityDescriptorOwner");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetSecurityDescriptorDacl(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetSecurityDescriptorDacl");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_SetSecurityDescriptorDacl(
	_env: JniEnv,
	_this: JClass,
	_desc_address: jlong,
	_acl_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#SetSecurityDescriptorDacl");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetAclInformation0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_obj: JObject,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetAclInformation0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetAce(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_ace_index: jint,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetAce");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_AddAccessAllowedAceEx(
	_env: JniEnv,
	_this: JClass,
	_acl_address: jlong,
	_flags: jint,
	_mask: jint,
	_sid_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#AddAccessAllowedAceEx");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_AddAccessDeniedAceEx(
	_env: JniEnv,
	_this: JClass,
	_acl_address: jlong,
	_flags: jint,
	_mask: jint,
	_sid_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#AddAccessDeniedAceEx");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_LookupAccountSid0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
	_obj: JObject,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#LookupAccountSid0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_LookupAccountName0(
	_env: JniEnv,
	_this: JClass,
	_name_address: jlong,
	_sid_address: jlong,
	_cb_sid: jint,
) -> jint {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#LookupAccountName0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetLengthSid(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) -> jint {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetLengthSid");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_ConvertSidToStringSid(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) -> JString {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#ConvertSidToStringSid");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_ConvertStringSidToSid0(
	_env: JniEnv,
	_this: JClass,
	_address: jlong,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#ConvertStringSidToSid0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetCurrentProcess(
	_env: JniEnv,
	_this: JClass,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetCurrentProcess");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetCurrentThread(
	_env: JniEnv,
	_this: JClass,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetCurrentThread");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_OpenProcessToken(
	_env: JniEnv,
	_this: JClass,
	_process: jlong,
	_desired_access: jint,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#OpenProcessToken");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_OpenThreadToken(
	_env: JniEnv,
	_this: JClass,
	_thread: jlong,
	_desired_access: jint,
	_open_as_self: jboolean,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#OpenThreadToken");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_DuplicateTokenEx(
	_env: JniEnv,
	_this: JClass,
	_token: jlong,
	_desired_access: jint,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#DuplicateTokenEx");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_SetThreadToken(
	_env: JniEnv,
	_this: JClass,
	_thread: jlong,
	_token: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#SetThreadToken");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetTokenInformation(
	_env: JniEnv,
	_this: JClass,
	_token: jlong,
	_token_info_class: jint,
	_token_info: jlong,
	_token_info_length: jint,
) -> jint {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetTokenInformation");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_AdjustTokenPrivileges(
	_env: JniEnv,
	_this: JClass,
	_token: jlong,
	_luid: jlong,
	_attributes: jint,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#AdjustTokenPrivileges");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_AccessCheck(
	_env: JniEnv,
	_this: JClass,
	_token: jlong,
	_security_info: jlong,
	_access_mask: jint,
	_generic_read: jint,
	_generic_write: jint,
	_generic_execute: jint,
	_generic_all: jint,
) -> jboolean {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#AccessCheck");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_LookupPrivilegeValue0(
	_env: JniEnv,
	_this: JClass,
	_name: jlong,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#LookupPrivilegeValue0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_CreateSymbolicLink0(
	_env: JniEnv,
	_this: JClass,
	_link_address: jlong,
	_target_address: jlong,
	_flags: jint,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#CreateSymbolicLink0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_CreateHardLink0(
	_env: JniEnv,
	_this: JClass,
	_new_file_address: jlong,
	_existing_file_address: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#CreateHardLink0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetFullPathName0(
	_env: JniEnv,
	_clz: JClass,
	_path_address: jlong,
) -> JString {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetFullPathName0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetFinalPathNameByHandle(
	_env: JniEnv,
	_this: JClass,
	_handle: jlong,
) -> JString {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetFinalPathNameByHandle");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_CreateIoCompletionPort(
	_env: JniEnv,
	_this: JClass,
	_file_handle: jlong,
	_existing_port: jlong,
	_completion_key: jlong,
) -> jlong {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#CreateIoCompletionPort");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetQueuedCompletionStatus0(
	_env: JniEnv,
	_this: JClass,
	_completion_port: jlong,
	_obj: JObject,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetQueuedCompletionStatus0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_PostQueuedCompletionStatus(
	_env: JniEnv,
	_this: JClass,
	_completion_port: jlong,
	_completion_key: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#PostQueuedCompletionStatus");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_CancelIo(
	_env: JniEnv,
	_this: JClass,
	_h_file: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#CancelIo");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_GetOverlappedResult(
	_env: JniEnv,
	_this: JClass,
	_h_file: jlong,
	_lp_overlapped: jlong,
) -> jint {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#GetOverlappedResult");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_WindowsNativeDispatcher_ReadDirectoryChangesW(
	_env: JniEnv,
	_this: JClass,
	_h_directory: jlong,
	_buffer_address: jlong,
	_buffer_length: jint,
	_watch_sub_tree: jboolean,
	_filter: jint,
	_bytes_returned_address: jlong,
	_p_overlapped: jlong,
) {
	unimplemented!("sun.nio.fs.WindowsNativeDispatcher#ReadDirectoryChangesW");
}
