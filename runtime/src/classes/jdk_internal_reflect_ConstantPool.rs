use crate::objects::class::Class;
use crate::objects::class_instance::ClassInstance;
use crate::objects::constant_pool::ConstantPool;
use crate::objects::instance::Instance;
use crate::objects::reference::{ClassInstanceRef, Reference};
use crate::thread::exceptions::Throws;
use crate::thread::JavaThread;

use classfile::FieldType;
use common::traits::PtrType;
use instructions::Operand;

/// Create a new instance of `jdk.internal.reflect.ConstantPool` for the given class' constant pool
///
/// NOTE: This takes a thread, as `jdk.internal.reflect.ConstantPool` is not initialized ahead of time.
pub fn new(class: &'static Class, thread: &JavaThread) -> Throws<ClassInstanceRef> {
	let cp_class = crate::globals::classes::jdk_internal_reflect_ConstantPool();
	cp_class.initialize(thread)?;

	let instance = ClassInstance::new(cp_class);
	set_constantPoolOop(instance.get_mut(), class);
	Throws::Ok(instance)
}

pub fn constantPoolOop(instance: &ClassInstance) -> &ConstantPool {
	let obj = instance
		.get_field_value0(constantPoolOop_field_offset())
		.expect_reference()
		.extract_mirror();
	obj.get().target_class().constant_pool().expect(
		"jdk.internal.reflect.ConstantPool objects should never be created for array classes",
	)
}

// We don't actually do anything special here. The JDK allows us to put anything here, but we can simply
// put a mirror, since the field is only ever used by native methods.
fn set_constantPoolOop(instance: &mut ClassInstance, class: &'static Class) {
	instance.put_field_value0(
		constantPoolOop_field_offset(),
		Operand::Reference(Reference::mirror(class.mirror())),
	);
}

super::field_module! {
	@CLASS jdk_internal_reflect_ConstantPool;

	@FIELDSTART
	/// `jdk.internal.reflect.ConstantPool#constantPoolOop` field offset
	///
	/// Expected type: `Reference`
	@FIELD constantPoolOop: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Object"),
}
