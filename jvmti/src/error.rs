use jvmti_sys::jvmtiError;

pub enum JvmtiError {
	InvalidThread,
	InvalidThreadGroup,
	InvalidPriority,
	ThreadNotSuspended,
	ThreadSuspended,
	ThreadNotAlive,
	InvalidObject,
	InvalidClass,
	ClassNotPrepared,
	InvalidMethodId,
	InvalidLocation,
	InvalidFieldId,
	InvalidModule,
	NoMoreFrames,
	OpaqueFrame,
	TypeMismatch,
	InvalidSlot,
	Duplicate,
	NotFound,
	InvalidMonitor,
	NotMonitorOwner,
	Interrupt,
	InvalidClassFormat,
	CircularClassDefinition,
	FailsVerification,
	UnsupportedRedefinitionMethodAdded,
	UnsupportedRedefinitionSchemaChanged,
	InvalidTypestate,
	UnsupportedRedefinitionHierarchyChanged,
	UnsupportedRedefinitionMethodDeleted,
	UnsupportedVersion,
	NamesDontMatch,
	UnsupportedRedefinitionClassModifiersChanged,
	UnsupportedRedefinitionMethodModifiersChanged,
	UnsupportedRedefinitionClassAttributeChanged,
	UnsupportedOperation,
	UnmodifiableClass,
	UnmodifiableModule,
	NotAvailable,
	MustPossessCapability,
	NullPointer,
	AbsentInformation,
	InvalidEventType,
	IllegalArgument,
	NativeMethod,
	ClassLoaderUnsupported,
	OutOfMemory,
	AccessDenied,
	WrongPhase,
	Internal,
	UnattachedThread,
	InvalidEnvironment,
}

impl JvmtiError {
	pub fn from_raw(raw: jvmtiError) -> Option<Self> {
		match raw {
			crate::sys::JVMTI_ERROR_INVALID_THREAD => Some(JvmtiError::InvalidThread),
			crate::sys::JVMTI_ERROR_INVALID_THREAD_GROUP => Some(JvmtiError::InvalidThreadGroup),
			crate::sys::JVMTI_ERROR_INVALID_PRIORITY => Some(JvmtiError::InvalidPriority),
			crate::sys::JVMTI_ERROR_THREAD_NOT_SUSPENDED => Some(JvmtiError::ThreadNotSuspended),
			crate::sys::JVMTI_ERROR_THREAD_SUSPENDED => Some(JvmtiError::ThreadSuspended),
			crate::sys::JVMTI_ERROR_THREAD_NOT_ALIVE => Some(JvmtiError::ThreadNotAlive),
			crate::sys::JVMTI_ERROR_INVALID_OBJECT => Some(JvmtiError::InvalidObject),
			crate::sys::JVMTI_ERROR_INVALID_CLASS => Some(JvmtiError::InvalidClass),
			crate::sys::JVMTI_ERROR_CLASS_NOT_PREPARED => Some(JvmtiError::ClassNotPrepared),
			crate::sys::JVMTI_ERROR_INVALID_METHODID => Some(JvmtiError::InvalidMethodId),
			crate::sys::JVMTI_ERROR_INVALID_LOCATION => Some(JvmtiError::InvalidLocation),
			crate::sys::JVMTI_ERROR_INVALID_FIELDID => Some(JvmtiError::InvalidFieldId),
			crate::sys::JVMTI_ERROR_INVALID_MODULE => Some(JvmtiError::InvalidModule),
			crate::sys::JVMTI_ERROR_NO_MORE_FRAMES => Some(JvmtiError::NoMoreFrames),
			crate::sys::JVMTI_ERROR_OPAQUE_FRAME => Some(JvmtiError::OpaqueFrame),
			crate::sys::JVMTI_ERROR_TYPE_MISMATCH => Some(JvmtiError::TypeMismatch),
			crate::sys::JVMTI_ERROR_INVALID_SLOT => Some(JvmtiError::InvalidSlot),
			crate::sys::JVMTI_ERROR_DUPLICATE => Some(JvmtiError::Duplicate),
			crate::sys::JVMTI_ERROR_NOT_FOUND => Some(JvmtiError::NotFound),
			crate::sys::JVMTI_ERROR_INVALID_MONITOR => Some(JvmtiError::InvalidMonitor),
			crate::sys::JVMTI_ERROR_NOT_MONITOR_OWNER => Some(JvmtiError::NotMonitorOwner),
			crate::sys::JVMTI_ERROR_INTERRUPT => Some(JvmtiError::Interrupt),
			crate::sys::JVMTI_ERROR_INVALID_CLASS_FORMAT => Some(JvmtiError::InvalidClassFormat),
			crate::sys::JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION => {
				Some(JvmtiError::CircularClassDefinition)
			},
			crate::sys::JVMTI_ERROR_FAILS_VERIFICATION => Some(JvmtiError::FailsVerification),
			crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED => {
				Some(JvmtiError::UnsupportedRedefinitionMethodAdded)
			},
			crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED => {
				Some(JvmtiError::UnsupportedRedefinitionSchemaChanged)
			},
			crate::sys::JVMTI_ERROR_INVALID_TYPESTATE => Some(JvmtiError::InvalidTypestate),
			crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED => {
				Some(JvmtiError::UnsupportedRedefinitionHierarchyChanged)
			},
			crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED => {
				Some(JvmtiError::UnsupportedRedefinitionMethodDeleted)
			},
			crate::sys::JVMTI_ERROR_UNSUPPORTED_VERSION => Some(JvmtiError::UnsupportedVersion),
			crate::sys::JVMTI_ERROR_NAMES_DONT_MATCH => Some(JvmtiError::NamesDontMatch),
			crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED => {
				Some(JvmtiError::UnsupportedRedefinitionClassModifiersChanged)
			},
			crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED => {
				Some(JvmtiError::UnsupportedRedefinitionMethodModifiersChanged)
			},
			crate::sys::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_ATTRIBUTE_CHANGED => {
				Some(JvmtiError::UnsupportedRedefinitionClassAttributeChanged)
			},
			crate::sys::JVMTI_ERROR_UNSUPPORTED_OPERATION => Some(JvmtiError::UnsupportedOperation),
			crate::sys::JVMTI_ERROR_UNMODIFIABLE_CLASS => Some(JvmtiError::UnmodifiableClass),
			crate::sys::JVMTI_ERROR_UNMODIFIABLE_MODULE => Some(JvmtiError::UnmodifiableModule),
			crate::sys::JVMTI_ERROR_NOT_AVAILABLE => Some(JvmtiError::NotAvailable),
			crate::sys::JVMTI_ERROR_MUST_POSSESS_CAPABILITY => {
				Some(JvmtiError::MustPossessCapability)
			},
			crate::sys::JVMTI_ERROR_NULL_POINTER => Some(JvmtiError::NullPointer),
			crate::sys::JVMTI_ERROR_ABSENT_INFORMATION => Some(JvmtiError::AbsentInformation),
			crate::sys::JVMTI_ERROR_INVALID_EVENT_TYPE => Some(JvmtiError::InvalidEventType),
			crate::sys::JVMTI_ERROR_ILLEGAL_ARGUMENT => Some(JvmtiError::IllegalArgument),
			crate::sys::JVMTI_ERROR_NATIVE_METHOD => Some(JvmtiError::NativeMethod),
			crate::sys::JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED => {
				Some(JvmtiError::ClassLoaderUnsupported)
			},
			crate::sys::JVMTI_ERROR_OUT_OF_MEMORY => Some(JvmtiError::OutOfMemory),
			crate::sys::JVMTI_ERROR_ACCESS_DENIED => Some(JvmtiError::AccessDenied),
			crate::sys::JVMTI_ERROR_WRONG_PHASE => Some(JvmtiError::WrongPhase),
			crate::sys::JVMTI_ERROR_INTERNAL => Some(JvmtiError::Internal),
			crate::sys::JVMTI_ERROR_UNATTACHED_THREAD => Some(JvmtiError::UnattachedThread),
			crate::sys::JVMTI_ERROR_INVALID_ENVIRONMENT => Some(JvmtiError::InvalidEnvironment),
			_ => None,
		}
	}
}
