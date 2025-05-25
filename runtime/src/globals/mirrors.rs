use crate::objects::mirror::MirrorInstance;
use crate::objects::reference::{MirrorInstanceRef, Reference};

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

define_primitive_mirrors!(
	Byte, Character, Double, Float, Integer, Long, Short, Boolean, Void,
);

pub fn primitive_mirror_for(ty: &FieldType) -> Reference {
	assert!(
		!matches!(ty, FieldType::Array(_) | FieldType::Object(_)),
		"`Array` and `Object` field types are incompatible with the primitive mirror"
	);

	match ty {
		FieldType::Byte => primitive_byte_mirror(),
		FieldType::Character => primitive_character_mirror(),
		FieldType::Double => primitive_double_mirror(),
		FieldType::Float => primitive_float_mirror(),
		FieldType::Integer => primitive_integer_mirror(),
		FieldType::Long => primitive_long_mirror(),
		FieldType::Short => primitive_short_mirror(),
		FieldType::Boolean => primitive_boolean_mirror(),
		FieldType::Void => primitive_void_mirror(),
		FieldType::Object(_) | FieldType::Array(_) => unreachable!(),
	}
}

pub fn primitive_array_mirror_for(ty: &FieldType) -> MirrorInstanceRef {
	assert!(
		matches!(ty, FieldType::Array(_)),
		"`Array` field type expected"
	);

	let FieldType::Array(ty) = ty else {
		unreachable!()
	};

	assert!(ty.is_primitive());

	match &**ty {
		FieldType::Byte => crate::globals::classes::byte_array().mirror(),
		FieldType::Character => crate::globals::classes::character_array().mirror(),
		FieldType::Double => crate::globals::classes::double_array().mirror(),
		FieldType::Float => crate::globals::classes::float_array().mirror(),
		FieldType::Integer => crate::globals::classes::integer_array().mirror(),
		FieldType::Long => crate::globals::classes::long_array().mirror(),
		FieldType::Short => crate::globals::classes::short_array().mirror(),
		FieldType::Boolean => crate::globals::classes::boolean_array().mirror(),
		_ => unreachable!(),
	}
}
