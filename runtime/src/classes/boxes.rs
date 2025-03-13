macro_rules! primitive_boxes {
	($($mod_name:ident, $ty:ident, $field_ty:pat, $unwrapper:ident, $field:ident, $converter:expr);+ $(;)?) => {
		$(
		pub mod $mod_name {
			use crate::objects::class_instance::ClassInstance;
			use crate::objects::instance::Instance;

			use jni::sys::$ty;
			use classfile::FieldType;

			pub fn value(instance: &ClassInstance) -> $ty {
				assert_eq!(instance.class(), crate::globals::classes::$mod_name());
				let $field = instance
					.get_field_value0(value_field_offset())
					.$unwrapper();
				$converter
			}

			crate::classes::field_module! {
				@CLASS $mod_name;

				@FIELDSTART
				@FIELD value: $field_ty,
			}
		}
		)+
	}
}

primitive_boxes!(
	java_lang_Boolean,   jboolean, FieldType::Boolean, expect_int,    field, field != 0;
	java_lang_Character, jchar,    FieldType::Character,    expect_int,    field, field as jchar;
	java_lang_Float,     jfloat,   FieldType::Float,   expect_float,  field, field as jfloat;
	java_lang_Double,    jdouble,  FieldType::Double,  expect_double, field, field as jdouble;
	java_lang_Byte,      jbyte,    FieldType::Byte,    expect_int,    field, field as jbyte;
	java_lang_Short,     jshort,   FieldType::Short,   expect_int,    field, field as jshort;
	java_lang_Integer,   jint,     FieldType::Integer,     expect_int,    field, field as jint;
	java_lang_Long,      jlong,    FieldType::Long,    expect_long,   field, field as jlong;
);
