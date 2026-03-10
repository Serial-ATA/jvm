use jni::objects::JObject;

macro_rules! define_object_types {
	(
        $(
        $(#[$meta:meta])*
        struct $name:ident($ty:path)
        );+
        $(;)?
    ) => {
        $(
        $(#[$meta])*
		#[repr(transparent)]
		#[derive(Copy, Clone, PartialEq, Eq)]
		pub struct $name($ty);

		impl $name {
            #[doc = "Create a "]
            #[doc = stringify!($name)]
            #[doc = " from a raw pointer"]
            ///
            /// # Safety
            ///
            /// The caller *must* ensure that the pointer provided was obtained from the VM.
			pub unsafe fn from_raw(raw: $ty) -> Self {
				Self(raw)
			}

            pub fn null() -> Self {
                Self(core::ptr::null_mut())
            }

			pub fn raw(&self) -> $ty {
				self.0
			}

            pub fn is_null(&self) -> bool {
		        self.raw().is_null()
	        }
		}

		impl From<$name> for JObject {
			fn from(value: $name) -> Self {
				unsafe { JObject::from_raw(value.0) }
			}
		}
        )+
	};
}

define_object_types! {
	struct JThread(crate::sys::jthread);
	struct JThreadGroup(crate::sys::jthreadGroup);
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct JRawMonitorId(crate::sys::jrawMonitorID);

impl JRawMonitorId {
	/// Create a JRawMonitorId from a raw pointer
	///
	/// # Safety
	///
	/// The caller *must* ensure that the pointer provided was obtained from the VM.
	pub unsafe fn from_raw(raw: crate::sys::jrawMonitorID) -> Self {
		Self(raw)
	}

	pub fn null() -> Self {
		Self(core::ptr::null_mut())
	}

	pub fn raw(&self) -> crate::sys::jrawMonitorID {
		self.0
	}

	pub fn is_null(&self) -> bool {
		self.raw().is_null()
	}
}
