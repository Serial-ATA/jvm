mod accessors;
mod error;
mod method;
mod type_system;

use crate::class::Class;
use crate::verifier::accessors::ClassAccessorExt;
use crate::verifier::method::MethodTypeCheckExt;
use error::{Error, Result};

use symbols::sym;

// classIsTypeSafe(Class) :-
//     classClassName(Class, Name),
//     classDefiningLoader(Class, L),
//     superclassChain(Name, L, Chain),
//     Chain \= [],
//     classSuperClassName(Class, SuperclassName),
//     loadedClass(SuperclassName, L, Superclass),
//     classIsNotFinal(Superclass),
//     classMethods(Class, Methods),
//     checklist(methodIsTypeSafe(Class), Methods).
pub fn class_is_type_safe(class: &Class) -> Result<()> {
	let name = class.class_name();

	tracing::debug!("Verifying class `{}`", name.as_str());

	if name == sym!(java_lang_Object) {
		return object_class_is_type_safe(class);
	}

	let loader = class.defining_loader();
	let super_class = class
		.super_class
		.as_ref()
		.expect("super class should exist");
	if !super_class.is_not_final() {
		return Err(Error::SuperClassFinal);
	}

	for method in class.methods() {
		method.is_type_safe()?;
	}

	Ok(())
}

// classIsTypeSafe(Class) :-
//     classClassName(Class, 'java/lang/Object'),
//     classDefiningLoader(Class, L),
//     isBootstrapLoader(L),
//     classMethods(Class, Methods),
//     checklist(methodIsTypeSafe(Class), Methods).
fn object_class_is_type_safe(class: &Class) -> Result<()> {
	let loader = class.defining_loader();
	todo!("isBootstrapLoader");
	for method in class.methods() {
		method.is_type_safe()?;
	}

	Ok(())
}
