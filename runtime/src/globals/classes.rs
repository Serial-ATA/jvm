use crate::reference::ClassRef;

use std::cell::UnsafeCell;

macro_rules! define_classes {
    ($($name:ident),+ $(,)?) => {
        paste::paste! {
            $(
			#[allow(non_upper_case_globals)]
            static mut [<$name _>]: UnsafeCell<Option<ClassRef>> = UnsafeCell::new(None);

            #[doc = "Set the loaded " $name " class"]
            ///
            /// # Safety
            ///
            /// This must only be called once
			#[allow(non_snake_case)]
            pub unsafe fn [<set_ $name>](class: ClassRef) {
                *[<$name _>].get_mut() = Some(class);
            }

            #[doc = "Get the loaded " $name " class"]
            ///
            /// # Panics
            ///
            /// This will panic if the class is not actually loaded.
			#[allow(non_snake_case)]
            pub fn $name() -> ClassRef {
                unsafe {
                    (*[<$name _>].get()).as_ref().map(ClassRef::clone).expect(concat!(stringify!($name), " not loaded"))
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
