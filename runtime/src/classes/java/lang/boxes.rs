macro_rules! primitive_boxes {
	($($mod_name:ident, $class_name:ident, $ty:ident, $field_ty:pat, $unwrapper:ident, $field:ident, $converter:expr);+ $(;)?) => {
		$(
		pub mod $mod_name {
			use crate::objects::instance::class::ClassInstanceRef;
			use crate::objects::instance::Instance;
			use crate::objects::instance::object::Object;

			use jni::sys::$ty;
			use classfile::FieldType;

			pub fn value(instance: ClassInstanceRef) -> $ty {
				assert_eq!(instance.class(), crate::globals::classes::$class_name());
				let $field = instance
					.get_field_value0(value_field_index())
					.$unwrapper();
				$converter
			}

			crate::classes::field_module! {
				@CLASS $class_name;

				@FIELDSTART
				@FIELD value: $field_ty,
			}
		}
		)+
	}
}

primitive_boxes!(
	Boolean,   java_lang_Boolean,   jboolean, FieldType::Boolean,   expect_int,    field, field != 0;
	Character, java_lang_Character, jchar,    FieldType::Character, expect_int,    field, field as jchar;
	Float,     java_lang_Float,     jfloat,   FieldType::Float,     expect_float,  field, field as jfloat;
	Double,    java_lang_Double,    jdouble,  FieldType::Double,    expect_double, field, field as jdouble;
	Byte,      java_lang_Byte,      jbyte,    FieldType::Byte,      expect_int,    field, field as jbyte;
	Short,     java_lang_Short,     jshort,   FieldType::Short,     expect_int,    field, field as jshort;
	Integer,   java_lang_Integer,   jint,     FieldType::Integer,   expect_int,    field, field as jint;
	Long,      java_lang_Long,      jlong,    FieldType::Long,      expect_long,   field, field as jlong;
);
