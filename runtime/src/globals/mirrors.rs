use crate::objects::mirror::MirrorInstance;
use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;

use classfile::FieldType;

macro_rules! define_primitive_mirrors {
    ($($name:ident),+ $(,)?) => {
        paste::paste! {
            $(
			#[allow(non_upper_case_globals)]
            static [<$name _>]: SyncUnsafeCell<Option<Reference>> = SyncUnsafeCell::new(None);

			#[allow(non_snake_case)]
            unsafe fn [<set_ $name:lower>](mirror: Reference) {
				let ptr = [<$name _>].get();
                unsafe { *ptr = Some(mirror); }
            }

            #[doc = "Get the primitive `" $name:lower "` mirror"]
            ///
            /// # Panics
            ///
            /// This will panic if the mirror is not actually initialized.
			#[allow(non_snake_case)]
            pub fn [<primitive_ $name:lower _mirror>]() -> Reference {
				let val_opt = unsafe { &*([<$name _>].get()) };
				val_opt
					.as_ref()
					.map(Reference::clone)
					.expect(concat!("primitive mirror ", stringify!($name), " not initialized"))
            }
            )+

            // TODO: Panic on double init
            pub fn init_primitive_mirrors() {
				$(
					let [<$name:lower _mirror>] = MirrorInstance::new_primitive(<FieldType>::$name);
					unsafe { [<set_ $name:lower>](Reference::mirror([<$name:lower _mirror>])); }
				)+
			}
        }
    };
}

define_primitive_mirrors!(Byte, Char, Double, Float, Int, Long, Short, Boolean, Void,);

pub fn primitive_mirror_for(ty: &FieldType) -> Reference {
	assert!(
		!matches!(ty, FieldType::Array(_) | FieldType::Object(_)),
		"`Array` and `Object` field types are incompatible with the primitive mirror"
	);

	match ty {
		FieldType::Byte => primitive_byte_mirror(),
		FieldType::Char => primitive_char_mirror(),
		FieldType::Double => primitive_double_mirror(),
		FieldType::Float => primitive_float_mirror(),
		FieldType::Int => primitive_int_mirror(),
		FieldType::Long => primitive_long_mirror(),
		FieldType::Short => primitive_short_mirror(),
		FieldType::Boolean => primitive_boolean_mirror(),
		FieldType::Void => primitive_void_mirror(),
		FieldType::Object(_) | FieldType::Array(_) => unreachable!(),
	}
}
