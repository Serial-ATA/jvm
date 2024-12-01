use crate::objects::class::Class;

use std::cell::UnsafeCell;

macro_rules! define_classes {
    ($($name:ident),+ $(,)?) => {
        paste::paste! {
            $(
			#[allow(non_upper_case_globals)]
            static mut [<$name _>]: UnsafeCell<Option<&'static Class>> = UnsafeCell::new(None);

            #[doc = "Set the loaded " $name " class"]
            ///
            /// # Safety
            ///
            /// This must only be called once
			#[allow(non_snake_case)]
            pub unsafe fn [<set_ $name>](class: &'static Class) {
                *[<$name _>].get_mut() = Some(class);
            }

            #[doc = "Get the loaded " $name " class"]
            ///
            /// # Panics
            ///
            /// This will panic if the class is not actually loaded.
			#[allow(non_snake_case)]
            pub fn $name() -> &'static Class {
                unsafe {
                    (*[<$name _>].get()).expect(concat!(stringify!($name), " not loaded"))
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
	java_lang_Thread,
	java_lang_Thread_FieldHolder,
	java_lang_ThreadGroup,
	java_lang_Throwable,
	java_lang_Cloneable,
	java_io_FileDescriptor,
	java_io_FileInputStream,
	java_io_FileOutputStream,
	// Primitive arrays
	bool_array,
	byte_array,
	char_array,
	double_array,
	float_array,
	int_array,
	long_array,
	short_array,
);
