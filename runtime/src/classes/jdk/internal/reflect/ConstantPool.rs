use crate::objects::class::ClassPtr;
use crate::objects::constant_pool::ConstantPool;
use crate::objects::instance::Instance;
use crate::objects::instance::class::{ClassInstance, ClassInstanceRef};
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::Throws;

use classfile::FieldType;
use instructions::Operand;

/// Create a new instance of `jdk.internal.reflect.ConstantPool` for the given class' constant pool
///
/// NOTE: This takes a thread, as `jdk.internal.reflect.ConstantPool` is not initialized ahead of time.
pub fn new(class: ClassPtr, thread: &'static JavaThread) -> Throws<ClassInstanceRef> {
	let cp_class = crate::globals::classes::jdk_internal_reflect_ConstantPool();
	cp_class.initialize(thread)?;

	let instance = ClassInstance::new(cp_class);
	set_constantPoolOop(instance, class);
	Throws::Ok(instance)
}

pub fn constantPoolOop(instance: ClassInstanceRef) -> &'static ConstantPool {
	let obj = instance
		.get_field_value0(constantPoolOop_field_index())
		.expect_reference()
		.extract_mirror();
	obj.target_class().constant_pool().expect(
		"jdk.internal.reflect.ConstantPool objects should never be created for array classes",
	)
}

// We don't actually do anything special here. The JDK allows us to put anything here, but we can simply
// put a mirror, since the field is only ever used by native methods.
fn set_constantPoolOop(instance: ClassInstanceRef, class: ClassPtr) {
	instance.put_field_value0(
		constantPoolOop_field_index(),
		Operand::Reference(Reference::mirror(class.mirror())),
	);
}

crate::classes::field_module! {
	@CLASS jdk_internal_reflect_ConstantPool;

	@FIELDSTART
	/// Injected mirror for the target class
	@INJECTED constantPoolOop: FieldType::Object((*b"java/lang/Object").into()) => jni::sys::jobject,
}
