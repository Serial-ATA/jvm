use crate::objects::class::Class;

use std::cell::SyncUnsafeCell;

macro_rules! define_classes {
    ($($name:ident),+ $(,)?) => {
        paste::paste! {
            $(
			#[allow(non_upper_case_globals)]
            static [<$name _>]: SyncUnsafeCell<Option<&'static Class>> = SyncUnsafeCell::new(None);

            #[doc = "Set the loaded " $name " class"]
            ///
            /// # Safety
            ///
            /// This must only be called once
			#[allow(non_snake_case)]
            pub unsafe fn [<set_ $name>](class: &'static Class) {
				let ptr = [<$name _>].get();
                unsafe { *ptr = Some(class); }
            }

            #[doc = "Get the loaded " $name " class"]
            ///
            /// # Panics
            ///
            /// This will panic if the class is not actually loaded.
			#[allow(non_snake_case)]
            pub fn $name() -> &'static Class {
                [<$name _opt>]().expect(concat!(stringify!($name), " not loaded"))
            }

            #[doc = "Get the loaded " $name " class, if it's loaded"]
			#[allow(non_snake_case)]
            pub fn [<$name _opt>]() -> Option<&'static Class> {
                unsafe {
                    (*[<$name _>].get())
                }
            }
            )+
        }
    };
}

define_classes!(
	java_lang_Object,
	java_lang_Class,
	java_lang_String,
	java_lang_ClassLoader,
	java_lang_System,
	java_lang_Thread,
	java_lang_Thread_FieldHolder,
	java_lang_ThreadGroup,
	java_lang_StackTraceElement,
	java_lang_Throwable,
	java_lang_Cloneable,
	java_io_Serializable,
	java_lang_Module,
	java_lang_invoke_MethodHandleNatives,
	java_lang_invoke_MemberName,
	java_lang_invoke_ResolvedMethodName,
	java_lang_ref_Reference,
	java_lang_ref_Finalizer,
	java_io_FileDescriptor,
	java_io_FileInputStream,
	java_io_FileOutputStream,
	java_io_File,
	jdk_internal_misc_UnsafeConstants,
	jdk_internal_reflect_MethodAccessorImpl,
	// Primitive types
	java_lang_Boolean,
	java_lang_Byte,
	java_lang_Character,
	java_lang_Double,
	java_lang_Float,
	java_lang_Integer,
	java_lang_Long,
	java_lang_Short,
	java_lang_Void,
	// Primitive arrays
	bool_array,
	byte_array,
	char_array,
	double_array,
	float_array,
	int_array,
	long_array,
	short_array,
	string_array,
);
