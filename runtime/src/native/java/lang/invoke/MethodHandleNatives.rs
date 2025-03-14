use crate::native::java::lang::invoke::MethodHandleNatives;
use crate::native::java::lang::String::{rust_string_from_java_string, StringInterner};
use crate::objects::class::Class;
use crate::objects::class_instance::ClassInstance;
use crate::objects::reference::{ClassInstanceRef, MirrorInstanceRef, Reference};
use crate::symbols::{sym, Symbol};
use crate::thread::exceptions::{handle_exception, throw, throw_and_return_null, Throws};
use crate::thread::JavaThread;
use crate::{classes, globals};

use std::fmt::Write;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};
use classfile::accessflags::FieldAccessFlags;
use classfile::constant_pool::types::ReferenceKind;
use common::traits::PtrType;

include_generated!("native/java/lang/invoke/def/MethodHandleNatives.registerNatives.rs");
include_generated!("native/java/lang/invoke/def/MethodHandleNatives.definitions.rs");
include_generated!("native/java/lang/invoke/def/MethodHandleNatives$Constants.constants.rs");

pub fn new_member_name(
	name: Symbol,
	descriptor: Symbol,
	callee_class: &'static Class,
) -> Throws<ClassInstanceRef> {
	let member_name_instance = ClassInstance::new(globals::classes::java_lang_invoke_MemberName());

	let member_name = member_name_instance.get_mut();

	classes::java_lang_invoke_MemberName::set_clazz(
		member_name,
		Reference::mirror(callee_class.mirror()),
	);
	classes::java_lang_invoke_MemberName::set_name(
		member_name,
		Reference::class(StringInterner::intern(name)),
	);
	// TODO: Not correct for field members
	classes::java_lang_invoke_MemberName::set_type(
		member_name,
		Reference::class(StringInterner::intern(descriptor)),
	);

	Throws::Ok(member_name_instance)
}

pub fn method_type_signature(method_type: Reference) -> Throws<Symbol> {
	if !method_type.is_instance_of(globals::classes::java_lang_invoke_MethodType()) {
		throw!(@DEFER InternalError, "not a MethodType");
	}

	let method_type = method_type.extract_class();

	let mut signature = String::new();
	signature.push('(');

	let parameters = classes::java_lang_invoke_MethodType::ptypes(&method_type.get());
	for param in parameters.get().as_slice() {
		if param.is_null() {
			signature.push_str("null");
			continue;
		}

		let mirror = param.extract_mirror();
		if mirror.get().is_primitive() {
			signature.push_str(&mirror.get().primitive_target().as_signature());
			continue;
		}

		if write!(signature, "{}", mirror.get().target_class().as_signature()).is_err() {
			throw!(@DEFER InternalError, "writing signature");
		}
	}

	signature.push(')');

	let return_type = classes::java_lang_invoke_MethodType::rtype(&method_type.get());

	if return_type.is_null() {
		signature.push_str("null");
	} else {
		let mirror_instance = return_type.extract_mirror();
		let mirror = mirror_instance.get();

		let result;
		if mirror.is_primitive() {
			result = write!(signature, "{}", mirror.primitive_target().as_signature());
		} else {
			result = write!(signature, "{}", mirror.target_class().as_signature());
		}

		if result.is_err() {
			throw!(@DEFER InternalError, "writing signature");
		}
	}

	Throws::Ok(Symbol::intern(signature))
}

pub fn resolve_member_name(
	member_name: &mut ClassInstance,
	ref_kind: ReferenceKind,
	calling_class: Option<&'static Class>,
	lookup_mode: jint,
) -> Throws<()> {
	if calling_class.is_none() {
		assert!(
			(lookup_mode & LM_TRUSTED) == LM_TRUSTED,
			"untrusted member resolution requires a calling class"
		);
	}

	let mut is_valid = true;
	let mut flags;

	let defining_class_field = classes::java_lang_invoke_MemberName::clazz(member_name)?;
	if defining_class_field.get().is_primitive() {
		throw!(@DEFER InternalError, "primitive class");
	}

	let defining_class = defining_class_field.get().target_class();

	let name_field = classes::java_lang_invoke_MemberName::name(member_name);
	let name_str = rust_string_from_java_string(name_field.extract_class());
	let name = Symbol::intern(name_str);

	let type_field = classes::java_lang_invoke_MemberName::type_(member_name);

	let descriptor: Symbol;
	if type_field.is_instance_of(globals::classes::java_lang_String()) {
		let descriptor_str = rust_string_from_java_string(type_field.extract_class());
		descriptor = Symbol::intern(descriptor_str);
	} else if type_field.is_instance_of(globals::classes::java_lang_Class()) {
		descriptor = type_field.extract_target_class().as_signature();
	} else if type_field.is_instance_of(globals::classes::java_lang_invoke_MethodType()) {
		descriptor = method_type_signature(type_field)?;
	} else {
		throw!(@DEFER InternalError, "unrecognized field");
	}

	match ref_kind {
		ReferenceKind::GetField
		| ReferenceKind::GetStatic
		| ReferenceKind::PutField
		| ReferenceKind::PutStatic => {
			// Already default initialized to `null`, just being explicit
			classes::java_lang_invoke_MemberName::set_method(member_name, Reference::null());

			let field = defining_class.resolve_field(name, descriptor)?;

			flags = field.access_flags.as_u2() as jint;
			flags |= MethodHandleNatives::MN_IS_FIELD;
			flags |= (ref_kind as jint) << MethodHandleNatives::MN_REFERENCE_KIND_SHIFT;

			if field.is_trusted_final() {
				flags |= MethodHandleNatives::MN_TRUSTED_FINAL;
			}

			match ref_kind {
				ReferenceKind::GetField | ReferenceKind::PutField => {
					// TODO: This isn't the full check
					is_valid = !field.is_static();
				},
				ReferenceKind::GetStatic | ReferenceKind::PutStatic => {
					// TODO: This isn't the full check
					is_valid = field.is_static();
				},
				_ => unreachable!(),
			}

			if !is_valid {
				throw!(@DEFER IllegalAccessError);
			}

			classes::java_lang_invoke_MemberName::set_vmindex(member_name, field.index() as jlong);
			classes::java_lang_invoke_MemberName::set_clazz(
				member_name,
				Reference::mirror(field.class.mirror()),
			);

			classes::java_lang_invoke_MemberName::set_name(
				member_name,
				Reference::class(StringInterner::intern(field.name)),
			);
			classes::java_lang_invoke_MemberName::set_type(
				member_name,
				Reference::class(StringInterner::intern(&*field.descriptor.as_signature())),
			);
		},
		ReferenceKind::InvokeVirtual
		| ReferenceKind::NewInvokeSpecial
		| ReferenceKind::InvokeStatic
		| ReferenceKind::InvokeSpecial
		| ReferenceKind::InvokeInterface => {
			let method = if ref_kind == ReferenceKind::InvokeInterface {
				defining_class.resolve_interface_method(name, descriptor)?
			} else {
				defining_class.resolve_method(name, descriptor)?
			};

			flags = method.access_flags.as_u2() as jint;
			flags |= MethodHandleNatives::MN_IS_METHOD;
			flags |= (ref_kind as jint) << MethodHandleNatives::MN_REFERENCE_KIND_SHIFT;

			if let Some(calling_class) = calling_class {
				match ref_kind {
					ReferenceKind::InvokeSpecial => {
						is_valid = !method.is_static()
							&& (method.class() == calling_class
								|| calling_class
									.parent_iter()
									.any(|super_class| super_class == calling_class)
								|| calling_class
									.interfaces
									.iter()
									.any(|interface| *interface == defining_class)
								|| method.class() == globals::classes::java_lang_Object());
					},
					ReferenceKind::NewInvokeSpecial => {
						flags |= MethodHandleNatives::MN_IS_CONSTRUCTOR;

						is_valid = method.name == sym!(object_initializer_name);
						if method.is_protected() {
							is_valid &= method.class().shares_package_with(calling_class);
						}
					},
					ReferenceKind::InvokeStatic => {
						is_valid = method.is_static();
					},
					ReferenceKind::InvokeVirtual => {
						if method.is_protected()
							&& !method.class().shares_package_with(calling_class)
						{
							is_valid = method.class().is_subclass_of(calling_class);
						}
					},
					ReferenceKind::InvokeInterface => {
						is_valid = !method.is_static();
					},
					_ => unreachable!(),
				}
			}

			if !is_valid {
				throw!(@DEFER IllegalAccessError);
			}

			if method.is_caller_sensitive() {
				flags |= MethodHandleNatives::MN_CALLER_SENSITIVE;
			}

			// Create the java.lang.invoke.ResolvedMethodName instance
			let resolved_method_name =
				ClassInstance::new(globals::classes::java_lang_invoke_ResolvedMethodName());

			classes::java_lang_invoke_ResolvedMethodName::set_vmholder(
				resolved_method_name.get_mut(),
				method.class().mirror(),
			);
			classes::java_lang_invoke_ResolvedMethodName::set_vmtarget(
				resolved_method_name.get_mut(),
				method,
			);

			classes::java_lang_invoke_MemberName::set_method(
				member_name,
				Reference::class(resolved_method_name),
			);

			let vmindex = defining_class
				.vtable()
				.iter()
				.position(|m| m == method)
				.expect("method must exist in vtable");
			classes::java_lang_invoke_MemberName::set_vmindex(member_name, vmindex as jlong);

			classes::java_lang_invoke_MemberName::set_clazz(
				member_name,
				Reference::mirror(method.class().mirror()),
			);
		},
	}

	classes::java_lang_invoke_MemberName::set_flags(member_name, flags);

	Throws::Ok(())
}

// -- MemberName support --

pub fn init(
	_env: JniEnv,
	_class: &'static Class,
	_self_: Reference, // java.lang.invoke.MemberName
	_ref_: Reference,  // java.lang.Object
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#init");
}

pub fn expand(
	_env: JniEnv,
	_class: &'static Class,
	_self_: Reference, // java.lang.invoke.MemberName
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#expand");
}

// throws LinkageError, ClassNotFoundException
pub fn resolve(
	_env: JniEnv,
	_class: &'static Class,
	self_: Reference,  // java.lang.invoke.MemberName
	caller: Reference, // java.lang.Class<?>
	lookup_mode: jint,
	speculative_resolve: jboolean,
) -> Reference /* java.lang.invoke.MemberName */ {
	if self_.is_null() {
		throw_and_return_null!(JavaThread::current(), NullPointerException);
	}

	let class_instance = self_.extract_class();

	let flags = classes::java_lang_invoke_MemberName::flags(class_instance.get());
	let Some(reference_kind) = ReferenceKind::from_u8((flags >> MN_REFERENCE_KIND_SHIFT) as u8)
	else {
		throw_and_return_null!(
			JavaThread::current(),
			InternalError,
			"obsolete MemberName format"
		);
	};

	// `LM_TRUSTED` implies a `null` `calling_class`
	let calling_class;
	if (lookup_mode & LM_TRUSTED) == LM_TRUSTED {
		calling_class = None;
	} else {
		calling_class = Some(caller.extract_target_class());
	}

	if let Throws::Exception(_e) = resolve_member_name(
		class_instance.get_mut(),
		reference_kind,
		calling_class,
		lookup_mode,
	) {
		if speculative_resolve {
			// Speculative resolution is allowed to fail
			return Reference::null();
		}

		if reference_kind.is_field() {
			throw_and_return_null!(
				JavaThread::current(),
				NoSuchFieldError,
				"field resolution failed"
			);
		} else {
			_e.throw(JavaThread::current());
			return Reference::null();
			// throw_and_return_null!(
			// 	JavaThread::current(),
			// 	NoSuchMethodError,
			// 	"method resolution failed"
			// );
		}
	}

	self_
}

// -- Field layout queries parallel to jdk.internal.misc.Unsafe --

fn find_member_offset(self_: Reference, is_static: bool) -> Throws<(jlong, MirrorInstanceRef)> {
	if self_.is_null() {
		throw!(@DEFER InternalError, "mname not resolved")
	}

	let clazz = classes::java_lang_invoke_MemberName::clazz(self_.extract_class().get())?;
	let flags = classes::java_lang_invoke_MemberName::flags(self_.extract_class().get());

	let acc_static = FieldAccessFlags::ACC_STATIC.as_u2() as jint;
	if flags & (MN_IS_FIELD as jint) != 0
		&& ((is_static && flags & acc_static != 0) || (!is_static && flags & acc_static == 0))
	{
		return Throws::Ok((
			classes::java_lang_invoke_MemberName::vmindex(self_.extract_class().get()) as jlong,
			clazz,
		));
	}

	if is_static {
		throw!(@DEFER InternalError, "static field required");
	} else {
		throw!(@DEFER InternalError, "non-static field required");
	}
}

pub fn objectFieldOffset(
	env: JniEnv,
	_class: &'static Class,
	self_: Reference, // java.lang.invoke.MemberName
) -> jlong {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	let (index, _) = handle_exception!(0, thread, find_member_offset(self_, false));
	index
}

pub fn staticFieldOffset(
	env: JniEnv,
	_class: &'static Class,
	self_: Reference, // java.lang.invoke.MemberName
) -> jlong {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	let (index, _) = handle_exception!(0, thread, find_member_offset(self_, true));
	index
}

pub fn staticFieldBase(
	env: JniEnv,
	_class: &'static Class,
	self_: Reference, // java.lang.invoke.MemberName
) -> Reference /* java.lang.Object */ {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	let (_, clazz) = handle_exception!(Reference::null(), thread, find_member_offset(self_, true));
	Reference::mirror(clazz)
}

pub fn getMemberVMInfo(
	_env: JniEnv,
	_class: &'static Class,
	_self_: Reference, // java.lang.invoke.MemberName
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandleNatives#getMemberVMInfo");
}

// -- CallSite support --

pub fn setCallSiteTargetNormal(
	_env: JniEnv,
	_class: &'static Class,
	_site: Reference,   // java.lang.invoke.CallSite
	_target: Reference, // java.lang.invoke.MethodHandle
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#setCallSiteTargetNormal");
}

pub fn setCallSiteTargetVolatile(
	_env: JniEnv,
	_class: &'static Class,
	_site: Reference,   // java.lang.invoke.CallSite
	_target: Reference, // java.lang.invoke.MethodHandle
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#setCallSiteTargetVolatile");
}

pub fn copyOutBootstrapArguments(
	_env: JniEnv,
	_class: &'static Class,
	_caller: Reference,     // java.lang.Class<?>
	_index_info: Reference, // int[]
	_start: jint,
	_end: jint,
	_buf: Reference, // java.lang.Object[]
	_pos: jint,
	_resolve: jboolean,
	_if_not_available: Reference, // java.lang.Object
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#copyOutBootstrapArguments");
}

pub fn clearCallSiteContext(
	_env: JniEnv,
	_class: &'static Class,
	_context: Reference, // java.lang.invoke.CallSiteContext
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#clearCallSiteContext");
}

pub fn getNamedCon(
	_env: JniEnv,
	_class: &'static Class,
	_which: jint,
	_name: Reference, // java.lang.Object[]
) -> jint {
	unimplemented!("java.lang.invoke.MethodHandleNatives#getNamedCon");
}
