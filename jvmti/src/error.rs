#[rustfmt::skip]
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum JvmtiError {
	InvalidThread                                 = crate::sys::JVMTI_ERROR_INVALID_THREAD as isize,
	InvalidThreadGroup                            = crate::sys::JVMTI_ERROR_INVALID_THREAD_GROUP as isize,
	InvalidPriority                               = crate::sys::JVMTI_ERROR_INVALID_PRIORITY as isize,
	ThreadNotSuspended                            = crate::sys::JVMTI_ERROR_THREAD_NOT_SUSPENDED as isize,
	ThreadSuspended                               = crate::sys::JVMTI_ERROR_THREAD_SUSPENDED as isize,
	ThreadNotAlive                                = crate::sys::JVMTI_ERROR_THREAD_NOT_ALIVE as isize,
	InvalidObject                                 = crate::sys::JVMTI_ERROR_INVALID_OBJECT as isize,
	InvalidClass                                  = crate::sys::JVMTI_ERROR_INVALID_CLASS as isize,
	ClassNotPrepared                              = crate::sys::JVMTI_ERROR_CLASS_NOT_PREPARED as isize,
	InvalidMethodId                               = crate::sys::JVMTI_ERROR_INVALID_METHODID as isize,
	InvalidLocation                               = crate::sys::JVMTI_ERROR_INVALID_LOCATION as isize,
	InvalidFieldId                                = crate::sys::JVMTI_ERROR_INVALID_FIELDID as isize,
	InvalidModule                                 = crate::sys::JVMTI_ERROR_INVALID_MODULE as isize,
	NoMoreFrames                                  = crate::sys::JVMTI_ERROR_NO_MORE_FRAMES as isize,
	OpaqueFrame                                   = crate::sys::JVMTI_ERROR_OPAQUE_FRAME as isize,
	TypeMismatch                                  = crate::sys::JVMTI_ERROR_TYPE_MISMATCH as isize,
	InvalidSlot                                   = crate::sys::JVMTI_ERROR_INVALID_SLOT as isize,
	Duplicate                                     = crate::sys::JVMTI_ERROR_DUPLICATE as isize,
	NotFound                                      = crate::sys::JVMTI_ERROR_NOT_FOUND as isize,
	InvalidMonitor                                = crate::sys::JVMTI_ERROR_INVALID_MONITOR as isize,
	NotMonitorOwner                               = crate::sys::JVMTI_ERROR_NOT_MONITOR_OWNER as isize,
	Interrupt                                     = crate::sys::JVMTI_ERROR_INTERRUPT as isize,
	InvalidClassFormat                            = crate::sys::JVMTI_ERROR_INVALID_CLASS_FORMAT as isize,
	CircularClassDefinition                       = crate::sys::JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION as isize,
	FailsVerification                             = crate::sys::JVMTI_ERROR_FAILS_VERIFICATION as isize,
	UnsupportedRedefinitionMethodAdded            = crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED as isize,
	UnsupportedRedefinitionSchemaChanged          = crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED as isize,
	InvalidTypestate                              = crate::sys::JVMTI_ERROR_INVALID_TYPESTATE as isize,
	UnsupportedRedefinitionHierarchyChanged       = crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED as isize,
	UnsupportedRedefinitionMethodDeleted          = crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED as isize,
	UnsupportedVersion                            = crate::sys::JVMTI_ERROR_UNSUPPORTED_VERSION as isize,
	NamesDontMatch                                = crate::sys::JVMTI_ERROR_NAMES_DONT_MATCH as isize,
	UnsupportedRedefinitionClassModifiersChanged  = crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED as isize,
	UnsupportedRedefinitionMethodModifiersChanged = crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED as isize,
	UnsupportedRedefinitionClassAttributeChanged  = crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_ATTRIBUTE_CHANGED as isize,
	UnsupportedOperation                          = crate::sys::JVMTI_ERROR_UNSUPPORTED_OPERATION as isize,
	UnmodifiableClass                             = crate::sys::JVMTI_ERROR_UNMODIFIABLE_CLASS as isize,
	UnmodifiableModule                            = crate::sys::JVMTI_ERROR_UNMODIFIABLE_MODULE as isize,
	NotAvailable                                  = crate::sys::JVMTI_ERROR_NOT_AVAILABLE as isize,
	MustPossessCapability                         = crate::sys::JVMTI_ERROR_MUST_POSSESS_CAPABILITY as isize,
	NullPointer                                   = crate::sys::JVMTI_ERROR_NULL_POINTER as isize,
	AbsentInformation                             = crate::sys::JVMTI_ERROR_ABSENT_INFORMATION as isize,
	InvalidEventType                              = crate::sys::JVMTI_ERROR_INVALID_EVENT_TYPE as isize,
	IllegalArgument                               = crate::sys::JVMTI_ERROR_ILLEGAL_ARGUMENT as isize,
	NativeMethod                                  = crate::sys::JVMTI_ERROR_NATIVE_METHOD as isize,
	ClassLoaderUnsupported                        = crate::sys::JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED as isize,
	OutOfMemory                                   = crate::sys::JVMTI_ERROR_OUT_OF_MEMORY as isize,
	AccessDenied                                  = crate::sys::JVMTI_ERROR_ACCESS_DENIED as isize,
	WrongPhase                                    = crate::sys::JVMTI_ERROR_WRONG_PHASE as isize,
	Internal                                      = crate::sys::JVMTI_ERROR_INTERNAL as isize,
	UnattachedThread                              = crate::sys::JVMTI_ERROR_UNATTACHED_THREAD as isize,
	InvalidEnvironment                            = crate::sys::JVMTI_ERROR_INVALID_ENVIRONMENT as isize,
}

const _: () = {
	assert!(size_of::<JvmtiError>() == size_of::<std::ffi::c_int>());
	assert!(size_of::<std::ffi::c_int>() == size_of::<i32>());
};

impl From<crate::sys::jvmtiError> for JvmtiError {
	fn from(value: crate::sys::jvmtiError) -> Self {
		if value <= crate::sys::JVMTI_ERROR_NONE || value > crate::sys::JVMTI_ERROR_MAX {
			return JvmtiError::Internal;
		}

		unsafe { std::mem::transmute(value) }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn jvmti_error_mappings() {
		for out_of_range in [-1, crate::sys::JVMTI_ERROR_MAX + 1] {
			assert_eq!(JvmtiError::from(out_of_range), JvmtiError::Internal);
		}

		assert_eq!(
			JvmtiError::from(crate::sys::JVMTI_ERROR_MAX),
			JvmtiError::InvalidEnvironment
		);
	}
}
