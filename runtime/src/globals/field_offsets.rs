#![allow(non_snake_case)]

//! Various offsets for fields of frequently accessed classes

pub mod java_lang_Class {
	static mut CLASS_NAME_FIELD_OFFSET: usize = 0;

	pub fn name_field_offset() -> usize {
		unsafe { CLASS_NAME_FIELD_OFFSET }
	}

	pub unsafe fn set_name_field_offset(value: usize) {
		CLASS_NAME_FIELD_OFFSET = value;
	}
}

pub mod java_lang_String {
	static mut STRING_VALUE_FIELD_OFFSET: usize = 0;
	static mut STRING_CODER_FIELD_OFFSET: usize = 0;

	/// `java.lang.String#value` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `jByteArray`
	pub fn value_field_offset() -> usize {
		unsafe { STRING_VALUE_FIELD_OFFSET }
	}
	/// `java.lang.String#coder` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `jint`
	pub fn coder_field_offset() -> usize {
		unsafe { STRING_CODER_FIELD_OFFSET }
	}

	pub unsafe fn set_field_offsets(value: usize, coder: usize) {
		STRING_VALUE_FIELD_OFFSET = value;
		STRING_CODER_FIELD_OFFSET = coder;
	}
}

pub mod java_lang_Module {
	static mut MODULE_NAME_FIELD_OFFSET: usize = 0;
	static mut MODULE_LOADER_FIELD_OFFSET: usize = 0;

	/// `java.lang.Module#name` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `Reference` to `java.lang.String`
	pub fn name_field_offset() -> usize {
		unsafe { MODULE_NAME_FIELD_OFFSET }
	}

	pub unsafe fn set_name_field_offset(value: usize) {
		MODULE_NAME_FIELD_OFFSET = value;
	}

	/// `java.lang.Module#loader` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `Reference` to `java.lang.ClassLoader`
	pub fn loader_field_offset() -> usize {
		unsafe { MODULE_LOADER_FIELD_OFFSET }
	}

	pub unsafe fn set_loader_field_offset(value: usize) {
		MODULE_LOADER_FIELD_OFFSET = value;
	}
}

pub mod java_lang_ref_Reference {
	static mut REFERENT_FIELD_OFFSET: usize = 0;

	/// `java.lang.ref.Reference#referent` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `Reference`
	pub fn referent_field_offset() -> usize {
		unsafe { REFERENT_FIELD_OFFSET }
	}

	pub unsafe fn set_referent_field_offset(value: usize) {
		REFERENT_FIELD_OFFSET = value;
	}
}

pub mod java_lang_Thread {
	static mut THREAD_EETOP_FIELD_OFFSET: usize = 0;
	static mut THREAD_HOLDER_FIELD_OFFSET: usize = 0;

	/// `java.lang.Thread#eetop` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `jlong`
	pub fn eetop_field_offset() -> usize {
		unsafe { THREAD_EETOP_FIELD_OFFSET }
	}

	pub unsafe fn set_eetop_field_offset(value: usize) {
		THREAD_EETOP_FIELD_OFFSET = value;
	}

	/// `java.lang.Thread#holder` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `java.lang.Thread$FieldHolder` reference
	pub fn holder_field_offset() -> usize {
		unsafe { THREAD_HOLDER_FIELD_OFFSET }
	}

	pub unsafe fn set_holder_field_offset(value: usize) {
		THREAD_HOLDER_FIELD_OFFSET = value;
	}

	pub mod holder {
		static mut FIELDHOLDER_STACK_SIZE_FIELD_OFFSET: usize = 0;
		static mut FIELDHOLDER_PRIORITY_FIELD_OFFSET: usize = 0;
		static mut FIELDHOLDER_DAEMON_FIELD_OFFSET: usize = 0;
		static mut FIELDHOLDER_THREAD_STATUS_FIELD_OFFSET: usize = 0;

		/// `java.lang.Thread$FieldHolder#stackSize` field offset
		///
		/// This will not change for the lifetime of the program.
		///
		/// Expected field type: `jint`
		pub fn stack_size_field_offset() -> usize {
			unsafe { FIELDHOLDER_STACK_SIZE_FIELD_OFFSET }
		}

		pub unsafe fn set_stack_size_field_offset(value: usize) {
			FIELDHOLDER_STACK_SIZE_FIELD_OFFSET = value;
		}

		/// `java.lang.Thread$FieldHolder#priority` field offset
		///
		/// This will not change for the lifetime of the program.
		///
		/// Expected field type: `jint`
		pub fn priority_field_offset() -> usize {
			unsafe { FIELDHOLDER_PRIORITY_FIELD_OFFSET }
		}

		pub unsafe fn set_priority_field_offset(value: usize) {
			FIELDHOLDER_PRIORITY_FIELD_OFFSET = value;
		}

		/// `java.lang.Thread$FieldHolder#daemon` field offset
		///
		/// **THIS IS A STATIC FIELD**
		///
		/// This will not change for the lifetime of the program.
		///
		/// Expected field type: `jboolean`
		pub fn daemon_field_offset() -> usize {
			unsafe { FIELDHOLDER_DAEMON_FIELD_OFFSET }
		}

		pub unsafe fn set_daemon_field_offset(value: usize) {
			FIELDHOLDER_DAEMON_FIELD_OFFSET = value;
		}

		/// `java.lang.Thread$FieldHolder#threadStatus` field offset
		///
		/// **THIS IS A STATIC FIELD**
		///
		/// This will not change for the lifetime of the program.
		///
		/// Expected field type: `jint`
		pub fn thread_status_field_offset() -> usize {
			unsafe { FIELDHOLDER_THREAD_STATUS_FIELD_OFFSET }
		}

		pub unsafe fn set_thread_status_field_offset(value: usize) {
			FIELDHOLDER_THREAD_STATUS_FIELD_OFFSET = value;
		}
	}
}
