use crate::objects::class::ClassPtr;

use std::cell::SyncUnsafeCell;

macro_rules! define_classes {
    ($($name:ident),+ $(,)?) => {
        paste::paste! {
            $(
			#[allow(non_upper_case_globals)]
            static [<$name _>]: SyncUnsafeCell<Option<ClassPtr>> = SyncUnsafeCell::new(None);

            #[doc = "Set the loaded " $name " class"]
            ///
            /// # Safety
            ///
            /// This must only be called once
			#[allow(non_snake_case)]
            pub unsafe fn [<set_ $name>](class: ClassPtr) {
				let ptr = [<$name _>].get();
                unsafe { *ptr = Some(class); }
            }

            #[doc = "Get the loaded " $name " class"]
            ///
            /// # Panics
            ///
            /// This will panic if the class is not actually loaded.
			#[allow(non_snake_case)]
            pub fn $name() -> ClassPtr {
                [<$name _opt>]().expect(concat!(stringify!($name), " not loaded"))
            }

            #[doc = "Get the loaded " $name " class, if it's loaded"]
			#[allow(non_snake_case)]
            pub fn [<$name _opt>]() -> Option<ClassPtr> {
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
	java_lang_invoke_MethodHandle,
	java_lang_invoke_MethodHandleNatives,
	java_lang_invoke_MemberName,
	java_lang_invoke_ResolvedMethodName,
	java_lang_invoke_MethodType,
	java_lang_invoke_VarHandle,
	java_lang_invoke_LambdaForm,
	java_lang_reflect_Constructor,
	java_lang_reflect_Method,
	java_lang_reflect_Field,
	java_lang_ref_Reference,
	java_lang_ref_Finalizer,
	java_io_FileDescriptor,
	java_io_FileInputStream,
	java_io_FileOutputStream,
	java_io_File,
	jdk_internal_misc_UnsafeConstants,
	jdk_internal_reflect_MethodAccessorImpl,
	jdk_internal_reflect_ConstantPool,
	java_lang_VirtualMachineError,
	jdk_internal_loader_NativeLibraries,
	jdk_internal_loader_NativeLibraries_NativeLibraryImpl,
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
	boolean_array,
	byte_array,
	character_array,
	double_array,
	float_array,
	integer_array,
	long_array,
	short_array,
	string_array,
);
